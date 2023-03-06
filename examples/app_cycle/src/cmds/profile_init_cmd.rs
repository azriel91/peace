use peace::{
    cfg::{app_name, AppName, Profile},
    cmd::{ctx::CmdCtx, scopes::MultiProfileNoFlowView},
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
        profile_to_create: Profile,
        env_type: EnvType,
    ) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError>,
    {
        let app_name = app_name!();
        let workspace = Workspace::new(
            app_name.clone(),
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
        )?;

        let profile_workspace_init = Profile::workspace_init();
        let cmd_ctx_builder =
            CmdCtx::builder_multi_profile_no_flow::<AppCycleError, _>(output, &workspace);
        crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

        let mut cmd_ctx = cmd_ctx_builder
            .with_profile_filter(|profile| profile != &profile_workspace_init)
            .await?;
        let MultiProfileNoFlowView {
            output,
            workspace,
            profiles,
            ..
        } = cmd_ctx.view();

        if profiles.contains(&profile_to_create) {
            return Err(AppCycleError::ProfileToCreateExists {
                profile_to_create,
                app_name,
            });
        }

        let cmd_ctx_builder =
            CmdCtx::builder_single_profile_no_flow::<AppCycleError, _>(output, workspace);
        crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

        // Creating the `CmdCtx` writes the workspace and profile params.
        // We don't need to run any flows with it.
        let _cmd_ctx = cmd_ctx_builder
            .with_workspace_param_value(String::from("profile"), Some(profile_to_create.clone()))
            .with_profile_param_value(String::from("env_type"), Some(env_type))
            .with_profile(profile_to_create)
            .await?;

        Ok(())
    }
}
