use std::marker::PhantomData;

/// Library crate for the `vanth` ECS-based database node.
use bevy_app::App;
use bevy_ecs::{prelude::*, query::QueryData};
use serde::{Deserialize, Serialize};

pub mod store;
pub mod entity;

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
}

#[derive(Debug, Deserialize, Component, Serialize)]
pub struct Vanth<T: VanthTuple + QueryData> {
    id: crate::entity::Id<Self>,
    _marker: PhantomData<T>,
}

impl <T: VanthTuple + QueryData> Vanth<T> {
    fn save_system(query: Query<T>) {
        
    }
}

// TODO: Impl for different tuple sizes
pub trait VanthTuple {
    
}

// use a macro to implement VanthTuiple here.

#[derive(Debug, Deserialize, Component, Serialize)]
pub struct ContentHash {
    pub hash: [u8; 32],
}

// TODO:
// A trait is derivable for ECS components
// The components must have a content hash, not the entity. For efficiency and ergonomics. This means that a hash of each relevant component must be stored in the Vanth component of the entity, in a `HashMap` or something. The ID of the component used by Vanth should be a method on the derived trait.
