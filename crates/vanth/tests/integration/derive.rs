use bevy_ecs::component::Component;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vanth::Vanth;

// TODO: derive `Vanth`
#[derive(Debug, Deserialize, Component, Serialize)]
struct Foo {
    
}

#[test]
fn test_derive() {
    #[derive(Deserialize, Serialize, Vanth)]
    struct Foo<T: Vanth> {
        field_a: i32,
        field_b: String,
        inner: T,
    }
    
    #[derive(Deserialize, Serialize, Vanth)]
    struct Bar {
        field_a: i32,
    }
    
    #[derive(Deserialize, Serialize, Vanth)]
    struct Qux<T: Vanth, S: Vanth> {
        field_a: i32,
        field_b: String,
        inner: T,
        inner_2: S,
    }
    
    let base = "integration::derive::";
    
    assert_eq!(Bar::ty(), format!("{base}Bar"));
    assert_eq!(Foo::<Bar>::ty(), format!("{base}Foo<{base}Bar>"));
    assert_eq!(Qux::<Bar, Foo<Bar>>::ty(), format!("{base}Qux<{base}Bar,{base}Foo<{base}Bar>>"));
}
