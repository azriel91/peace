# Output and Presentation

When the workflow is defined, we want to interact with it in different ways.

## Presentation

### Command Line Interface

<pre class="terminal">
<span class='shell'>&gt; </span><span class='cmd'>./envman</span> <span class='arg'>deploy</span>
Using profile <span style='color:#ffafff'><b>⦗demo⦘</b></span> -- type <span style='color:#5fafff'>`development`</span>

✅ <span style='color:#fff'>1.</span> <span style='color:#5fafff'>file_download</span>    <span style='color:#00af5f00; background-color:#00af5f' class="shell_progress_bar">▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰</span> done!
⏳ <span style='color:#fff'>2.</span> <span style='color:#5fafff'>server_instance</span>  <span style='color:#0087d700; background-color:#0087d7' class="shell_progress_bar">▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱</span> starting up <span style='color:#af00d7'>(el: 6s, eta: 0s)</span>
⏳ <span style='color:#fff'>3.</span> <span style='color:#5fafff'>file_upload</span>      <span style='color:#555'>▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱</span> <span style='color:#af00d7'>(el: 0s, eta: 0s)</span>
</pre>


### Continuous Integration

```text
2023-05-07T01:30:31.135Z | file_download   | downloaded at /tmp/.a9efb2/web_app.tar
2023-05-07T01:30:31.754Z | server_instance | launched: inst-12345
2023-05-07T01:30:32.246Z | server_instance | ip address: 10.0.0.17
2023-05-07T01:30:32.687Z | server_instance | starting up
```


### Web API

```json
// GET /api/v1/demo/env_deploy/progress
{
    "item_spec_id": "server_instance",
    "progress_update": { "Delta": "Tick" },
    "msg_update": { "Set": "starting up" }
    // ..
}
```


### Web page

<div class="column_half">

```dot process
digraph {
    graph [
        margin    = 0.0
        penwidth  = 0
        nodesep   = 0.0
        ranksep   = 0.02
        bgcolor   = "transparent"
        fontname  = "helvetica"
        fontcolor = "#7f7f7f"
        rankdir   = TB
        splines   = line
    ]
    node [
        fontcolor = "#111111"
        fontname  = "monospace"
        fontsize  = 12
        shape     = "circle"
        style     = "filled"
        width     = 0.3
        height    = 0.3
        margin    = 0.04
        color     = "#9999aa"
        fillcolor = "#ddddf5"
        penwidth  = 2
    ]
    edge [
        arrowsize = 0.7
        color     = "#7f7f7f"
        fontcolor = "#7f7f7f"
        penwidth  = 2
    ]

    subgraph cluster_a {
        node [color="#449966" fillcolor="#99dd99"]
        a [label = <<b>a</b>>]
        a_text [shape="plain" style="none" fontcolor="#7f7f7f" label = <file<br/>download>]
        a -> a_text [style = invis]
    }

    subgraph cluster_b {
        node [color="#446699" fillcolor="#99aaee"]
        b [label = <<b>b</b>> class="item_b_in_progress" style="dashed,filled"]
        b_text [shape="plain" style="none" fontcolor="#7f7f7f" label = <server<br/>instance>]
        b -> b_text [style = invis]
    }

    subgraph cluster_c {
        c [label = <<b>c</b>>]
        c_text [shape="plain" style="none" fontcolor="#7f7f7f" label = <file<br/>upload>]
        c -> c_text [style = invis]
    }

    a_text -> c [minlen = 9, color="#66aa66"]
    b_text -> c [minlen = 9, color="#6688bb"]
}
```

</div><div class="column_half">

<div class="peace_button_container">
    <div class="peace_button">
        <div class="peace_button_label">Current<br/>State</div>
    </div>
    <div class="peace_button peace_button_left peace_button_running">
        <div class="peace_button_label">Deploy<br/>(preview)</div>
    </div>
    <div class="peace_button peace_button_right peace_button_running">
        <div class="peace_button_label"><div class="peace_button_spinner"></div></div>
    </div>
    <div class="peace_button peace_button_left">
        <div class="peace_button_label">Clean<br/>(preview)</div>
    </div>
    <div class="peace_button peace_button_right">
        <div class="peace_button_label">&#9654;</div>
    </div>
</div>

</div>


## Code Structure

All these endpoints and interfaces means, output code must be completely separate from the process automation logic.

### Swappable

#### CLI

```rust ,ignore
// defaults to:
//
// * stdout for outcome
// * stderr for progress
let mut output = CliOutput::new();
let mut cmd_context = CmdContext::builder()
    .with_output(&mut output)
    // ..
    .build();

StatusCmd::exec(&mut cmd_context).await?;
```

#### Web API: Synchronous Execution

Execute automation within request:

```diff
-let mut output = CliOutput::new();
+// Writes execution outcome to the response stream, no progress is written.
+let mut output = ResponseWriter::new(outcome_tx);
 let mut cmd_context = CmdContext::builder()
     // ..
```


#### Web API: Asynchronous Execution

* Asynchronous to a web request.
* Write outcome to a database, write progress to in-memory map.
* Return outcome or progress based on subsequent requests.

```diff
-let mut output = CliOutput::new();
+let mut output = SplitWriter {
+    outcome_writer: db_writer,       // persistent
+    progress_writer: session_writer, // transient
+}
 let mut cmd_context = CmdContext::builder()
     // ..
```

<details style="display: none;">
<summary>How To Hold Output</summary>

`output` is passed into the commands to output progress. Presenting the outcome of a command is still a responsibility of the caller.

```rust ,ignore
let mut output = CliOutput::new();
let mut cmd_context = CmdContext::builder()
    .with_output(&mut output)
    // ..
    .build();

StatusCmd::exec(&mut cmd_context).await?;
```

```rust ,ignore
let mut flow_context = FlowContext::builder()
    // ..
    .build();

let mut output = CliOutput::new();
StatusCmd::exec(&mut flow_context, &mut output).await?;
```

</details>

<style type="text/css">
.peace_button_container {
    display: flex;
    flex-direction: row;
}
.peace_button {
    background: linear-gradient(0deg, #ddddee, #eeeeff);
    border: 1px solid #9999aa;
    border-radius: 3px;
    padding: 0.3em 0.7em;
    font-size: 0.8em;
    font-weight: 550;
    text-align: center;
    flex-grow: 1;
    flex-shrink: 1;
    flex-basis: auto;
    cursor: pointer;
    /**/
    display: flex;
    align-items: center;
    justify-content: center;
}
.peace_button:hover {
    background: linear-gradient(0deg, #eeeeff, #fafaff);
    border: 1px solid #bbbbcc;
}
.peace_button:active {
    background: linear-gradient(0deg, #d0d0e0, #e0e0f0);
    border: 1px solid #9090a0;
}
.peace_button_running,
.peace_button_running:hover,
.peace_button_running:active {
    /* background: linear-gradient(0deg, #aaaaee, #ccccff); */
    background-color: #aabbf0;
    background-image: none; /* disables gradient */
    color: #0022558f;
    border: 1px solid #7070c0;
}
.peace_button_label {
    flex-grow: 1;
    flex-shrink: 1;
    flex-basis: auto;
}
.peace_button_left,
.peace_button_left:hover,
.peace_button_left:active {
    border-bottom-right-radius: 0px;
    border-top-right-radius: 0px;
}
.peace_button_right,
.peace_button_right:hover,
.peace_button_right:active {
    border-left: 0px;
    border-bottom-left-radius: 0px;
    border-top-left-radius: 0px;
}
.peace_button_spinner {
    display: inline-block;
    vertical-align: text-bottom;
    border: 2px solid transparent;
    background-clip: border-box, content-box;
    background-image: conic-gradient(#0022551f, #002255ff), conic-gradient(#aabbf0ff, #aabbf0ff);
    background-size: 100%;
    background-origin: border-box, content-box;
    background-blend-mode: screen;
    border-radius: 50%;
    width: 1em;
    height: 1em;
    animation: 3.0s linear forwards 0.0s infinite peace_button_spinner;
}
@keyframes peace_button_spinner {
    0% {
        transform: rotate(0deg);
    }
    100% {
        transform: rotate(360deg);
    }
}
.item_b_in_progress ellipse {
    animation: 10.0s linear forwards 0.0s infinite item_b_in_progress;
}
@keyframes item_b_in_progress {
    0% {
        transform-origin: 101px -196.31px;
        transform: rotate(0deg);
    }
    100% {
        transform-origin: 101px -196.31px;
        transform: rotate(360deg);
    }
}
.shell_progress_bar {
    background-repeat: no-repeat;
    background-blend-mode: lighten;
    background-clip: text;
    background-size: 50px;
    text-fill-color: transparent;
    animation: 2.5s linear forwards 1.0s infinite shell_progress_bar;
}
@keyframes shell_progress_bar {
    0% {
        background-position-x: left;
        background-image: linear-gradient(90deg, #000000 0%, #ffffffff 90%, #000000 100%);
    }
    30% {
        background-position-x: right;
        background-image: linear-gradient(90deg, #000000 0%, #ffffffff 90%, #000000 100%);
    }
    31% {
        background-position-x: right;
        background-image: linear-gradient(90deg, #000000 0%, #000000 100%);
    }
}
</style>
