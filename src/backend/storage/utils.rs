use std::time::{Duration, SystemTime, UNIX_EPOCH};
use http_mitm_proxy::hyper::http;
use http_body_util::Full;
use http_mitm_proxy::hyper::body::Bytes;

pub(super) fn ts(t: SystemTime) -> (u64, u32) {
    let d = t.duration_since(UNIX_EPOCH).unwrap_or_default();
    (d.as_secs(), d.subsec_nanos())
}

pub(super) fn un_ts(t: Option<(u64, u32)>) -> Option<SystemTime> {
    t.map(|(s, n)| UNIX_EPOCH + Duration::new(s, n))
}

pub(super) fn hdrs(h: &http::HeaderMap) -> Vec<(String, Vec<u8>)> {
    h.iter().map(|(k, v)| (k.as_str().to_string(), v.as_bytes().to_vec())).collect()
}

pub(super) fn body_bytes(b: &Full<Bytes>) -> Vec<u8> {
    b.clone().into_inner().unwrap_or_default().to_vec()
}
