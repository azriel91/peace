# Interruptibility

How should the interrupt channel be initialized and stored?

## Use Cases

When automation software is used as:

1. CLI interactive / non-interactive.
2. Web service + browser access.
3. CLI + web service + browser access.


In all cases, we need to:

1. Initialize the `CmdCtx` with the `interrupt_rx`
2. Spawn the listener that will send `InterruptSignal` in `interrupt_tx`.


## Imagined Code

### CLI Interactive / Non-interactive

Both interactive and non-interactive can listen for `SIGINT`:

* Interactive: `SIGINT` will be sent by the user pressing `Ctrl + C`.
* Non-interactive: `SIGINT` could be sent by a CI thread.

```rust
let (interrupt_tx, interrupt_rx) = oneshot::channel::<InterruptSignal>();

tokio::task::spawn(async move {
    // Note: Once tokio takes over the process' `SIGINT` handler, it cannot be undone.
    //
    // This limitation is due to how Linux currently works.
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to initialize signal handler for SIGINT");

    let (Ok(()) | Err(InterruptSignal)) = interrupt_tx.send(InterruptSignal);
});

let mut cmd_ctx = CmdCtx::single_profile_single_flow(output, workspace, interrupt_rx)
    .build();

let cmd_outcome = EnsureCmd::exec(&mut cmd_ctx).await?;
```


### Web Service

The `interrupt_tx` must be accessible from a separate web request.

```rust
async fn cmd_exec_start_handler(params: Params) -> CmdExecutionId {
    let (interrupt_tx, interrupt_rx) = oneshot::channel::<InterruptSignal>();
    let mut cmd_ctx = CmdCtx::single_profile_single_flow(output, workspace, interrupt_rx)
        .build();

    let cmd_execution_id = EnsureCmd::exec_bg(cmd_ctx);

    let cmd_execution_by_id = cmd_execution_by_id
        .lock()
        .await;
    cmd_execution_by_id.insert(cmd_execution_id, interrupt_tx);

    cmd_execution_id
}

/// Returns the progress of the `CmdExecution`.
async fn cmd_exec_progress_handler(cmd_execution_id: CmdExecutionId) -> Result<CmdProgress, E> {
    self.cmd_progress_storage.get(cmd_execution_id).await
}

async fn cmd_exec_interrupt_handler(cmd_execution_id: CmdExecutionId) -> Result<(), E> {
    let cmd_execution_by_id = cmd_execution_by_id
        .lock()
        .await;

    if let Some(interrupt_tx) = cmd_execution_by_id.get(cmd_execution_id) {
        let (Ok(()) | Err(InterruptSignal)) = interrupt_tx.send(InterruptSignal);

        Ok(())
    } else {
        Err(E::from(Error::CmdExecutionIdNotFound { cmd_execution_id }))
    }
}
```


### CLI + Web Service

There are two variants of CLI and web service:

1. CLI command running on the user's machine, web service that is a UI for that one command execution.
2. CLI client to a web service, so the CLI is just a REST client.


#### CLI on User's Machine + Web UI

For the first variant, the `CmdExecution` invocation is similar to Web Service, with the following differences:

* Output progress is pushed to both CLI and `CmdProgress` storage.
* Interruptions are received from both process `SIGINT` and client requests.

```rust
async fn cmd_exec_start(params: Params) {
    let (interrupt_tx, interrupt_rx) = oneshot::channel::<InterruptSignal>();
    let mut cmd_ctx = CmdCtx::single_profile_single_flow(output, workspace, interrupt_rx)
        .build();

    let cmd_execution_id = EnsureCmd::exec_bg(cmd_ctx);

    // We store an `interrupt_tx` per `CmdExecutionId`,
    // as well as spawn a Ctrl C handler.
    let cmd_execution_by_id = cmd_execution_by_id
        .lock()
        .await;
    cmd_execution_by_id.insert(cmd_execution_id, interrupt_tx.clone());

    tokio::task::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to initialize signal handler for SIGINT");

        let (Ok(()) | Err(InterruptSignal)) = interrupt_tx.send(InterruptSignal);
    });

    // TODO: store `cmd_execution_id` as the only running `CmdExecution`.
}

/// Returns the progress of the `CmdExecution`.
async fn cmd_exec_progress_handler(cmd_execution_id: CmdExecutionId) -> Result<CmdProgress, E> {
    self.cmd_progress_storage.get(cmd_execution_id).await
}

async fn cmd_exec_interrupt_handler(cmd_execution_id: CmdExecutionId) -> Result<(), E> {
    let cmd_execution_by_id = cmd_execution_by_id
        .lock()
        .await;

    if let Some(interrupt_tx) = cmd_execution_by_id.get(cmd_execution_id) {
        let (Ok(()) | Err(InterruptSignal)) = interrupt_tx.send(InterruptSignal);

        Ok(())
    } else {
        Err(E::from(Error::CmdExecutionIdNotFound { cmd_execution_id }))
    }
}
```

#### CLI as Rest Client to Web Service

This is essentially the Web Service implementation, but rendering the progress on the machine with the CLI.
