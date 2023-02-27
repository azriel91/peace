use peace::{
    cfg::{app_name, item_spec_id, AppName, ItemSpecId},
    rt::cmds::{EnsureCmd, StatesDiscoverCmd},
    rt_model::{
        output::OutputWrite, ItemSpecGraph, ItemSpecGraphBuilder, Workspace, WorkspaceSpec,
    },
};
use peace_item_specs::{
    file_download::{FileDownloadItemSpec, FileDownloadParams},
    tar_x::{TarXItemSpec, TarXParams},
};

use crate::{
    cmds::CmdContextBuilder,
    model::{AppCycleError, WebAppFileId},
};

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

        let cmd_context = CmdContextBuilder::new(&workspace, &graph, output)
            .with_web_app_file_download_params(web_app_file_download_params)
            .with_web_app_tar_x_params(web_app_tar_x_params)
            .await?;
        StatesDiscoverCmd::exec(cmd_context).await?;

        let cmd_context = CmdContextBuilder::new(&workspace, &graph, output).await?;
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
