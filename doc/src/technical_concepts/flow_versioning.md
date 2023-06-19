# Flow Versioning

A flow's version needs to be increased whenever:

* **Item:** An associated data type is changed -- `State`, `Diff`, `Params`.
* **Flow:** An item is added.
* **Flow:** An item is removed.
* **Flow:** An item is renamed.


## Flow Version Maintenance

For developers, maintaining multiple versions of flows is a cost, and usually one that is necessary. As items are created, modified, and removed throughout a flow's evolution, automation software needs to manage stored information from previous versions of the flow.

There are a number of ways to manage and ship flow versions:

* **One version:** Only ship with one flow version.

    Items never persist beyond the lifetime of the automation software.

    Suitable for once off flows where ensure (and clean up) are done with the same version of the automation software.

* **Blue-green versioning:** Ship with the previous and the next version.

    Automation software is compatible with the previous version of each flow, as well as the next.

    All stored information must be upgraded to the newer version before a new flow version is shipped.

    Suitable for flows that do not evolve often, and not all previous versions of flows need to be supported.

* **`n`-versioning:** Ship with `n`-versions of flows.

    `n` may be all flows.

    Each flow's state will need to adhere to the following, mainly for deserialization:

    - Either never evolve its data type, or be an enum, with a variant per version / evolution.
    - Ship with upgrade code from old versions to the current version.

    This is the highest cost in terms of code maintenance, and the shipped binary may grow unreasonably large over time.


## Flow Upgrades

### Upgrade Strategies

For blue-green / `n` versioning, there are a number of strategies for upgrading stored information in the flow:

* **Automatic upgrade:**

    - Implementors and developers need to ship with `v1 -> v2` migration code.
    - Simplest for users, assuming migration path is well understood / no surprises.
    - Developers may want to diff this.

* **Manual upgrade:**

    - Implementors and developers need to ship with `v1 -> v2` migration code.
    - Users have to choose to migrate.
    - Developers and users may want to diff things.

For `n`-versioning, upgrading from `v1 -> v3` should use `v1 -> v2 -> v3`, so that developers don't have to write migration code form every version to the latest.

A flow upgrade may take the form of:

1. Clean some items.
2. Modify some items.

For actual item modification, the following needs to happen:

```rust ,ignore
fn env_ensure_cmd() {
    env_flow_current_version_ensure();
    env_flow_ensure();
}
```

For cmd ctx build, because state is in the previous version, it needs to be deserialized with the previous version.

TODO: figure this out.


## Execution History

To render old either we have one standard format that doesn't need old data types to present, or we ship those types.

Makes sense to have a standard format -- shipping with previous data types creates bloat, and makes it impossible to support execution history for shipping single versions of flows.
