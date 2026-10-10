use super::FLUSH_EVERY;
use super::exchange::{Decision, Exchange, Status};
use crate::backend::RequestRow;
use crate::backend::storage::disk::Db;
use crate::globals::{CLEAR_HTTP_HISTORY_WHEN_NO_BROWSER, PERSIST_HTTP_HISTORY_DATA};
use gpui_kit::gpui::SharedString;
use gpui_kit::http_client::{Request, Response};
use http_body_util::Full;
use http_mitm_proxy::hyper::StatusCode;
use http_mitm_proxy::hyper::body::Bytes;
use http_mitm_proxy::hyper::header;
use std::collections::BTreeMap;
use std::mem::take;
use std::ops;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::SystemTime;
use tokio::sync::{broadcast, oneshot::Sender};

#[derive(Clone)]
pub enum ProxyEvent {
    Intercepted(usize, RequestRow),
    History(usize, Status),
}

type Pending = StdMutex<BTreeMap<usize, (Request<Full<Bytes>>, RequestRow, Sender<Decision>)>>;

// TODO: make this store on disk if persistance is on
pub struct ProxyState {
    // intercepted or not
    pub intercepted: AtomicBool,
    // fencing token
    pub index: AtomicUsize,
    // the entire history, i guess? (cool video tho ngl)
    pub history: StdMutex<Vec<Exchange>>,
    // index on history, the diff (need to find a good datatype here)
    // ones that are intercepted, with the request so the ui can show it
    pub pending: Pending,
    pub event_tx: broadcast::Sender<ProxyEvent>,

    // persistance
    pub disk: Option<Db>,
    pub flush_cursor: AtomicUsize,
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
            // TODO: remove this unwrap later
            disk: Db::new().ok(),
            flush_cursor: AtomicUsize::new(0),
        })
    }

    pub fn set_intercept(&self, enabled: bool) {
        let mut pending = self.pending.lock().unwrap();
        self.intercepted.store(enabled, Ordering::Release);
        // turning it off must not strand whatever is still parked
        if !enabled {
            for (_, (_, _, tx)) in take(&mut *pending) {
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
        self.pending.lock().unwrap().get(&id).map(|(request, ..)| {
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

    pub fn history_count(&self) -> usize {
        self.history.lock().unwrap().len()
    }

    pub fn history_slice(&self, range: ops::Range<usize>) -> Vec<(usize, RequestRow)> {
        let history = self.history.lock().unwrap();
        let start = range.start.min(history.len());
        let end = range.end.min(history.len());
        history[start..end]
            .iter()
            .enumerate()
            .map(|(i, exchange)| (start + i, exchange.row.clone()))
            .collect()
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
        for (_, (_, _, tx)) in take(&mut *pending) {
            tx.send(Decision::Drop).ok();
        }
    }

    pub fn set_status(&self, index: usize, status: Status) {
        {
            let mut history = self.history.lock().unwrap();
            history[index].status = status;
            if status == Status::Dropped {
                history[index].row.status = Some(StatusCode::NO_CONTENT);
            } else if status == Status::Failed {
                history[index].row.status = Some(StatusCode::BAD_GATEWAY);
            }
        }
        self.event_tx.send(ProxyEvent::History(index, status)).ok();
        self.maybe_flush();
    }

    pub fn set_request(&self, index: usize, request: Request<Full<Bytes>>) {
        self.history.lock().unwrap()[index].request = request;
    }

    pub fn set_response(&self, index: usize, response: Response<Full<Bytes>>) {
        {
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

            let mime = SharedString::from(
                response
                    .headers()
                    .get(header::CONTENT_TYPE)
                    .and_then(|value| value.to_str().ok())
                    .unwrap_or_default()
                    .to_string(),
            );
            let row = &mut history[index].row;
            row.status = Some(status);
            row.length = length;
            row.end_response_timer = Some(SystemTime::now());
            row.mime = mime;
        }

        self.event_tx
            .send(ProxyEvent::History(index, Status::Done))
            .ok();
        self.maybe_flush();
    }

    fn maybe_flush(&self) {
        let cursor = self.flush_cursor.load(Ordering::Relaxed);
        let items_to_flush = {
            let history = self.history.lock().unwrap();
            if history.len() - cursor >= FLUSH_EVERY {
                Some(history[cursor..].to_vec())
            } else {
                None
            }
        };

        if let Some(items) = items_to_flush
            && let Some(db) = &self.disk
        {
            if let Err(e) = db.flush(&items, cursor) {
                eprintln!("history flush failed: {e}");
                return;
            }
            self.flush_cursor
                .store(cursor + items.len(), Ordering::Relaxed);
        }
    }

    pub fn clear_history_on_browser_close(&self) {
        if CLEAR_HTTP_HISTORY_WHEN_NO_BROWSER {
            // clear ram history
            self.history.lock().unwrap().clear();
            self.flush_cursor.store(0, Ordering::Relaxed);

            // if persistence is off, wipe the disk too bwahaha
            if !PERSIST_HTTP_HISTORY_DATA && let Some(db) = &self.disk {
                db.clear_history().ok();
            }
            self.event_tx
                .send(ProxyEvent::History(0, Status::Dropped))
                .ok();
        }
    }
}
