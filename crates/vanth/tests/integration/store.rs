use vanth::store::Store;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_sqlite_store() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.db");
    let mut store = Store::from_path(path.clone()).unwrap();
    
    assert_eq!(store.read(b"key_1"), Ok(None));
    assert_eq!(store.write(b"key_1", b"value_1"), Ok(()));
    
    let value = store.read(b"key_1").unwrap();
    assert_eq!(value.as_deref(), Some(b"value_1" as &[u8]));
    
    drop(store);
    
    let mut store = Store::from_path(path.clone()).unwrap();
    
    let value = store.read(b"key_1").unwrap();
    assert_eq!(value.as_deref(), Some(b"value_1" as &[u8]));
    
    store.delete(b"key_1").unwrap();
    assert_eq!(store.read(b"key_1"), Ok(None));
}
