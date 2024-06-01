# Seamless Execution

Seamless execution when multiple flow versions exist requires handling the following:

* Reading and writing (serialization) stored state / outcomes from different flow versions.
* Reading and writing to (adding, removing, modifying) the items from different flow versions.


## Stored State

Stored state is accessed when:

* `CmdCtx` is built: Read and write.
* Commands are run:
    - `StatesCurrentStoredReadCmd`, `StatesGoalReadCmd`, `ExecutionHistoryListCmd` / `ExecutionHistoryShowCmd`: Read.
    - `StatesDiscoverCmd`, `EnsureCmd`, `CleanCmd`: Write.


### `CmdCtx` Build

For cmd ctx build, because state could be in any version (current or any previous version), deserialization needs to be stateful -- like how `type_reg::untagged::TypeReg` is done.

```rust ,ignore
fn envman_cmd_ctx() {
    let flow_versions = FlowVersions::builder()
        .add_flow(flow_v1)
        .add_flow(flow_v2)
        .add_flow(flow_v3)
        .build();

    let cmd_ctx = CmdCtx::builder_single_profile_single_flow(&mut output, &workspace)
        // .with_flow(flow)
        .with_flow_versions(flow_versions)
        .await?;
}
```


### Cmd States and Outcome: Read

`StatesCurrentStoredReadCmd` and `StatesGoalReadCmd` needs to read states from any supported flow version.

`ExecutionHistoryListCmd` / `ExecutionHistoryShowCmd` (not implemented yet) need to read outcomes (and hence states and diffs) from any previous flow version.


### Cmd States and Outcome: Write

* `StatesDiscoverCmd`: Some overlap with migrating stored states to latest format.
* `EnsureCmd` / `CleanCmd`: Write new version after ensuring.

    What about interruptions?

    We'd probably have to write the version of each item state then. Enum variant per version solves this.


```rust ,ignore
/// Indicates when stored state can be upgraded.
///
/// Note: Stored outcomes also contain state, so for an upgrade between two
/// given versions, an `OutcomeUpgradeReq` will be the same as this.
///
/// However, should we mutate stored outcomes? I imagine not.
enum StateUpgradeReq {
    /// Data can be upgraded without discovering state.
    None,
    /// State needs to be discovered in order to store the new version.
    ///
    /// Existing stored state doesn't carry enough information to construct the
    /// new version, but the existing item does.
    Discover,
    /// Item needs an ensure / clean to be run to store the new version.
    ///
    /// The existing item doesn't carry enough information to construct the new
    /// state, and needs `apply` to be run to do so.
    ///
    /// This means `ApplyCmd`, and hence `Item{,Rt,Wrapper}`, need to work with
    /// different state versions.
    Apply,
}
```


## Items: Use Latest / Migrate to Latest

An item upgrade may be one of:

* Only modifying the data type, and changing the information stored about the item (see `StateUpgradeReq::Discover` above).
* Modifying the item in place.
* Cleaning up the item then re-ensuring it.

Notes:

* It may not be possible to migrate a step to a newer version without cleaning up its successors.
* It is possible if successors don't "live inside" the item, but only need their parameters changed.
* it is not possible if successors live inside the item, and need to be deleted in order to modify the item.

Whether or not a successor needs to be cleaned up likely should be encoded into the `Edge` type, not part of the item upgrade parameters -- a step is unable to know if its successors live inside it or point to it.

```rust ,ignore
/// Indicates what is needed for an instance of a step to be upgraded.
enum ItemUpgradeReq {
    /// Item instance does not need to be upgraded, i.e. only state data type is changed.
    None,
    /// Item can and needs to be re-ensured as part of this upgrade.
    Modify,
    /// Item needs to be cleaned and re-ensured as part of this upgrade.
    Replace,
}
```


## Running The Upgrade Code

The following use cases seem natural for developers / users:

1. Manually upgrade state after automation software has been updated.
2. Automatically upgrade state as part of using regular commands.
3. Manually upgrade items after automation software has been updated.
4. Automatically upgrade items as part of using regular commands.

Some use cases are mutually exclusive, though mutually exclusive may still exist in the same automation software, for different version or kinds of upgrades.

For example, automatic item upgrades as part of using regular commands are appropriate when nothing needs to be cleaned up, but manual item upgrades should be used when some items need to be cleaned as part of the upgrade.


### Manual State Upgrade

Developers must provide an upgrade command, to call `StateUpgradeCmd::exec`.

```rust ,ignore
impl StateUpgradeCmd {
    /// # Note
    ///
    /// This is single profile single flow.
    ///
    /// To upgrade the states of multiple profiles and multiple flows, this
    /// must be called per flow, and maybe per profile as well.
    ///
    /// `MultiProfileSingleFlow` may only be safe for `StateUpgradeReq::None`
    /// and `StateUpgradeReq::Discover`.
    async fn state_upgrade_exec(cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow>)
    -> Result<(), Error> {
        match state_upgrade_req {
            StateUpgradeReq::None => {
                let states = resources.get::<StatesCurrentStored>();
                let states_upgraded = flow
                    .graph()
                    .iter()
                    .for_each(|item| item.state_upgrade(states));
                states_serializer.serialize(&states_upgraded).await?;
            }
            StateUpgradeReq::Discover => {
                let CmdOutcome {
                    value: (states_current, states_goal),
                    errors,
                } = StatesDiscoverCmd::current_and_goal(&mut cmd_ctx).await?;
                states_serializer.serialize_current(&states_upgraded).await?;
                states_serializer.serialize_goal(&states_upgraded).await?;
            }
            StateUpgradeReq::Apply => {
                let CmdOutcome {
                    value: states_upgraded,
                    errors,
                } = EnsureCmd::exec_with(&mut cmd_ctx.as_sub_cmd()).await?;
            }
        }
    }
}
```


### Automatic State Upgrade

The above method, but called automatically from a set of chosen commands: `EnsureCmd`, `CleanCmd`.

May need to think about this more carefully.


### Manual Item Upgrade

Developers must provide an upgrade command.

For actual item modification, it may be one of:

```rust ,ignore
// Only discovery needed
fn env_ensure_cmd() {
    env_flow_current_version_ensure();
    env_flow_ensure();
}

// apply needed
fn env_ensure_cmd() {
    // Item implementation needs to handle upgrade from whatever version the
    // state is in, to the current version.
    //
    // Meaning, there is no API support / constraints from the `peace`
    // framework. It's all handled from within the item implementation.
    env_flow_ensure();
}
```


### Automatic Item Upgrade

Same as manual item upgrade, except the upgrade command is not separate to the regular ensure command.

---

For item upgrades, state could always be an enum, and `peace` doesn't include API support, constraints, or special handling for migrating between item versions -- the current building blocks are enough to implement seamless upgrades if items are implemented well.

For flow upgrades, we still need to handle clean up of old items no longer needed in subsequent flow versions, meaning the items must still be included in the development tool for their clean up logic.

For the developer, this could be either:

* Passing in the previous and current flow versions, and the framework working out which items need to be cleaned up.
* Passing in the current flow version, and old items and params for the framework to clean up those items.

---

Is this enough to start working on `StatesSerde`?

Yes:

* `StatesSerde` should hold unknown entries (removed items).
* `States` should be mapped `From<StatesSerde>` after deserialization.
* Users should be informed when there are unknown entries.
* `States` should be mapped `Into<StatesSerde>`, with an entry for each item in the flow.
