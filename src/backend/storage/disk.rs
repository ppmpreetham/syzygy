use std::fs::create_dir_all;

use super::super::proxy::certificate::storage_path;
use super::Exchange;
use super::serializer::serialize;
use anyhow::{Result, anyhow};
use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode};

// find the db, if there, use it, else make a new db.

pub struct Db {
    db: Database,
}

impl Db {
    pub fn new() -> Result<Self> {
        let path = storage_path()
            .ok_or_else(|| anyhow!("failed to get path"))?
            .join("db");

        if !path.exists() {
            create_dir_all(&path).unwrap();
        }
        let db = Database::builder(&path).open()?;
        Ok(Self { db })
    }

    pub fn get_history(&self) -> Result<Keyspace> {
        let items = self
            .db
            .keyspace("history", KeyspaceCreateOptions::default)?;
        Ok(items)
    }

    // TODO: make this for a range of items later (contiguous ofc)
    pub fn insert_history(&self, idx: usize, item: &str) -> Result<()> {
        let history = self.get_history()?;
        history.insert(idx.to_be_bytes(), item)?;
        self.db.persist(PersistMode::SyncAll)?;
        Ok(())
    }

    pub fn get_history_item(&self, idx: usize) -> Result<Option<String>> {
        let byte_idx = idx.to_be_bytes();
        let history = self.get_history()?;
        let item = history
            .get(byte_idx)?
            .map(|v| String::from_utf8_lossy(&v).into_owned());

        Ok(item)
    }

    pub fn clear_history(&self) -> Result<()> {
        let history = self.get_history()?;
        history.clear()?;
        Ok(())
    }

    // TODO: flush once closing, or periodically(every 100 exchanges) or when memory is reaching THRESHOLD
    pub fn flush(&self, exchanges: &[Exchange], start: usize) -> Result<()> {
        let history = self.get_history()?;
        let mut batch = self.db.batch();
        for (i, ex) in exchanges.iter().enumerate() {
            batch.insert(&history, (start + i).to_be_bytes(), serialize(ex));
        }
        batch.commit()?; // single atomic journal write
        self.db.persist(PersistMode::SyncAll)?; // fsync once per 100
        Ok(())
    }
}
