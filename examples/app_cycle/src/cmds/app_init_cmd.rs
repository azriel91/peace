use peace::{
    cfg::{app_name, profile, AppName, Profile},
    cmd::ctx::CmdCtx,
    fmt::{presentable::CodeInline, presentln},
    rt::cmds::StatesDiscoverCmd,
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};
use semver::Version;
use url::Url;

use crate::{
    flows::EnvDeployFlow,
    model::{AppCycleError, EnvType, RepoSlug},
};

/// Default development profile.
const DEV_PROFILE: Profile = profile!("dev");

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
        let workspace = Workspace::new(
            app_name!(),
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
        )?;

        Self::dev_profile_init(output, &workspace).await?;

        let (web_app_file_download_params, web_app_tar_x_params) =
            EnvDeployFlow::params(slug, version, url)?;
        let flow = EnvDeployFlow::flow().await?;
        let profile_key = String::from("profile");

        let mut cmd_ctx = {
            let cmd_ctx_builder =
                CmdCtx::builder_single_profile_single_flow::<AppCycleError, _>(output, &workspace);
            crate::cmds::ws_profile_and_flow_params_augment!(cmd_ctx_builder);

            cmd_ctx_builder
                .with_profile_from_workspace_param(&profile_key)
                .with_flow(&flow)
                .with_flow_param_value(
                    String::from("web_app_file_download_params"),
                    Some(web_app_file_download_params),
                )
                .with_flow_param_value(
                    String::from("web_app_tar_x_params"),
                    Some(web_app_tar_x_params),
                )
                .await?
        };

        let (_states_current, _states_desired) = StatesDiscoverCmd::exec(&mut cmd_ctx).await?;
        presentln!(
            output,
            [
                "Initialized profile ",
                &DEV_PROFILE,
                " using ",
                &CodeInline::new(format!("{slug}@{version}").into()),
                "."
            ]
        );

        Ok(())
    }

    async fn dev_profile_init<O>(output: &mut O, workspace: &Workspace) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError>,
    {
        // Set workspace default profile to dev, and initialize the dev profile to be a
        // `development` environment type.
        let _cmd_ctx = {
            let cmd_ctx_builder =
                CmdCtx::builder_single_profile_no_flow::<AppCycleError, _>(output, workspace);
            crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

            cmd_ctx_builder
                .with_workspace_param_value(String::from("profile"), Some(DEV_PROFILE))
                .with_profile(DEV_PROFILE)
                .with_profile_param_value(String::from("env_type"), Some(EnvType::Development))
                .await?
        };

        Ok(())
    }
}
