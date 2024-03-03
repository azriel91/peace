# Render Technology

Currently:

1. Graphviz `dot` is used to render the SVG.
2. Tailwind CSS is used to define and generate styles.
3. `leptos` is used with `axum` to serve the graph in a web application.
4. `dot_ix` connects those together.

There are other use cases and desirable features that are difficult to implement with the current technology, namely:

1. Rendering the outcome on a command line interface.
2. Richer formatting, also by item implementors.
3. Controllable / predictable / consistent node positioning.
4. Serializing the outcome rendering, for later display.
5. Diffing two outcome diagrams.
6. Rendering errors and recovery hints.


## Command Line Interface

Rendering the outcome on the CLI is not really necessary when the user is able to use a web browser to see the outcome. If the user is connected to a server via SSH, the tool could technically serve the web interface locally, and an SSH tunnel used to forward the traffic.

[`dioxus`] may allow us to have single source for both the web and CLI, but this is not known for sure -- [`plasmo`], the crate that takes HTML and renders it on the terminal does not handle `<input>` elements on the terminal, so exploration is needed to determine if this is suitable.


[`dioxus`]: https://github.com/DioxusLabs/dioxus
[`plasmo`]: https://github.com/DioxusLabs/dioxus/tree/master/packages/plasmo


## Rich Formatting

For richer formatting, the `Presentable` trait was intended to capture this. However, serialization of different types makes the code complex -- I haven't figured out a way to transfer arbitrary types across the server to a client.

One option is to use markdown, and transfer the plain markdown to the target output, which can render it in its own means, e.g. [`syntect`] for CLI, and [`comrak`] or [`pulldown-cmark`] (what `mdbook` uses) to generate HTML for the web.

We would have to automatically nest markdown by indenting inner types' markdown.

[`syntect`]: https://github.com/trishume/syntect
[`pulldown-cmark`]: https://github.com/pulldown-cmark/pulldown-cmark
[`layout`]: https://github.com/nadavrot/layout


## Controllable Node Positioning

[`layout`] is a Rust port of a subset of `dot`. Essentially there are no HTML-like labels, but it is written in Rust. It likely has consistent node positioning, so using it over `dot` has that advantage, with the added benefits of performance and portability. [`vizdom`] uses it to generate graphs in real time.

Instead of using a `dot`-like library, generating elements with a flexbox layout, and drawing arrows may be the way to go.


[`comrak`]: https://hrzn.ee/kivikakk/comrak


## Serializing Outcome Rendering

As long as we have a serializable form, which is *stable*, we can re-render the diagram later.

This means all information about the flow, parameters, and errors need to be serializable.


## Diffing Outcome Diagrams

[`vizdom`] does this really nicely with `dot` graphs, see its examples.

Should we do a visual diff? Or clever node matching, then a styling diff?


[`vizdom`]: https://www.vizdom.dev/


## Rendering Errors and Recovery Hints

For rendering errors, we need to know whether the error is to do with:

* a host / a function run by a host that failed
* a connection between hosts

then styling it.

For recovery hints, we need to make it clear where the error shown on the the outcome diagram is related to input parameters, whether it is a file, or a value produced by a host.

If we are using `dot`-like or a CSS-enabled technology, then we can style the relevant edge / node with Tailwind CSS styles.


## Tailwind CSS generation

[`encre-css`] is likely what we will use, as it is TailwindCSS compatible.


[`encre-css`]: https://crates.io/crates/encre-css
