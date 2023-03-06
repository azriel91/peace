use peace::{
    cfg::{app_name, AppName, Profile},
    cmd::ctx::CmdCtx,
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};

use crate::model::{AppCycleError, EnvType};

/// Flow to initialize and set the default profile.
#[derive(Debug)]
pub struct ProfileInitCmd;

impl ProfileInitCmd {
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

        let cmd_ctx_builder =
            CmdCtx::builder_single_profile_no_flow::<AppCycleError, _>(output, &workspace);
        crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

        // Creating the `CmdCtx` writes the workspace and profile params.
        // We don't need to run any flows with it.
        let _cmd_ctx = cmd_ctx_builder
            .with_workspace_param_value(String::from("profile"), Some(profile.clone()))
            .with_profile_param_value(String::from("env_type"), Some(env_type))
            .with_profile(profile)
            .await?;

        Ok(())
    }
}
