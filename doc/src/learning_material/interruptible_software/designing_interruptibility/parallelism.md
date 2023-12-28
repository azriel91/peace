# 🔀 Parallelism

> Parallelism and concurrency

<object
    id="diagram_in_progress_1"
    type="image/svg+xml"
    data="parallelism/diagram_in_progress_1.svg"
    style="margin-left: 48px; transform-origin: top left; scale: 1.2; margin-bottom: -162px; display: block;"></object>

<object
    id="diagram_in_progress_2"
    type="image/svg+xml"
    data="parallelism/diagram_in_progress_2.svg"
    style="margin-left: 48px; transform-origin: top left; scale: 1.2; margin-bottom: -162px; display: none;"></object>

<object
    id="diagram_in_progress_3"
    type="image/svg+xml"
    data="parallelism/diagram_in_progress_3.svg"
    style="margin-left: 48px; transform-origin: top left; scale: 1.2; margin-bottom: -162px; display: none;"></object>

<object
    id="diagram_done_1"
    type="image/svg+xml"
    data="parallelism/diagram_done_1.svg"
    style="margin-left: 48px; transform-origin: top left; scale: 1.2; margin-bottom: -162px; display: none;"></object>

<object
    id="diagram_done_2"
    type="image/svg+xml"
    data="parallelism/diagram_done_2.svg"
    style="margin-left: 48px; transform-origin: top left; scale: 1.2; margin-bottom: -162px; display: none;"></object>

<object
    id="diagram_done_3"
    type="image/svg+xml"
    data="parallelism/diagram_done_3.svg"
    style="margin-left: 48px; transform-origin: top left; scale: 1.2; margin-bottom: -162px; display: none;"></object>

<script type="text/javascript">
const RESET = 0;
const INTERRUPT_1 = 1;
const INTERRUPT_2 = 2;
const INTERRUPT_3 = 3;
const STOP_1 = 4;
const STOP_2 = 5;
const STOP_3 = 6;
function visibility_update(variant) {
    let diagram_in_progress_1 = 'none';
    let diagram_in_progress_2 = 'none';
    let diagram_in_progress_3 = 'none';
    let diagram_done_1 = 'none';
    let diagram_done_2 = 'none';
    let diagram_done_3 = 'none';
    let interruption_point_1 = '0';
    let interruption_point_2 = '0';
    let interruption_point_3 = '0';
    let stopping_point_1 = '0';
    let stopping_point_2 = '0';
    let stopping_point_3 = '0';
    switch (variant) {
        case RESET:
            diagram_in_progress_1 = 'block';
            break;
        case INTERRUPT_1:
            diagram_in_progress_1 = 'block';
            interruption_point_1 = '1.0';
            break;
        case INTERRUPT_2:
            diagram_in_progress_2 = 'block';
            interruption_point_2 = '1.0';
            break;
        case INTERRUPT_3:
            diagram_in_progress_3 = 'block';
            interruption_point_3 = '1.0';
            break;
        case STOP_1:
            diagram_done_1 = 'block';
            interruption_point_1 = '1.0';
            stopping_point_1 = '1.0';
            break;
        case STOP_2:
            diagram_done_2 = 'block';
            interruption_point_2 = '1.0';
            stopping_point_2 = '1.0';
            break;
        case STOP_3:
            diagram_done_3 = 'block';
            interruption_point_3 = '1.0';
            stopping_point_3 = '1.0';
            break;
    }
    document
        .getElementById('diagram_in_progress_1')
        .style
        .setProperty('display', diagram_in_progress_1);
    document
        .getElementById('diagram_in_progress_2')
        .style
        .setProperty('display', diagram_in_progress_2);
    document
        .getElementById('diagram_in_progress_3')
        .style
        .setProperty('display', diagram_in_progress_3);
    document
        .getElementById('diagram_done_1')
        .style
        .setProperty('display', diagram_done_1);
    document
        .getElementById('diagram_done_2')
        .style
        .setProperty('display', diagram_done_2);
    document
        .getElementById('diagram_done_3')
        .style
        .setProperty('display', diagram_done_3);
    document
        .getElementById('interruption_point_1')
        .style
        .setProperty('opacity', interruption_point_1);
    document
        .getElementById('interruption_point_2')
        .style
        .setProperty('opacity', interruption_point_2);
    document
        .getElementById('interruption_point_3')
        .style
        .setProperty('opacity', interruption_point_3);
    document
        .getElementById('stopping_point_1')
        .style
        .setProperty('opacity', stopping_point_1);
    document
        .getElementById('stopping_point_2')
        .style
        .setProperty('opacity', stopping_point_2);
    document
        .getElementById('stopping_point_3')
        .style
        .setProperty('opacity', stopping_point_3);
}
</script>

<div style="
    width: 100%;
" inert>
    <!-- Interruption points -->
    <div id="interruption_point_1" style="
        position: relative;
        left: 77px;
        top: -50px;
        display: inline-flex;
        flex-direction: column;
        justify-content: center;
        opacity: 0;
    ">
        <div style="
            display: inline-block;
            height: 210px;
            border-left-color: #f59e0b;
            border-left-style: dashed;
            border-left-width: 3px;
        "></div>
        <div style="
            display: inline-block;
            font-weight: bold;
            font-size: 20px;
            margin-left: -50%;
        ">🛑 Interrupt</div>
    </div>
    <div id="interruption_point_2" style="
        position: relative;
        left: 163px;
        top: -50px;
        display: inline-flex;
        flex-direction: column;
        justify-content: center;
        opacity: 0;
    ">
        <div style="
            display: inline-block;
            height: 210px;
            border-left-color: #f59e0b;
            border-left-style: dashed;
            border-left-width: 3px;
        "></div>
        <div style="
            display: inline-block;
            font-weight: bold;
            font-size: 20px;
            margin-left: -50%;
        ">🛑 Interrupt</div>
    </div>
    <div id="interruption_point_3" style="
        position: relative;
        left: 208px;
        top: -50px;
        display: inline-flex;
        flex-direction: column;
        justify-content: center;
        opacity: 0;
    ">
        <div style="
            display: inline-block;
            height: 210px;
            border-left-color: #f59e0b;
            border-left-style: dashed;
            border-left-width: 3px;
        "></div>
        <div style="
            display: inline-block;
            font-weight: bold;
            font-size: 20px;
            margin-left: -50%;
        ">🛑 Interrupt</div>
    </div>
    <!-- Stopping points -->
    <div id="stopping_point_1" style="
        position: relative;
        left: -174px;
        display: inline-flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
        opacity: 0;
    ">
        <div style="
            display: inline-block;
            height: 265px;
            border-left-color: #f59e0b;
            border-left-style: dashed;
            border-left-width: 3px;
        "></div>
        <div style="
            display: inline-block;
            font-weight: bold;
            font-size: 20px;
            margin-left: -50%;
        ">🚏 Stop</div>
    </div>
    <div id="stopping_point_2" style="
        position: relative;
        left: -105px;
        display: inline-flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
        opacity: 0;
    ">
        <div style="
            display: inline-block;
            height: 265px;
            border-left-color: #f59e0b;
            border-left-style: dashed;
            border-left-width: 3px;
        "></div>
        <div style="
            display: inline-block;
            font-weight: bold;
            font-size: 20px;
            margin-left: -50%;
        ">🚏 Stop</div>
    </div>
    <div id="stopping_point_3" style="
        position: relative;
        left: 7px;
        display: inline-flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
        opacity: 0;
    ">
        <div style="
            display: inline-block;
            height: 265px;
            border-left-color: #f59e0b;
            border-left-style: dashed;
            border-left-width: 3px;
        "></div>
        <div style="
            display: inline-block;
            font-weight: bold;
            font-size: 20px;
            margin-left: -50%;
        ">🚏 Stop</div>
    </div>
</div>

<div style="text-align: right;">
    <input
        type="button"
        value="Interrupt 1"
        onclick="visibility_update(INTERRUPT_1);"
    ></input>
    <input
        type="button"
        value="Finish 1"
        onclick="visibility_update(STOP_1);"
    ></input>
    <input
        type="button"
        value="Interrupt 2"
        onclick="visibility_update(INTERRUPT_2);"
    ></input>
    <input
        type="button"
        value="Finish 2"
        onclick="visibility_update(STOP_2);"
    ></input>
    <input
        type="button"
        value="Interrupt 3"
        onclick="visibility_update(INTERRUPT_3);"
    ></input>
    <input
        type="button"
        value="Finish 3"
        onclick="visibility_update(STOP_3);"
    ></input>
    <input
        type="button"
        value="Reset"
        onclick="visibility_update(RESET);"
    ></input>
</div>

## Safe Interruption Rules

1. 🔵 Finish everything in progress.
2. ⚫ Don't start anything new.

<details>
<summary>See <code>fn_graph</code> on Github.</summary>

* [Queuer](https://github.com/azriel91/fn_graph/blob/1ef048a6f3827d64fd4eca5dd90a871798bf25ea/src/fn_graph.rs#L1529-L1536):
    - Sends IDs of steps that can be executed.
    - Receives IDs of steps that are complete.
    - Checks for interruption.
* [Scheduler](https://github.com/azriel91/fn_graph/blob/1ef048a6f3827d64fd4eca5dd90a871798bf25ea/src/fn_graph.rs#L1550-L1575)
    - Receives IDs of steps that can be executed.
    - Sends IDs of steps that are complete.

</details>

