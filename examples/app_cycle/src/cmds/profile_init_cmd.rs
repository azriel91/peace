use peace::{
    cfg::{app_name, AppName, Profile},
    cmd::{ctx::CmdCtx, scopes::MultiProfileNoFlowView},
    fmt::{presentable::CodeInline, presentln},
    rt::cmds::StatesDiscoverCmd,
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};
use semver::Version;
use url::Url;

use crate::{
    flows::{EnvDeployFlow, EnvDeployFlowParams},
    model::{AppCycleError, EnvType, RepoSlug},
};

/// Flow to initialize and set the default profile.
#[derive(Debug)]
pub struct ProfileInitCmd;

impl ProfileInitCmd {
    /// Stores profile init parameters.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    /// * `profile`: Name of the profile to create.
    /// * `type`: Type of the environment.
    pub async fn run<O>(
        output: &mut O,
        profile_to_create: Profile,
        env_type: EnvType,
        slug: &RepoSlug,
        version: &Version,
        url: Option<Url>,
    ) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError>,
    {
        let app_name = app_name!();
        let workspace = Workspace::new(
            app_name.clone(),
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
        )?;

        let profile_workspace_init = Profile::workspace_init();
        let cmd_ctx_builder =
            CmdCtx::builder_multi_profile_no_flow::<AppCycleError, _>(output, &workspace);
        crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

        let cmd_ctx_result = cmd_ctx_builder
            .with_profile_filter(|profile| profile != &profile_workspace_init)
            .await;
        match cmd_ctx_result {
            Ok(mut cmd_ctx) => {
                let MultiProfileNoFlowView { profiles, .. } = cmd_ctx.view();

                if profiles.contains(&profile_to_create) {
                    return Err(AppCycleError::ProfileToCreateExists {
                        profile_to_create,
                        app_name,
                    });
                }
            }
            Err(_e) => {
                // On first invocation, the `.peace` app dir will not exist, so
                // we won't be able to list any profiles.
            }
        }

        let cmd_ctx_builder =
            CmdCtx::builder_single_profile_no_flow::<AppCycleError, _>(output, &workspace);
        crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

        // Creating the `CmdCtx` writes the workspace and profile params.
        // We don't need to run any flows with it.
        let _cmd_ctx = cmd_ctx_builder
            .with_workspace_param_value(String::from("profile"), Some(profile_to_create.clone()))
            .with_profile_param_value(String::from("env_type"), Some(env_type))
            .with_profile(profile_to_create.clone())
            .await?;

        // --- //

        let EnvDeployFlowParams {
            web_app_file_download_params,
            web_app_tar_x_params,
            iam_policy_params,
            iam_role_params,
        } = EnvDeployFlow::params(&profile_to_create, slug, version, url)?;
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
                .with_flow_param_value(String::from("iam_policy_params"), Some(iam_policy_params))
                .with_flow_param_value(String::from("iam_role_params"), Some(iam_role_params))
                .await?
        };

        let (_states_current, _states_desired) = StatesDiscoverCmd::exec(&mut cmd_ctx).await?;
        presentln!(
            output,
            [
                "Initialized profile ",
                &profile_to_create,
                " using ",
                &CodeInline::new(format!("{slug}@{version}").into()),
                "."
            ]
        );

        Ok(())
    }
}
