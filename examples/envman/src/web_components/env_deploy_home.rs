use leptos::{component, server, spawn_local, view, IntoView, ServerFnError};
use peace::webi_components::FlowGraph;

#[server]
async fn discover_cmd_exec() -> Result<(), ServerFnError> {
    use tokio::sync::mpsc;

    use crate::web_components::CmdExecRequest;

    let cmd_exec_request_tx = leptos::use_context::<mpsc::Sender<CmdExecRequest>>();

    leptos::logging::log!("Discover clicked.");
    if let Some(cmd_exec_request_tx) = cmd_exec_request_tx {
        match cmd_exec_request_tx.try_send(CmdExecRequest::Discover) {
            Ok(()) => {
                leptos::logging::log!("Sent Discover cmd.");
            }
            Err(e) => {
                leptos::logging::log!("Failed to send Discover cmd: {e}");
            }
        }
    } else {
        leptos::logging::log!("`cmd_exec_request_tx` is None");
    }

    Ok(())
}

/// Top level component of the `WebiOutput`.
#[component]
pub fn EnvDeployHome() -> impl IntoView {
    view! {
        <div>
            <h1>"Environment"</h1>

            <h2>"Example"</h2>
            <FlowGraph />

            <h2>"Current"</h2>
            <button
                on:click=move |_| {
                    spawn_local(async {
                        discover_cmd_exec()
                            .await
                            .expect("Expected `discover_cmd_exec` call to succeed.");
                    });
                }
            >
                "Discover"
            </button>
        </div>
    }
}
