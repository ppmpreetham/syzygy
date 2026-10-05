use super::super::proxy::certificate::storage_path;
use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode};
use anyhow::{anyhow, Result};

// find the db, if there, use it, else make a new db.

struct Db {
  db: Database
}

impl Db {
  fn new() -> Result<Self> {
    let path = storage_path().ok_or_else(|| anyhow!("failed to get path"))?.join("db");

    if !path.exists() {
      std::fs::create_dir_all(&path).unwrap();
    }
    let db = Database::builder(&path).open()?;
    Ok(Self { db })
  }

  fn get_history(&self) -> Result<Keyspace> {
    let items = self.db.keyspace("history", KeyspaceCreateOptions::default)?;
    Ok(items)
  }

  // TODO: make this for a range of items later (contiguous ofc)
  fn insert_history(&self, idx: usize, item: &str) -> Result<()> {
    let history = self.get_history()?;
    history.insert(idx.to_be_bytes(), item)?;
    self.db.persist(PersistMode::SyncAll)?;
    Ok(())
  }


  fn get_history_item(&self, idx: usize) -> Result<Option<String>> {
    let byte_idx = idx.to_be_bytes();
    let history = self.get_history()?;
    let item = history.get(byte_idx)?
        .map(|v| String::from_utf8_lossy(&v).into_owned());

    Ok(item)
  }

  fn clear_history(&self) -> Result<()> {
    let history = self.get_history()?;
    history.clear()?;
    Ok(())
  }
}
