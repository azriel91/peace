# Interruption

User has sent an interruption request. These are received differently depending on the `Output` that presented the interface to the user.

* **CLI:**

    - The developer needs to [define the `SIGINT` handler][sigint_handler_def], and pass it into Peace ([1][interruptibility_augment_add], [2][interruptibility_augment_call]).
    - The user presses Ctrl C to interrupt the command execution.

* **Web:**

    - The framework needs to track an execution ID to the `CmdExecution`'s interrupt sender.
    - User sends an interrupt request with the execution ID.


## Implementation

This follows on from the [**Cmd Invocation > Web Interface**](cmd_invocation.md#web-interface) design.


### 4. Web Component `InterruptSenders` Access

`InterruptSenders` may be a newtype for `Map<ExecutionId, Sender<InterruptSignal>>`

See:

* [`leptos`: actions](https://book.leptos.dev/async/13_actions.html).
* [`leptos_router::ActionForm`](https://docs.rs/leptos_router/latest/leptos_router/fn.ActionForm.html)
* [`leptos::create_server_action`](https://docs.rs/leptos/latest/leptos/fn.create_server_action.html)
* [`todo_app_sqlite`](https://github.com/leptos-rs/leptos/blob/main/examples/todo_app_sqlite/src/todo.rs)


```rust ,ignore
#[leptos::server(endpoint = "/cmd_exec_interrupt")]
pub async fn cmd_exec_interrupt(
    execution_id: &ExecutionId,
) -> Result<(), ServerFnError<NoCustomError>> {
    let interrupt_senders = leptos::use_context::<InterruptSenders>()
        .ok_or_else(|| {
            ServerFnError::<NoCustomError>::ServerError(
                "`InterruptSenders` was not set.".to_string()
            )
        })?;
    if let Some(interrupt_sender) = interrupt_senders.get(execution_id) {
        interrupt_sender.send(InterruptSignal).await;
    }

    Ok(())
}

#[component]
pub fn InterruptButton(
    execution_id: ReadSignal<ExecutionId>,
) -> impl IntoView {
    let cmd_exec_interrupt_action = leptos::create_action(
        |execution_id: &ExecutionId| {
            let execution_id = execution_id.clone();
            async move { cmd_exec_interrupt(&execution_id).await }
        },
    );
    let submitted = cmd_exec_interrupt_action.input(); // RwSignal<Option<String>>
    let pending = cmd_exec_interrupt_action.pending(); // ReadSignal<bool>
    let todo_id = cmd_exec_interrupt_action.value(); // RwSignal<Option<Uuid>>

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default(); // don't reload the page.
                cmd_exec_interrupt_action.dispatch(execution_id.get());
            }
        >
            // Execution ID
            <button type="submit">"Interrupt"</button>
        </form>
        // use our loading state
        <p>{move || pending().then("Loading...")}</p>
    }
}
```

[sigint_handler_def]: https://github.com/azriel91/peace/blob/4e8077103b6361e3e9a58e2adf177df1eec1490b/examples/envman/src/cmds/cmd_ctx_builder.rs#L29-L37
[interruptibility_augment_add]: https://github.com/azriel91/peace/blob/4e8077103b6361e3e9a58e2adf177df1eec1490b/examples/envman/src/cmds/cmd_ctx_builder.rs#L39-L43
[interruptibility_augment_call]: https://github.com/azriel91/peace/blob/4e8077103b6361e3e9a58e2adf177df1eec1490b/examples/envman/src/cmds/env_cmd.rs#L68
