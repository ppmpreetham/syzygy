use std::sync::Arc;

use crate::backend::proxy::certificate::certificate_issuer;
use gpui_kit::http_client::Response;
use http_body_util::Full;
use http_mitm_proxy::hyper::body::Bytes;
use http_mitm_proxy::hyper::service::service_fn;
use http_mitm_proxy::moka::sync::Cache;
use http_mitm_proxy::{DefaultClient, MitmProxy};

use super::interception::catcher;
use super::state::ProxyState;

pub fn bad_gateway() -> Response<Full<Bytes>> {
    Response::builder()
        .status(502)
        .body(Full::new(Bytes::new()))
        .unwrap()
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
