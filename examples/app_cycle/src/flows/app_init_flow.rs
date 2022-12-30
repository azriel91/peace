use peace::{
    cfg::{flow_id, item_spec_id, profile, FlowId, ItemSpecId, Profile},
    rt::cmds::{EnsureCmd, StatesDiscoverCmd},
    rt_model::{
        CmdContext, ItemSpecGraph, ItemSpecGraphBuilder, OutputWrite, Workspace, WorkspaceSpec,
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

        let web_app_tar_x_params = {
            let tar_path = web_app_file_download_params.dest().to_path_buf();
            let dest = web_app_file_download_params
                .dest()
                .parent()
                .unwrap()
                .join("web_app"); // TODO: get the name

            TarXParams::<WebAppFileId>::new(tar_path, dest)
        };

        let cmd_context = CmdContext::builder(&workspace, &graph, output)
            .with_workspace_param(
                "web_app_file_download_params".to_string(),
                Some(web_app_file_download_params),
            )
            .with_workspace_param(
                "web_app_tar_x_params".to_string(),
                Some(web_app_tar_x_params),
            )
            .await?;
        StatesDiscoverCmd::exec(cmd_context).await?;

        let cmd_context = CmdContext::builder(&workspace, &graph, output)
            .with_workspace_param(
                "web_app_file_download_params".to_string(),
                None::<FileDownloadParams<WebAppFileId>>,
            )
            .with_workspace_param(
                "web_app_tar_x_params".to_string(),
                None::<TarXParams<WebAppFileId>>,
            )
            .await?;
        EnsureCmd::exec(cmd_context).await?;

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
