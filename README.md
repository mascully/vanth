# Vanth

Vanth is a content-addressed database as a library designed for entity-component-system (ECS) applications.

It is currently experimental and should not be used for anything.

## Library usage

Any type that implements `serde::Serialize` can be hashed using `vanth::hash` to produce a `vanth::ContentHash`.

```rust
let x = "hello";
assert_eq!(vanth::hash(&x).hex(), "ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f");
```

Derive or implement the `vanth::Vanth` trait for types you want to store in the database.

```rust
use serde::{Deserialize, Serialize};
use vanth::Vanth;

#[derive(Deserialize, Serialize, Vanth)]
struct Data {
    value: u32,
}
```

This generates a method `Vanth::ty()` which returns a `vanth::Ty`. This should represent the type's fully qualified name - its module path followed by the type itself and any generics it has. E.g. `Data::<ty().to_string()` could return `my::crate::module::Data`.

The derivation only works for basic types right now and is not implemented for `std` types. Moving or renaming types or modules will change the type name, necessitating a database migration. This is not supported yet.

This should be used with caution. There are good reasons why `std::any::TypeId` is opaque.

### Database storage

```rust
use vanth::store::{Store, StoreParams};

// Or use `Store::in_memory`.
let mut store = Store::sqlite_from_path(
    "path/to/my_database.sqlite".into(),
    StoreParams { create_if_not_exists: true, ..Default::default() },
);
let hash = store.write(Data { value: 5 }).unwrap();
let my_data: Data = store.get_from_hash(hash).unwrap();
```

## CLI usage

You can run the Vanth CLI with Nix using `nix run https://git.mascully.com/mascully/vanth`.

Use `--` to pass arguments, e.g. `nix run https://git.mascully.com/mascully/vanth -- --help`.

### Syntax

Write a component to the database:

```bash
$ vanth write --db /path/to/db.sqlite --ty my::type::Name --value '{ "field": "value" }'
a1cf81d8afe4e72604ea5c13bbd9b6cce14bd98c3a2f036f7156c4464a88ec09
```

Get a component from the database:

```bash
$ vanth get --db /path/to/db.sqlite --ty my::type::Name a1cf81d8afe4e72604ea5c13bbd9b6cce14bd98c3a2f036f7156c4464a88ec09
{"field":"value"}
```

Get all components of a type from the database:

```bash
$ vanth get-all --db /path/to/db.sqlite --ty my::type::Name
{"field":"value1"}
{"field":"value2"}
{"field":"value3"}
```

Delete a component by its content hash:

```bash
$ vanth delete --db /path/to/db.sqlite ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f
```

Delete all components of a type:

```bash
$ vanth delete-all --db /path/to/db.sqlite --ty my::type::Name
```
