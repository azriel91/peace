use leptos::{component, view, IntoView};
use peace::webi_components::FlowGraph;
use tokio::sync::mpsc::{self, error::SendError};

use crate::web_components::CmdExecRequest;

/// Top level component of the `WebiOutput`.
#[component]
pub fn EnvDeployHome() -> impl IntoView {
    let discover_cmd_exec = leptos::create_action(|(): &()| {
        let cmd_exec_request_tx = leptos::use_context::<mpsc::Sender<CmdExecRequest>>();
        let cmd_exec_request_tx = cmd_exec_request_tx.clone();

        async move {
            if let Some(cmd_exec_request_tx) = cmd_exec_request_tx {
                match cmd_exec_request_tx.send(CmdExecRequest::Discover).await {
                    Ok(()) => {
                        leptos::logging::log!("Sent Discover cmd.");
                    }
                    Err(SendError(_)) => {
                        leptos::logging::log!("Failed to send Discover cmd.");
                    }
                }
            }
        }
    });

    view! {
        <div>
            <h1>"Environment"</h1>

            <h2>"Example"</h2>
            <FlowGraph />

            <h2>"Current"</h2>
            <button
                on:click=move |_| discover_cmd_exec.dispatch(())
            >
                "Discover"
            </button>
        </div>
    }
}
