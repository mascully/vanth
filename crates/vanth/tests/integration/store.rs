use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tempfile::TempDir;
use vanth::{Vanth, hash, store::{Store, StoreParams}};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Vanth)]
struct Foo {
    inner: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Vanth)]
struct Bar {
    inner: String,
}

#[test]
fn test_sqlite_store() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.db");
    let mut store = Store::sqlite_from_path(path.clone(), StoreParams::default()).unwrap();

    let foo_1 = Foo { inner: 1 };
    let foo_2 = Foo { inner: 2 };
    let bar_1 = Bar { inner: "hello".into() };

    assert_eq!(store.get_all_of_type::<Foo>().unwrap().len(), 0);
    assert_eq!(store.get_all_of_type::<Bar>().unwrap().len(), 0);

    store.write(&foo_1).unwrap();
    store.write(&foo_2).unwrap();
    store.write(&bar_1).unwrap();
    assert_eq!(store.get_all_of_type::<Foo>().unwrap().len(), 2);
    assert_eq!(store.get_all_of_type::<Bar>().unwrap().len(), 1);

    let foo_2_hash = hash(&foo_2);
    let foo_2_fetched = store.get_from_hash(foo_2_hash).unwrap().unwrap();
    assert_ne!(foo_1, foo_2_fetched);
    assert_eq!(foo_2, foo_2_fetched);

    store.delete::<Foo>(foo_2_hash).unwrap();
    assert_eq!(store.get_all_of_type::<Foo>().unwrap().len(), 1);

    store.delete_all::<Foo>().unwrap();
    store.delete_all::<Bar>().unwrap();
    assert_eq!(store.get_all_of_type::<Foo>().unwrap().len(), 0);
    assert_eq!(store.get_all_of_type::<Bar>().unwrap().len(), 0);
}
