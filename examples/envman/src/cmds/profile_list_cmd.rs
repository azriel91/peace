use peace::{
    cfg::{app_name},
    cmd::{ctx::CmdCtx, scopes::MultiProfileNoFlowView},
    fmt::presentable::{Heading, HeadingLevel},
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};

use crate::model::{EnvManError, EnvType, ProfileParamsKey};

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
        O: OutputWrite<EnvManError>,
    {
        let workspace = Workspace::new(
            app_name!(),
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
        )?;

        let cmd_ctx_builder =
            CmdCtx::builder_multi_profile_no_flow::<EnvManError, _>(output, &workspace);
        crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

        let mut cmd_ctx = cmd_ctx_builder.await?;
        let MultiProfileNoFlowView {
            output,
            profile_to_profile_params,
            ..
        } = cmd_ctx.view();

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
