/// Library crate for the `vanth` ECS-based database node.
use bevy_app::App;
use bevy_ecs::prelude::*;

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
        // Query for no components returns one item per entity.
        self.app.world().entities().len()
    }
}
