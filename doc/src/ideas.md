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

Note that this places a burden on the `ItemSpec` implementor to return the partial state ensured (which may conflict with keeping the `State` simple), as well as make the `EnsureOpSpec::exec` return value more complex.

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


## Notes

<details>
<summary>1. SSH-like on Windows</summary>
<div>

* psexec
* [Windows powershell and WinRM](https://stackoverflow.com/questions/10237083/how-to-programmatically-remotely-execute-a-program-in-ec2-windows-instance/13284313#13284313)

</div>
</details>

<details>
<summary>2. Referential lookup of values in state / item spec params</summary>
<div>

Since `State` must be serializable, not sure if it is a good idea to store `!Ref $id`s as serialized values. If we did that, then reading the states desired YAML file would need the user to trace through the refs.

Item spec params needs:

* To be named differently / easily distinguished from workspace/profile/flow params.
* To have static values able to be specified (already possible).
* To be able to reference other item specs' values, likely through a lookup function.


Maybe:

1. Somehow have the item spec params take in `T`.

    Derive a builder for the item spec params struct:

    - **Consumers:** When creating a flow and inserting flow params, call:

        ```rust
        ItemSpecParams::builder()
            .with_x("value")
            .with_x_ref::<T>()
            .with_x_from::<T, U>(|t| t.u())
            .build()
        ```

    - **Implementors:** For each field have a `with_x`, `with_x_ref::<T>()`, and `with_x_from::<T, U>(|t| -> U {})`

        ```rust
        #[derive(Params)]
        pub struct Params {
            /// ID of something generated.
            id_to_attach: String,
        }
        ```

    - **Peace:**

        1. Hold `&mut Resources` and insert `State`s into it as they are finished.
        2. Or should we use the existing Resources? -- likely, some things write to `W<'op, T>`
        3. Whenever we run an any function exec (state current / ensure op spec exec / etcetera all apply), we take `ItemSpec::Params`, and run `ItemSpecParamsBuilder::build_from(resources)`, which will use the static value / fetched and mapped values to return `ItemSpecParams`.
        4. Somehow that gets injected into the exec function -- not sure if we put a separate parameter whether that's nice or not, or if we can merge the `ItemSpecParams` type into the `Data`.

            Maybe instead of `W<'_, Params>`, we have a `P<'_, Params'>`, whose implementation does step 4.

        Still need to work out how to insert state into `Resources`. Do we silently include a `W<'_, State>` alongside every item spec's data writes? Maybe.

        ```rust
        params.fields()
            .iter()
            .map(|field_rt| field_rt.value(resources));

        impl FieldRt for Field<T, U> {
            fn fetch(resources: &Resources) -> Ref<'_, T> {
                match field {
                    FieldParam::Value(value) => value,
                    FieldParam::Ref => resources.borrow::<T>(),
                    FieldParam::From => {
                        let t = resources.borrow::<T>();
                        U::from(t)
                    }
                }
            }
        }
        ```
2. Then the item spec's data accessed includes `R<'op, T>` for data dependency calculation.
3. Item spec functions will be given the params with the values read from `Resources`.

</div>
</details>


```bash
fd -Ftf 'blank' -x bash -c 'mv $0 ${0/blank/ec2_instance}' {}
sd -s 'blank' 'ec2_instance' $(fd -tf)
sd -s 'Blank' 'Ec2Instance' $(fd -tf)
sd '/// (.+) ec2_instance(.*)' '/// $1 ec2 instance$2' $(fd -tf)
cargo fmt --all
```

