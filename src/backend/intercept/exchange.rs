use gpui_kit::http_client::{Request, Response};
use http_body_util::Full;
use http_mitm_proxy::hyper::body::Bytes;

use crate::backend::uimeta::RequestRow;

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
    pub request: Request<Full<Bytes>>,
    pub response: Option<Response<Full<Bytes>>>,
    pub status: Status,
    pub row: RequestRow,
}
