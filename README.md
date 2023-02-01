# 🕊️ peace &ndash; zero stress automation

[![Crates.io](https://img.shields.io/crates/v/peace.svg)](https://crates.io/crates/peace)
[![docs.rs](https://img.shields.io/docsrs/peace)](https://docs.rs/peace)
[![CI](https://github.com/azriel91/peace/workflows/CI/badge.svg)](https://github.com/azriel91/peace/actions/workflows/ci.yml)
[![Coverage Status](https://codecov.io/gh/azriel91/peace/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/peace)

`peace` is a framework to build user friendly software automation.

See:

* [`MOTIVATION.md`](MOTIVATION.md) for the motivation to create this framework.
* [Operations UX](https://azriel.im/ops_ux/) for a book about the dimensions considered during `peace`'s design and development.


## Guiding Principles

* A joy to use.
* Ergonomic API and guidance to do the right thing.
* Understandable output.


## Features

| Symbol | Meaning              |
|:------:|:---------------------|
|   🟢   | Works well           |
|   🟡   | Partial support      |
|   ⚫   | Planned              |
|   🔵   | Compatible by design |
|   🟣   | Works, "fun idea"    |

* 🟢 Define items to manage with automation
* 🟢 Define dependencies between items
* 🟢 Discover current and desired states
* 🟢 Show diff: what would change
* 🟢 Store and recall parameters across commands
* 🟢 Concurrent task execution via [`fn_graph`]
* 🟢 Skip unnecessary work
* 🟢 Idempotence: Multiple executions
* 🟢 Show state differences
* 🟢 Namespaced profile directories
* 🟢 Resource clean up
* 🟢 Understandable progress ([#42])
* 🟡 Feature-gated incremental functionality
* 🟡 Off-the-shelf support for common items
* 🟡 Dry run
* 🔵 Understandable error reporting via [`miette`]
* 🔵 Actionable error messages
* 🟣 WASM support
* ⚫ Tutorial for writing a software lifecycle management tool
* ⚫ Built-in application execution methods -- CLI, web service
* ⚫ `peace` binary for configuration based workflows
* ⚫ Web based UI
* ⚫ Agent mode to run `peace` on servers (Web API invocation)

Further ideas:

* Back up current state
* Restore previous state
* Telemetry logging for monitoring
* Metrics collection for analysis


## Examples

Examples are run using `--package` instead of `--example`, as each example is organized as its own crate.

```bash
cargo run --package $example_name --all-features

# e.g.
cargo build --package download --all-features
cargo run -q --package download --all-features -- init https://ifconfig.me ip.json

for cmd in status desired diff ensure diff clean diff
do
    printf "=== ${cmd} ===\n"
    cargo run -q --package download --all-features -- $cmd
    printf '\n'
done

# Look at metadata that Peace has saved
find .peace -type f -exec bash -c 'echo \# {}; cat {}; echo' \;

# Clean up the metadata directory
rm -rf .peace
```

### WASM

The `download` example can be built as a web assembly application using [`wasm-pack`]:

```bash
cd examples/download
wasm-pack build --target web
```

In the `examples/download` directory, start an [HTTP server], and open <http://localhost:8000/>:

```bash
python3 -m http.server 8000 # or
simple-http-server --nocache --port 8000 -i
```


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


[#42]: https://github.com/azriel91/peace/issues/42
[`fn_graph`]: https://github.com/azriel91/fn_graph
[`miette`]: https://github.com/zkat/miette
[`wasm-pack`]: https://rustwasm.github.io/
[HTTP server]: https://crates.io/crates/simple-http-server
