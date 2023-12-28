# 🗺️ Strategy

<object
    id="diagram_in_progress"
    type="image/svg+xml"
    data="strategy/diagram_in_progress.svg"
    style="margin-left: 48px; transform-origin: top left; scale: 1.2; margin-bottom: -162px; display: block;"></object>

<object
    id="diagram_done_1"
    type="image/svg+xml"
    data="strategy/diagram_done_1.svg"
    style="margin-left: 48px; transform-origin: top left; scale: 1.2; margin-bottom: -162px; display: none;"></object>

<object
    id="diagram_done_2"
    type="image/svg+xml"
    data="strategy/diagram_done_2.svg"
    style="margin-left: 48px; transform-origin: top left; scale: 1.2; margin-bottom: -162px; display: none;"></object>

<object
    id="diagram_done_3"
    type="image/svg+xml"
    data="strategy/diagram_done_3.svg"
    style="margin-left: 48px; transform-origin: top left; scale: 1.2; margin-bottom: -162px; display: none;"></object>

<script type="text/javascript">
const RESET = 0;
const INTERRUPT = 1;
const STOP_1 = 2;
const STOP_2 = 3;
const STOP_3 = 4;
function visibility_update(variant) {
    let diagram_in_progress = 'none';
    let diagram_done_1 = 'none';
    let diagram_done_2 = 'none';
    let diagram_done_3 = 'none';
    let interruption_point = '0';
    let stopping_point_1 = '0';
    let stopping_point_2 = '0';
    let stopping_point_3 = '0';
    switch (variant) {
        case RESET:
            diagram_in_progress = 'block';
            break;
        case INTERRUPT:
            diagram_in_progress = 'block';
            interruption_point = '1.0';
            break;
        case STOP_1:
            diagram_done_1 = 'block';
            interruption_point = '1.0';
            stopping_point_1 = '1.0';
            break;
        case STOP_2:
            diagram_done_2 = 'block';
            interruption_point = '1.0';
            stopping_point_2 = '1.0';
            break;
        case STOP_3:
            diagram_done_3 = 'block';
            interruption_point = '1.0';
            stopping_point_3 = '1.0';
            break;
    }
    document
        .getElementById('diagram_in_progress')
        .style
        .setProperty('display', diagram_in_progress);
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
        .getElementById('interruption_point')
        .style
        .setProperty('opacity', interruption_point);
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
    <div id="interruption_point" style="
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
    <!-- Stopping points -->
    <div id="stopping_point_1" style="
        position: relative;
        left: 72px;
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
        left: 137px;
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
        left: 253px;
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
        onclick="visibility_update(INTERRUPT);"
    ></input>
    <input
        type="button"
        value="Finish 1"
        onclick="visibility_update(STOP_1);"
    ></input>
    <input
        type="button"
        value="Finish 2"
        onclick="visibility_update(STOP_2);"
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

```rust ,ignore
/// How to poll an underlying stream when an interruption is received.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterruptStrategy {
    /// On interrupt, keep going.
    IgnoreInterruptions,
    /// On interrupt, wait for the current future's to complete and yield its
    /// output, but do not poll the underlying stream for any more futures.
    FinishCurrent,
    /// On interrupt, continue polling the stream for the next `n` futures.
    ///
    /// `n` is an upper bound, so fewer than `n` futures may be yielded if the
    /// underlying stream ends early.
    PollNextN(u64),
}
```

<!--
1. Think back to the bus example.
2. What if, we want to press the stop button, but we want the bus to stop not at the immediate next stop, but a number of stops down the line.
3. Like before you get on the bus, you schedule that you want the bus to stop 8 stops away -- because that's where your workplace is.
4. That way, you don't have to be alert to press the stop button just before your stop.
5. When writing complex automation with hundreds of steps, this is what we want to for testing.
-->
