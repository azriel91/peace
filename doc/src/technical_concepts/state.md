# State

In `peace`, [`State`] represents the values of an item, and has the following usages:

* Showing users the state of an item.
* Allowing users to describe the state that an item should be.
* Determining what needs to change between the current state and the desired state.

Therefore, `State` should be:

* Serializable
* Displayable in a human readable format
* Relatively lightweight &ndash; e.g. does not necessarily contain file contents, but a hash of it.


## Logical and Physical State

State can be separated into two parts:

* **Logical state:** Information that is functionally important, and can be specified by the user ahead of time.

    Examples of logical state are:

    - File contents
    - An application version
    - Server operating system version
    - Server CPU capacity
    - Server RAM capacity

* **Physical state:** Information that is discovered / produced when the automation is executed.

    Examples of physical state are:

    - ETag of a downloaded file.
    - Execution time of a command.
    - Server ID that is generated on launch.
    - Server IP address.


## Defining State

### Fully Logical

If an item's state can be fully described before the item exists, and can be made to happen without interacting with an external service, then the state is fully logical.

For example, copying a file from one directory to another. The state of the file in the source directory and destination directories are fully discoverable, and there is no information generated during automation that is needed to determine if the states are equivalent.


### Logical and Physical

If an item's desired state can be described before the item exists, but interacts with an external service which produces additional information to bring that desired state into existence, then the state has both logical and physical parts.

For example, launching a server or virtual machine. The operating system, CPU capacity, and RAM are logical information, and can be determined ahead of time. However, the server ID and IP address are produced by the virtual machine service provider, which is physical state.


### Fully Physical

If an item's desired state is simply, "automation has been executed after these files have been modified", then the state has no logical component.

For example, running a compilation command only if the compilation artifact doesn't exist, or the source files have changed since the last time the compilation has been executed.


---

The remaining pages in this section explain how to define the logical and physical state types when implementing an item spec.

[`State`]: https://docs.rs/peace_cfg/latest/peace_cfg/state/struct.State.html
