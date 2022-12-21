# State

In `peace`, `State` represents the values of an item, and has the following usages:

* Showing users the state of an item.
* Allowing users to describe the state that an item should be.
* Determining what needs to change between the current state and the desired state.

Therefore, `State` should be:

* Serializable
* Displayable in a human readable format
* Relatively lightweight &ndash; e.g. does not necessarily contain file contents, but a hash of it.

State is separated into two parts:

* **Logical state:** The part that can be specified by the user, and controlled by automation.
* **Physical state:** The part that is computed / generated, and is generally not controllable.

Whether something is logical or physical state depends on whether it is managed by the user / code, or if it is computed / generated based on time, or by an external service beyond the control of the item spec.

Examples of logical state are:

* Contents of a file download
* Whether a certificate is imported
* Whether an application is compiled
* Deterministic file names
* Deterministic IP addresses

Examples of physical state are:

* Calculated file names
* Randomly allocated IP addresses

The pages in this section explain how to define the logical and physical state types when implementing an item spec.
