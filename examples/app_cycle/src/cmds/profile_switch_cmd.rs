use peace::{
    cfg::{app_name, AppName, Profile},
    cmd::{ctx::CmdCtx, scopes::MultiProfileNoFlowView},
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};

use crate::model::AppCycleError;

/// Command to switch between profiles.
#[derive(Debug)]
pub struct ProfileSwitchCmd;

impl ProfileSwitchCmd {
    /// Switches to another profile.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    pub async fn run<O>(output: &mut O, profile_to_switch_to: &Profile) -> Result<(), AppCycleError>
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

        if !profiles.contains(profile_to_switch_to) {
            // TODO: return error
        } else {
            let cmd_ctx_builder =
                CmdCtx::builder_no_profile_no_flow::<AppCycleError, _>(output, &workspace);
            crate::cmds::ws_params_augment!(cmd_ctx_builder);
            cmd_ctx_builder
                .with_workspace_param_value(
                    String::from("profile"),
                    Some(profile_to_switch_to.clone()),
                )
                .build()
                .await?;
        }

        output
            .present(&(
                String::from("Switched to profile: "),
                profile_to_switch_to,
                String::from("\n"),
            ))
            .await?;

        Ok(())
    }
}
