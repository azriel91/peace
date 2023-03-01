use peace::{
    cfg::{app_name, AppName, Profile},
    cmd::ctx::{CmdCtx, CmdCtxView},
    fmt::presentln,
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};

use crate::model::{AppCycleError, EnvType};

/// Command to show the current profile.
#[derive(Debug)]
pub struct ProfileShowCmd;

impl ProfileShowCmd {
    /// Shows the currently active profile.
    ///
    /// The active profile is stored in workspace params.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    pub async fn run<O>(output: &mut O) -> Result<(), AppCycleError>
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

        // new CmdCtx
        let cmd_ctx_builder =
            CmdCtx::builder_single_profile_no_flow::<AppCycleError>(output, &workspace);
        crate::cmds::params_augment!(cmd_ctx_builder);

        let profile_key = String::from("profile");
        let mut cmd_ctx = cmd_ctx_builder
            .with_profile_from_workspace_param(&profile_key)
            .await?;
        let CmdCtxView { output, scope, .. } = cmd_ctx.view();

        let workspace_params = scope.workspace_params();
        let profile_params = scope.profile_params();

        let profile = workspace_params.get::<Profile, _>("profile");
        let env_type = profile_params.get::<EnvType, _>("env_type");

        if let Some((profile, env_type)) = profile.zip(env_type) {
            presentln!(output, ["Using profile ", profile]);
            presentln!(output, ["Environment type: ", env_type]);
        }

        Ok(())
    }
}
