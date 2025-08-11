#![doc = include_str!("../../../README.md")]

use std::marker::PhantomData;

use bevy_ecs::{prelude::*, query::QueryData};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::entity::EntityId;

pub mod entity;
pub mod hashing_serializer;
pub mod store;

pub use hashing_serializer::hash;
pub use vanth_derive::Vanth;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Other(String),
}

/// A Vanth server.
pub struct Node {
    
}

impl Node {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn entity_count(&self) -> usize {
        todo!()
    }

    pub fn run() {
        todo!()
    }

    pub fn save(entity_id: impl Into<EntityId>) -> Result<()> {
        // TODO
        Ok(())
    }

    // pub fn load(entity_id: impl Into<EntityId>) -> Result<Option<EntityContents>> {
    //     // TODO
    //     Ok(None)
    // }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HashedValue {
    content_hash: ContentHash,
    inner: Value,
}

impl From<Value> for HashedValue {
    fn from(value: Value) -> Self {
        Self {
            content_hash: hash(&value),
            inner: value,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Value {
    ty: Ty,
    data: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash)]
pub struct Ty {
    pub path: Vec<String>,
}

impl ToString for Ty {
    fn to_string(&self) -> String {
        self.path.join("::")
    }
}

impl PartialEq for Ty {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl<T: AsRef<str>> PartialEq<T> for Ty {
    fn eq(&self, other: &T) -> bool {
        self.to_string() == *other.as_ref()
    }
}

pub trait Vanth {
    fn ty() -> Ty;
}

// TODO: Impl for different tuple sizes
pub trait VanthTuple {}

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct EntityContents {
//     components: Vec<ComponentContents>
// }

#[derive(Clone, Debug)]
pub struct ComponentContents<T: Vanth> {
    content_hash: ContentHash,
    data: Vec<u8>,
    _marker: PhantomData<T>,
}

pub trait Component: Serialize {
    fn id() -> String;
}

// use a macro to implement VanthTuiple here.

#[derive(Copy, Clone, Debug, Deserialize, Component, Serialize, PartialEq, Eq, Hash)]
pub struct ContentHash {
    pub hash: [u8; 32],
}

impl ContentHash {
    pub fn hex(&self) -> String {
        self.hash.iter().map(|b| format!("{:02x}", b)).collect::<String>()
    }
}

#[derive(Clone, Debug, Deserialize, Component, Serialize)]
pub struct Reference<T: Clone + Serialize> {
    value: ReferenceValue,
    _marker: PhantomData<T>,
}

#[derive(Clone, Debug, Deserialize, Component, Serialize)]
pub enum ReferenceValue {
    Absent,
    Retrieving(ReferenceRetrievalTask),
    Present(Vec<u8>),
}

impl<T: Clone + Serialize> Reference<T> {
    pub async fn take() -> T {
        todo!()
    }

    pub async fn get() -> Handle<T> {
        todo!()
    }
}

#[derive(Component, Clone, Debug, Deserialize, Serialize)]
pub struct ReferenceRetrievalTask {}

impl Future for ReferenceRetrievalTask {
    type Output = Vec<u8>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

pub struct Handle<T> {
    _marker: PhantomData<T>,
}


// fn run_reference_tasks(tasks: Query<(&ReferenceGetTask<>)>) {

// }

/// A world which Vanth entities live in. Lifetimes `'v` of [`Vanth<'v>`] types are tied to the lifetime of the `Root`.
pub struct Root {}
