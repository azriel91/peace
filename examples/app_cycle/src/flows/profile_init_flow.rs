use peace::{
    cfg::{flow_id, FlowId, Profile},
    rt::cmds::{sub::StatesSavedReadCmd, StatesDiscoverCmd},
    rt_model::{
        output::OutputWrite, CmdContext, ItemSpecGraph, ItemSpecGraphBuilder, Workspace,
        WorkspaceSpec,
    },
};
use peace_item_specs::{file_download::FileDownloadParams, tar_x::TarXParams};

use crate::model::{AppCycleError, EnvType, WebAppFileId};

/// Flow to initialize and set the default profile.
#[derive(Debug)]
pub struct ProfileInitFlow;

impl ProfileInitFlow {
    /// Stores profile init parameters.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    /// * `profile`: Name of the profile to create.
    /// * `type`: Type of the environment.
    pub async fn run<O>(
        output: &mut O,
        profile: Profile,
        env_type: EnvType,
    ) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError>,
    {
        let workspace = Workspace::new(
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
            profile.clone(),
            flow_id!("profile_init_flow"),
        )?;
        let graph = Self::graph()?;

        let cmd_context = CmdContext::builder(&workspace, &graph, output)
            // TODO: extract workspace param set up somewhere else
            .with_workspace_param(
                "web_app_file_download_params".to_string(),
                Option::<FileDownloadParams<WebAppFileId>>::None,
            )
            .with_workspace_param(
                "web_app_tar_x_params".to_string(),
                Option::<TarXParams<WebAppFileId>>::None,
            )
            // This is a workspace param, as it tells the command context which profile to use.
            .with_workspace_param("profile".to_string(), Option::<Profile>::Some(profile))
            .with_profile_param("env_type".to_string(), Option::<EnvType>::Some(env_type))
            .await?;
        StatesDiscoverCmd::exec(cmd_context).await?;

        let cmd_context = CmdContext::builder(&workspace, &graph, output)
            // TODO: extract workspace param set up somewhere else
            .with_workspace_param(
                "web_app_file_download_params".to_string(),
                Option::<FileDownloadParams<WebAppFileId>>::None,
            )
            .with_workspace_param(
                "web_app_tar_x_params".to_string(),
                Option::<TarXParams<WebAppFileId>>::None,
            )
            // This is a workspace param, as it tells the command context which profile to use.
            .with_workspace_param("profile".to_string(), Option::<Profile>::None)
            .with_profile_param("env_type".to_string(), Option::<EnvType>::None)
            .await?;
        StatesSavedReadCmd::exec(cmd_context).await?;

        Ok(())
    }

    fn graph() -> Result<ItemSpecGraph<AppCycleError>, AppCycleError> {
        let graph_builder = ItemSpecGraphBuilder::<AppCycleError>::new();

        // No item specs, as we are just storing profile init params.

        Ok(graph_builder.build())
    }
}
