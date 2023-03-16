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

1. Each item spec is a node.
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
<summary>4. Clean Operation Retains History</summary>
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

When an item spec ensure does multiple writes, there is a possibility of not all of those writes occur during execution:

* user interrupts the execution.
* internet connection drops.
* machine loses power.

In the last case, we cannot safely write state to disk, so a `StateCurrent` discover is needed to bring `StatesSaved` up to date. However, the previous two cases, it is possible for `ItemSpec`s to return `State` that has been partially ensured, without making any further outgoing calls -- i.e. infer `StatesEnsured` based on the successful writes so far.

Note that this places a burden on the `ItemSpec` implementor to return the partial state ensured (which may conflict with keeping the `State` simple), as well as make the `ApplyOpSpec::exec` return value more complex.

The trade off may not be worthwhile.

</div>
</details>

<details>
<summary>8. Adding / removing / modifying item specs in flows</summary>
<div>

Implementors may add/remove/modify item specs in flows.

Peace needs to be designed such that these changes do not cause already-existent flows to not be loadable, i.e. when:

* `states_*.yaml` contains state for which an item spec no longer exists in the flow.
* `states_*.yaml` does not contain state for an item spec that is newly added to the flow.
* `states_*.yaml` contains state whose fields are different to a new version of an item spec.

    This one can be addressed by having `State` be an enum, with versioned variants.

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

The [`tokio-graceful-shutdown`] library can be used to introduce interrupt safety into item spec executions. This is particularly useful for write operations.

See the [`is_shutdown_requested`] method in particular.

[`tokio-graceful-shutdown`]: https://github.com/Finomnis/tokio-graceful-shutdown
[`is_shutdown_requested`]: https://docs.rs/tokio-graceful-shutdown/latest/tokio_graceful_shutdown/struct.SubsystemHandle.html#method.is_shutdown_requested

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
<summary>2. Learnings from app_cycle end-to-end implementation.</summary>
<div>

1. Referential lookup of values in state / item spec params. ([#94])
2. AWS SDK is not WASM ready -- includes `mio` unconditionally through `tokio` (calls UDP). ([aws-sdk-rust#59])
3. AWS SDK does not always include error detail -- S3 `head_object`. ([aws-sdk-rust#227])
4. Progress output should enable-able for state current / desired discover / clean functions.
5. Flow params are annoying to register every time we add another item spec.
6. Blank item spec needs a lot of rework to be easier to implement an item spec. ([67], [#96])

[#67]: https://github.com/azriel91/peace/issues/67
[#94]: https://github.com/azriel91/peace/issues/94
[#96]: https://github.com/azriel91/peace/issues/96
[aws-sdk-rust#59]: https://github.com/awslabs/aws-sdk-rust/issues/59
[aws-sdk-rust#227]: https://github.com/awslabs/aws-sdk-rust/issues/227

</div>
</details>


```bash
fd -Ftf 'blank' -x bash -c 'mv $0 ${0/blank/ec2_instance}' {}
sd -s 'blank' 'ec2_instance' $(fd -tf)
sd -s 'Blank' 'Ec2Instance' $(fd -tf)
sd '/// (.+) ec2_instance(.*)' '/// $1 ec2 instance$2' $(fd -tf)
cargo fmt --all
```

