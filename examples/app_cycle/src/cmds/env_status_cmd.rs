use peace::{
    cfg::{app_name, AppName, Profile},
    cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlowView},
    fmt::presentln,
    rt::cmds::sub::StatesSavedReadCmd,
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};

use crate::{
    flows::EnvDeployFlow,
    model::{AppCycleError, EnvType},
};

/// Shows the current state of the environment.
#[derive(Debug)]
pub struct EnvStatusCmd;

impl EnvStatusCmd {
    /// Shows the current state of the environment.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    /// * `slug`: Username and repository of the application to download.
    /// * `version`: Version of the application to download.
    /// * `url`: URL to override where to download the application from.
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
        let flow = EnvDeployFlow::flow().await?;
        let profile_key = String::from("profile");

        let mut cmd_ctx = {
            let cmd_ctx_builder =
                CmdCtx::builder_single_profile_single_flow::<AppCycleError, _>(output, &workspace);
            crate::cmds::ws_profile_and_flow_params_augment!(cmd_ctx_builder);

            cmd_ctx_builder
                .with_profile_from_workspace_param(&profile_key)
                .with_flow(&flow)
                .await?
        };
        let states_saved = StatesSavedReadCmd::exec(&mut cmd_ctx).await?;

        let SingleProfileSingleFlowView {
            output,
            workspace_params,
            profile_params,
            ..
        } = cmd_ctx.view();

        let profile = workspace_params.get::<Profile, _>("profile");
        let env_type = profile_params.get::<EnvType, _>("env_type");

        if let Some((profile, env_type)) = profile.zip(env_type) {
            presentln!(
                output,
                ["Using profile ", profile, " -- type ", env_type, "\n"]
            );
        }
        presentln!(output, [&states_saved]);

        Ok(())
    }
}
