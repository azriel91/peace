use peace::{
    cfg::{app_name, AppName, Profile},
    cmd::{ctx::CmdCtx, scopes::MultiProfileNoFlowView},
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};

use crate::{
    cmds::ProfileInitCmd,
    model::{AppCycleError, ProfileSwitch},
};

/// Command to switch between profiles.
#[derive(Debug)]
pub struct ProfileSwitchCmd;

impl ProfileSwitchCmd {
    /// Switches to another profile.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    pub async fn run<O>(output: &mut O, profile_switch: ProfileSwitch) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError>,
    {
        let app_name = app_name!();
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

        match profile_switch {
            ProfileSwitch::ToExisting {
                profile: profile_to_switch_to,
            } => {
                if !profiles.contains(&profile_to_switch_to) {
                    return Err(AppCycleError::ProfileSwitchToNonExistent {
                        profile_to_switch_to,
                        app_name,
                    });
                } else {
                    let cmd_ctx_builder =
                        CmdCtx::builder_no_profile_no_flow::<AppCycleError, _>(output, workspace);
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
            }
            ProfileSwitch::CreateNew {
                profile: profile_to_create,
                env_type,
            } => {
                ProfileInitCmd::run(output, profile_to_create.clone(), env_type).await?;

                output
                    .present(&(
                        String::from("Switched to a new profile: "),
                        profile_to_create,
                        String::from("\n"),
                    ))
                    .await?;
            }
        }

        Ok(())
    }
}
