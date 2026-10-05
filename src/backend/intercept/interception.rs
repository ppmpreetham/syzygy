use std::convert::Infallible;
use std::sync::Arc;
use std::sync::atomic::{Ordering};

use anyhow::Result;
use gpui_kit::http_client::{Request, Response};
use http_body_util::{BodyExt, Full};
use http_mitm_proxy::hyper::body::{Bytes, Incoming};
use http_mitm_proxy::{DefaultClient};
use tokio::sync::{oneshot};
use crate::backend::uimeta::row_converter;
use super::exchange::{Decision, Exchange, Status};
use super::state::{ProxyEvent, ProxyState};
use super::server::bad_gateway;

pub async fn catcher(
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
