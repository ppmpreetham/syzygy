use http_mitm_proxy::default_client::Error as MITMError;
use http_mitm_proxy::{
  DefaultClient, MitmProxy,
  hyper::{HeaderMap, Method, service::service_fn},
  moka::sync::Cache,
};
use std::sync::Arc;
use tokio::sync::mpsc;

use super::certificate::certificate_issuer;

#[derive(Debug, Clone)]
pub struct TrafficEvent {
  pub method: Method,
  pub uri: String,
  pub status: u16,
  pub req_headers: HeaderMap,
  pub res_headers: HeaderMap,
}

pub async fn start_proxy() {
  // send and come back from ui
  let (tx, mut rx) = mpsc::unbounded_channel::<TrafficEvent>();
  let client = Arc::new(DefaultClient::new());

  let root_issuer = certificate_issuer().expect("Failed to initialize Root CA framework");
  let proxy = MitmProxy::new(Some(root_issuer), Some(Cache::new(256)));

  let tx = Arc::new(tx);
  let server = proxy
    .bind(
      ("127.0.0.1", 8080),
      service_fn({
        let client = Arc::clone(&client);
        let tx = Arc::clone(&tx);

        move |req| {
          let client = Arc::clone(&client);
          let tx = Arc::clone(&tx);

          async move {
            let method = req.method().clone();
            let uri = req.uri().to_string();
            let req_headers = req.headers().clone();

            // INTERCEPTION POINT
            let (res, upgrade) = client.send_request(req).await?;

            if let Some(u) = upgrade {
              tokio::spawn(async move {
                if let Err(e) = u.await {
                  eprintln!("Upgrade tunnel error: {}", e);
                }
              });
            }

            // RESPONSE INTERCEPTION POINT
            tx.send(TrafficEvent {
              method,
              uri,
              status: res.status().as_u16(),
              req_headers,
              res_headers: res.headers().clone(),
            })
            .ok();

            Ok::<_, MITMError>(res)
          }
        }
      }),
    )
    .await
    .unwrap();

  tokio::spawn(server);
  tokio::spawn(async move {
    while let Some(event) = rx.recv().await {
      println!(
        "\nTraffic Loop {} {} -> {}",
        event.method, event.uri, event.status
      );

      for (k, v) in &event.req_headers {
        if let Ok(val) = v.to_str() {
          println!("-> {}: {}", k, val);
        }
      }
      for (k, v) in &event.res_headers {
        if let Ok(val) = v.to_str() {
          println!("<- {}: {}", k, val);
        }
      }
    }
  });
}
