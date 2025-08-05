use std::{collections::HashMap, path::PathBuf};

use rusqlite::{Connection, params};

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Component, Serialize)]
pub struct Store {
    backend: Backend,
}

type Result<T> = std::result::Result<T, String>;

impl Store {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let conn = Connection::open(&path).map_err(|e| e.to_string())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS kv (key BLOB PRIMARY KEY, value BLOB)",
            params![],
        ).map_err(|e| e.to_string())?;
        Ok(Self {
            backend: Backend::Sqlite(Sqlite { path }),
        })
    }
    
    pub fn in_memory() -> Result<Self> {
        Ok(Self {
            backend: Backend::Memory(Memory::new()),
        })
    }
    
    pub fn read(&mut self, key: impl AsRef<[u8]>) -> Result<Option<Vec<u8>>> {
        match &mut self.backend {
            Backend::Memory(mem) => Ok(mem.values.get(key.as_ref()).cloned()),
            Backend::Sqlite(sql) => {
                let conn = Connection::open(&sql.path).map_err(|e| e.to_string())?;
                let mut stmt = conn.prepare("SELECT value FROM kv WHERE key = ?1").map_err(|e| e.to_string())?;
                let mut rows = stmt.query(params![key.as_ref()]).map_err(|e| e.to_string())?;
                if let Some(row) = rows.next().map_err(|e| e.to_string())? {
                    let value: Vec<u8> = row.get(0).map_err(|e| e.to_string())?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }
        }
    }
    
    pub fn write(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<()> {
        match &mut self.backend {
            Backend::Memory(mem) => {
                mem.values.insert(key.as_ref().to_vec(), value.as_ref().to_vec());
                Ok(())
            }
            Backend::Sqlite(sql) => {
                let conn = Connection::open(&sql.path).map_err(|e| e.to_string())?;
                conn.execute(
                    "INSERT OR REPLACE INTO kv (key, value) VALUES (?1, ?2)",
                    params![key.as_ref(), value.as_ref()],
                ).map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }
    
    pub fn delete(&mut self, key: impl AsRef<[u8]>) -> Result<()> {
        match &mut self.backend {
            Backend::Memory(mem) => {
                mem.values.remove(key.as_ref());
                Ok(())
            }
            Backend::Sqlite(sql) => {
                let conn = Connection::open(&sql.path).map_err(|e| e.to_string())?;
                conn.execute(
                    "DELETE FROM kv WHERE key = ?1",
                    params![key.as_ref()],
                ).map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }
}

#[derive(Debug, Deserialize, Component, Serialize)]
pub struct Cache {
    size_limit_bytes: u64,
    backend: Backend,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Backend {
    Memory(Memory),
    Sqlite(Sqlite)
}

/// One table, key-value store. Keys and values are both blobs.
#[derive(Debug, Deserialize, Serialize)]
pub struct Sqlite {
    path: PathBuf,
}

/// One table, key-value store. Keys and values are both blobs.
#[derive(Debug, Deserialize, Serialize)]
pub struct Memory {
    values: HashMap<Vec<u8>, Vec<u8>>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}
