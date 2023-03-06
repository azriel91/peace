use peace::{
    cfg::{app_name, AppName, Profile},
    cmd::{ctx::CmdCtx, scopes::SingleProfileNoFlowView},
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

        let cmd_ctx_builder =
            CmdCtx::builder_single_profile_no_flow::<AppCycleError, _>(output, &workspace);
        crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

        let profile_key = String::from("profile");
        let mut cmd_ctx = cmd_ctx_builder
            .with_profile_from_workspace_param(&profile_key)
            .await?;
        let SingleProfileNoFlowView {
            output,
            workspace_params,
            profile_params,
            ..
        } = cmd_ctx.view();

        let profile = workspace_params.get::<Profile, _>("profile");
        let env_type = profile_params.get::<EnvType, _>("env_type");

        if let Some((profile, env_type)) = profile.zip(env_type) {
            presentln!(output, ["Using profile ", profile]);
            presentln!(output, ["Environment type: ", env_type]);
        }

        Ok(())
    }
}
