use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};
use vanth::Vanth;

// TODO: derive `Vanth`
#[derive(Debug, Deserialize, Component, Serialize)]
struct Foo {
    
}
