use super::super::intercept::exchange::{Exchange, Status};
use crate::backend::uimeta::{AddressType, Dirn, RequestRow};
use gpui_kit::accesskit::Uuid;
use gpui_kit::gpui::SharedString;
use gpui_kit::http_client::{Request, Response};
use http_body_util::Full;
use http_mitm_proxy::hyper::body::Bytes;
use http_mitm_proxy::hyper::http;
use super::datastructure::{StoredExchange, StoredRow};
use super::utils::{hdrs, body_bytes, ts, un_ts};

pub(super) fn serialize(ex: &Exchange) -> Vec<u8> {
    let stored = StoredExchange {
        row: StoredRow {
            id: ex.row.id.as_u128(),
            time: ts(ex.row.time),
            addr_type: ex.row.addr_type as u8,
            dirn: ex.row.dirn as u8,
            method: ex.row.method.as_str().to_string(),
            host: ex.row.host.to_string(),
            uri: ex.row.uri.to_string(),
            status: ex.row.status.map(|s| s.as_u16()),
            length: ex.row.length as u64,
            tls: ex.row.tls,
            ip: ex.row.ip.to_string(),
            port: ex.row.port,
            mime: ex.row.mime.to_string(),
            extension: ex.row.extension.as_ref().map(|s| s.to_string()),
            title: ex.row.title.as_ref().map(|s| s.to_string()),
            cookies: ex.row.cookies.iter().map(|c| c.as_bytes().to_vec()).collect(),
            start_response_timer: ex.row.start_response_timer.map(ts),
            end_response_timer: ex.row.end_response_timer.map(ts),
            websocket_id: ex.row.websocket_id,
            notes: ex.row.notes.to_string(),
        },
        status: ex.status as u8,
        req_headers: hdrs(ex.request.headers()),
        req_body: body_bytes(ex.request.body()),
        res_headers: ex.response.as_ref().map(|r| hdrs(r.headers())),
        res_body: ex.response.as_ref().map(|r| body_bytes(r.body())),
    };
    rkyv::to_bytes::<rkyv::rancor::Error>(&stored)
        .expect("serialize cannot fail")
        .to_vec()
}

pub(super) fn deserialize(bytes: &[u8]) -> Option<Exchange> {
    let s: StoredExchange =
        rkyv::from_bytes::<StoredExchange, rkyv::rancor::Error>(bytes).ok()?;

    let mut req = Request::builder().method(s.row.method.as_str()).uri(s.row.uri.as_str());
    for (k, v) in &s.req_headers {
        req = req.header(k.as_str(), http::HeaderValue::from_bytes(v).ok()?);
    }
    let request = req.body(Full::new(Bytes::from(s.req_body))).ok()?;

    let response = s.res_headers.map(|hs| {
        let mut r = Response::builder();
        for (k, v) in &hs {
            r = r.header(k.as_str(), http::HeaderValue::from_bytes(v).ok()?);
        }
        r.body(Full::new(Bytes::from(s.res_body.unwrap_or_default()))).ok()
    }).flatten();

    let row = RequestRow {
        id: Uuid::from_u128(s.row.id),
        time: un_ts(Some(s.row.time))?,
        addr_type: if s.row.addr_type == 0 { AddressType::HTTP } else { AddressType::WS },
        dirn: if s.row.dirn == 0 { Dirn::In } else { Dirn::Out },
        method: http::Method::from_bytes(s.row.method.as_bytes()).ok()?,
        host: SharedString::from(s.row.host),
        uri: s.row.uri.parse().ok()?,
        status: s.row.status.and_then(|c| http::StatusCode::from_u16(c).ok()),
        length: s.row.length as usize,
        tls: s.row.tls,
        ip: s.row.ip.parse().ok()?,
        port: s.row.port,
        mime: SharedString::from(s.row.mime),
        extension: s.row.extension.map(SharedString::from),
        title: s.row.title.map(SharedString::from),
        cookies: s.row.cookies.iter()
            .filter_map(|c| http::HeaderValue::from_bytes(c).ok()).collect(),
        start_response_timer: un_ts(s.row.start_response_timer),
        end_response_timer: un_ts(s.row.end_response_timer),
        websocket_id: s.row.websocket_id,
        notes: SharedString::from(s.row.notes),
    };

    let status = match s.status {
        0 => Status::Intercepted,
        1 => Status::Pending,
        2 => Status::Dropped,
        3 => Status::Failed,
        _ => Status::Done,
    };

    Some(Exchange { request, response, status, row })
}
