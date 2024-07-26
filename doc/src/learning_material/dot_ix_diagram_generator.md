# Dot IX: Diagram Generator

Picture Your Understanding

1. Problem: Diverge
    1. Diagram types:
        1. Process diagrams
        2. Deployment diagrams (outcome)
        3. Class Diagrams
        4. Gantt charts
    2. Clarity / Appearance / Styling
        1. Nodes, Edges, Text
        2. Animations
        3. Lossless Resolution
        4. Interactive
        5. Level of detail
    3. Input
        1. Labels for nodes, edges
        2. Styles
        3. Common input
        4. Software library
    4. Where the diagrams will be displayed
        1. Web app
        2. Github comments
        3. Any documentation software
2. Problem: Converge
    1. Diagram types:
        1. Process diagrams
        2. Deployment diagrams (outcome)
        3. ~~Class Diagrams~~
        4. ~~Gantt charts~~
    2. Clarity / Appearance / Styling
        1. Nodes, Edges, Text
        2. Animations
        3. Lossless Resolution
        4. Interactive
        5. Level of detail
    3. Input
        1. Labels for nodes, edges
        2. Styles
        3. Common input
        4. Software library
    4. Where the diagrams will be displayed
        1. Web app
        2. Github comments
        3. Any documentation software
3. Solution: Diverge
    
    > * What solutions can we do?
    > * How does each solution address the concerns in the problem space we have chosen?

    Diagrams tend to be an abstraction over something actual.

    1. Output format:
        1. SVG
        2. HTML elements
        3. HTML canvas
        4. Image (pixels)
    2. Input format:
        1. Visual drawing tool
        2. Structured input, e.g. YAML, JSON
        3. Software library

4. Solution: Converge
    1. Output format:
        1. SVG: without `foreignObject`
        2. HTML elements
        3. ~~HTML canvas~~
        4. ~~Image (pixels)~~
    2. Input format:
        1. Visual drawing tool
        2. Structured input, e.g. YAML, pass through graphviz for layout
        3. Software library
