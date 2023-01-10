# Ideas

This page records ideas that I'd like, but there isn't enough mental capacity and time to design and work on them yet.

1. Abstraction over native storage and web storage &ndash; use IndexDB instead of WebStorage APIs.
2. Graphical user interface that renders each flow's graph.

    1. Each item spec is a node.
    2. User can select which nodes to run &ndash; these may be a subset of the flow.
    3. User can select beginning and ending nodes &ndash; and these can be in reverse order.

        <!--  -->

    **Note:** Graphviz is compiled to WASM and published by [hpcc-systems/hpcc-js-wasm](https://github.com/hpcc-systems/hpcc-js-wasm). May be able to use that to render.

    [graphviz-visual-editor](https://github.com/magjac/graphviz-visual-editor) is a library that allows basic editing of a graphviz graph. It's not yet developed to a point that is intuitive for users.

3. Tool that uses `peace` to check consumer code whether it adheres to best practices.
