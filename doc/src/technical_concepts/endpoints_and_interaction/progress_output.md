# Progress Output

> See also [Execution Progress](/technical_concepts/output/execution_progress.md).

The way progress is transferred to the user varies based on the `OutputWrite` and build  `target`.

1. **CLI:** This is pushed straight to the terminal
2. **Web:** This is pulled by the client from the server based on execution ID.
3. **WASM:** This is pushed by the WASM binary within the client.


## Implementation

### Web Interface

```rust ,ignore
#[component]
pub fn FlowGraph(execution_id: ReadSignal<ExecutionId>) -> impl IntoView {
    let progress_dot_resource = leptos::create_resource(
        || (),
        move |()| async move { progress_dot_graph(execution_id.get()).await.unwrap() },
    );
    let progress_dot_graph = move || {
        let progress_dot_graph = progress_dot_resource
            .get()
            .expect("Expected `progress_dot_graph` to always be generated successfully.");

        Some(progress_dot_graph)
    };

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

/// Returns the graph representing item execution progress.
#[leptos::server(endpoint = "/flow_graph")]
pub async fn progress_dot_graph(execution_id: ExecutionId) -> Result<DotSrcAndStyles, ServerFnError<NoCustomError>> {
    use dot_ix::{
        model::common::{graphviz_dot_theme::GraphStyle, GraphvizDotTheme},
        rt::IntoGraphvizDotSrc,
    };
    use peace_flow_model::FlowSpecInfo;

    let flow_spec_info = leptos::use_context::<FlowSpecInfo>().ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("`FlowSpecInfo` was not set.".to_string())
    })?;
    let cmd_progress_trackers = leptos::use_context::<CmdProgressTrackers>().ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("`CmdProgressTrackers` was not set.".to_string())
    })?;
    if let Some(cmd_progress_tracker) = cmd_progress_trackers.get(&execution_id) {
        // TODO: adjust styles on graph.
    }

    let progress_info_graph = flow_spec_info.into_progress_info_graph();
    Ok(IntoGraphvizDotSrc::into(
        &progress_info_graph,
        &GraphvizDotTheme::default().with_graph_style(GraphStyle::Circle),
    ))
}

```
