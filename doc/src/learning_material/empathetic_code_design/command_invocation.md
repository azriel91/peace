# Command Invocation

```dot process
digraph {
    graph [
        margin    = 0.0
        penwidth  = 0
        nodesep   = 0.0
        ranksep   = 0.02
        bgcolor   = "transparent"
        fontname  = "helvetica"
        fontcolor = "#7f7f7f"
        rankdir   = LR
        // splines   = curved
    ]
    node [
        fontcolor = "#111111"
        fontname  = "monospace"
        fontsize  = 12
        shape     = "circle"
        style     = "filled"
        width     = 0.3
        height    = 0.3
        margin    = 0.04
        color     = "#9999aa"
        fillcolor = "#ddddf5"
    ]
    edge [
        arrowsize = 0.7
        color     = "#7f7f7f"
        fontcolor = "#7f7f7f"
    ]

    subgraph cluster_a {
        a [label = <<b>a</b>>]
        a_text [shape="plain" style="none" fontcolor="#7f7f7f" label = <file<br/>download>]
    }

    subgraph cluster_b {
        b [label = <<b>b</b>>]
        b_text [shape="plain" style="none" fontcolor="#7f7f7f" label = <server<br/>instance>]
    }

    subgraph cluster_c {
        c [label = <<b>c</b>>]
        c_text [shape="plain" style="none" fontcolor="#7f7f7f" label = <file<br/>upload>]
    }

    a -> b [minlen = 15, weight = 5]
    b -> c [minlen = 15, weight = 5]
}
```

```rust ,ignore
let mut cmd_context = CmdContext::builder()
    .with_flow(&flow)
    .with_item_spec_params::<_>(..) // per item
    .with_output(&mut cli_output)
    .build();

StatusCmd::exec(&mut cmd_context).await?;
DeployCmd::exec(&mut cmd_context).await?;
```

That's nice, but:

```bash
envman init \
  --url https://example.com/app.tar \
  --image-id img-12345 \
  --instance-size xlarge

# yay!

# oh..
envman status \
  --url https://example.com/app.tar \
  --image-id img-12345 \
  --instance-size xlarge

envman deploy \
  --url https://example.com/app.tar \
  --image-id img-12345 \
  --instance-size xlarge
```

We need to store and reload the parameters passed in previously.
