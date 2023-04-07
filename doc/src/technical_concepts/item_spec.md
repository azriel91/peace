# Item Spec

> **Note:** 🚧 means the concept is not yet implemented.

An **item** is something that can be created, inspected, and cleaned up by automation.

An item specification defines data types and logic to manage that item.

The [`ItemSpec`][`ItemSpec`] and associated types are how consumers integrate with the Peace framework. Consumers provide a unique ID, data types, and functions, which will be selectively executed by the framework to provide lean, robust automation, and a good user experience.

This logical breakdown guides automation developers to structure logic to handle cases that are not normally considered of when writing automation. Combined with trait requirements on data types, the framework is able to provide commands, workflow optimizations, and understandable output to ensure a pleasant automation experience.


## Data Types

1. **Error:** The umbrella error type returned when *anything* goes wrong when managing the item.
2. **State:** Information about a managed item, can be divided into logical and physical state.
3. **Logical state:** Part of the state that can be controlled, e.g. application server's existence.
4. **Physical state:** Part of the state that cannot be controlled, e.g. application server's instance ID.
5. **Current state:** Current **State** of the managed item, both logical and physical.
6. **Desired state:** **Logical State** that one wants the item to be in.
7. **State difference:** Difference between the current **Logical state** and the **Desired state**.


## Logic &ndash; Building Blocks

<details>
<summary>1. Define an ID.</summary>
<div>

`ItemSpec::id`

Provide the framework with a unique ID for this item.

These are intended to be safe to use as file names, as well as avoid surprises, and so have been limited to alphanumeric characters and underscores, and cannot begin with a number. This is validated at compile time by using the `item_spec_id!("..")` macro.

The examples in the `peace` repository will use `snake_case`, but the rules are flexible enough to accept `PascalCase` or `camelCase` if that is preferred.

### Examples

* Item spec that manages a file download: `"download"`.
* Item spec that manages a server: `"server_existence"`.

</div>
</details>

<details>
<summary>2. Initialize framework with parameters to manage item.</summary>
<div>

`ItemSpec::setup`

*🚧 parameters are passed in for each command*

<!-- We should take in a serializable type for initialization. Serializable because it will allow the item to be initialized on a separate host. -->

Provide the framework with enough information to begin managing the item.

This function also instantiates the data types referenced by this `ItemSpec` into the `Resources` map.

### Examples

* Item spec that manages a file download:

	Required parameters are the URL to download from, and the destination file path.

* Item spec that manages a server:

	Required parameters are the base image ID to launch the server with, and hardware specs.

</div>
</details>

<details>
<summary>3. Fetch current item state.</summary>
<div>

`ItemSpec::state_current`

This may not necessarily be a cheap operation, for example if it needs to make web requests that take seconds to complete.

### Examples

* Item spec that manages a file download:

	Current state is checking a file's existence and contents.

* Item spec that manages a server:

	Current state is checking a server's existence and its base image ID.

</div>
</details>

<details>
<summary>4. Fetch desired item state.</summary>
<div>

`ItemSpec::StateDesiredFn`

This may not necessarily be a cheap operation, for example if it needs to make web requests that take seconds to complete.

### Examples

* Item spec that manages a file download:

	Desired state is file metadata retrieved from a remote server.

* Item spec that manages a server:

	Desired state is one server exists with the specified the base image ID.

</div>
</details>

<details>
<summary>5. Return the difference between current and desired states.</summary>
<div>

`ItemSpec::StateDiffFn`

It is important that both the `from` and `to` are shown for values that have changed, and values that have not changed or are not relevant, are not returned.

### Examples

* Item spec that manages a file download:

	State difference is a change from a file that does not exist, to a file with contents `"abc"`.

* Item spec that manages a server:

	State difference is a change from a non-existent server, to a server exists with the specified the base image ID.

</div>
</details>

<details>
<summary>6. Ensure that the item is in the desired state.</summary>
<div>

Transforms the current state to the desired state.

1. `check`: Returns whether `exec` needs to be run to transform the current state into the desired state.
2. `exec`: Actual logic to transform the current state to the desired state.
3. `exec_dry`: Dry-run transform of the current state to the desired state.

	Like `exec`, but all interactions with external services, or writes to the file system should be substituted with mocks.

</div>
</details>

<details>
<summary>7. Clean up the item.</summary>
<div>

Cleans up the item from existence.

1. `check`: Returns whether `exec` needs to be run to clean up the item.
2. `exec`: Actual logic to clean up the item.
3. `exec_dry`: Dry-run clean up of the item.

</div>
</details>


## Comparison with `git`

Readers may notice the function breakdown is `git`-like. The following table compares the concepts:

| Subject                | Peace                                                    | Git                                                                                      |
|:-----------------------|:---------------------------------------------------------|:-----------------------------------------------------------------------------------------|
| Item                   | Any consumer defined item.                               | A directory of files.                                                                    |
| Project initialization | 🚧 `init` command takes in parameters to manage the item. | Uses the current directory or passed in directory.                                       |
| State                  | Consumer defined information about the item.             | Current state is the latest commit, desired state is the working directory.              |
| State retrieval        | 🚧 On request by user using the `StatesDiscover` command. | Retrieved each time the `status` command is run, cheap since it is all local.            |
| State display          | On request using `state` and `desired` commands          | `show $revision:path` command shows the state at a particular `$revision`.               |
| State difference       | On request using `diff` command                          | `status` command shows a summary, `show` and `diff` commands shows the state difference. |
| State application      | On request through the `ensure` command.                 | On request through `commit` and `push` commands.                                         |
| Environments           | 🚧 Handled by updating initialization parameters.         | Defined through `remote` URLs.                                                           |



[`ItemSpec`]: https://docs.rs/peace_cfg/latest/peace_cfg/trait.ItemSpec.html
