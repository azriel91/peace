<div style="
    display: flex;
    flex-wrap: wrap;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    height: 80vh;
">
    <div>
        <object
            type="image/svg+xml"
            data="dot_ix_diagram_generator/landing_diagram.svg"
            width="200"
            style="margin-right: -50px;"></object>
        <object
            type="image/svg+xml"
            data="why_rust/rustacean-flat-happy.svg"
            width="200"></object>
        <br />
        <small><a href="https://azriel.im/dot_ix/#src=MQAgogHghgtgDgGwKYC4QAUBOB7AxkgZwJAHUBLAFwAsQBJAOwDNsAoAWg5YBMzMlcKZbPTQA3JJkG4oCFlTISomXFQCeKFiBC5hFKGXoSNWrVDQBvAL6aTAIwvWT2hza1cXLeti5IA+vVhCY21dfUNMNAAiSJszEEitGK17eMSbXCi0t0z4lhYkLgBzINiUgG0oABoQWwBdG1sMkDLbatx65Pdm1pAuDu04stxqswIAd36uQa4RlAIkepZCzCg4KlEyAC9fKAoKTAJgguLfHXp9sIpD1wG0Rhl5m6m7h6Q86iQYVBsCClVka5OBBkQpUChoABkwNBFBuWkYZAQCF8BCoUB8-mwmBgMjQAHIAIwABiJeLhIARSJRaIxVGw4giIDxAFZSeTKcjUei-MxcABXQ5M4lspzwxGcml+KACMjifHCsmikC-HAAaz8XIxXmxuKZABYSYrRSrsOrqdzfHSGfiAMyG8kms2aqUyuVM1ki437U0av7INBcbB7Ap5JxnPQGIzkgA80bQACpoWCHWi4H4dAgsfjbAg+UgyUA">source</a></small>
    </div>
    <div style="font-size: 3.5em; font-weight: bold;">Dot IX: Diagram Generator</div>
    <div style="font-size: 2.5em;">Picture Your Understanding</div>
    <div style="height: 100px;"></div>
    <div style="font-size: 2.0em;">Azriel Hoh</div>
    <div style="font-size: 1.5em;">August 2024</div>
</div>

<div class="hidden">

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

</div>
