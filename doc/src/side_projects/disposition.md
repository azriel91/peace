# 📐 disposition

Diagrams to SVG.

* Source: <https://github.com/azriel91/disposition>

Decisions:

* **Diagrams:** Represents things or a process.
* **SVG:** Portable, supports styling, interactivity, can be generated in-memory -- browser not needed.

This is a new library / app intended to take the place of [`dot_ix`][`dot_ix`].


## Background

[`dot_ix`][`dot_ix`] is a useful tool to generate diagrams backing on to GraphViz and Tailwind CSS.

The following learnings have come from using [`dot_ix`][`dot_ix`] for 2 years:

1. Requiring GraphViz to be installed / a browser to run the WASM version limits the ability to write a headless application and how cleanly we can write a web UI.
2. GraphViz's layout engine is not predictable -- positioning of nodes is unstable when edges are added -- and this requires the input to be tuned to get an "expected" positioning of nodes / edges.
3. The `dot_ix` input structure is relatively good, but can be further refined with better top-level concepts (`node_type`, better tag support).
4. Native markdown support is desired.

### Alternatives

<details>

* [`dot_ix`][`dot_ix`]: This is/was the previous project, so we're creating the next evolution of it.
* **Browser web driver:** We generate HTML, browser renders it, and we generate SVG off the browser DOM element positions.

    Doesn't work great for CLI -- need to have a headless browser.

</details>


## Design

There are multiple parts to generating a diagram:

1. **Diagram Structure:** Capturing the information that the diagram represents in a suitable data structure.
2. **Layout Document Object Model (DOM):** Calculating the DOM element structure (visual hierarchy) of that information.
3. **Layout:** Calculating the positions of the layout DOM on a viewport with fixed dimensions.
4. **Full Document Object Model (DOM):** Adding edges after the elements are positioned, and adding the attributes that the layout DOM doesn't have.
5. **Rendering:** Producing the visual representation from the full DOM elements.


### 1. High Level Diagram Structure / Capturing Information

Capturing the information for the diagram, in a structure that is easy to reason about and work with. Ideally easy for both humans and computers to read and write.

* **Input:** Input formats, e.g. JSON, YAML, in-memory objects, etc.
* **Output:** High Level diagram data structure.


#### 1.1. Nodes / Clusters

1. Stable IDs
2. Display names
3. Descriptions / additional detail
4. "type"(s) -- the primary way how this node / cluster should be rendered
5. Tags -- what groups it is part of / affected by.
6. Hierarchy / Nesting / Level of detail

Kinds of diagrams we want to support:

1. **Things:** Shows things, where they are, and their relationship with other things.
2. **Process:** Shows steps in a process, and status / progress of each step.

We don't want to use the names "entity diagram" or "sequence diagram", because it can cause confusion with those terms in the software development context.


#### 1.2. Edges

1. From / to which node / cluster.
2. Direction
3. Type
4. Multiple edges between nodes
5. Edges on the correct point (north, south, east, west) on the node.


### 2. Intermediate Representation (IR) Diagram Structure

* **Input:** High Level Diagram Structure, or serialized IR, e.g. JSON, YAML.
* **Output:** IR Diagram data structure.

Similar to 1., except we want to define everything in terms of nodes and edges.

Technically we can begin at this step instead of 1., and define the high level diagram structure later, as long as we can represent the complex diagram in this intermediate representation.


### 3. Document Object Model (DOM)

Turn the diagram data structure into DOM elements.

* **Input:** IR Diagram data structure.
* **Output:** Layout DOM elements which are not viewport bound.

We need to choose one or a combination of:

* HTML DOM
* [`taffy`][`taffy`] format.

ℹ️ **Note:** we also need to consider edge descriptions -- how do we place these in the DOM? `taffy` will be used for flex layout, but where would we place edge DOM elements?


### 4. Layout

Placement of nodes, padding, reflowing text, etc.

* **Input:** Layout DOM elements which are not viewport bound.
* **Output:** DOM elements / text with XY coordinates in a fixed viewport, with tailwind classes.

ℹ️ **Note:** To make markdown content calculations easier, we will render content as text (e.g. lists and tables are still rendered as `* description` and `| key | value |`). This is because rendering tables as proper elements is pretty much reinventing HTML.

If we use:

* [`taffy`][`taffy`]: We need to translate its output into the SVG DOM.
* [`blitz`][`blitz`]: We need to translate HTML elements into SVG DOM.
* Headless browser: We need to translate HTML elements into SVG DOM, and we'd need a headless browser, which isn't convenient for CI.

ℹ️ **Note:** we also need to consider edge descriptions -- if there is a lot of text, should we have spacing for those labels?


#### 4.1. DOM representation

Because we want node descriptions to be markdown, we need to convert them to an appropriate DOM structure that can represent the rendered markdown, as well as encode the layout and styling information.


##### 4.1.1. Option 1: SVG

1. We have to calculate the positions of nodes and text ourselves, including padding etc.
2. Markdown is converted to HTML, then we use those to position the text.
3. i.e. we'd have to know / calculate the font metrics for bold/italicized text.
4. [`taffy`][`taffy`] is what `servo` uses, and does element layouting.
5. [`cosmic-text`](https://crates.io/crates/cosmic-text) is needed for text width calculations.
6. See the [`cosmic_text` example](https://github.com/DioxusLabs/taffy/blob/v0.9.1/examples/cosmic_text/src/main.rs) -- you need font metrics to know how text renders.


##### 4.1.2. Option 2: HTML + HTML to SVG

1. HTML rendering engine does the layout of text positioning for us.
2. Markdown will easily be supported here, because we can convert to HTML, then the rendering engine takes care of the rest.


#### 4.2. Images

Images can be inlined in markdown, and based on the image data or a provided value, we can pass that to `taffy` to calculate the position.

If we use [`comrak`][`comrak`], then we need to wait for [`comrak#586`][`comrak#586`] to be resolved to get the passed in dimensions of the image.

Note: [`comrak`][`comrak`] also supports [`syntect`][`syntect`] for highlighting code blocks.

If we use [`pulldown-cmark`][`pulldown-cmark`], then we need to wait for [`pulldown-cmark#992`][`pulldown-cmark#992`] to be resolved to get the passed in dimensions of the image. Or, look at [`pulldown-cmark#462`][`pulldown-cmark#462`] to see if we can extract image dimensions from the URL.

Side note, [`pulldown-cmark`][`pulldown-cmark`] was deemed 1.9x faster than [`comrak`][`comrak`] in 2017 ([source](https://users.rust-lang.org/t/release-comrak-commonmark-gfm-compatible-markdown-parser/10340)). [`pulldown-cmark`][`pulldown-cmark`] doesn't construct an AST, so it should be faster than [`comrak`][`comrak`], though it may not be as feature-rich.


### 5. Full Document Object Model (DOM)

Adding edges after the elements are positioned, and adding the attributes that the layout DOM doesn't have.

* **Input:** Layout DOM elements with fixed coordinates.
* **Output:** Render DOM elements (including text) with XY coordinates in a fixed viewport, with tailwind classes.

#### 5.1. Edges

For flex type layouts / non-rank layouts, edges are intended to be hidden until a process / a step in a process is selected. This means there is no need to consider edges crossing each other, and they should generally have their start and end points in the middle of a `thing`'s border. Multiple edges may be offset from the middle by a few points so that it's clear there are multiple edges.

For rank type layouts, edges *may* be always visible, and are highlighted when a `thing` is focused. The highlighting makes it clearer when things are related, and layout stability is a goal of this tool, so edge crossing minimization will not be done at the expense of stable node positions.

[`kurbo`][`kurbo`] may be useful to calculate the coordinates along the curve for the path. Check how SVG paths take in input for curved lines -- we might not need to use `kurbo` if the SVG renderer calculates the curves.


### 6. Rendering

Rendering of the DOM into a visual and interactive format.

* **Input:** Render DOM elements with XY coordinates in a fixed viewport, with tailwind classes.
* **Output:** Visual and interactive diagram.

Any browser could render HTML / SVG. If we want a non-browser solution, look at:

1. [`stylo`](https://crates.io/crates/stylo) is the CSS engine servo uses. Do we need it? If we do, there's [`stylo_taffy`](https://crates.io/crates/stylo_taffy)
2. [`blitz`][`blitz`] seems to be doing what we want (and more), but in HTML.
3. [`blitz`#260](https://github.com/DioxusLabs/blitz/issues/260) is where they'd add SVG support.


### Solution

Probably:

1. [x] Define high level diagram structure based on concepts we want to display.
2. [x] Define intermediate diagram structure based on `dot_ix`'s learnings.
3. [ ] Compute the rendered content text.
4. [x] Map the IR to [`taffy`][`taffy`]'s nodes with `NodeContext`.

    Taffy nodes know their `x/y/w/h/content_w/content_h/border/padding`.

    `content_w` and `content_h` is the width and height of the content, which may exceed w/h when `taffy` is used in a UI that can scroll.

    What we need in the `NodeContext` is then:

    1. The IR entity ID (`node_id`, `edge_group_id`, `edge_id`).
    2. What kind of entity it is (`Thing`, `Process`, `ProcessStep`, `Tag`, `EdgeGroup`, `Edge`).

        With that, we can work out:

        1. The text content of the node.
        2. Any interaction buttons / menu (includes node copy text).

5. [x] Use [`taffy`][`taffy`] to lay out the diagram.

    We need to compute measurements for a number of variations for the diagram in order to provide responsiveness.

    1. The list of dimensions (small, medium, large) that the full diagram should be responsive to.
    2. Whether zero, one, or many processes are included.
    3. If one SVG diagram should be generated per step in the process.
    4. Whether tags are included.

    The `measure_function` for each node needs:

    1. The text content.
    2. Font metrics.
    3. Image dimensions.
    4. Whether an interaction menu button is present.

6. [ ] Convert to SVG, adding edges and attributes from the input structure. [`kurbo`][`kurbo`] may be useful to compute the edge path coordinates.

    Need:

    1. The markdown events that lets us write out an svg element.
    2. The text broken into lines.
    3. But, the markdown events don't know that some text needs to move to the next line.
    4. Also, the text broken into lines needs to have been measured based on the condensed / rendered text.
    5. Need the coordinates of each line computed by `cosmic-text`.

7. [ ] Return that to the caller -- SVG can be rendered in a browser. In the future, we might use [`blitz`][`blitz`] to render the SVG.

Not yet implemented:

1. [ ] Rendered text.
2. [ ] Copy text.
3. [ ] Node menu button.
4. [ ] Images.
5. [ ] Combine processes into a "drop down menu" (using `:target`), by finding the longest process name, and the one with the most steps.


### Computing rendered text

We'll render the text as monospace, but styled like [`syntect`][`syntect`]. It may be worth collapsing certain things:

- Links like `[something](url)` will only take up `something`'s space.
- Images like `![alt](url#w=320&h=240)` (syntax may differ) will take up 320 x 240.
- See [`pulldown-cmark/feature/attributes-extension`](https://github.com/azriel91/pulldown-cmark/tree/feature/attributes-extension) for a branch that adds support for attributes: `![](url){width=320 height=240}`.
- We probably will end up using `comrak` because it has `syntect` support.

See the `taffy` [`cosmic_text`](https://github.com/DioxusLabs/taffy/blob/v0.9.2/examples/cosmic_text/src/main.rs) example for width and height.

We should use [`cosmic_text::Buffer::set_rich_text`][`cosmic_text::Buffer::set_rich_text`] to calculate the width and height of the styled compressed text, though since we'd likely be using monospace font, the non-styled text may be the same as the styled text.

What we can't avoid is determining the compressed text. i.e.

1. `Some [link text](https://example.com)` will take up the space of `Some link text`.
2. `![alt](https://example.com/image.png)` should be the image's width and height, and not measured by `cosmic-text`. Possibly place the image on its own line.

So we need to compute a data structure holding:

````yaml
provided_desc: |
  Some provided text:

  - **Item 1:** Some description with [link](https://example.com).
  - **Item 2:** Some description with ![image](https://example.com/image.png).
  - **Item 3:** Some code:

      ```yaml
      key: value
      ```
rendered_desc:
  # spans
  - text:
      value: "Some provided text:\n\n- "
      attrs: { family: "Monospace" }
  - text:
      value: "Item 1:"
      attrs: { family: "Monospace", weight: "Bold" }
  - text:
      value: " Some description with "
      attrs: { family: "Monospace" }
  - text:
      value: " link"
      attrs: { family: "Monospace", color: Color::rgb(0, 0, 255) }
  - text:
      value: ".\n- "
      attrs: { family: "Monospace" }
  - text:
      value: "Item 2:"
      attrs: { family: "Monospace", weight: "Bold" }
  - text:
      value: " Some description with "
      attrs: { family: "Monospace" }
  - image:
      src: "https://example.com/image.png"
      alt: "Image"
  - text:
      value: ".\n- "
      attrs: { family: "Monospace" }
  - text:
      value: "Item 3:"
      attrs: { family: "Monospace", weight: "Bold" }
  - text:
      value: " Some code:\n\n```yaml\n"
      attrs: { family: "Monospace" }
  # syntect highlighted
  - text:
      value: "key:"
      attrs: { family: "Monospace", color: Color::rgb(0, 255, 100) }
  - text:
      value: " "
      attrs: { family: "Monospace" }
  - text:
      value: "value"
      attrs: { family: "Monospace", color: Color::rgb(0, 100, 255) }
  - text:
      value: "\n```\n"
      attrs: { family: "Monospace" }
````


## Ideas / Learnings from `dot_ix`

1. ✅ Ability to combine both `Thing` diagrams and `Process` diagrams. i.e. a process diagram whose steps show what is happening on the things. Maybe we just have one kind of diagram that does both.

    1. ✅ Maybe we just have one kind of diagram that does both.
    2. ✅ What `dot_ix` has as tags can be distinguished as `Process`es or `TagGroup`s.

2. When a node is styled with certain colours, apply it to all child nodes.
3. ✅ Light / Medium / Dark presets for shading.
4. ✅ Dependency diagrams: is it possible to select a node, and a menu appears, with buttons each to highlight:

    1. All transitive dependencies this depends on.
    2. All transitive dependents that depend on this.
    3. Immediate neighbours.

    Need to experiment with `group-focus-within` from tailwind.

    Looks like it may be possible with different CSS pseudo classes / pseudo elements.

    e.g. when an element is clicked on, it becomes the `:target` element in the document, and the css selector `#element-id:target ~ #other` allows you to style `#other` when `#element-id` was clicked, presumably when the focus is changed from  `#element-id` to something else.

    Tried this, it works if we use `:target` -- see experiment 4.

    Notes:

    1. We must have a `group/root` top level element, so that we can use `group-[:has(#proc-three:target)]/root:visible` to set visibility based on the document containing a particular `:target`.
    2. `z-index` has no effect on the stacking order of elements in SVGs, so process steps must be placed before the process nodes to be rendered below them.
    3. That also means, `tabindex="0"` cannot be used on every element, because we want the process nodes to be tabbed to before process steps.
    4. `encrecss` doesn't correctly handle IDs with underscores in class names -- it changes them to spaces. We can probably get this bug fixed.

5. Use [`async-lsp`][`async-lsp`] to provide context-aware completions.
6. ❌ SVG has the [`<set>`][`<set>`] element which allows you to change the value of an attribute over time, or on event. Try it with the [`end`][`end`] attribute, which lets you go `end="elementId.click"`. Maybe we can use `<set attributeName="class" to="round" begin="me.focus" end="me.blur" />`.

    Tried this -- see experiment 3. It functionally could work, but github blocks SVG animations.

    So the animation state *could* be used to store the selected process state, but we can't rely on it working in all cases.

7. Render `taffy` trees for different screen sizes, and use CSS media queries to position the nodes instead of `x` and `y` coordinates.

    ```svg
    <svg viewBox="0 0 240 220" xmlns="http://www.w3.org/2000/svg">
    <style>
        rect {
        fill: black;
        transform: translateX(10px);
        }
    </style>
    <style media="(width >= 600px)">
        rect {
        fill: seagreen;
        transform: translateX(50px);
        }
    </style>

    <text y="15">Resize the window to see the effect</text>
    <rect y="20" width="200" height="200" />
    </svg>
    ```

[`async-lsp`]: https://github.com/oxalica/async-lsp
[`blitz`]: https://github.com/DioxusLabs/blitz
[`comrak`]: https://github.com/kivikakk/comrak
[`comrak#586`]: https://github.com/kivikakk/comrak/issues/586
[`cosmic_text::Buffer::set_rich_text`]: https://docs.rs/cosmic-text/0.16.0/cosmic_text/struct.Buffer.html#method.set_rich_text
[`end`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/end
[`pulldown-cmark`]: https://github.com/pulldown-cmark/pulldown-cmark
[`pulldown-cmark#462`]: https://github.com/pulldown-cmark/pulldown-cmark/issues/462
[`pulldown-cmark#992`]: https://github.com/pulldown-cmark/pulldown-cmark/issues/992
[`dot_ix`]: https://azriel.im/dot_ix/
[`kurbo`]: https://github.com/linebender/kurbo
[`set`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/set
[`syntect`]: https://github.com/trishume/syntect
[`taffy`]: https://github.com/DioxusLabs/taffy


## Example Input

See [`example_input.md`](example_input.md) for an example.


## Example Intermediate Representation (IR)

See [`example_intermediate_representation.md`](example_intermediate_representation.md) for an example.


## Experiments

### 1. Group Focus

<object
    type="image/svg+xml"
    data="disposition/group_focus_experiment.svg"
    /></object>
<br/>

### 2. Peer Data Attribute

<object
    type="image/svg+xml"
    data="disposition/peer_data_attribute_experiment.svg"
    /></object>
<br/>

### 3. Animation / `<set>` for selecting process

The concept looks good using `<set>`, but in [`dot_ix#38 (comment)`](https://github.com/azriel91/dot_ix/pull/38#issuecomment-3691322557) we should that it doesn't work -- the rendered SVG's animations don't show the processes.

<object
    type="image/svg+xml"
    data="disposition/process_input_set_experiment.svg"
    /></object>
<br/>

### 4. `:target` for selecting process

This works, as seen on [`dot_ix#38 (comment)`](https://github.com/azriel91/dot_ix/pull/38#issuecomment-3691322557). To not affect the document's bookmark, the SVG needs to be inside its own frame / opened in its own tab.

<object
    type="image/svg+xml"
    data="disposition/process_input_target_experiment.svg"
    /></object>
<br/>
