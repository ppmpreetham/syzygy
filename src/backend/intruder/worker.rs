// TODO: this entire file
//
use super::compiler::CompiledSegments;
use reqwest::{Client, Response};
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::{sync::{mpsc, Semaphore}, task::JoinSet, time::Instant};

#[derive(Debug)]
pub struct Hit {
    pub bucket: usize,
    pub payload: String,
    pub status: Option<u16>,
    pub elapsed_ms: u128,
    pub error: Option<String>,
}

pub struct Bucket {
    pub id: usize,
    pub segments: CompiledSegments,
    // TODO: NOT STRINGS, need to fix this
    pub payloads: Vec<String>,
}


/// inject payload -> raw request bytes
fn inject(segments: &CompiledSegments, payload: &str, buf: &mut Vec<u8>) {
    segments.reconstruct(payload, buf);
    unimplemented!()
}


/// raw bytes -> reqwest request -> response
async fn fire(client: &Client, req: Vec<u8>) -> Result<Response> {
    // todo!("parse req into method/headers/body or send via hyper/TcpStream")
    let _ = (client, req);
    unimplemented!()
}

// currently thinking of 2 ways of sending requests. (sync/async)
// sync is when you want the buckets to run one after another
// async is when you want both the buckets to run parallelly
async fn run_bucket(
    bucket: Bucket,
    client: Arc<Client>,
    hits_tx: mpsc::Sender<Hit>,
    limit: Arc<Semaphore>,
) {
    let Bucket { id, segments, payloads } = bucket;
    let mut buf = Vec::new();

    for payload in payloads {
        let _permit = limit.acquire().await.unwrap();
        inject(&segments, &payload, &mut buf);

        let t = Instant::now();
        let hit = match fire(&client, buf.clone()).await {
            Ok(r) => Hit {
                bucket: id,
                payload: payload.clone(),
                status: Some(r.status().as_u16()),
                elapsed_ms: t.elapsed().as_millis(),
                error: None
            },
            Err(e) => Hit {
                bucket: id,
                payload,
                status: None,
                elapsed_ms: t.elapsed().as_millis(),
                error: Some(e.to_string())
            },
        };
        if hits_tx.send(hit).await.is_err() { return; }
    }
}


// sync
pub async fn sync_request_sender(
    buckets: Vec<Bucket>,
    client: Arc<Client>,
    hits_tx: mpsc::Sender<Hit>,
    limit: Arc<Semaphore>,
) {
    for bucket in buckets {
        run_bucket(bucket, client.clone(), hits_tx.clone(), limit.clone()).await;
    }
}

// async
pub async fn async_request_sender(
    buckets: Vec<Bucket>,
    client: Arc<Client>,
    hits_tx: mpsc::Sender<Hit>,
    limit: Arc<Semaphore>,
) -> Result<()> {
    let mut set = JoinSet::new();
    for bucket in buckets {
        set.spawn(run_bucket(bucket, client.clone(), hits_tx.clone(), limit.clone()));
    }
    while let Some(r) = set.join_next().await {
        r.context("bucket panicked")?;
    }
    Ok(())
}
