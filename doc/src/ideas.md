# Ideas

This page records ideas that I'd like, but there isn't enough mental capacity and time to design and work on them yet.

<details>
<summary>1. Abstraction over native storage and web storage &ndash; use IndexedDB instead of `WebStorage` APIs.</summary>
<div>
</div>
</details>

<details>
<summary>2. Graphical user interface that renders each flow's graph.</summary>
<div>

1. Each item is a node.
2. User can select which nodes to run &ndash; these may be a subset of the flow.
3. User can select beginning and ending nodes &ndash; and these can be in reverse order.

    <!--  -->

**Note:** Graphviz is compiled to WASM and published by [hpcc-systems/hpcc-js-wasm](https://github.com/hpcc-systems/hpcc-js-wasm). May be able to use that to render.

[graphviz-visual-editor](https://github.com/magjac/graphviz-visual-editor) is a library that allows basic editing of a graphviz graph. It's not yet developed to a point that is intuitive for users.

</div>
</details>

<details>
<summary>3. Tool that uses `peace` to check consumer code whether it adheres to best practices.</summary>
<div>
</div>
</details>
<details>
<summary>4. Clean Command Retains History</summary>
<div>

End users may want to see what was previously deployed.

If we retain a `${profile}/.history` directory with all previous execution information, it allows:

* Re-attempting clean up.
* Reporting on what was cleaned up.
* Computing costs of all executions

Perhaps we should make the API be, on `visit`, return a list of identifiers for things to clean up.

</div>
</details>
<details>
<summary>5. Types / proc macros to place constraints at compile time for aesthetic reports.</summary>
<div>

* short summary sentences
* 2 ~ 3 sentence paragraphs / word limit


```rust
/// An ID
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Id<'s>(Cow<'s, str>);

/// Single line description, hard limit of 200 characters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DescShort<'s>(Cow<'s, str>);
```

</div>
</details>
<details>
<summary>6. <code>MultiProfile</code> Cmd scope profile sort function</summary>
<div>

Users may want to sort profiles in the profile directory differently to their alphabetical / lexicographical sorting.

This may be dependent on profile params &ndash; sort env based on env type, last execution time &ndash; profile history.

</div>
</details>

<details>
<summary>7. Cancel-safe state storage</summary>
<div>

When an item ensure does multiple writes, there is a possibility of not all of those writes occur during execution:

* user interrupts the execution.
* internet connection drops.
* machine loses power.

In the last case, we cannot safely write state to disk, so a `StateCurrent` discover is needed to bring `StatesCurrentStored` up to date. However, the previous two cases, it is possible for `Item`s to return `State` that has been partially ensured, without making any further outgoing calls -- i.e. infer `StatesEnsured` based on the successful writes so far.

Note that this places a burden on the `Item` implementor to return the partial state ensured (which may conflict with keeping the `State` simple), as well as make the `ApplyFns::exec` return value more complex.

The trade off may not be worthwhile.

</div>
</details>

<details>
<summary>8. Adding / removing / modifying items in flows</summary>
<div>

Implementors may add/remove/modify items in flows.

Peace needs to be designed such that these changes do not cause already-existent flows to not be loadable, i.e. when:

* `states_*.yaml` contains state for which an item no longer exists in the flow.
* `states_*.yaml` does not contain state for an item that is newly added to the flow.
* `states_*.yaml` contains state whose fields are different to a new version of an item.

    This one can be addressed by having `State` be an enum, with versioned variants.

Also see 14.

</div>
</details>

<details>
<summary>9. Item expiry</summary>
<div>

For items that cost, it is useful to have an expiry time that causes it to be deleted.

* This would have to be supported by the service that hosts the item.
* There should be a way to notify the user of items that are about to expire.
* There should also be a way to extend the item expiry times easily.

</div>
</details>

<details>
<summary>10. Interrupt / cancel safety</summary>
<div>

The [`tokio-graceful-shutdown`] library can be used to introduce interrupt safety into item executions. This is particularly useful for write functions.

See the [`is_shutdown_requested`] method in particular.

[`tokio-graceful-shutdown`]: https://github.com/Finomnis/tokio-graceful-shutdown
[`is_shutdown_requested`]: https://docs.rs/tokio-graceful-shutdown/latest/tokio_graceful_shutdown/struct.SubsystemHandle.html#method.is_shutdown_requested

</div>
</details>

<details>
<summary>11. Diffable item params</summary>
<div>

`DiffCmd` originally was written to diff the current and goal states. However, with the second use case of "diff states between two profiles", it is also apparent that other related functionality is useful:

* Diff profile params / flow params.
* Diff item params between profiles for a given flow.

Because of diffable params, and [#94], the `Item` should likely have:

* `type Params: ItemParams + Serialize + DeserializeOwned`.
* feature gated `fn item_params_diff(..)`.

`fn item_params_diff(..)` should likely have a similar signature to `fn state_diff(..)`, whereby if one uses  `XData<'_>`, the other should as well for consistency:

* For `MultiProfileSingleFlow` commands, a diff for item params which contains a referential value (e.g. "use the `some_predecessor.ip_address()`") may(?) need information about `some_predecessor` through `Resources` / `Data`.

We should work out the design of that before settling on what `state_diff` and `item_params_diff`'s function parameters will be. See **Design Thoughts** on [#94] for how it may look like.

</div>
</details>

<details>
<summary>12. Default item params</summary>
<div>

An `Item`'s params may not necessarily be mandatory. From the `Params` type (and corresponding trait), Peace may:

* Insert default param values, if the `Item` implementor provides a default
* Still make the params required if there is no default.
* Provide a way for `ParamsSpec` for each field to be the default, or a mapping function.

</div>
</details>

<details>
<summary>13. Upgrades: Tolerate optional or different workspace params / item params / states</summary>
<div>

When new workspace params are added, or new items are added to a flow, existing `*_params.yaml`, `item_params.yaml`, and `states_*.yaml` may not contain values for those newly added params / items.

Automation software should be able to:

* Work with missing parameters.
* Work with changed parameter types.

When workspace params / items are removed from a flow, leftover params / state are no longer used. However, we may want to do one of:

* Notify the user to clean up unused params
* Peace should ignore it
* Inform the automator to still register the item, so that old execution may be loaded.

</div>
</details>

<details>
<summary>14. Store params per execution, pass previous execution's params to clean cmd</summary>
<div>

Instead of requiring `Item::State` to store the params used when applied, maybe we should store the params used in the last ensure alongside that item's state.

Users are concerned with the current state of the item. They also may be concerned with the parameters used to produce that state. Requiring item implementors to store paths / IP addresses within the state that has been ensured feels like unnecessary duplication.

However, when comparing diffs, we would hope either:

* The params used to discover the current and goal states are the same, or
* The "params and states" pairs are both compared.

Also:

* `apply_check` needs to have both the old and new params to determine whether apply needs to be executed.
* `State` as the output API, should not necessarily include params.
* When parameters change, and an apply is interrupted, then we may have earlier items using the new parameters, and later items still on the previous parameters. More complicated still, is if parameters change *in the middle of an interruption*, and re-applied.

Perhaps there should be a `(dest_parameters, Item::State)` current state, and a `(src_parameters, Item::State)` goal state. That makes sense for file downloads if we care about cleaning up the previous `dest_path`, to move a file to the new `dest_path`.

Or, all dest parameters should be in `Item::State`, because that's what's needed to know if something needs to change.

Another thought:

`states_*.yaml` should store this per item:

* params used for apply
* values resolved for those params
* current state

Also see 8.

</div>
</details>

<details>
<summary>15. Use <a href="https://openlayers.org/">openlayers</a> for tiling level of detail</summary>
<div>

Generate dot diagram using graphviz with full resolution, and then convert to tiles, then display different styling depending on the state of each item.

</div>
</details>

<details>
<summary>16. Combine <code>data</code> and <code>params{,_partial}</code> into <code>FnCtx</code></summary>
<div>

`Item` functions take in `FnCtx`, `data`, and item `params` as separate arguments.

This was done to:

* Reduce the additional layer to get `Item::Params`, or `Item::ParamsPartial`.
* Avoid progress sender from being passed in to function that didn't need it.

However, functions don't necessarily need runtime `fn_ctx` or `data`, making it noise in the signature.

Should we combine all 3 into `FnCtx`? It would make `FnCtx` type parameterized over `Params` and `ParamsPartial`.

</div>
</details>

<details>
<summary>16. Combine <code>data</code> and <code>params{,_partial}</code> into <code>FnCtx</code></summary>
<div>

`Item` functions take in `FnCtx`, `data`, and item `params` as separate arguments.

This was done to:

* Reduce the additional layer to get `Item::Params`, or `Item::ParamsPartial`.
* Avoid progress sender from being passed in to function that didn't need it.

However, functions don't necessarily need runtime `fn_ctx` or `data`, making it noise in the signature.

Should we combine all 3 into `FnCtx`? It would make `FnCtx` type parameterized over `Params` and `ParamsPartial`.

</div>
</details>

<details>
<summary>17. Style edges / items red when an error occurs.</summary>
<div>

When we hit an error, can we go through parameters / states to determine whether the error is to do with an item itself, or a link between the item and its predecessor?

Then style that link red.

</div>
</details>

<details>
<summary>18. Markdown text instead of <code>Presentable</code>.</summary>
<div>

Instead of requiring developers to `impl Presentable for` all the different types that use, and use different `Presentable` methods, we could require them to implement `Display`, using a markdown string.

Then, for different `OutputWrite` implementations, we would do something like this:

* `CliOutput`: `syntect` highlight things.
* `WebiOutput`: Use commonmark to generate HTML elements, and with 19, use that as part of each item node's content.

</div>
</details>

<details>
<summary>19. HTML with SVG arrows and flexbox instead of <code>dot</code>.</summary>
<div>

Instead of using `dot`, we just use flexbox and generate arrows between HTML `div`s.

This trades 

</div>
</details>


## Notes

<details>
<summary>1. SSH-like on Windows</summary>
<div>

* psexec
* [Windows powershell and WinRM](https://stackoverflow.com/questions/10237083/how-to-programmatically-remotely-execute-a-program-in-ec2-windows-instance/13284313#13284313)

</div>
</details>

<details>
<summary>2. Learnings from envman end-to-end implementation.</summary>
<div>

1. Referential lookup of values in state / item params. ([#94])
2. AWS SDK is not WASM ready -- includes `mio` unconditionally through `tokio` (calls UDP). ([aws-sdk-rust#59])
3. AWS SDK does not always include error detail -- S3 `head_object`. ([aws-sdk-rust#227])
4. Progress output should enable-able for state current / goal discover / clean functions.
5. Flow params are annoying to register every time we add another item. Maybe split end user provided params from item params.
6. Blank item needs a lot of rework to be easier to implement an item. ([67], [#96])
7. For `ApplyCmd`, collect `StateCurrent`, `StateGoal`, `StateDiff` in execution report.
8. AWS errors' `code` and `message` should be shown to the user.
9. Progress limit should not be returned in `ApplyFns::check`, but sent through `progress_sender.limit(ProgressLimit)`. This simplifies `check`, and allows state current/goal discovery to set the limits easily.
10. Consolidate `StatesDiscoverCmd` and `ApplyCmd`, so the outcome of a command is generic. Maybe use a trait and structs, instead of enum variants and hardcoded inlined functions, so that it is extendable.
11. Add an `ListKeysAligned` presentable type so `Presenter`s can align keys of a list dynamically.
12. Remove the `peace_cfg::State` type.
13. Contextual presentable strings, for states and diffs.

    What command is this called for:

    - state current: "is .."
    - goal state: "should be .."
    - diff between current and goal: "will change from .. to .."
    - diff between current and cleaned: "will change from .. to .."
    - diff between two profiles' current states: : "left is .., right is .."

    Maybe we don't burden the presenter implementation, but Peace will insert the contextual words

14. Easy API functions for diffing -- current vs goal, between profiles' current states.
15. What about diffing states of different state versions?

    Maybe this is already taken care of -- `state_diff` is already passed in both `State`s, so implementors had to manage it already.


[#67]: https://github.com/azriel91/peace/issues/67
[#94]: https://github.com/azriel91/peace/issues/94
[#96]: https://github.com/azriel91/peace/issues/96
[aws-sdk-rust#59]: https://github.com/awslabs/aws-sdk-rust/issues/59
[aws-sdk-rust#227]: https://github.com/awslabs/aws-sdk-rust/issues/227

</div>
</details>


```bash
fd -Ftd 'app_cycle' -x bash -c 'mv $0 ${0/app_cycle/envman}' {}
fd -Ftf 'app_cycle' -x bash -c 'mv $0 ${0/app_cycle/envman}' {}
sd -s 'app_cycle' 'envman' $(fd -tf)
sd -s 'app cycle' 'envman' $(fd -tf)
sd -s 'App Cycle' 'Env Man' $(fd -tf)
sd -s 'AppCycle' 'EnvMan' $(fd -tf)
cargo fmt --all
```
