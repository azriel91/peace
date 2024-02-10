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
    sd -s 'version = "0.0.13"' 'version = "0.0.14"' $(fd -tf -F toml) README.md src/lib.rs

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

**Note:**

An alternative to `cargo-release` is [`cargo-workspaces`], which may be used in case crates need to be published one by one -- if many new crates are being published, `cargo-release` gates the number of crates that can be published at one go.

```bash
cargo workspaces \
  publish \
  --from-git \
  --allow-branch main \
  --force '*' \
  --no-verify \
  --no-git-tag
```

[`cargo-workspaces`]: https://github.com/pksunkara/cargo-workspaces


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

5. Install [`cargo-leptos`]:

    ```bash
    cargo install --git https://github.com/leptos-rs/cargo-leptos.git --locked cargo-leptos
    ```


**Notes:**

* `pnpm` is used because it downloads each version of each library once, whereas `npm` downloads all dependencies recursively, even if the same dependency is already existent in the dependency tree.
* This is installed as a global binary instead of as a dev dependency within the repository. This is more aligned with Rust's single-binary installation model.


[`cargo-leptos`]: https://github.com/leptos-rs/cargo-leptos
[`nvm`]: https://github.com/nvm-sh/nvm
[tailwindcss]: https://tailwindcss.com/


### Development

> ℹ️ These commands assume you are running them from the repository root directory.

Build and serve the `envman` example:

```bash
cargo leptos watch --project "envman" -v
```

You can also use `trunk` to build the client side `csr` app.

```bash
(cd examples/envman && trunk build -d ../../dist)
````

### Uninstallation

To uninstall web tooling:

```bash
pnpm uninstall --global tailwindcss
nvm uninstall $version
rm -rf ~/.nvm
```
