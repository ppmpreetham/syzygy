use gpui_kit::http_client::Request;
use http_body_util::Full;
use http_mitm_proxy::hyper::body::Bytes;
use http_mitm_proxy::hyper::header::{self, TRANSFER_ENCODING};
use http_mitm_proxy::hyper::{self, Version};

pub fn parse_version(version: &str) -> Option<Version> {
    match version {
        "HTTP/0.9" => Some(hyper::Version::HTTP_09),
        "HTTP/1.0" => Some(hyper::Version::HTTP_10),
        "HTTP/1.1" => Some(hyper::Version::HTTP_11),
        "HTTP/2" | "HTTP/2.0" => Some(hyper::Version::HTTP_2),
        _ => None,
    }
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
    let version = parse_version(request_line.next()?)?;
    let mut builder = Request::builder().method(method).uri(uri).version(version);
    for line in lines {
        let (name, value) = line.split_once(':')?;
        builder = builder.header(name.trim(), value.trim());
    }
    let mut request = builder
        .body(Full::new(Bytes::copy_from_slice(body.as_bytes())))
        .ok()?;
    request.headers_mut().remove(TRANSFER_ENCODING);
    request
        .headers_mut()
        .insert(header::CONTENT_LENGTH, body.len().into());
    Some(request)
}
