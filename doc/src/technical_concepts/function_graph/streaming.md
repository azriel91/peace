# Streaming

When ordering has been defined, consumers can use [`FnGraph::iter`][`FnGraph::iter`] to iterate through the graph and invoke each function sequentially. When the `"async"` feature is enabled, which is on by default, [`FnGraph::stream`][`FnGraph::stream`] will produce each function once all of its predecessors have been streamed and returned.

To know when a function is available to be streamed, [`fn_graph`][`fn_graph`] uses the following algorithm:

* Track the number of predecessors of each node.
* When streaming, each time a function is streamed and the closure returns, subtract 1 from the predecessor counts of each of that function's successors.
* If the predecessor count of a function is 0, it is now available to be streamed.

## Visualized

1. In the example scenario, each function has a number of predecessors:

    ```dot process
    digraph {
        graph [
            penwidth  = 0
            nodesep   = 0.25
            ranksep   = 0.3
            bgcolor   = "transparent"
            fontcolor = "#555555"
            splines   = line
            rankdir   = LR
        ]
        node [
            fontcolor = "#111111"
            fontname  = "monospace"
            fontsize  = 10
            shape     = "circle"
            style     = "filled"
            width     = 0.4
            height    = 0.4
            margin    = 0.04
            color     = "#aaaabb"
            fillcolor = "#eeeef5"
        ]
        edge [
            arrowsize = 0.7
            color     = "#555555"
            fontcolor = "#555555"
        ]

        fn1 [label = <<b>fn1:</b>0>, color = "#88bbff", fillcolor = "#bbddff"]
        fn3 [label = <<b>fn3:</b>2>]
        fn2 [label = <<b>fn2:</b>1>]
        fn5 [label = <<b>fn5:</b>2>]
        fn4 [label = <<b>fn4:</b>2>]

        fn1 -> fn2 [weight = 2]
        fn1 -> fn3 [weight = 2, minlen = 2]
        fn2 -> fn4 [weight = 5, minlen = 2]
        fn3 -> fn4
        fn2 -> fn5
        fn3 -> fn5 [weight = 2]

        fn2 -> fn3 [style = "dashed", color = "#555555"]
    }
    ```

2. When a function completes, subtract 1 from each of its successors' predecessor counts:

    ```dot process
    digraph {
        graph [
            penwidth  = 0
            nodesep   = 0.25
            ranksep   = 0.3
            bgcolor   = "transparent"
            fontcolor = "#555555"
            splines   = line
            rankdir   = LR
        ]
        node [
            fontcolor = "#111111"
            fontname  = "monospace"
            fontsize  = 10
            shape     = "circle"
            style     = "filled"
            width     = 0.4
            height    = 0.4
            margin    = 0.04
            color     = "#aaaabb"
            fillcolor = "#eeeef5"
        ]
        edge [
            arrowsize = 0.7
            color     = "#555555"
            fontcolor = "#555555"
        ]

        fn1 [label = <<b>fn1:</b>0>, color = "#88ffbb", fillcolor = "#bbffdd"]
        fn3 [label = <<b>fn3:</b>1>]
        fn2 [label = <<b>fn2:</b>0>, color = "#88bbff", fillcolor = "#bbddff"]
        fn5 [label = <<b>fn5:</b>2>]
        fn4 [label = <<b>fn4:</b>2>]

        fn1 -> fn2 [weight = 2, color = "#33aa55", fillcolor = "#55aa88"]
        fn1 -> fn3 [weight = 2, minlen = 2, color = "#33aa55", fillcolor = "#55aa88"]
        fn2 -> fn4 [weight = 5, minlen = 2]
        fn3 -> fn4
        fn2 -> fn5
        fn3 -> fn5 [weight = 2]

        fn2 -> fn3 [style = "dashed", color = "#555555"]
    }
    ```

3. This applies to both logical and data dependencies:

    ```dot process
    digraph {
        graph [
            penwidth  = 0
            nodesep   = 0.25
            ranksep   = 0.3
            bgcolor   = "transparent"
            fontcolor = "#555555"
            splines   = line
            rankdir   = LR
        ]
        node [
            fontcolor = "#111111"
            fontname  = "monospace"
            fontsize  = 10
            shape     = "circle"
            style     = "filled"
            width     = 0.4
            height    = 0.4
            margin    = 0.04
            color     = "#aaaabb"
            fillcolor = "#eeeef5"
        ]
        edge [
            arrowsize = 0.7
            color     = "#555555"
            fontcolor = "#555555"
        ]

        fn1 [label = <<b>fn1:</b>0>, color = "#88ffbb", fillcolor = "#bbffdd"]
        fn3 [label = <<b>fn3:</b>0>, color = "#88bbff", fillcolor = "#bbddff"]
        fn2 [label = <<b>fn2:</b>0>, color = "#88ffbb", fillcolor = "#bbffdd"]
        fn5 [label = <<b>fn5:</b>1>]
        fn4 [label = <<b>fn4:</b>1>]

        fn1 -> fn2 [weight = 2]
        fn1 -> fn3 [weight = 2, minlen = 2]
        fn2 -> fn4 [weight = 5, minlen = 2, color = "#33aa55", fillcolor = "#55aa88"]
        fn3 -> fn4
        fn2 -> fn5 [color = "#33aa55", fillcolor = "#55aa88"]
        fn3 -> fn5 [weight = 2]

        fn2 -> fn3 [style = "dashed", color = "#33aa55", fillcolor = "#55aa88"]
    }
    ```

4. Performance is gained when multiple functions can be executed concurrently:

    ```dot process
    digraph {
        graph [
            penwidth  = 0
            nodesep   = 0.25
            ranksep   = 0.3
            bgcolor   = "transparent"
            fontcolor = "#555555"
            splines   = line
            rankdir   = LR
        ]
        node [
            fontcolor = "#111111"
            fontname  = "monospace"
            fontsize  = 10
            shape     = "circle"
            style     = "filled"
            width     = 0.4
            height    = 0.4
            margin    = 0.04
            color     = "#aaaabb"
            fillcolor = "#eeeef5"
        ]
        edge [
            arrowsize = 0.7
            color     = "#555555"
            fontcolor = "#555555"
        ]

        fn1 [label = <<b>fn1:</b>0>, color = "#88ffbb", fillcolor = "#bbffdd"]
        fn3 [label = <<b>fn3:</b>0>, color = "#88ffbb", fillcolor = "#bbffdd"]
        fn2 [label = <<b>fn2:</b>0>, color = "#88ffbb", fillcolor = "#bbffdd"]
        fn5 [label = <<b>fn5:</b>0>, color = "#88bbff", fillcolor = "#bbddff"]
        fn4 [label = <<b>fn4:</b>0>, color = "#88bbff", fillcolor = "#bbddff"]

        fn1 -> fn2 [weight = 2]
        fn1 -> fn3 [weight = 2, minlen = 2]
        fn2 -> fn4 [weight = 5, minlen = 2]
        fn3 -> fn4 [color = "#33aa55", fillcolor = "#55aa88"]
        fn2 -> fn5
        fn3 -> fn5 [weight = 2, color = "#33aa55", fillcolor = "#55aa88"]

        fn2 -> fn3 [style = "dashed"]
    }
    ```

This can be seen by timing the executions:

```rust ,ignore
# // [dependencies]
# // fn_graph = { version = "0.5.4", features = ["fn_meta", "resman"] }
# // futures = "0.3.21"
# // resman = { version = "0.15.0", features = ["fn_meta", "fn_res"] }
# // tokio = { version = "1.20.0", features = ["rt", "macros", "time"] }
# use std::{
#     fmt,
#     ops::{AddAssign, Deref, DerefMut},
# };
#
# use fn_graph::FnGraphBuilder;
# use futures::stream::StreamExt;
# use resman::{IntoFnRes, Resources};
#
# // Define newtypes for each parameter.
# #[derive(Debug)]
# struct A(u32);
# #[derive(Debug)]
# struct B(u32);
# #[derive(Debug)]
# struct C(u32);
# #[derive(Debug)]
# struct D(u32);
#
# fn main() {
#     // Initialize data.
#     let mut resources = Resources::new();
#     resources.insert(A(0));
#     resources.insert(B(0));
#     resources.insert(C(0));
#     resources.insert(D(0));
#     let resources = &resources; // Now the map is compile time immutable.
#
#     // Define logic and insert them into graph structure.
#     type Fn1 = fn(&mut A, &mut B, &mut C, &mut D);
#     type Fn2 = fn(&A, &mut C);
#     type Fn3 = fn(&B, &mut C);
#     type Fn4 = fn(&C, &mut D);
#     type Fn5 = fn(&A, &B, &C);
#
#     let fn_graph = {
#         let fn1: Fn1 = |a, b, c, d| { a.0 = 1; b.0 = 2; c.0 = 0; d.0 = 0; };
#         let fn2: Fn2 = |a,    c   | *c += a.0;
#         let fn3: Fn3 = |   b, c   | *c += b.0;
#         let fn4: Fn4 = |      c, d| *d += c.0;
#         let fn5: Fn5 = |a, b, c   | println!("{a} + {b} = {c}");
#
#         let mut fn_graph_builder = FnGraphBuilder::new();
#
#         // Store functions in graph.
#         let [fn_id1, fn_id2, fn_id3, fn_id4, fn_id5] = fn_graph_builder.add_fns([
#             fn1.into_fn_res(),
#             fn2.into_fn_res(),
#             fn3.into_fn_res(),
#             fn4.into_fn_res(),
#             fn5.into_fn_res(),
#         ]);
#
#         // Define dependencies to control ordering.
#         fn_graph_builder
#             .add_logic_edges([
#                 (fn_id1, fn_id2),
#                 (fn_id1, fn_id3),
#                 (fn_id2, fn_id4),
#                 (fn_id2, fn_id5),
#                 (fn_id3, fn_id4),
#                 (fn_id3, fn_id5),
#             ])
#             .unwrap();
#
#         fn_graph_builder.build()
#     };
#
// Invoke logic over data.
let sequential_start = tokio::time::Instant::now();
fn_graph.iter().for_each(|fun| {
    fun.call(resources);
    std::thread::sleep(std::time::Duration::from_millis(10));
});
let sequential_elapsed = sequential_start.elapsed();
println!("sequential_elapsed: {sequential_elapsed:?}");

// prints:
// 1 + 2 = 3
// sequential_elapsed: 50.683709ms

let rt = tokio::runtime::Builder::new_current_thread()
    .enable_time()
    .build()
    .unwrap();
rt.block_on(async move {
    let concurrent_start = tokio::time::Instant::now();
    fn_graph
        .stream()
        .for_each_concurrent(None, |fun| async move {
            fun.call(resources);
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        })
        .await;
    let concurrent_elapsed = concurrent_start.elapsed();
    println!("concurrent_elapsed: {concurrent_elapsed:?}");
});

// prints:
// 1 + 2 = 3
// concurrent_elapsed: 44.740638ms
# }
#
# macro_rules! u32_newtype {
#     ($name:ident) => {
#         impl fmt::Display for $name {
#             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
#                 self.0.fmt(f)
#             }
#         }
#         impl Deref for $name {
#             type Target = u32;
#
#             fn deref(&self) -> &Self::Target {
#                 &self.0
#             }
#         }
#         impl DerefMut for $name {
#             fn deref_mut(&mut self) -> &mut Self::Target {
#                 &mut self.0
#             }
#         }
#         impl AddAssign<u32> for $name {
#             fn add_assign(&mut self, other: u32) {
#                 *self = Self(self.0 + other);
#             }
#         }
#     };
# }
# u32_newtype!(A);
# u32_newtype!(B);
# u32_newtype!(C);
# u32_newtype!(D);
```

Notably there is some overhead with the asynchronous execution, but as the number of functions grow, so should the concurrency, and the performance gains should increase proportionally.

[`FnGraph::iter`]: https://docs.rs/fn_graph/latest/fn_graph/struct.FnGraph.html#method.iter
[`FnGraph::stream`]: https://docs.rs/fn_graph/latest/fn_graph/struct.FnGraph.html#method.stream
[`fn_graph`]: https://github.com/azriel91/fn_graph
