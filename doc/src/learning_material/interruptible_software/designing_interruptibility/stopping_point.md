# 🚏 Stopping Point

<object
    type="image/svg+xml"
    data="stopping_point/process.svg"
    style="margin-left: 23px; transform-origin: top left; scale: 1.3; margin-bottom: -50px;"></object>

<div style="
    width: 100%;
">
    <!-- Interruption point -->
    <div style="
        position: relative;
        left: 220px;
        top: -40px;
        display: inline-flex;
        flex-direction: column;
        justify-content: center;
    ">
        <div style="
            display: inline-block;
            height: 70px;
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
    <!-- Stopping point -->
    <div style="
        position: relative;
        left: 171px;
        display: inline-flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
    ">
        <div style="
            display: inline-block;
            height: 135px;
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

## No Interruptibility

```rust ,ignore
fn execute(
    params: Params,
) -> Outcome {
    let output_1 = step_1(params);
    let output_2 = step_2(output_1);
    let outcome = step_3(output_2);

    return outcome;
}
```

## Basic

```rust ,ignore
fn execute(
    interrupt_rx: mut Receiver<InterruptSignal>,
    params: Params,
) -> ControlFlow<InterruptSignal, Outcome> {
    let () = interruptibility_check(&mut interrupt_rx)?;
    let output_1 = step_1(params);

    let () = interruptibility_check(&mut interrupt_rx)?;
    let output_2 = step_2(output_1);

    let () = interruptibility_check(&mut interrupt_rx)?;
    let outcome = step_3(output_2);

    ControlFlow::Continue(outcome)
}

fn interruptibility_check(receiver: &mut Receiver<InterruptSignal>)
-> ControlFlow<InterruptSignal, ()> {
    if let Ok(interrupt_signal) = interrupt_rx.try_recv() {
        ControlFlow::Continue(())
    } else {
        ControlFlow::Break(interrupt_signal)
    }
}
```

## Fine Grained Interruptibility

<object
    type="image/svg+xml"
    data="stopping_point/fine_grained_interruptibility.svg"
    style="width: 630px; height: 230px; border: 0; transform-origin: top left; scale: 1.2;"></object>

```rust ,ignore
fn execute(
    interrupt_rx: mut Receiver<InterruptSignal>,
    params: Params,
) -> ControlFlow<InterruptSignal, Outcome> {
    let () = interruptibility_check(&mut interrupt_rx)?;
    let output_1 = step_1(&mut interrupt_rx, params);

    // ..
}

fn step_1(
    interrupt_rx: mut Receiver<InterruptSignal>,
    params: Params,
) -> ControlFlow<Output1, Outcome> {
    let mut output_1 = Output1::new();
    for i in 0..1_000_000 {
        if i % 1_000 == 0 {
            let () = interruptibility_check(&mut interrupt_rx)?;
        }
        do_something(output_1, params, i);
    }

    ControlFlow::Continue(output_1)
}
```

<!--
1. Now that the user has sent us their intent to stop, we need to find a safe place to stop.
2. i.e. we need to build our bus stops.
3. We can of course insert interruption checks every so often.
4. So in this code, you can see that we check for interruptions before every step.
5. If our steps do a lot of work, we can pass the interrupt signal receiver down to each step if necessary.
-->
