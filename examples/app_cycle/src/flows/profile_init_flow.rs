use peace::{
    cfg::{app_name, AppName, FlowId, Profile},
    cmd::ctx::CmdCtx,
    rt::cmds::StatesDiscoverCmd,
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
        let flow = Flow::new(FlowId::profile_init(), graph);

        let cmd_ctx_builder = CmdCtx::builder_single_profile_single_flow(output, &workspace);
        crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);
        let mut cmd_ctx = cmd_ctx_builder
            .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
            .with_profile_param_value(String::from("env_type"), Some(env_type))
            .with_profile(profile)
            .with_flow(&flow)
            .await?;
        let (states_current, _states_desired) = StatesDiscoverCmd::exec(&mut cmd_ctx).await?;
        cmd_ctx.output_mut().present(&states_current).await?;

        Ok(())
    }

    fn graph() -> Result<ItemSpecGraph<AppCycleError>, AppCycleError> {
        let graph_builder = ItemSpecGraphBuilder::<AppCycleError>::new();

        // No item specs, as we are just storing profile init params.

        Ok(graph_builder.build())
    }
}
