use peace::{
    cfg::{app_name, AppName, Profile},
    cmd::ctx::CmdCtx,
    resources::states::StatesSaved,
    rt::cmds::{EnsureCmd, StatesDiscoverCmd},
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};
use semver::Version;
use url::Url;

use crate::{
    flows::EnvDeployFlow,
    model::{AppCycleError, RepoSlug},
};

/// Takes app init parameters and runs the [`AppInitFlow`].
#[derive(Debug)]
pub struct AppInitCmd;

impl AppInitCmd {
    /// Takes app init parameters and runs the [`AppInitFlow`].
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    /// * `slug`: Username and repository of the application to download.
    /// * `version`: Version of the application to download.
    /// * `url`: URL to override where to download the application from.
    pub async fn run<O>(
        output: &mut O,
        slug: &RepoSlug,
        version: &Version,
        url: Option<Url>,
    ) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError>,
    {
        let (web_app_file_download_params, web_app_tar_x_params) =
            EnvDeployFlow::params(slug, version, url)?;
        let workspace = Workspace::new(
            app_name!(),
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
        )?;
        let flow = EnvDeployFlow::flow().await?;

        let mut cmd_ctx = {
            let cmd_ctx_builder =
                CmdCtx::builder_single_profile_single_flow::<AppCycleError, _>(output, &workspace);
            crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

            cmd_ctx_builder
                .with_profile(Profile::workspace_init())
                .with_flow(&flow)
                .with_workspace_param_value(
                    String::from("web_app_file_download_params"),
                    Some(web_app_file_download_params),
                )
                .with_workspace_param_value(
                    String::from("web_app_tar_x_params"),
                    Some(web_app_tar_x_params),
                )
                .await?
        };

        let (states_current, _states_desired) = StatesDiscoverCmd::exec(&mut cmd_ctx).await?;
        let states_saved = StatesSaved::from(states_current);

        let states_ensured = EnsureCmd::exec(&mut cmd_ctx, &states_saved).await?;
        cmd_ctx.output_mut().present(&states_ensured).await?;

        Ok(())
    }
}
