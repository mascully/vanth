use std::{collections::HashMap, marker::PhantomData, path::PathBuf};

use rusqlite::{Connection, named_params, params};

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tracing::trace;

use crate::{ComponentContents, ContentHash, Ty, Vanth, hash};

#[derive(Debug)]
pub struct Store {
    backend: Box<dyn Backend>,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    Serializiation(String),
    SqliteTableDoesNotExist { table_name: String },
    SqliteUnknown(String),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serializiation(err.to_string())
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        if let rusqlite::Error::SqliteFailure(_, Some(ref message)) = err
            && let Some(table_name) = message.strip_prefix("no such table: ")
        {
            return Error::SqliteTableDoesNotExist {
                table_name: table_name.into(),
            };
        }
        Error::SqliteUnknown(err.to_string())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoreParams {
    pub create_if_not_exists: bool,
    pub read_only: bool,
}

impl Default for StoreParams {
    fn default() -> Self {
        Self {
            create_if_not_exists: true,
            read_only: false,
        }
    }
}

impl Store {
    /// Use an SQLite backend with a database file at the provided path.
    pub fn sqlite_from_path(path: PathBuf, params: StoreParams) -> Result<Self> {
        use rusqlite::OpenFlags;
        // Base flags for URI handling and disabling mutexes.
        let mut flags = OpenFlags::SQLITE_OPEN_URI | OpenFlags::SQLITE_OPEN_NO_MUTEX;

        // Read‑only takes precedence over read‑write.
        if params.read_only {
            flags |= OpenFlags::SQLITE_OPEN_READ_ONLY;
        } else {
            flags |= OpenFlags::SQLITE_OPEN_READ_WRITE;
        }

        // Create the file if allowed.
        if params.create_if_not_exists {
            flags |= OpenFlags::SQLITE_OPEN_CREATE;
        }

        // Open the SQLite connection with the computed flags.
        let connection = rusqlite::Connection::open_with_flags(path, flags)?;
        Ok(Self {
            backend: Box::new(Sqlite { connection }),
        })
    }

    /// Use an in-memory backend.
    pub fn in_memory() -> Result<Self> {
        Ok(Self {
            backend: Box::new(Memory::new()),
        })
    }

    pub fn get_from_hash<T: Vanth + DeserializeOwned>(&mut self, content_hash: ContentHash) -> Result<Option<T>> {
        let Some(raw) = self.get_from_hash_raw(T::ty(), content_hash)? else {
            return Ok(None);
        };

        let deserialized: T = serde_json::from_slice(&raw)?;
        Ok(Some(deserialized))
    }

    pub fn get_from_hash_raw(&mut self, ty: Ty, content_hash: ContentHash) -> Result<Option<Vec<u8>>> {
        self.backend.get_from_hash(ty, content_hash)
    }

    pub fn get_all_of_type<T: Vanth>(&mut self) -> Result<Vec<ComponentContents<T>>> {
        let raw_items = self.backend.get_all_of_ty(T::ty())?;
        let mut results = Vec::new();
        for (content_hash, data) in raw_items {
            results.push(ComponentContents {
                _marker: PhantomData,
                content_hash,
                data,
            });
        }
        Ok(results)
    }

    pub fn write<T: Vanth + Serialize>(&mut self, value: &T) -> Result<()> {
        let content_hash = hash(&value);
        let data = serde_json::to_vec(&value)?;
        self.backend.write(T::ty(), content_hash, data)
    }

    pub fn write_raw(&mut self, ty: Ty, content_hash: ContentHash, content: Vec<u8>) -> Result<()> {
        self.backend.write(ty, content_hash, content)
    }

    pub fn delete<T: Vanth>(&mut self, content_hash: ContentHash) -> Result<()> {
        self.backend.delete_by_hash(T::ty(), content_hash)
    }

    pub fn delete_all<T: Vanth>(&mut self) -> Result<()> {
        self.backend.delete_all_of_ty(T::ty())
    }

    pub fn get_all_of_type_raw(&mut self, ty: Ty) -> Result<Vec<(ContentHash, Vec<u8>)>> {
        self.backend.get_all_of_ty(ty)
    }

    pub fn delete_raw(&mut self, ty: Ty, content_hash: ContentHash) -> Result<()> {
        self.backend.delete_by_hash(ty, content_hash)
    }

    pub fn delete_all_raw(&mut self, ty: Ty) -> Result<()> {
        self.backend.delete_all_of_ty(ty)
    }
}

#[derive(Debug, Deserialize, Component, Serialize)]
pub struct Cache {
    size_limit_bytes: u64,
    // backend: Backend,
}

pub trait Backend: std::fmt::Debug + Send {
    fn get_from_hash(&mut self, ty: Ty, content_hash: ContentHash) -> Result<Option<Vec<u8>>>;

    fn get_all_of_ty(&mut self, ty: Ty) -> Result<Vec<(ContentHash, Vec<u8>)>>;

    fn write(&mut self, ty: Ty, content_hash: ContentHash, content: Vec<u8>) -> Result<()>;

    fn delete_by_hash(&mut self, ty: Ty, content_hash: ContentHash) -> Result<()>;

    fn delete_all_of_ty(&mut self, ty: Ty) -> Result<()>;
}

/// One table per type. Keys and values are both blobs.
#[derive(Debug)]
pub struct Sqlite {
    connection: Connection,
}

impl Sqlite {
    // TODO: Use this instead of directly constructing the connection in `Store`.
    pub fn new(path: PathBuf) -> Result<Self> {
        use rusqlite::OpenFlags;
        // Remove the `SQLITE_OPEN_CREATE` flag because we do not want to create databases if they don't exist.
        let connection = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_URI | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        Ok(Self { connection })
    }

    fn table_name(ty: &Ty) -> String {
        format!("ty_{}", ty.to_string())
    }
}

impl Backend for Sqlite {
    fn get_from_hash(&mut self, ty: Ty, content_hash: ContentHash) -> Result<Option<Vec<u8>>> {
        let table_name = Self::table_name(&ty);
        let query = format!("SELECT content FROM \"{}\" WHERE content_hash = :hash", table_name);

        match self
            .connection
            .query_row(&query, named_params! {":hash": content_hash.hash.as_slice()}, |row| {
                row.get::<_, Vec<u8>>(0)
            }) {
            Ok(content) => Ok(Some(content)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn get_all_of_ty(&mut self, ty: Ty) -> Result<Vec<(ContentHash, Vec<u8>)>> {
        let transaction = self.connection.transaction()?;
        let table_name = Self::table_name(&ty);
        let query = format!("SELECT content_hash, content FROM \"{}\"", table_name);

        let mut statement = match transaction.prepare(&query).map_err(Into::into) {
            Err(Error::SqliteTableDoesNotExist { .. }) => return Ok(Vec::new()),
            other => other?,
        };

        let rows = statement.query_map([], |row| {
            let hash_bytes: Vec<u8> = row.get(0)?;
            let content: Vec<u8> = row.get(1)?;
            let mut hash_array = [0u8; 32];
            hash_array.copy_from_slice(&hash_bytes);
            Ok((ContentHash { hash: hash_array }, content))
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }

        drop(statement);
        transaction.commit()?;
        
        Ok(results)
    }

    fn write(&mut self, ty: Ty, content_hash: ContentHash, content: Vec<u8>) -> Result<()> {
        let table_name = Self::table_name(&ty);
        let create_table_query = format!(
            "CREATE TABLE IF NOT EXISTS \"{}\" (
                content_hash BLOB PRIMARY KEY,
                content BLOB NOT NULL
            )",
            table_name
        );
        self.connection.execute(&create_table_query, [])?;
        let query = format!(
            "INSERT OR REPLACE INTO \"{}\" (content_hash, content) VALUES (:hash, :content)",
            table_name
        );
        self.connection.execute(
            &query,
            named_params! {":hash": content_hash.hash.as_slice(), ":content": content},
        )?;
        Ok(())
    }

    fn delete_by_hash(&mut self, ty: Ty, content_hash: ContentHash) -> Result<()> {
        let table_name = Self::table_name(&ty);
        let query = format!("DELETE FROM \"{}\" WHERE content_hash = :hash", table_name);
        self.connection
            .execute(&query, named_params! {":hash": content_hash.hash.as_slice()})?;
        Ok(())
    }

    fn delete_all_of_ty(&mut self, ty: Ty) -> Result<()> {
        let table_name = Self::table_name(&ty);
        let query = format!("DROP TABLE IF EXISTS \"{}\"", table_name);
        self.connection.execute(&query, [])?;
        Ok(())
    }
}

/// In-memory storage with one table per type.
#[derive(Debug, Deserialize, Serialize)]
pub struct Memory {
    tables: HashMap<Ty, HashMap<ContentHash, Vec<u8>>>,
}

impl Memory {
    pub fn new() -> Self {
        Self { tables: HashMap::new() }
    }
}

impl Backend for Memory {
    fn get_from_hash(&mut self, ty: Ty, content_hash: ContentHash) -> Result<Option<Vec<u8>>> {
        Ok(self.tables.get(&ty).and_then(|table| table.get(&content_hash)).cloned())
    }

    fn get_all_of_ty(&mut self, ty: Ty) -> Result<Vec<(ContentHash, Vec<u8>)>> {
        Ok(self
            .tables
            .get(&ty)
            .map(|table| table.iter().map(|(k, v)| (*k, v.clone())).collect())
            .unwrap_or_else(Vec::new))
    }

    fn write(&mut self, ty: Ty, content_hash: ContentHash, content: Vec<u8>) -> Result<()> {
        self.tables
            .entry(ty)
            .or_insert_with(HashMap::new)
            .insert(content_hash, content);
        Ok(())
    }

    fn delete_by_hash(&mut self, ty: Ty, content_hash: ContentHash) -> Result<()> {
        if let Some(table) = self.tables.get_mut(&ty) {
            table.remove(&content_hash);
        }
        Ok(())
    }

    fn delete_all_of_ty(&mut self, ty: Ty) -> Result<()> {
        self.tables.remove(&ty);
        Ok(())
    }
}
