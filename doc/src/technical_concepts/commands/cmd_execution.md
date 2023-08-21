# Cmd Execution

When we run a `CmdExecution`, possible outcomes are:

1. In a `CmdBlock`, failure within one or more items.
2. In a `CmdBlock`, failure within block code.
3. Interruption / partial execution.
4. Successful execution of all blocks.

Each `CmdBlock` has its own block outcome type, and its own .


## Framework

### Use Cases: Framework Implementor

#### `*Cmd` Impls

* It would be nice to not have to write interruption handling code in every `CmdBlock`.
* Also, having `CmdBlock`s abstracted allows timings to be collected per block in a common place.

#### What does a `CmdExecution` implementor want to do?

* `DiffCmd`:
    - `StateDiscoveryBlock`: Return item failures to caller.
    - `StateDiscoveryBlock`: Return block failure / interruption to caller.
    - `DiffBlock`: Return item failures to caller.
    - `DiffBlock`: Return block failure / interruption to caller.

* `EnsureCmd`:
    - `StateDiscoveryBlock`: Return item failures to caller.
    - `StateDiscoveryBlock`: Return block failure / interruption to caller.
    - `DiffBlock`: Return item failures to caller.
    - `DiffBlock`: Return block failure / interruption to caller.
    - `EnsureBlock`: Serialize `States` on item failure, return `States` and item failures to caller.
    - `EnsureBlock`: Return block failure / interruption to caller.
    - `CleanBlock`: Serialize `States` on item failure, return `States` and item failures to caller.
    - `CleanBlock`: Return block failure / interruption to caller.
    - `EnsureBlock`: ditto.
    - `CleanBlock`: ditto.

    The desired return type would be:

    - `StatesEnsured` on success.

* How should we reply to the caller?
    - `CmdBlock` block level error.
    - `IndexMap<ItemId, E>` item level error.

    Error handlers per `CmdBlock` will do any serialization work if desired.

### Use Cases: Automation Developer / End User

* Store, retrieve, and display execution report.


## How do we do this in code?

What it could look like conceptually:

### `DiffCmd`

```rust
let cmd_execution = CmdExecution::builder()
    .with_block(StatesDiscoverCmdBlock::<CurrentAndGoal>::new())
    .with_block(DiffCmdBlock::new())
    .build()
    .await;
```

### `EnsureCmd` -- "simple" version

```rust
let cmd_execution = CmdExecution::builder()
    .with_block(StatesCurrentReadCmdBlock::new())
    .with_block(StatesDiscoverCmdBlock::new(Discover::Current))
    // Compares current with stored, and fails if
    // current stored doesn't match current discovered.
    .with_block(ApplyGuardCmdBlock::new())  // `Outcome` and `OutcomeAcc` are `()`
    .with_block(ApplyCmdBlock::new())       // state_goal, diff and apply.
    .build()
    .await;
```


### `EnsureCmd` -- "complex" version

```rust
let cmd_execution = CmdExecution::builder()
    .with_block(StatesCurrentReadCmdBlock::new())
    .with_block(StatesDiscoverCmdBlock::new(Discover::Current))
    .with_block(ApplyGuardCmdBlock::new())
    .with_block(EnsureCmdBlock::new(item_ids_no_blockage))  // state_goal, diff and apply.
    .with_block(CleanCmdBlock::new(item_ids_blocking))    // state_clean, diff and apply.
    .with_block(EnsureCmdBlock::new(item_ids_unblocked))  // state_goal, diff and apply.
    .with_block(CleanCmdBlock::new(item_ids_obsolete))    // state_clean, diff and apply.
    .build()
    .await;

// `item_ids_unblocked` is a superset of the `item_ids_blocking`.
//
// ⚠️ We could have a more granular design version that starts ensuring unblocked items
// while `cleaning` is happening.
```


### Error Handling and Interruptions

When a `CmdBlock` fails, we need to consider what to do with:

* `CmdBlock::OutcomeAcc`.
* item errors.
* `CmdBlock` error.
* interruptions.

Originally an additional closure argument next to each `CmdBlock` argument was considered:

```rust ,ignore
|_input, _outcome_acc, cmd_block_error| async move { cmd_block_error }
```

For example:

```rust ,ignore
|(_states_current, _states_goal), states_ensured_mut, apply_error| async move {
    let serialize_result = StatesSerializer::serialize(states_ensured_mut).await;

    apply_error.or(serialize_result)
},
```

However, most usages would just pass through the error.

Before this `CmdExecution` design, `ApplyCmd` would:

1. In the producer, send the intermediate result to the outcome collator and stop execution ([apply_cmd.rs#L699-L710](https://github.com/azriel91/peace/blob/0.0.11/crate/rt/src/cmds/sub/apply_cmd.rs#L699-L710)).
2. In the collator:

    - Intermediate `CmdBlock` errors are returned as is. ([apply_cmd.rs#L521-L525](https://github.com/azriel91/peace/blob/0.0.11/crate/rt/src/cmds/sub/apply_cmd.rs#L521-L525)).
    - Intermediate discovery errors are collated into the `CmdExecution` outcome type ([apply_cmd.rs#L526-L558](https://github.com/azriel91/peace/blob/0.0.11/crate/rt/src/cmds/sub/apply_cmd.rs#L526-L558)).


From the use cases, what we care about are:

* Returning `CmdBlock` errors to the caller.
* Returning item errors to the caller.
* Collating intermediate `CmdBlock` item values into the `CmdExecution` outcome type if possible.
* Serializing `States` to storage: `EnsureCmdBlock` and `CleanCmdBlock` should still serialize each item's current state, on failure / interruption / success.
* Serializing the `CmdBlock::Outcome` in the execution outcome report.

Deferred:

* Serializing the `CmdBlock::OutcomeAcc` in the execution outcome report.
* Deserializing the `CmdBlock::OutcomeAcc` to be displayed.


## Interruptions

1. Interrupt before execution.
2. Interrupt within a block.
3. Interrupt between blocks.
4. Interrupt after last block.
