use peace::{
    cfg::app_name,
    cmd_ctx::{CmdCtxMpnf, CmdCtxMpnfFields},
    profile_model::Profile,
    rt_model::output::OutputWrite,
};

use crate::{
    cmds::{
        common::{env_man_flow, workspace},
        ProfileInitCmd,
    },
    model::{EnvManError, EnvManFlow, ProfileSwitch, WorkspaceParamsKey},
    rt_model::EnvmanCmdCtxTypes,
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
        O: OutputWrite,
        EnvManError: From<<O as OutputWrite>::Error>,
    {
        let app_name = app_name!();
        let workspace = workspace()?;

        let env_man_flow = env_man_flow(output, &workspace).await?;

        let cmd_ctx_builder = CmdCtxMpnf::<EnvmanCmdCtxTypes<O>>::builder()
            .with_output(output.into())
            .with_workspace((&workspace).into())
            .with_workspace_param::<Profile>(WorkspaceParamsKey::Profile, None)
            .with_workspace_param::<EnvManFlow>(WorkspaceParamsKey::Flow, None);
        // .with_profile_param::<EnvType>(ProfileParamsKey::EnvType, None);

        let mut cmd_ctx = cmd_ctx_builder.await?;
        let CmdCtxMpnf {
            ref mut output,
            fields:
                CmdCtxMpnfFields {
                    workspace,
                    profiles,
                    ..
                },
        } = cmd_ctx;

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
                    // Switches profile
                    let _cmd_ctx = CmdCtxMpnf::<EnvmanCmdCtxTypes<O>>::builder()
                        .with_output(output.reborrow())
                        .with_workspace(workspace)
                        .with_workspace_param(
                            WorkspaceParamsKey::Profile,
                            Some(profile_to_switch_to.clone()),
                        )
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
                    &mut **output,
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
