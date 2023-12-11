# 🛑 Stop Button

<div style="
    display: grid;
    grid-column: 3;
    grid-template-columns: repeat(3, 1fr);
    gap: 20px;
">
    <!-- Labels -->
    <h3 style="
        grid-column: 1;
        grid-row: 1;
    ">Command Line</h3>
    <h3 style="
        grid-column: 2;
        grid-row: 1;
    ">Web</h3>
    <h3 style="
        grid-column: 3;
        grid-row: 1;
    ">Desktop</h3>
    <!-- Pictures -->
    <div style="
        grid-column: 1;
        grid-row: 2;
        border-radius: 10px;
        width: 180px;
        height: 80px;
        background-color: black;
        color: white;
        font-size: 30px;
        font-weight: bold;
        font-family: monospace;
        padding: 20px;
    ">
        <span style="color: lightgreen;">&gt;</span> deploy_
    </div>
    <div style="
        grid-column: 2;
        grid-row: 2;
        font-size: 120px;
        margin-top: -25px;
    ">
        <span style="display: inline-block; width: 40px;">🗄️</span><span style="display: inline-block; width: 80px; margin-right: -10px;">🗄️</span><span style="display: inline-block; font-size: 60px; vertical-align: top; margin-top: -10px;">🌐</span>
    </div>
    <div style="
        grid-column: 3;
        grid-row: 2;
        border-radius: 10px;
        width: 210px;
        height: 110px;
        background-image: linear-gradient(
            180deg,
            rgba(50, 150, 200, 1) 0%,
            rgba(20, 50, 90, 1) 100%
        );
        padding: 3px;
    ">
        <!-- Title bar -->
        <div style="
            border-top-left-radius: 10px;
            border-top-right-radius: 10px;
            width: 210px;
            height: 20px;
            background-image: linear-gradient(180deg,
                rgba(30, 180, 200, 1) 0%,
                rgba(0, 150, 180, 1) 100%
            );
            color: white;
            font-size: 13px;
            font-weight: bold;
            font-family: monospace;
            display: flex;
            justify-content: flex-end;
            align-items: center;
        ">
            <div style="
                border-radius: 4px;
                width: 18px;
                height: 18px;
                text-align: center;
                background-image: linear-gradient(180deg,
                    rgba(200, 30, 10, 1) 0%,
                    rgba(127, 15, 5, 1) 100%
                );">x</div>
        </div>
        <!-- Window contents -->
        <div style="
            display: flex;
            align-items: center;
            justify-content: space-around;
            border-bottom-left-radius: 10px;
            border-bottom-right-radius: 10px;
            width: 210px;
            height: 90px;
            background-color: #eefefe;
            color: white;
            font-size: 20px;
            font-weight: bold;
            font-family: monospace;
        ">
            <div style="
                border-radius: 4px;
                width: 40px;
                padding: 2px;
                text-align: center;
                background-image: linear-gradient(180deg,
                    rgba(100, 200, 255, 1) 0%,
                    rgba(50, 150, 200, 1) 100%
                );">go</div>
        </div>
    </div>
</div>

## ❌ Don't

```rust ,ignore
let outcome = tokio::select! {
    value = process.next()          => Outcome::Done(value),
        _ = tokio::signal::ctrl_c() => Outcome::Interrupted,
};
```

## ✅ Do

```rust ,ignore
struct InterruptSignal;
let (interrupt_tx, interrupt_rx) = tokio::sync::mpsc::<InterruptSignal>(8);

let outcome = tokio::select! {
    value = process.next()      => Outcome::Done(value),
        _ = interrupt_rx.recv() => Outcome::Interrupted,
};
```

and:

```rust ,ignore
// CLI
tokio::spawn(async move {
    tokio::signal::ctrl_c().await.unwrap();
    let _ = interrupt_tx.send(InterruptSignal).await;
});

// Web / Desktop
async fn handle_request(request: Request) -> Response {
    match request {
        Request::Interrupt { execution_id } => {
            let executions = executions.lock().await;
            let execution = executions.get(execution_id);
            let interrupt_tx = execution.interrupt_tx();
            let _ = interrupt_tx.send(InterruptSignal).await;
        }
        // ..
    }
}

```

<!--
1. Automation software can be implemented for use on the command line, through a web interface, and as a desktop application.
2. This is significant because to support all three, you need to remember:
3. Command line tools tend to execute tasks on the main thread.
4. Web applications and desktop applications tend to execute tasks on a background thread.
5. Web applications listen for requests, and we need to map between the request and the execution, to interrupt it.
6. Desktop applications have an event loop to listen for user actions, and we need to map between the user action and the execution, to interrupt it.
7. If you want to support all these interfaces, don't use the operating system's interrupt signal as *the* interrupt signal.
8. What you should do is have an application level `InterruptSignal`.
9. And depending on how the automation logic is surfaced to the user, we translate between that usage and your interrupt signal.
-->
