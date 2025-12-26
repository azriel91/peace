# Document Object Model (DOM)

We need to work out the following:

Input:

1. Node hierarchy
2. Padding
3. Markdown content

Output:

1. `taffy::TaffyTree` and `taffy::tree::Layout`s for each node.

    These will give us the x/y coordinates to place SVG elements for:

    - `<rect>`: each node's box
    - `<text>`: elements -- node text content.
    - `<image>`: elements from markdown content.

2. SVG elements for text
