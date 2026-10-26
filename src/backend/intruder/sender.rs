use reqwest::{Client, Method, Response, header::HeaderMap};
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::{sync::{Semaphore, mpsc}, task::JoinSet, time::Instant};
use futures::future::join_all;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct URL<'a> {
    pub base: &'a str,
    pub method: Method,
    pub headers: HeaderMap,
    pub body: Option<Bytes>,
}

// payload is dealt in another function
async fn send_url(client: &Client, url: URL<'_>) -> Result<Response> {
    client
        .request(url.method, url.base)
        .headers(url.headers)
        .body(url.body.unwrap_or_default())
        .send()
        .await
        .context("failed to execute request")
}
