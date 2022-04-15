# Development

## Dependencies

```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov

# Optional: Use `nextest` to run tests
cargo install cargo-nextest
```


## Running Tests

```bash
cargo test --workspace
cargo nextest run --workspace
```


## Coverage

Collect coverage and output as `html`.

```bash
cargo llvm-cov --workspace --open --output-dir ./target/coverage

# With `nextest`:
# https://github.com/taiki-e/cargo-llvm-cov/issues/151
cargo coverage
# This is an alias defined in `.cargo/config.toml` to:
cargo llvm-cov --workspace nextest --open --output-dir ./target/coverage --workspace
```

Collect coverage and output as `lcov`.

```bash
cargo llvm-cov --workspace --lcov --output-path ./target/coverage/lcov.info

# With `nextest`:
# https://github.com/taiki-e/cargo-llvm-cov/issues/151
cargo llvm-cov --workspace nextest --workspace --lcov --output-path ./target/coverage/lcov.info
```
