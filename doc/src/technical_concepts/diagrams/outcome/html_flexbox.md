# HTML + Flexbox

{{#include html_flexbox/example.html}}

<details><summary>diagram source</summary>

```html
{{#include html_flexbox/example.html}}
```

</details>

It is possible to produce a diagram in a similar style to dot using HTML elements and JS to draw an SVG arrow. The above uses [`leader-line`], which has been archived by the original author.

It doesn't support adding an ID or CSS classes, so either we use its many configuration options, or we find another library which supports rendering arrows and setting classes.

[`leader-line`]: https://www.npmjs.com/package/leader-line
