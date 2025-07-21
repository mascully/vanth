use std::path::PathBuf;

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Component, Serialize)]
pub struct Store {
    pub path: PathBuf,
}

impl Store {
    
}
