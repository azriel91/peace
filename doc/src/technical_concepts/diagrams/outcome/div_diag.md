# Div Diag

Attempt at using `dot_ix` to generate a diagram using the same `InfoGraph` input model.


## Desireables

1. Consistent layout.
2. Rendering arrows between nodes.
3. Styling nodes.
4. Styling arrows.
5. Consistent styling input with the `DotSvg` component.
6. Rendered SVG so that it can be uploaded as attachments and rendered inline, e.g. GitHub comments, mdbook.


## Solutioning

### Options

* **A:** Layout.
* **B:** Arrows.
* **C:** Styling.

#### 🟢 A1: Flexbox + HTML to SVG Translation

1. HTML elements with Flexbox, easy to integrate rendered Markdown
2. Translate HTML to SVG.


#### 🔴 A2: Render SVG directly

1. Need to implement layout algorithm that a browser already does.

    e.g. Flexbox, or somehow computer width/height with font metrics, unicode widths, potentially images.


#### 🟡 B1: Arrow Overlay

1. Separately draw arrows as SVG over the laid out diagram as an overlay layer.
2. Redraw arrows when layout changes.


#### 🟢 B2: Draw Arrows Inline

1. Layout diagram.
2. Convert to SVG.
3. Draw arrows within the SVG.


#### 🟡 C1: CSS Utility Classes, e.g. TailwindCSS

1. Take class names directly from consumer / user.
2. Apply those to the HTML / SVG elements.

Requires element structure to be stable, and consumers to know the structure.


#### 🟢 C2: CSS Utility Classes Keyed

1. Take in colours and border styles etc.
2. Prefix these with the appropriate structure, e.g. `[>path]:`, `hover:`, etc.
3. Apply these to the HTML / SVG elements.

Provides a stabler interface for users, and less knowledge of internals needed. Also a bit more freedom for maintainers.


### Existing Tech

#### HTML to SVG

* ⚪ [`dom-to-svg`]\: Typescript. Still need to try this.
* ⚪ [`vertopal`]\: Python. Still need to try this.
* ⚪ Build one, with a cut down version of the diagram.

Online tools have not been able to produce an accurate SVG rendering.


#### Drawing Arrows

* 🟡 [`leader-line`]\: Generates SVG arrows, with good configuration options. Out of the box, the SVG structure is not suitable for TailwindCSS classes.
* 🟡 Modify [`leader-line`]\: Restructure the elements in the SVG, and add classes.
* ⚪ Build one, so that the elements are structured to match graphviz, and have classes added.


#### Styling

* 🟡 [TailwindCSS]\: Versatile CSS library. See [utility-first] rationale.
* 🟢 [`encre-css`]\: This is a TailwindCSS compatible Rust library.


[`dom-to-svg`]: https://github.com/felixfbecker/dom-to-svg
[`encre-css`]: https://crates.io/crates/encre-css
[`leader-line`]: https://anseki.github.io/leader-line/
[`vertopal`]: https://github.com/vertopal/vertopal-cli
[TailwindCSS]: https://tailwindcss.com/
[utility-first]: https://tailwindcss.com/docs/utility-first
