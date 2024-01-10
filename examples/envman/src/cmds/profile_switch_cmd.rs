use peace::{
    cfg::{app_name},
    cmd::{ctx::CmdCtx, scopes::MultiProfileNoFlowView},
    rt_model::output::OutputWrite,
};

use crate::{
    cmds::{
        common::{env_man_flow, workspace},
        ProfileInitCmd,
    },
    model::{EnvManError, ProfileSwitch, WorkspaceParamsKey},
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
    pub async fn run<O>(output: &mut O, profile_switch: ProfileSwitch) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError>,
    {
        let app_name = app_name!();
        let workspace = workspace()?;

        let env_man_flow = env_man_flow(output, &workspace).await?;

        let cmd_ctx_builder =
            CmdCtx::builder_multi_profile_no_flow::<EnvManError, _>(output, &workspace);
        crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

        let mut cmd_ctx = cmd_ctx_builder.await?;
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
                    return Err(EnvManError::ProfileSwitchToNonExistent {
                        profile_to_switch_to,
                        app_name,
                    });
                } else {
                    let cmd_ctx_builder =
                        CmdCtx::builder_no_profile_no_flow::<EnvManError, _>(output, workspace);
                    crate::cmds::ws_params_augment!(cmd_ctx_builder);
                    cmd_ctx_builder
                        .with_workspace_param_value(
                            WorkspaceParamsKey::Profile,
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
                slug,
                version,
                url,
            } => {
                ProfileInitCmd::run(
                    output,
                    profile_to_create.clone(),
                    env_man_flow,
                    env_type,
                    &slug,
                    &version,
                    url,
                    false,
                )
                .await?;

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
