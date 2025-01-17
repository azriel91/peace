use leptos::{
    component,
    prelude::{ClassAttribute, ElementChild, OnAttribute, ServerFnError},
    server,
    task::spawn_local,
    view, IntoView,
};
use peace::webi_components::{FlowGraph, FlowGraphCurrent};

use crate::web_components::TabLabel;

#[server]
async fn discover_cmd_exec() -> Result<(), ServerFnError> {
    use tokio::sync::mpsc;

    use crate::web_components::CmdExecRequest;

    let cmd_exec_request_tx = leptos::context::use_context::<mpsc::Sender<CmdExecRequest>>();

    if let Some(cmd_exec_request_tx) = cmd_exec_request_tx {
        match cmd_exec_request_tx.try_send(CmdExecRequest::Discover) {
            Ok(()) => {}
            Err(e) => {
                leptos::logging::log!("Failed to send Discover cmd: {e}");
            }
        }
    } else {
        leptos::logging::log!("`cmd_exec_request_tx` is None");
    }

    Ok(())
}

#[server]
async fn deploy_cmd_exec() -> Result<(), ServerFnError> {
    use tokio::sync::mpsc;

    use crate::web_components::CmdExecRequest;

    let cmd_exec_request_tx = leptos::context::use_context::<mpsc::Sender<CmdExecRequest>>();

    if let Some(cmd_exec_request_tx) = cmd_exec_request_tx {
        match cmd_exec_request_tx.try_send(CmdExecRequest::Ensure) {
            Ok(()) => {}
            Err(e) => {
                leptos::logging::log!("Failed to send Ensure cmd: {e}");
            }
        }
    } else {
        leptos::logging::log!("`cmd_exec_request_tx` is None");
    }

    Ok(())
}

#[server]
async fn clean_cmd_exec() -> Result<(), ServerFnError> {
    use tokio::sync::mpsc;

    use crate::web_components::CmdExecRequest;

    let cmd_exec_request_tx = leptos::context::use_context::<mpsc::Sender<CmdExecRequest>>();

    if let Some(cmd_exec_request_tx) = cmd_exec_request_tx {
        match cmd_exec_request_tx.try_send(CmdExecRequest::Clean) {
            Ok(()) => {}
            Err(e) => {
                leptos::logging::log!("Failed to send Clean cmd: {e}");
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
    let button_tw_classes = "\
        border \
        rounded \
        px-4 \
        py-3 \
        text-m \
        \
        border-slate-400 \
        bg-gradient-to-b \
        from-slate-200 \
        to-slate-300 \
        \
        hover:border-slate-300 \
        hover:bg-gradient-to-b \
        hover:from-slate-100 \
        hover:to-slate-200 \
        \
        active:border-slate-500 \
        active:bg-gradient-to-b \
        active:from-slate-300 \
        active:to-slate-400 \
    ";

    view! {
        <div>
            <h1>"Environment"</h1>

            <TabLabel
                tab_group_name="environment_tabs"
                tab_id="tab_environment_example"
                label="Environment Example"
                checked=true
            />
            <TabLabel
                tab_group_name="environment_tabs"
                tab_id="tab_environment_current"
                label="Current"
            />

            <div class="\
                invisible \
                clear-both \
                h-0 \
                peer-checked/tab_environment_example:visible \
                peer-checked/tab_environment_example:h-full \
                "
            >
                <FlowGraph />
            </div>

            <div class="\
                invisible \
                clear-both \
                h-0 \
                peer-checked/tab_environment_current:visible \
                peer-checked/tab_environment_current:h-full \
                "
            >
                <button
                    on:click=move |_| {
                        spawn_local(async {
                            discover_cmd_exec()
                                .await
                                .expect("Expected `discover_cmd_exec` call to succeed.");
                        });
                    }
                    class=button_tw_classes
                >
                    "🗺️ Discover"
                </button>
                <button
                    on:click=move |_| {
                        spawn_local(async {
                            deploy_cmd_exec()
                                .await
                                .expect("Expected `deploy_cmd_exec` call to succeed.");
                        });
                    }
                    class=button_tw_classes
                >
                    "🚀 Deploy"
                </button>
                <button
                    on:click=move |_| {
                        spawn_local(async {
                            clean_cmd_exec()
                                .await
                                .expect("Expected `clean_cmd_exec` call to succeed.");
                        });
                    }
                    class=button_tw_classes
                >
                    "🧹 Clean"
                </button>
                <FlowGraphCurrent />
            </div>

        </div>
    }
}
