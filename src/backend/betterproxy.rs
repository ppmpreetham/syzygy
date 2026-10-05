use std::collections::BTreeMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use anyhow::Result;
use gpui_kit::http_client::{Request, Response};
use http_body_util::{BodyExt, Full};
use http_mitm_proxy::hyper::body::{Bytes, Incoming};
use http_mitm_proxy::hyper::service::service_fn;
use http_mitm_proxy::moka::sync::Cache;
use http_mitm_proxy::{DefaultClient, MitmProxy};
use tokio::sync::{broadcast, oneshot};

use super::certificate::certificate_issuer;
use super::uimeta::{RequestRow, row_converter};

/// either we stop the request or we forward it, via ui
pub enum Decision {
    /// when it's dropped
    Drop,
    /// when it's forwarded, optionally with an edited request to send instead
    Forward(Option<Request<Full<Bytes>>>),
}

/// where an exchange currently is in its lifecycle
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// waiting for the user to forward or drop it
    Intercepted,
    /// forwarded, waiting for the upstream response
    Pending,
    /// the user dropped it
    Dropped,
    /// upstream request or body failed
    Failed,
    /// got a response
    Done,
}

/// things we store in HTTP history
pub struct Exchange {
    request: Request<Full<Bytes>>,
    response: Option<Response<Full<Bytes>>>,
    status: Status,
    row: RequestRow,
}

// TODO: make this store on disk if persistance is on
pub struct ProxyState {
    // intercepted or not
    intercepted: AtomicBool,
    // fencing token
    index: AtomicUsize,
    // the entire history, i guess? (cool video tho ngl)
    history: StdMutex<Vec<Exchange>>,
    // index on history, the diff (need to find a good datatype here)
    // ones that are intercepted, with the request so the ui can show it
    pending:
        StdMutex<BTreeMap<usize, (Request<Full<Bytes>>, RequestRow, oneshot::Sender<Decision>)>>,
    pub event_tx: broadcast::Sender<ProxyEvent>,
}

pub fn parse_request(raw: &str) -> Option<Request<Full<Bytes>>> {
    let (head, body) = raw
        .split_once("\r\n\r\n")
        .or_else(|| raw.split_once("\n\n"))
        .unwrap_or((raw, ""));
    let mut lines = head.lines();
    let mut request_line = lines.next()?.split_whitespace();
    let method = request_line.next()?;
    let uri = request_line.next()?;
    let version = match request_line.next()? {
        "HTTP/0.9" => http_mitm_proxy::hyper::Version::HTTP_09,
        "HTTP/1.0" => http_mitm_proxy::hyper::Version::HTTP_10,
        "HTTP/1.1" => http_mitm_proxy::hyper::Version::HTTP_11,
        "HTTP/2" | "HTTP/2.0" => http_mitm_proxy::hyper::Version::HTTP_2,
        _ => return None,
    };
    let mut builder = Request::builder().method(method).uri(uri).version(version);
    for line in lines {
        let (name, value) = line.split_once(':')?;
        builder = builder.header(name.trim(), value.trim());
    }
    let mut request = builder
        .body(Full::new(Bytes::copy_from_slice(body.as_bytes())))
        .ok()?;
    request
        .headers_mut()
        .remove(http_mitm_proxy::hyper::header::TRANSFER_ENCODING);
    request.headers_mut().insert(
        http_mitm_proxy::hyper::header::CONTENT_LENGTH,
        body.len().into(),
    );
    Some(request)
}

#[derive(Clone)]
pub enum ProxyEvent {
    Intercepted(usize, RequestRow),
    History(usize, Status),
}

impl ProxyState {
    pub fn new() -> Arc<Self> {
        let (event_tx, _) = broadcast::channel(1024);
        Arc::new(Self {
            intercepted: AtomicBool::new(false),
            index: AtomicUsize::new(1),
            history: StdMutex::new(Vec::new()),
            pending: StdMutex::new(BTreeMap::new()),
            event_tx,
        })
    }

    pub fn set_intercept(&self, enabled: bool) {
        let mut pending = self.pending.lock().unwrap();
        self.intercepted.store(enabled, Ordering::Release);
        // turning it off must not strand whatever is still parked
        if !enabled {
            for (_, (_, _, tx)) in std::mem::take(&mut *pending) {
                tx.send(Decision::Forward(None)).ok();
            }
        }
    }

    pub fn intercept_enabled(&self) -> bool {
        self.intercepted.load(Ordering::Acquire)
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ProxyEvent> {
        self.event_tx.subscribe()
    }

    /// snapshot of what the ui should show as intercepted
    pub fn pending_requests(&self) -> Vec<(usize, Request<Full<Bytes>>)> {
        self.pending
            .lock()
            .unwrap()
            .iter()
            .map(|(id, (request, _, _))| (*id, request.clone()))
            .collect()
    }

    pub fn pending_rows(&self) -> Vec<(usize, RequestRow)> {
        self.pending
            .lock()
            .unwrap()
            .iter()
            .map(|(id, (_, row, _))| (*id, row.clone()))
            .collect()
    }

    pub fn pending_request_text(&self, id: usize) -> Option<String> {
        self.pending
            .lock()
            .unwrap()
            .get(&id)
            .map(|(request, _, _)| {
                let headers = request
                    .headers()
                    .iter()
                    .filter_map(|(name, value)| {
                        value
                            .to_str()
                            .ok()
                            .map(|value| format!("{name}: {value}\r\n"))
                    })
                    .collect::<String>();
                let body = request.body().clone().into_inner().unwrap_or_default();
                format!(
                    "{} {} {:?}\r\n{}\r\n{}",
                    request.method(),
                    request.uri(),
                    request.version(),
                    headers,
                    String::from_utf8_lossy(&body)
                )
            })
    }

    pub fn history_rows(&self) -> Vec<(usize, RequestRow)> {
        self.history
            .lock()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(history_idx, exchange)| (history_idx, exchange.row.clone()))
            .collect()
    }

    pub fn forward(&self, id: usize, edited: Option<Request<Full<Bytes>>>) -> bool {
        self.pending
            .lock()
            .unwrap()
            .remove(&id)
            .is_some_and(|(_, _, tx)| tx.send(Decision::Forward(edited)).is_ok())
    }

    pub fn forward_all(&self) {
        self.set_intercept(false);
    }

    pub fn drop_request(&self, id: usize) -> bool {
        self.pending
            .lock()
            .unwrap()
            .remove(&id)
            .is_some_and(|(_, _, tx)| tx.send(Decision::Drop).is_ok())
    }

    pub fn drop_all(&self) {
        let mut pending = self.pending.lock().unwrap();
        self.intercepted.store(false, Ordering::Release);
        for (_, (_, _, tx)) in std::mem::take(&mut *pending) {
            tx.send(Decision::Drop).ok();
        }
    }

    async fn set_status(&self, index: usize, status: Status) {
        let mut history = self.history.lock().unwrap();
        history[index].status = status;
        if status == Status::Dropped {
            history[index].row.status = Some(http_mitm_proxy::hyper::StatusCode::NO_CONTENT);
        } else if status == Status::Failed {
            history[index].row.status = Some(http_mitm_proxy::hyper::StatusCode::BAD_GATEWAY);
        }
        _ = self.event_tx.send(ProxyEvent::History(index, status));
    }

    async fn set_request(&self, index: usize, request: Request<Full<Bytes>>) {
        self.history.lock().unwrap()[index].request = request;
    }

    async fn set_response(&self, index: usize, response: Response<Full<Bytes>>) {
        let mut history = self.history.lock().unwrap();
        history[index].response = Some(response);
        history[index].status = Status::Done;
        let response = history[index].response.as_ref().unwrap();
        let status = response.status();
        let length = response
            .body()
            .clone()
            .into_inner()
            .map_or(0, |body| body.len());
        let mime = response
            .headers()
            .get(http_mitm_proxy::hyper::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default()
            .to_string();
        let row = &mut history[index].row;
        row.status = Some(status);
        row.length = length;
        row.end_response_timer = Some(std::time::SystemTime::now());
        row.mime = mime;
        // Notify UI that a request completed
        _ = self.event_tx.send(ProxyEvent::History(index, Status::Done));
    }
}

fn bad_gateway() -> Response<Full<Bytes>> {
    Response::builder()
        .status(502)
        .body(Full::new(Bytes::new()))
        .unwrap()
}

async fn catcher(
    req: Request<Incoming>,
    client: &DefaultClient,
    state: Arc<ProxyState>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let id = state.index.fetch_add(1, Ordering::Relaxed);
    let converted = row_converter(req);
    let mut row = converted.row;
    let parts = converted.parts;
    let body = converted.body;
    let bytes = match body.collect().await {
        Ok(body) => body.to_bytes(),
        Err(_) => return Ok(bad_gateway()),
    };
    row.length = bytes.len();
    let mut req = Request::from_parts(parts, Full::new(bytes));
    // logged before the decision so dropped requests show up too
    let history_index = {
        let mut history = state.history.lock().unwrap();
        let index = history.len();
        history.push(Exchange {
            request: req.clone(),
            response: None,
            status: Status::Pending,
            row: row.clone(),
        });
        index
    };

    let rx = {
        let mut pending = state.pending.lock().unwrap();
        if state.intercepted.load(Ordering::Acquire) {
            let (tx, rx) = oneshot::channel();
            pending.insert(id, (req.clone(), row.clone(), tx));
            _ = state
                .event_tx
                .send(ProxyEvent::Intercepted(id, row.clone()));
            Some(rx)
        } else {
            None
        }
    };

    if let Some(rx) = rx {
        state.set_status(history_index, Status::Intercepted).await;
        match rx.await {
            Ok(Decision::Forward(edited)) => {
                if let Some(edited) = edited {
                    req = edited;
                    state.set_request(history_index, req.clone()).await;
                }
                state.set_status(history_index, Status::Pending).await;
            }
            Ok(Decision::Drop) | Err(_) => {
                state.set_status(history_index, Status::Dropped).await;
                return Ok(Response::builder()
                    .status(204)
                    .body(Full::new(Bytes::new()))
                    .unwrap());
            }
        }
    }

    let (response, _upgrade) = match client.send_request(req).await {
        Ok(response) => response,
        Err(_) => {
            state.set_status(history_index, Status::Failed).await;
            return Ok(bad_gateway());
        }
    };
    let (parts, body) = response.into_parts();
    let bytes = match body.collect().await {
        Ok(body) => body.to_bytes(),
        Err(_) => {
            state.set_status(history_index, Status::Failed).await;
            return Ok(bad_gateway());
        }
    };
    let history_response = Response::from_parts(parts.clone(), Full::new(bytes.clone()));
    let response = Response::from_parts(parts, Full::new(bytes));
    state.set_response(history_index, history_response).await;
    Ok(response)
}

pub async fn start_proxy(state: Arc<ProxyState>) {
    let client = Arc::new(DefaultClient::new());
    let issuer = certificate_issuer().expect("Failed to initialize Root CA framework");
    let proxy = MitmProxy::new(Some(issuer), Some(Cache::new(256)));
    let server = match proxy
        .bind(
            ("127.0.0.1", 8080),
            service_fn({
                let client = Arc::clone(&client);
                move |req| {
                    let client = Arc::clone(&client);
                    let state = Arc::clone(&state);
                    async move { catcher(req, &client, state).await }
                }
            }),
        )
        .await
    {
        Ok(server) => server,
        Err(error) => {
            eprintln!("Failed to bind proxy server: {error}");
            return;
        }
    };
    server.await;
}
