# Development

## Dependencies

```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
cargo install cargo-nextest
```


## Running Tests

```bash
cargo nextest run --workspace --all-features

# To test individual features
for i in {0..3}; do cargo test_$i || break; done
```


## Coverage

Collect coverage and output as `lcov`.

```bash
./coverage.sh
```

Collect coverage and open `html` report.

```bash
./coverage.sh && cargo coverage_open
```


## Releasing

1. Update crate versions.

    ```bash
    sd -s 'version = "0.0.9"' 'version = "0.0.10"' $(fd -tf -F toml) README.md src/lib.rs

    # Make sure only `peace` crates are updated.
    git --no-pager diff | rg '^[+]' | rg -v '(peace)|(\+\+\+)|\+version'
    ```

2. Update `CHANGELOG.md` with the version and today's date.
3. Push a tag to the repository.

    The [`publish`] GitHub workflow will automatically publish the crates to [`crates.io`].

[`publish`]: https://github.com/azriel91/peace/actions/workflows/publish.yml
[`crates.io`]:https://crates.io/
