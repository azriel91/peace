# Output Presentation

For human readable output, it is desirable for consumers as well as the framework to be able to:

* output arbitrary types
* have them formatted by the output implementation
* without needing the output implementation to know the specific type that is being output.

The `OutputWrite` trait was written to handle different output formats &ndash; human-readable vs CI logging vs structured text &ndash; but not how to *present* the output.

Peace should provide two traits:

* `OutputWrite`: Maps between the information and the output format, e.g. human readable vs parseable.

    Examples:

    - **CLI output:** Writes progress as log messages (CI), or progress bar (interactive), or nothing (when piped).
    - **Web output:** Write JSON.
    - **Native application:** Update a UI component.

* `OutputFormatter`: Maps between the information and the display format, e.g. write this as an ID, or short text, or long text.

    Examples:

    - **CLI output:** Colour text with ANSI colour codes.
    - **Web output:** Create and style web elements.
    - **Native application:** Create and style UI components.


## Current State

The `OutputWrite` trait has methods for:

* writing progress: setting up state, updating progress, and ending.
* writing states (current, desired, diff, etc.)
* writing error

These methods are specific to `State`s, and if we add methods per type, it doesn't allow any arbitrary type to be formatted and written.


## Desired State

To be usable with arbitrary information, `OutputWrite` should have methods to output different *kinds* of information. These information *kinds* are based on the purpose of the information, not on how they should be grouped or presented.

### Information Kinds

* **Progress:** Information about the execution of automation.
* **Outcome:** Information that the automation is purposed to produce.
* **Notes:** Meta information about the outcome or progress -- informatives, warnings.

    These can be used to refine the automation.

For each information kind, `OutputWrite` should be able to:

* Write one or many of that information kind
* Reason over the parameters of that information, and potentially pass it to a formatter.

For structured output, all information should be serializable.


### Presentation / Formatting

For human readable output to be *understandable*, the `OutputWrite` implementation should improve clarity by adding styling or reducing information overload. For this to work with arbitrary types, the `OutputWrite` needs additional hints to determine how to format the information.

Examples:

* An object may be presented as a list, and the type needs to define which fields that list is built from.
* When presenting a list of named items, the type needs to define both the name and the description, which allows the names to be styled differently to the descriptions.
* When presenting a large object, the density of information can be reduced through collapsible sections, and more detail displayed when the sections are expanded.

#### Implementation

To achieve this, we can:

* Define a `peace::fmt::Presentable` trait, analogous to `std::fmt::Display`
* Define a `peace::fmt::Presenter` struct, analogous to `std::fmt::Formatter`
* `Presenter` has methods to format:

    - short text descriptions
    - long text descriptions (e.g. always split at `\n`s)
    - names, e.g. always bold
    - lists of `Presentable`s
    - groups, e.g. always collapsible, or presenter may choose to not display if level of detail is too high

* Implementors will `impl Presentable for MyType`. This can be made easier with a derive macro.
* Update `OutputWrite` to take in `&dyn Presentable` instead of concrete types, and the `OutputWrite` implementation can decide whether or not to delegate to `Presenter` for presentation information. e.g. a serializing output write may not need to.

**Note:** Structured output that is read by humans (e.g. prettified YAML or JSON) is not a `peace::fmt::Presentable` concern, but an `OutputWrite` parameter, as it is a standard format serialization parameter, not formatting hints that the output endpoint needs.


Instead of using `&str`s for what is presented, we could add type safety to:

* Enforce certain constraints, e.g. short descriptions must be one line, less than 200 characters
* For human readable output, instead of `std::fmt::Display`, types implement `peace::fmt::Presentable` trait where a `peace::fmt::Presenter` is passed in.

The type safety can be opt-in, e.g. allow `&str`s, but if using the type-safe wrappers, you get compilation errors when the constraints are not met.


#### Recursion

If the `Presentable` trait is recursive like `Debug`, then we need to make sure implementors understand that a "name" will always be styled as a name, unless one creates a wrapping type that does not delegate to the name's underlying `Presentable` implementation (just like `Debug`).
