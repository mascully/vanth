use std::path::PathBuf;

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Component, Serialize)]
pub struct Store {
    pub path: PathBuf,
}

impl Store {
    
}

#[derive(Debug, Deserialize, Component, Serialize)]
pub struct Cache {
    size_limit_bytes: u64,
    backend: Backend,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Backend {
    Memory,
    Sqlite(Sqlite)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sqlite {
    path: PathBuf,
}
