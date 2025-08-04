use serde::{Deserialize, Serialize};
use vanth::{Component, Node, Reference};

// #[test]
// fn test_store() {
//     #[derive(Clone, Deserialize, Serialize)]
//     struct Foo {
//         bar: Reference<Bar>,
//     }
    
//     #[derive(Clone, Deserialize, Serialize)]
//     struct Bar {
//         foo: Reference<Foo>,
//     }
    
//     impl Component for Foo {
//         fn id() -> String {
//             "foo".into()
//         }
//     }
    
//     let node = Node::in_memory();
    
//     let entity_id = "entity_1";
//     let entity_components = (Foo { a: 5, b: 6.0 },);
    
//     node.save("entity_1", entity_components);
// }
