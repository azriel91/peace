use peace::{
    cfg::app_name,
    cmd_ctx::{CmdCtxSpnf, CmdCtxSpnfFields, ProfileSelection},
    fmt::presentln,
    profile_model::Profile,
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};

use crate::{
    model::{EnvManError, EnvType, ProfileParamsKey, WorkspaceParamsKey},
    rt_model::EnvmanCmdCtxTypes,
};

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
    pub async fn run<O>(output: &mut O) -> Result<(), EnvManError>
    where
        O: OutputWrite,
        EnvManError: From<<O as OutputWrite>::Error>,
    {
        let workspace = Workspace::new(
            app_name!(),
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
        )?;

        let cmd_ctx_builder = CmdCtxSpnf::<EnvmanCmdCtxTypes<O>>::builder()
            .with_output(output.into())
            .with_workspace((&workspace).into());
        crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

        let profile_key = WorkspaceParamsKey::Profile;
        let mut cmd_ctx = cmd_ctx_builder
            .with_profile_selection(ProfileSelection::FromWorkspaceParam(profile_key.into()))
            .await?;
        let CmdCtxSpnf {
            ref mut output,
            fields:
                CmdCtxSpnfFields {
                    workspace_params,
                    profile_params,
                    ..
                },
        } = cmd_ctx;

        let profile = workspace_params.get::<Profile, _>(&profile_key);
        let env_type = profile_params.get::<EnvType, _>(&ProfileParamsKey::EnvType);

        if let Some((profile, env_type)) = profile.zip(env_type) {
            presentln!(output, ["Using profile ", profile]);
            presentln!(output, ["Environment type: ", env_type]);
        }

        Ok(())
    }
}
