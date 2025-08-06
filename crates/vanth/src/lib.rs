use std::marker::PhantomData;

/// Library crate for the `vanth` ECS-based database node.
use bevy_app::{App, Plugin};
use bevy_ecs::{prelude::*, query::QueryData};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::entity::EntityId;

pub mod store;
pub mod entity;
pub mod hashing_serializer;

pub use hashing_serializer::hash;
pub use vanth_derive::Vanth;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Other(String),
}

/// A server node wrapping a Bevy App without running it.
pub struct Node {
    app: App,
}

impl Node {
    /// Creates a new server node with an empty Bevy App.
    pub fn new() -> Self {
        let app = App::new();
        Node { app }
    }

    /// Returns the number of entities currently in the world.
    pub fn entity_count(&self) -> usize {
        todo!()
        // Query for no components returns one item per entity.
        // self.app.world().entities().len()
    }
    
    // TODO
    pub fn run() {
        
    }
    
    pub fn save(entity_id: impl Into<EntityId>) -> Result<()> {
        // TODO
        Ok(())
    }
    
    pub fn load(entity_id: impl Into<EntityId>) -> Result<Option<EntityContents>> {
        // TODO
        Ok(None)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HashedValue {
    content_hash: ContentHash,
    inner: Value
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ty {
    pub path: Vec<String>,
}

impl PartialEq for Ty {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl <T: AsRef<str>> PartialEq<T> for Ty {
    fn eq(&self, other: &T) -> bool {
        self.path.join("::") == *other.as_ref()
    }
}

pub trait Vanth {
    fn ty() -> Ty;
}

// TODO: Impl for different tuple sizes
pub trait VanthTuple {
    
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntityContents {
    components: Vec<ComponentContents>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComponentContents {
    id: String,
    content_hash: ContentHash,
    data: Vec<u8>,
}

pub trait Component: Serialize {
    fn id() -> String;
}

// use a macro to implement VanthTuiple here.

#[derive(Copy, Clone, Debug, Deserialize, Component, Serialize)]
pub struct ContentHash {
    pub hash: [u8; 32],
}

#[derive(Clone, Debug, Deserialize, Component, Serialize)]
pub struct Reference<T: Clone + Serialize> {
    value: ReferenceValue,
    _marker: PhantomData<T>
}

#[derive(Clone, Debug, Deserialize, Component, Serialize)]
pub enum ReferenceValue {
    Absent,
    Retrieving(ReferenceRetrievalTask),
    Present(Vec<u8>)
}

impl <T: Clone + Serialize> Reference<T> {
    pub async fn take() -> T {
        todo!()
    }
    
    pub async fn get() -> Handle<T> {
        todo!()
    }
}

#[derive(Component, Clone, Debug, Deserialize, Serialize)]
pub struct ReferenceRetrievalTask {
    
}

impl Future for ReferenceRetrievalTask {
    type Output = Vec<u8>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

pub struct Handle<T> {
    _marker: PhantomData<T>
}

// TODO:
// A trait is derivable for ECS components
// The components must have a content hash, not the entity. For efficiency and ergonomics. This means that a hash of each relevant component must be stored in the Vanth component of the entity, in a `HashMap` or something. The ID of the component used by Vanth should be a method on the derived trait.

pub struct VanthPlugin;

impl Plugin for VanthPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

// fn run_reference_tasks(tasks: Query<(&ReferenceGetTask<>)>) {

// }

/// A world which Vanth entities live in. Lifetimes `'v` of [`Vanth<'v>`] types are tied to the lifetime of the `Root`.
pub struct Root {
    
}
