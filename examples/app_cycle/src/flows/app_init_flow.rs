use peace::{
    cfg::{app_name, item_spec_id, AppName, FlowId, ItemSpecId, Profile},
    cmd::ctx::CmdCtx,
    resources::states::StatesSaved,
    rt::cmds::{EnsureCmd, StatesDiscoverCmd},
    rt_model::{
        output::OutputWrite, Flow, ItemSpecGraph, ItemSpecGraphBuilder, Workspace, WorkspaceSpec,
    },
};
use peace_item_specs::{
    file_download::{FileDownloadItemSpec, FileDownloadParams},
    tar_x::{TarXItemSpec, TarXParams},
};

use crate::model::{AppCycleError, WebAppFileId};

/// Flow to download a web application.
#[derive(Debug)]
pub struct AppInitFlow;

impl AppInitFlow {
    /// Sets up this workspace
    pub async fn run<O>(
        output: &mut O,
        web_app_file_download_params: FileDownloadParams<WebAppFileId>,
        web_app_tar_x_params: TarXParams<WebAppFileId>,
    ) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError>,
    {
        let workspace = Workspace::new(
            app_name!(),
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
        )?;
        let graph = Self::graph()?;
        let flow = Flow::new(FlowId::workspace_init(), graph.clone());

        let cmd_ctx_builder = CmdCtx::builder_single_profile_single_flow(output, &workspace);
        crate::cmds::params_augment!(cmd_ctx_builder);
        let mut cmd_ctx = cmd_ctx_builder
            .with_workspace_param_value(
                String::from("web_app_file_download_params"),
                Some(web_app_file_download_params),
            )
            .with_workspace_param_value(
                String::from("web_app_tar_x_params"),
                Some(web_app_tar_x_params),
            )
            .with_profile(Profile::workspace_init())
            .with_flow(&flow)
            .await?;

        let (states_current, _states_desired) = StatesDiscoverCmd::exec(&mut cmd_ctx).await?;
        let states_saved = StatesSaved::from(states_current);

        let states_ensured = EnsureCmd::exec(&mut cmd_ctx, &states_saved).await?;
        cmd_ctx.output_mut().present(&states_ensured).await?;

        Ok(())
    }

    fn graph() -> Result<ItemSpecGraph<AppCycleError>, AppCycleError> {
        let mut graph_builder = ItemSpecGraphBuilder::<AppCycleError>::new();

        let web_app_download_id = graph_builder.add_fn(
            FileDownloadItemSpec::<WebAppFileId>::new(item_spec_id!("web_app_download")).into(),
        );
        let web_app_extract_id = graph_builder
            .add_fn(TarXItemSpec::<WebAppFileId>::new(item_spec_id!("web_app_extract")).into());

        graph_builder.add_edge(web_app_download_id, web_app_extract_id)?;

        Ok(graph_builder.build())
    }
}
