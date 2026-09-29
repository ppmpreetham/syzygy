use http_mitm_proxy::{DefaultClient, MitmProxy, hyper::{Method, service::service_fn}, moka::sync::Cache};
use std::sync::Arc;
use tokio::sync::mpsc;
use http_mitm_proxy::default_client::Error as MITMError;

#[derive(Debug, Clone)]
pub struct TrafficEvent {
    pub method: Method,
    pub uri: String,
    pub status: u16,
}

p

pub async fn start_proxy() {
    // send and come back from ui
    let (tx, mut rx) = mpsc::unbounded_channel::<TrafficEvent>();

    let client = Arc::new(DefaultClient::new());

    // TODO
    let root_issuer = todo!("load / gen rcgen CA certificate issuer");
    let proxy = MitmProxy::new(Some(root_issuer), Some(Cache::new(256)));

    let server = proxy.bind(("127.0.0.1", 8080), service_fn({
        let client = Arc::clone(&client);
        let tx = tx.clone();

        move |req| {
            let client = Arc::clone(&client);
            let tx = tx.clone();

            async move {
                let uri = req.uri().clone();
                let method = req.method().clone();

                // INTERCEPTION POINT

                let (res, _upgrade) = client.send_request(req).await?;

                // RESPONSE INTERCEPTION POINT
                let status = res.status().as_u16();

                _ = tx.send(TrafficEvent {
                    method,
                    uri: uri.to_string(),
                    status,
                });
                Ok::<_, MITMError>(res)
            }
        }
    })).await.unwrap();

    tokio::spawn(server);
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            println!("UI Caught Traffic: {} {} -> {}", event.method, event.uri, event.status);
        }
    });
}
