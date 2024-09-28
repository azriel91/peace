use std::{net::SocketAddr, path::Path};

use axum::Router;
use futures::stream::{self, StreamExt, TryStreamExt};
use indexmap::IndexSet;
use leptos::view;
use leptos_axum::LeptosRoutes;
use peace_cmd_model::CmdExecutionId;
use peace_core::FlowId;
use peace_webi_components::Home;
use peace_webi_model::{CmdExecRequest, OutcomeInfoGraphVariant, WebiError};
use tokio::{io::AsyncWriteExt, sync::mpsc};
use tower_http::services::ServeDir;

use crate::{
    CmdExecSpawnCtx, CmdExecToLeptosCtx, FlowWebiFns, OutcomeInfoGraphCalculator, WebiOutput,
};

/// Maximum number of `CmdExecRequest`s to queue up.
const CMD_EXEC_REQUEST_CHANNEL_LIMIT: usize = 1024;

/// Web server that runs the following work:
///
/// * UI rendering with `leptos`.
/// * `CmdExecution` through receiving requests from leptos.
/// * Updating `leptos` context data for components to render.
#[derive(Debug)]
pub struct WebiServer;

impl WebiServer {
    /// Starts the web server.
    ///
    /// ## Parameters
    ///
    /// * `socker_addr`: IP address and port to listen on.
    pub async fn start<E>(
        socket_addr: Option<SocketAddr>,
        flow_webi_fns: FlowWebiFns<E>,
    ) -> Result<(), WebiError>
    where
        E: 'static,
    {
        let cmd_exec_to_leptos_ctx = CmdExecToLeptosCtx::default();
        let (cmd_exec_request_tx, cmd_exec_request_rx) =
            mpsc::channel(CMD_EXEC_REQUEST_CHANNEL_LIMIT);

        let flow_id = flow_webi_fns.flow.flow_id().clone();
        let webi_server_task = Self::leptos_server_start(
            socket_addr,
            cmd_exec_request_tx,
            cmd_exec_to_leptos_ctx.clone(),
            flow_id,
        );
        let cmd_execution_listener_task = Self::cmd_execution_listener(
            cmd_exec_request_rx,
            cmd_exec_to_leptos_ctx,
            flow_webi_fns,
        );

        tokio::try_join!(webi_server_task, cmd_execution_listener_task).map(|((), ())| ())
    }

    async fn cmd_execution_listener<E>(
        mut cmd_exec_request_rx: mpsc::Receiver<CmdExecRequest>,
        cmd_exec_to_leptos_ctx: CmdExecToLeptosCtx,
        flow_webi_fns: FlowWebiFns<E>,
    ) -> Result<(), WebiError>
    where
        E: 'static,
    {
        // TODO:
        //
        // 1. Listen for params specs
        // 2. Instantiate `CmdCtx`
        // 3. Calculate example `info_graph`s
        // 4. Insert into `FlowInfoGraphs`.
        let FlowWebiFns {
            flow,
            outcome_info_graph_fn,
            cmd_exec_spawn_fn,
        } = flow_webi_fns;
        let outcome_info_graph_fn = &outcome_info_graph_fn;

        let CmdExecToLeptosCtx {
            flow_progress_example_info_graphs,
            flow_progress_actual_info_graphs,
            flow_outcome_example_info_graphs,
            flow_outcome_actual_info_graphs,
            mut cmd_exec_interrupt_txs,
        } = cmd_exec_to_leptos_ctx;

        // TODO: remove this mock?
        // Should we have one WebiOutput for the whole server? doesn't seem right.
        let (web_ui_update_tx, _web_ui_update_rx) = mpsc::channel(128);
        let mut webi_output_mock = WebiOutput::new(web_ui_update_tx);
        let flow_spec_info = flow.flow_spec_info();
        let flow_progress_example_info_graph = flow_spec_info.to_progress_info_graph();
        let flow_outcome_example_info_graph = outcome_info_graph_fn(
            &mut webi_output_mock,
            Box::new(|flow, params_specs, resources| {
                OutcomeInfoGraphCalculator::calculate::<E>(
                    flow,
                    params_specs,
                    resources,
                    OutcomeInfoGraphVariant::Example,
                )
            }),
        )
        .await;

        let flow_id = flow.flow_id();
        if let Ok(mut flow_progress_example_info_graphs) = flow_progress_example_info_graphs.lock()
        {
            flow_progress_example_info_graphs
                .insert(flow_id.clone(), flow_progress_example_info_graph);
        }
        if let Ok(mut flow_outcome_example_info_graphs) = flow_outcome_example_info_graphs.lock() {
            flow_outcome_example_info_graphs
                .insert(flow_id.clone(), flow_outcome_example_info_graph);
        }

        let (cmd_exec_join_handle_tx, mut cmd_exec_join_handle_rx) = mpsc::channel(128);

        let cmd_execution_starter_task = async move {
            let mut cmd_execution_id_next = CmdExecutionId::new(0u64);
            while let Some(cmd_exec_request) = cmd_exec_request_rx.recv().await {
                // TODO: depending on the request, run the appropriate cmd.
                let CmdExecRequest {} = cmd_exec_request;
                let (web_ui_update_tx, web_ui_update_rx) = mpsc::channel(128);
                let webi_output = WebiOutput::new(web_ui_update_tx);

                let CmdExecSpawnCtx {
                    interrupt_tx,
                    cmd_exec_task,
                } = cmd_exec_spawn_fn(webi_output.clone());

                let cmd_execution_id = cmd_execution_id_next;
                cmd_execution_id_next = CmdExecutionId::new(*cmd_execution_id + 1);

                let cmd_exec_join_handle = tokio::task::spawn_local(cmd_exec_task);
                cmd_exec_join_handle_tx
                    .send((
                        cmd_execution_id,
                        webi_output,
                        cmd_exec_join_handle,
                        web_ui_update_rx,
                    ))
                    .await
                    .expect("Expected `cmd_execution_receiver_task` to be running.");

                if let Some(interrupt_tx) = interrupt_tx {
                    cmd_exec_interrupt_txs.insert(cmd_execution_id, interrupt_tx);
                }
            }
        };

        let cmd_execution_receiver_task = async move {
            while let Some((
                cmd_execution_id,
                mut webi_output,
                cmd_exec_join_handle,
                mut web_ui_update_rx,
            )) = cmd_exec_join_handle_rx.recv().await
            {
                let flow_progress_actual_info_graphs = flow_progress_actual_info_graphs.clone();
                let flow_outcome_actual_info_graphs = flow_outcome_actual_info_graphs.clone();
                let flow_spec_info = flow_spec_info.clone();

                // Update `InfoGraph`s every time `progress_update` is sent.
                let web_ui_update_task = async move {
                    // Keep track of item execution progress.
                    let mut item_ids_in_progress = IndexSet::new();
                    let mut item_ids_completed = IndexSet::new();

                    while let Some(web_ui_update) = web_ui_update_rx.recv().await {
                        // TODO: augment progress information.
                        let flow_progress_actual_info_graph =
                            flow_spec_info.to_progress_info_graph();

                        if let Ok(mut flow_progress_actual_info_graphs) =
                            flow_progress_actual_info_graphs.lock()
                        {
                            flow_progress_actual_info_graphs
                                .insert(cmd_execution_id, flow_progress_actual_info_graph);
                        }

                        let item_ids_in_progress_snapshot = item_ids_in_progress.clone();
                        let item_ids_completed_snapshot = item_ids_completed.clone();

                        let flow_outcome_actual_info_graph = outcome_info_graph_fn(
                            &mut webi_output,
                            Box::new(move |flow, params_specs, resources| {
                                let item_ids_in_progress = item_ids_in_progress_snapshot.clone();
                                let item_ids_completed = item_ids_completed_snapshot.clone();

                                OutcomeInfoGraphCalculator::calculate::<E>(
                                    flow,
                                    params_specs,
                                    resources,
                                    OutcomeInfoGraphVariant::Current {
                                        item_ids_in_progress,
                                        item_ids_completed,
                                    },
                                )
                            }),
                        )
                        .await;

                        if let Ok(mut flow_outcome_actual_info_graphs) =
                            flow_outcome_actual_info_graphs.lock()
                        {
                            flow_outcome_actual_info_graphs
                                .insert(cmd_execution_id, flow_outcome_actual_info_graph);
                        }
                    }
                };

                let cmd_exec_join_task = async move {
                    match cmd_exec_join_handle.await {
                        Ok(()) => {}
                        Err(join_error) => {
                            eprintln!(
                                "Failed to wait for `cmd_execution` to complete. {join_error}"
                            );
                            // TODO: insert CmdExecution failed status
                        }
                    }
                };

                tokio::join!(web_ui_update_task, cmd_exec_join_task);
            }
        };

        tokio::join!(cmd_execution_starter_task, cmd_execution_receiver_task);

        Ok(())
    }

    ///
    /// # Parameters
    ///
    /// * `socket_addr`: IP address and port to listen on.
    async fn leptos_server_start(
        socket_addr: Option<SocketAddr>,
        cmd_exec_request_tx: mpsc::Sender<CmdExecRequest>,
        cmd_exec_to_leptos_ctx: CmdExecToLeptosCtx,
        flow_id: FlowId,
    ) -> Result<(), WebiError> {
        // Setting this to None means we'll be using cargo-leptos and its env vars
        let conf = leptos::get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let socket_addr = socket_addr.unwrap_or(leptos_options.site_addr);
        let routes = leptos_axum::generate_route_list(move || view! { <Home /> });

        stream::iter(crate::assets::ASSETS.iter())
            .map(Result::<_, WebiError>::Ok)
            .try_for_each(|(path_str, contents)| async move {
                let asset_path = Path::new(path_str);
                if let Some(parent_dir) = asset_path.parent() {
                    tokio::fs::create_dir_all(parent_dir)
                        .await
                        .map_err(|error| WebiError::AssetDirCreate {
                            asset_dir: parent_dir.to_path_buf(),
                            error,
                        })?;
                }

                tokio::fs::write(asset_path, contents)
                    .await
                    .map_err(|error| WebiError::AssetWrite {
                        asset_path: asset_path.to_path_buf(),
                        error,
                    })?;

                Ok(())
            })
            .await?;

        let router = Router::new()
            // serve the pkg directory
            .nest_service(
                "/pkg",
                ServeDir::new(Path::new(leptos_options.site_pkg_dir.as_str())),
            )
            // serve the `webi` directory
            .nest_service("/webi", ServeDir::new(Path::new("webi")))
            // serve the SSR rendered homepage
            .leptos_routes_with_context(
                &leptos_options,
                routes,
                move || {
                    // Add global state here if necessary
                    let CmdExecToLeptosCtx {
                        flow_progress_example_info_graphs,
                        flow_progress_actual_info_graphs,
                        flow_outcome_example_info_graphs,
                        flow_outcome_actual_info_graphs,
                        cmd_exec_interrupt_txs,
                    } = cmd_exec_to_leptos_ctx.clone();

                    let (flow_id, flow_id_set) = leptos::create_signal(flow_id.clone());

                    leptos::provide_context(flow_id);
                    leptos::provide_context(flow_id_set);
                    leptos::provide_context(flow_progress_example_info_graphs.clone());
                    leptos::provide_context(flow_progress_actual_info_graphs.clone());
                    leptos::provide_context(flow_outcome_example_info_graphs.clone());
                    leptos::provide_context(flow_outcome_actual_info_graphs.clone());
                    leptos::provide_context(cmd_exec_interrupt_txs.clone());
                    leptos::provide_context(cmd_exec_request_tx.clone());
                },
                move || view! { <Home /> },
            )
            .with_state(leptos_options);

        let listener = tokio::net::TcpListener::bind(socket_addr)
            .await
            .unwrap_or_else(|e| panic!("Failed to listen on {socket_addr}. Error: {e}"));
        let (Ok(()) | Err(_)) = tokio::io::stderr()
            .write_all(format!("listening on http://{}\n", socket_addr).as_bytes())
            .await;
        let (Ok(()) | Err(_)) = tokio::io::stderr()
            .write_all(
                format!(
                    "working dir: {}\n",
                    std::env::current_dir().unwrap().display()
                )
                .as_bytes(),
            )
            .await;
        axum::serve(listener, router)
            .await
            .map_err(|error| WebiError::ServerServe { socket_addr, error })
    }
}
