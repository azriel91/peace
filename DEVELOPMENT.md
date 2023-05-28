# Development

## Rust Development

### Dependencies

```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
cargo install cargo-nextest
```


### Running Tests

```bash
cargo nextest run --workspace --all-features

# To test individual features
for i in {0..3}; do cargo test_$i || break; done
```


### Coverage

Collect coverage and output as `lcov`.

```bash
./coverage.sh
```

Collect coverage and open `html` report.

```bash
./coverage.sh && cargo coverage_open
```


### Releasing

1. Update crate versions.

    ```bash
    sd -s 'version = "0.0.9"' 'version = "0.0.10"' $(fd -tf -F toml) README.md src/lib.rs

    # Make sure only `peace` crates are updated.
    git --no-pager diff | rg '^[+]' | rg -v '(peace)|(\+\+\+)|\+version'
    ```

2. Update `CHANGELOG.md` with the version and today's date.
3. If dependency versions have changed, update `licenses.html`:

    ```bash
    cargo about generate --workspace --all-features about.hbs > doc/src/licenses.html
    ```

4. Push a tag to the repository.

    The [`publish`] GitHub workflow will automatically publish the crates to [`crates.io`].

[`publish`]: https://github.com/azriel91/peace/actions/workflows/publish.yml
[`crates.io`]:https://crates.io/


## Web Development

### Set Up

These instructions are for Linux. They may work on OS X, but for Windows, please visit each linked site for specific instructions.

1. Install [`nvm`]:

    ```bash
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash
    ```

2. Install node and set the default version:

    ```bash
    nvm install 20
    nvm alias default 20
    ```

3. Install and set up `pnpm`:

    ```bash
    npm install -g pnpm
    pnpm setup
    ```

4. Install [tailwindcss]:

    ```bash
    pnpm install --global tailwindcss
    ```

5. Install [`cargo-watch`]

    ```bash
    cargo install cargo-watch
    ```


**Notes:**

* `pnpm` is used because it downloads each version of each library once, whereas `npm` downloads all dependencies recursively, even if the same dependency is already existent in the dependency tree.
* This is installed as a global binary instead of as a dev dependency within the repository. This is more aligned with Rust's single-binary installation model.


[`cargo-watch`]: https://github.com/watchexec/cargo-watch
[`nvm`]: https://github.com/nvm-sh/nvm
[tailwindcss]: https://tailwindcss.com/


### Development

> ℹ️ These commands assume you are running them from the repository root directory.

Run the `tailwindcss` command to generate the example's CSS:

```bash
tailwindcss \
  -i examples/envman/src/web/tailwind.css \
  -o target/web/envman/public/css/tailwind.css \
  --watch
```

Build and serve the example:

```bash
# For watching and auto reloading
cargo watch \
  -x 'build --package envman --all-features' \
  -s "bash -c '(cd target/web/envman && ../../debug/envman web)'"

# For a single execution
cargo build --package envman --all-features &&
  (cd target/web/envman && ../../debug/envman web)
```

### Uninstallation

To uninstall web tooling:

```bash
pnpm uninstall --global tailwindcss
nvm uninstall $version
rm -rf ~/.nvm
```
