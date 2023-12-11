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

<iframe
    src="http://localhost:7890/?src=hierarchy%3A%0A%20%20server%3A%0A%20%20%20%20server_inner%3A%0A%20%20upload%3A%0A%20%20%20%20upload_part_1%3A%0A%20%20%20%20upload_part_2%3A%0A%20%20%20%20upload_part_3%3A%0A%20%20config%3A%0A%20%20%20%20config_inner%3A%0A%20%20start%3A%0A%20%20%20%20start_inner%3A%0Anode_infos%3A%0A%20%20server%3A%20%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22%22%20%7D%0A%20%20server_inner%3A%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22..%3Cbr%20%2F%3E..%22%20%7D%0A%20%20upload%3A%20%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%22App%3Cbr%20%2F%3EUpload%22%20%20%20%20%7D%0A%20%20upload_part_1%3A%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%221%22%20%7D%0A%20%20upload_part_2%3A%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%222%22%20%7D%0A%20%20upload_part_3%3A%20%7B%20emoji%3A%20%F0%9F%93%A4%2C%20name%3A%20%223%22%20%7D%0A%20%20config%3A%20%20%7B%20emoji%3A%20%F0%9F%9B%A0%EF%B8%8F%2C%20name%3A%20%22%22%20%7D%0A%20%20config_inner%3A%20%7B%20emoji%3A%20%F0%9F%96%A5%EF%B8%8F%2C%20name%3A%20%22..%3Cbr%20%2F%3E..%22%20%7D%0A%20%20start%3A%20%20%20%7B%20emoji%3A%20%F0%9F%94%81%2C%20name%3A%20%22%22%20%20%20%7D%0A%20%20start_inner%3A%20%7B%20emoji%3A%20%F0%9F%94%81%2C%20name%3A%20%22..%3Cbr%20%2F%3E..%22%20%7D%0Aedges%3A%0A%20%20server__upload%3A%20%5Bserver%2C%20upload%5D%0A%20%20upload__config%3A%20%5Bupload%2C%20config%5D%0A%20%20config__start%3A%20%5Bconfig%2C%20start%5D%0A%20%20upload_part_1__upload_part_2%3A%20%5Bupload_part_1%2C%20upload_part_2%5D%0A%20%20upload_part_2__upload_part_3%3A%20%5Bupload_part_2%2C%20upload_part_3%5D%0Atailwind_classes%3A%0A%20%20server_inner%3A%20hidden%0A%20%20config_inner%3A%20hidden%0A%20%20start_inner%3A%20hidden%0A%20%20server%3A%20%26green%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Afill-green-300%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-green-600%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Afill-green-200%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Astroke-green-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A%20%20upload%3A%20%26blue%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Afill-blue-300%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-blue-600%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Afill-blue-100%0A%20%20%20%20%5B%26%3Epath%5D%3Ahover%3Astroke-blue-400%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Afill-blue-200%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-blue-500%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A%20%20upload_part_1%3A%20%2Agreen%0A%20%20upload_part_2%3A%20%2Ablue%0A%20%20server__upload%3A%20%3E-%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epath%5D%3Astroke-green-700%0A%20%20%20%20%5B%26%3Epolygon%5D%3Afill-green-700%0A%20%20%20%20%5B%26%3Epolygon%5D%3Astroke-1%0A%20%20%20%20%5B%26%3Epolygon%5D%3Astroke-green-700%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-1%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-lime-600%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Aoutline-dashed%0A%20%20%20%20%5B%26%3Epath%5D%3Afocus%3Arounded-xl%0A%20%20%20%20cursor-pointer%0A&diagram_only=true"
    width="630" height="230"
    style="border: 0; transform-origin: top left; scale: 1.2; margin-bottom: -50px;">
</iframe>

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
