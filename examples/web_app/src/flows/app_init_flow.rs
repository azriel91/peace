use peace::{
    cfg::{flow_id, item_spec_id, profile, FlowId, ItemSpecId, Profile},
    rt::cmds::{EnsureCmd, StatesDiscoverCmd},
    rt_model::{
        CmdContext, ItemSpecGraph, ItemSpecGraphBuilder, OutputWrite, Workspace, WorkspaceSpec,
    },
};
use peace_item_specs::file_download::{FileDownloadItemSpec, FileDownloadParams};

use crate::model::{WebAppError, WebAppFileId};

/// Flow to download a web application.
#[derive(Debug)]
pub struct AppInitFlow;

impl AppInitFlow {
    /// Sets up this workspace
    pub async fn run<O>(
        output: &mut O,
        web_app_file_download_params: FileDownloadParams<WebAppFileId>,
    ) -> Result<(), WebAppError>
    where
        O: OutputWrite<WebAppError>,
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
                web_app_file_download_params,
            ))
            .await?;
        StatesDiscoverCmd::exec(cmd_context).await?;

        let cmd_context = CmdContext::builder(&workspace, &graph, output)
            .with_workspace_init::<FileDownloadParams<WebAppFileId>>(None)
            .await?;
        EnsureCmd::exec(cmd_context).await?;

        Ok(())
    }

    fn graph() -> Result<ItemSpecGraph<WebAppError>, WebAppError> {
        let mut graph_builder = ItemSpecGraphBuilder::<WebAppError>::new();

        graph_builder
            .add_fn(FileDownloadItemSpec::<WebAppFileId>::new(item_spec_id!("web_app")).into());

        Ok(graph_builder.build())
    }
}
