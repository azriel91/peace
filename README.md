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

| Symbol | Meaning          |
| :----: | ---------------- |
|   🟢   | Supported        |
|   🟡   | Work in progress |
|   ⚫   | Planned          |

* 🟢 Fetch current state.
* 🟢 Fetch desired state.
* 🟢 Workflow graph with task dependencies
* 🟢 Concurrent task execution
* 🟢 Dry run
* 🟢 Skip unnecessary work
* ⚫ Understandable error reporting
* ⚫ Feature-gated incremental functionality
* ⚫ Built-in application execution methods -- CLI, web service
* ⚫ Understandable progress
* ⚫ Actionable error messages
* ⚫ Namespaced working directory
* ⚫ Resource clean up
* ⚫ `peace` binary for configuration based workflows
* ⚫ Off-the-shelf support for common tasks
* ⚫ Web based UI
* ⚫ Agent mode to run `peace` on servers (Web API invocation)

Further ideas:

* Back up current state
* Restore previous state
* Telemetry logging for monitoring
* Metrics collection for analysis


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
