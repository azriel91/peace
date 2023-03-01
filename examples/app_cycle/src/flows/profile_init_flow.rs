use peace::{
    cfg::{app_name, AppName, FlowId, Profile},
    cmd::ctx::CmdCtx,
    rt::cmds::{StatesDiscoverCmd, StatesSavedDisplayCmd},
    rt_model::{
        output::OutputWrite, Flow, ItemSpecGraph, ItemSpecGraphBuilder, Workspace, WorkspaceSpec,
    },
};

use crate::model::{AppCycleError, EnvType};

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
            app_name!(),
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
        )?;
        let graph = Self::graph()?;

        let cmd_ctx_builder =
            CmdCtx::builder_single_profile_single_flow::<AppCycleError>(output, &workspace);
        crate::cmds::params_augment!(cmd_ctx_builder);
        let cmd_ctx = cmd_ctx_builder
            .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
            .with_profile_param_value(String::from("env_type"), Some(env_type))
            .with_profile(profile)
            .with_flow(Flow::new(FlowId::profile_init(), graph.clone()))
            .await?;
        StatesDiscoverCmd::exec_v2(cmd_ctx).await?;

        let cmd_ctx_builder =
            CmdCtx::builder_single_profile_single_flow::<AppCycleError>(output, &workspace);
        crate::cmds::params_augment!(cmd_ctx_builder);
        let profile_key = String::from("profile");
        let cmd_ctx = cmd_ctx_builder
            .with_profile_from_workspace_param(&profile_key)
            .with_flow(Flow::new(FlowId::profile_init(), graph.clone()))
            .await?;
        StatesSavedDisplayCmd::exec_v2(cmd_ctx).await?;

        Ok(())
    }

    fn graph() -> Result<ItemSpecGraph<AppCycleError>, AppCycleError> {
        let graph_builder = ItemSpecGraphBuilder::<AppCycleError>::new();

        // No item specs, as we are just storing profile init params.

        Ok(graph_builder.build())
    }
}
