# Flow Definition

Specify:

1. Items: Steps of a process.
2. Ordering between items.

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
        a_text [shape="plain" style="" fontcolor="#7f7f7f" label = <file<br/>download>]
    }

    subgraph cluster_b {
        b [label = <<b>b</b>>]
        b_text [shape="plain" style="" fontcolor="#7f7f7f" label = <server<br/>instance>]
    }

    subgraph cluster_c {
        c [label = <<b>c</b>>]
        c_text [shape="plain" style="" fontcolor="#7f7f7f" label = <file<br/>upload>]
    }

    a -> b [minlen = 9]
    b -> c [minlen = 9]
}
```

```rust ,ignore
let flow = {
    let mut flow_builder = Flow::builder()
        .with_flow_id(flow_id!("app_deploy"));

    let [a, b, c] = flow_builder.add_items([
        FileDownloadItem::<WebApp>::new(item_id!("a")).into(),
        ServerInstanceItem::<WebApp>::new(item_id!("b")).into(),
        FileUploadItem::<WebApp>::new(item_id!("c")).into(),
    ]);

    flow_builder.add_edges([
        (a, b),
        (b, c),
    ])?;

    flow_builder.build()
};
# let flow = {
#     let mut graph_builder = ItemGraphBuilder::new();
#     let [a, b, c] = graph_builder.add_fns([
#         FileDownloadItem::<WebApp>::new(item_id!("a")).into(),
#         ServerInstanceItem::<WebApp>::new(item_id!("b")).into(),
#         FileUploadItem::<WebApp>::new(item_id!("c")).into(),
#     ]);
#
#     graph_builder.add_edges([
#         (a, b),
#         (b, c),
#     ])?;
#
#     Flow::new(flow_id!("app_deploy"), graph_builder.build())
# };
```

## Non-linear Ordering

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
        splines   = line
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
        a_text [shape="plain" style="" fontcolor="#7f7f7f" label = <file<br/>download>]
    }

    subgraph cluster_b {
        b [label = <<b>b</b>>]
        b_text [shape="plain" style="" fontcolor="#7f7f7f" label = <server<br/>instance>]
    }

    subgraph cluster_c {
        c [label = <<b>c</b>>]
        c_text [shape="plain" style="" fontcolor="#7f7f7f" label = <file<br/>upload>]
    }

    a -> c [minlen = 9]
    b -> c [minlen = 9]
}
```

```diff
 graph_builder.add_edges([
-    (a, b),
+    (a, c),
     (b, c),
 ])?;
```

---

### Command Context

```rust ,ignore
let mut cmd_context = CmdContext::builder()
    .with_flow(&flow)
    .build();
```
