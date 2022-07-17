# Development

## Dependencies

```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
cargo install cargo-nextest
```


## Running Tests

```bash
cargo nextest run --workspace
```


## Coverage

Collect coverage and output as `lcov`.

```bash
cargo coverage
```

Collect coverage and open `html` report.

```bash
cargo coverage && cargo coverage_open
```


## Releasing

Update crate versions, then push a tag to the repository. The [`publish`] GitHub workflow will automatically publish the crates to [`crates.io`].

[`publish`]: https://github.com/azriel91/peace/actions/workflows/publish.yml
[`crates.io`]:https://crates.io/
