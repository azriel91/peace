use peace::{
    cfg::{app_name, AppName, Profile},
    cmd::{ctx::CmdCtx, scopes::MultiProfileNoFlowView},
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};

use crate::model::{AppCycleError, EnvType};

/// Command to list initialized profiles.
#[derive(Debug)]
pub struct ProfileListCmd;

impl ProfileListCmd {
    /// Lists all profiles.
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

        let profile_workspace_init = Profile::workspace_init();
        let cmd_ctx_builder =
            CmdCtx::builder_multi_profile_no_flow::<AppCycleError, _>(output, &workspace);
        crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

        let mut cmd_ctx = cmd_ctx_builder
            .with_profile_filter(|profile| profile != &profile_workspace_init)
            .await?;
        let MultiProfileNoFlowView {
            output,
            profile_to_profile_params,
            ..
        } = cmd_ctx.view();

        output.present("# Profiles\n\n").await?;

        let profiles_presentable = profile_to_profile_params
            .iter()
            .filter_map(|(profile, profile_params)| {
                let env_type = profile_params.get::<EnvType, _>("env_type");
                env_type.map(|env_type| (profile, " - type: ".to_string(), env_type))
            })
            .collect::<Vec<_>>();
        output.present(&profiles_presentable).await?;

        Ok(())
    }
}
