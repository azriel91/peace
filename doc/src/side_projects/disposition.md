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


### 1. Diagram Structure / Capturing Information

Capturing the information for the diagram, in a structure that is easy to reason about and work with. Ideally easy for both humans and computers to read and write.

* **Input:** Input formats, e.g. JSON, YAML, in-memory objects, etc.
* **Output:** Diagram data structure.


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


### 2. Document Object Model (DOM)

Turn the diagram data structure into DOM elements.

* **Input:** Diagram data structure.
* **Output:** Layout DOM elements which are not viewport bound.

We need to choose one or a combination of:

* HTML DOM
* [`taffy`][`taffy`] format.


### 3. Layout

Placement of nodes, padding, reflowing text, etc.

* **Input:** Layout DOM elements which are not viewport bound.
* **Output:** DOM elements / text with XY coordinates in a fixed viewport, with tailwind classes.

If we use:

* [`taffy`][`taffy`]: We need to translate its output into the SVG DOM.
* [`blitz`][`blitz`]: We need to translate HTML elements into SVG DOM.
* Headless browser: We need to translate HTML elements into SVG DOM, and we'd need a headless browser, which isn't convenient for CI.


#### 3.1. SVG

1. We have to calculate the positions of nodes and text ourselves, including padding etc.
2. Markdown is converted to HTML, then we use those to position the text.
3. i.e. we'd have to know / calculate the font metrics for bold/italicized text.
4. [`taffy`][`taffy`] is what `servo` uses, and does element layouting.
5. [`cosmic-text`](https://crates.io/crates/cosmic-text) is needed for text width calculations.
6. See the [`cosmic_text` example](https://github.com/DioxusLabs/taffy/blob/v0.9.1/examples/cosmic_text/src/main.rs) -- you need font metrics to know how text renders.


#### 3.2. HTML + HTML to SVG

1. HTML rendering engine does the layout of text positioning for us.
2. Markdown will easily be supported here, because we can convert to HTML, then the rendering engine takes care of the rest.


### 4. Full Document Object Model (DOM)

Adding edges after the elements are positioned, and adding the attributes that the layout DOM doesn't have.

* **Input:** Layout DOM elements with fixed coordinates.
* **Output:** Render DOM elements (including text) with XY coordinates in a fixed viewport, with tailwind classes.


### 5. Rendering

Rendering of the DOM into a visual and interactive format.

* **Input:** Render DOM elements with XY coordinates in a fixed viewport, with tailwind classes.
* **Output:** Visual and interactive diagram.

Any browser could render HTML / SVG. If we want a non-browser solution, look at:

1. [`stylo`](https://crates.io/crates/stylo) is the CSS engine servo uses. Do we need it? If we do, there's [`stylo_taffy`](https://crates.io/crates/stylo_taffy)
2. [`blitz`][`blitz`] seems to be doing what we want (and more), but in HTML.
3. [`blitz`#260](https://github.com/DioxusLabs/blitz/issues/260) is where they'd add SVG support.


### Solution

Probably:

1. Define diagram structure based on `dot_ix`'s learnings.
2. Map the structure to [`taffy`][`taffy`]'s elements.
3. Use [`taffy`][`taffy`] to lay out the diagram.
4. Convert to SVG, adding edges and attributes from the input structure.
5. Return that to the caller -- SVG can be rendered in a browser. In the future, we might use [`blitz`][`blitz`] to render the SVG.

[`blitz`]: https://github.com/DioxusLabs/blitz
[`dot_ix`]: https://azriel.im/dot_ix/
[`taffy`]: https://github.com/DioxusLabs/taffy
