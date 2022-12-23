use peace::{
    cfg::{flow_id, item_spec_id, profile, FlowId, ItemSpecId, Profile},
    rt::cmds::{EnsureCmd, StatesDiscoverCmd},
    rt_model::{
        CmdContext, ItemSpecGraph, ItemSpecGraphBuilder, OutputWrite, Workspace, WorkspaceSpec,
    },
};
use peace_item_specs::{
    file_download::{FileDownloadItemSpec, FileDownloadParams},
    tar_x::TarXItemSpec,
};

use crate::model::{AppCycleError, WebAppFileId};

/// Flow to download a web application.
#[derive(Debug)]
pub struct AppInitFlow;

impl AppInitFlow {
    /// Sets up this workspace
    pub async fn run<O>(
        output: &mut O,
        app_cycle_file_download_params: FileDownloadParams<WebAppFileId>,
    ) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError>,
    {
        let workspace = Workspace::new(
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
            profile!("workspace_init"),
            flow_id!("app_init_flow"),
        )?;
        let graph = Self::graph()?;

        let cmd_context = CmdContext::builder(&workspace, &graph, output)
            .with_workspace_init::<FileDownloadParams<WebAppFileId>>(Some(
                app_cycle_file_download_params,
            ))
            .await?;
        StatesDiscoverCmd::exec(cmd_context).await?;

        let cmd_context = CmdContext::builder(&workspace, &graph, output)
            .with_workspace_init::<FileDownloadParams<WebAppFileId>>(None)
            .await?;
        EnsureCmd::exec(cmd_context).await?;

        todo!(
            "add `TarXParams` to command context somehow. \
            See <https://github.com/azriel91/peace/issues/45>"
        );
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