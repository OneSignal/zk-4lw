zk-4lw
======

ZooKeeper Four Letter Words Client Library

## Examples

```rust
use zk_4lw::{Client, Mntr};

// Create a client for ZooKeeper
let client = Client::new("localhost:2181");

// Run the "mntr" command
let res: ::zk_4lw::mntr::Response = client.exec::<Mntr>().unwrap();
```

## Run the tests

Static tests can be run with a simple `cargo test`. Additionally, commands can
be run against a ZooKeeper instance with `cargo test --features with-client`.
