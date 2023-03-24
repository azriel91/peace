# 🕊️ peace &ndash; zero stress automation

[![Crates.io](https://img.shields.io/crates/v/peace.svg)](https://crates.io/crates/peace)
[![docs.rs](https://img.shields.io/docsrs/peace)](https://docs.rs/peace)
[![CI](https://github.com/azriel91/peace/workflows/CI/badge.svg)](https://github.com/azriel91/peace/actions/workflows/ci.yml)
[![Coverage Status](https://codecov.io/gh/azriel91/peace/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/peace)

`peace` is a framework to build empathetic and forgiving software automation.

See:

* [peace.mk](https://peace.mk) for the project vision.
* [Background](https://peace.mk/book/background.html) for the motivation to create this framework.
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

* 🟢 **Idempotent:** Multiple invocations result in the desired outcome.
* 🟢 **Clean:** Every item creation is paired with how it is cleaned up.
* 🟢 **Understandable:** Progress is shown at an understandable level of detail.
* 🔵 **Understandable:** Error reporting is compatible with [`miette`].
* 🟡 **Interruptible:** Execution can be interrupted.
* 🟢 **Resumable:** Automation resumes where it was interrupted.
* 🟢 **Diffable:** States and diffs are serialized as YAML.
* 🟢 **Efficient:** Tasks are concurrently executed via [`fn_graph`].
* 🟢 **Namespaced:** Profile directories isolate environments from each other.
* 🟢 **Type Safe:** Items and parameters are defined in code, not configuration.


### Roadmap

* 🟢 Define items to manage with automation.
* 🟢 Define dependencies between items.
* 🟢 Define "apply" logic.
* 🟢 Define "clean up" logic.
* 🟢 Discover current and desired states.
* 🟢 Define diff calculation between states.
* 🟢 Store and recall parameters across commands.
* 🟡 Feature-gated incremental functionality.
* 🟡 Off-the-shelf support for common items.
* 🟡 Dry run.
* 🟣 WASM support.
* ⚫ Referential parameters ([#94]) -- ability to specify usage of values generated during automation as parameters to subsequent items.
* ⚫ Cancel-safe interruption via [`tokio-graceful-shutdown`].
* ⚫ Diff states between multiple profiles.
* ⚫ Secure-by-design Support: Encrypted value storage, decrypted per execution / time based agent.
* ⚫ Tutorial for writing a software lifecycle management tool.
* ⚫ Built-in application execution methods -- CLI, web service.
* ⚫ `peace` binary for configuration based workflows.
* ⚫ Web based UI with interactive graph.
* ⚫ Agent mode to run `peace` on servers (Web API invocation).

Further ideas:

* Back up current state.
* Restore previous state.
* Telemetry / metrics logging for analysis.


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
[#94]: https://github.com/azriel91/peace/issues/94
[`fn_graph`]: https://github.com/azriel91/fn_graph
[`miette`]: https://github.com/zkat/miette
[`tokio-graceful-shutdown`]: https://docs.rs/tokio-graceful-shutdown/latest/tokio_graceful_shutdown/
[`wasm-pack`]: https://rustwasm.github.io/
[HTTP server]: https://crates.io/crates/simple-http-server
