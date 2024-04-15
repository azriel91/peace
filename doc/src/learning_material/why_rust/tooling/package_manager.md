# Package Manager

We have one: `cargo`.

All commands are standard, you don't have to define scripts to build or run:

```bash
cargo init
cargo build
cargo run
cargo test
cargo publish
```

```toml
[package]
name = "scratch"
version = "0.1.0"

[dependencies]
serde = { version = "1.0.197", features = ["derive"] }
serde_json = "1.0.115"
```
