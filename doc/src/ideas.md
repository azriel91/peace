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


## Notes

<details>
<summary>1. SSH-like on Windows</summary>
<div>

* psexec
* [Windows powershell and WinRM](https://stackoverflow.com/questions/10237083/how-to-programmatically-remotely-execute-a-program-in-ec2-windows-instance/13284313#13284313)

</div>
</details>

