use serde::{Deserialize, Serialize};
use vanth::{hash, Vanth};

#[derive(Clone, Debug, Deserialize, Serialize, Vanth)]
struct Foo {
    value: i32,
}

fn main() {
    let x = "hello";
    println!("Hash: {:?}", hash(&x).hex());
}
