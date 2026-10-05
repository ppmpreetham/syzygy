use std::path::Path;

use gpui_kit::accesskit::Uuid;
use gpui_kit::http_client::Request;
use http_mitm_proxy::hyper::body::Incoming;
use http_mitm_proxy::hyper::http::header::COOKIE;
use http_mitm_proxy::hyper::http::{HeaderValue, Method};
use std::net::{IpAddr, Ipv4Addr};
use std::time::SystemTime;
use wry::http;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressType {
    HTTP,
    WS,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirn {
    In,
    Out,
}

/// The table how it looks there
#[derive(Clone, PartialEq)]
pub struct RequestRow {
    pub id: Uuid,
    pub time: SystemTime,
    pub addr_type: AddressType,
    pub dirn: Dirn,
    pub method: Method,
    pub host: String,
    // evreything after the host
    pub uri: http::Uri,
    pub status: Option<http::StatusCode>,
    pub length: usize,
    pub tls: bool,
    // TODO: idk how to implement the ip yet, currently so nvm
    pub ip: IpAddr,
    // TODO: this aswell
    pub port: u16,
    // TODO: this aswell
    pub mime: String,
    pub extension: Option<String>,
    pub title: Option<String>,
    pub cookies: Vec<HeaderValue>,
    pub start_response_timer: Option<SystemTime>,
    pub end_response_timer: Option<SystemTime>,
    pub websocket_id: Option<u64>,
    pub notes: String,
}

pub struct FullRequest {
    pub forwarded: bool,
    pub row: RequestRow,
    pub parts: http::request::Parts,
    pub body: Incoming,
}

pub fn row_converter(req: Request<Incoming>) -> FullRequest {
    // TODO: make this global later
    let (parts, body) = req.into_parts();

    let host = String::from(parts.uri.host().unwrap_or("localhost"));
    let tls = matches!(parts.uri.scheme_str(), Some("https" | "wss"));
    let extension = Path::new(parts.uri.path())
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_string());
    let addr_type =
        if let Some("websocket") = parts.headers.get("upgrade").and_then(|v| v.to_str().ok()) {
            AddressType::WS
        } else {
            AddressType::HTTP
        };

    let cookies = parts
        .headers
        .get_all(COOKIE)
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    let row = RequestRow {
        id: Uuid::new_v4(),
        time: SystemTime::now(),
        method: parts.method.clone(),
        addr_type,
        dirn: Dirn::In,
        host,
        uri: parts.uri.clone(),
        status: None,
        length: 0,
        tls,
        // TODO: idk how to implement the ip yet, currently so nvm
        ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        // TODO: idk how to implement the ip yet, currently so nvm
        port: 8080,
        mime: String::new(),
        extension,
        title: None,
        cookies,
        start_response_timer: Some(SystemTime::now()),
        end_response_timer: None,
        websocket_id: None,
        notes: String::new(),
    };

    FullRequest {
        forwarded: false,
        row,
        parts,
        body,
    }
}
