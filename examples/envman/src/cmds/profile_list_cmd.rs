use peace::{
    cmd_ctx::{CmdCtxMpnf, CmdCtxMpnfFields},
    fmt::presentable::{Heading, HeadingLevel},
    rt_model::output::OutputWrite,
};

use crate::{
    cmds::common::workspace,
    model::{EnvManError, EnvManFlow, EnvType, ProfileParamsKey, WorkspaceParamsKey},
    rt_model::EnvmanCmdCtxTypes,
};

/// Command to list initialized profiles.
#[derive(Debug)]
pub struct ProfileListCmd;

impl ProfileListCmd {
    /// Lists all profiles.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    pub async fn run<O>(output: &mut O) -> Result<(), EnvManError>
    where
        O: OutputWrite,
        EnvManError: From<<O as OutputWrite>::Error>,
    {
        let cmd_ctx_builder = CmdCtxMpnf::<EnvmanCmdCtxTypes<O>>::builder()
            .with_output(output.into())
            .with_workspace(workspace()?.into())
            .with_workspace_param::<peace::profile_model::Profile>(
                WorkspaceParamsKey::Profile,
                None,
            )
            .with_workspace_param::<EnvManFlow>(WorkspaceParamsKey::Flow, None);

        let mut cmd_ctx = cmd_ctx_builder.await?;
        let CmdCtxMpnf {
            ref mut output,
            fields:
                CmdCtxMpnfFields {
                    profile_to_profile_params,
                    ..
                },
        } = cmd_ctx;

        output
            .present(Heading::new(HeadingLevel::Level1, String::from("Profiles")))
            .await?;

        let profiles_presentable = profile_to_profile_params
            .iter()
            .filter_map(|(profile, profile_params)| {
                let env_type = profile_params.get::<EnvType, _>(&ProfileParamsKey::EnvType);
                env_type.map(|env_type| (profile, " - type: ".to_string(), env_type))
            })
            .collect::<Vec<_>>();
        output.present(&profiles_presentable).await?;

        Ok(())
    }
}
