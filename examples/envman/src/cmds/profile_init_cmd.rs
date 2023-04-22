use peace::{
    cfg::{app_name, item_spec_id, AppName, ItemSpecId, Profile},
    cmd::{ctx::CmdCtx, scopes::MultiProfileNoFlowView},
    fmt::{presentable::CodeInline, presentln},
    rt::cmds::StatesDiscoverCmd,
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};
use peace_item_specs::{file_download::FileDownloadItemSpec, tar_x::TarXItemSpec};
use semver::Version;
use url::Url;

use crate::{
    flows::{EnvDeployFlow, EnvDeployFlowParams},
    item_specs::{
        peace_aws_iam_policy::IamPolicyItemSpec, peace_aws_iam_role::IamRoleItemSpec,
        peace_aws_instance_profile::InstanceProfileItemSpec, peace_aws_s3_bucket::S3BucketItemSpec,
        peace_aws_s3_object::S3ObjectItemSpec,
    },
    model::{EnvManError, EnvType, ProfileParamsKey, RepoSlug, WebAppFileId, WorkspaceParamsKey},
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
    ) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError>,
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
            CmdCtx::builder_multi_profile_no_flow::<EnvManError, _>(output, &workspace);
        crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

        let cmd_ctx_result = cmd_ctx_builder
            .with_profile_filter(|profile| profile != &profile_workspace_init)
            .await;
        match cmd_ctx_result {
            Ok(mut cmd_ctx) => {
                let MultiProfileNoFlowView { profiles, .. } = cmd_ctx.view();

                if profiles.contains(&profile_to_create) {
                    return Err(EnvManError::ProfileToCreateExists {
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
            CmdCtx::builder_single_profile_no_flow::<EnvManError, _>(output, &workspace);
        crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

        // Creating the `CmdCtx` writes the workspace and profile params.
        // We don't need to run any flows with it.
        let _cmd_ctx = cmd_ctx_builder
            .with_workspace_param_value(
                WorkspaceParamsKey::Profile,
                Some(profile_to_create.clone()),
            )
            .with_profile_param_value(ProfileParamsKey::EnvType, Some(env_type))
            .with_profile(profile_to_create.clone())
            .await?;

        // --- //

        let EnvDeployFlowParams {
            app_download_params,
            app_extract_params,
            iam_policy_params,
            iam_role_params,
            instance_profile_params,
            s3_bucket_params,
            s3_object_params,
        } = EnvDeployFlow::params(&profile_to_create, slug, version, url)?;
        let flow = EnvDeployFlow::flow().await?;
        let profile_key = WorkspaceParamsKey::Profile;

        let mut cmd_ctx = {
            let cmd_ctx_builder =
                CmdCtx::builder_single_profile_single_flow::<EnvManError, _>(output, &workspace);
            crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

            cmd_ctx_builder
                .with_profile_from_workspace_param(&profile_key)
                .with_flow(&flow)
                .with_item_spec_params::<FileDownloadItemSpec<WebAppFileId>>(
                    item_spec_id!("app_download"),
                    app_download_params,
                )
                .with_item_spec_params::<TarXItemSpec<WebAppFileId>>(
                    item_spec_id!("app_extract"),
                    app_extract_params,
                )
                .with_item_spec_params::<IamPolicyItemSpec<WebAppFileId>>(
                    item_spec_id!("iam_policy"),
                    iam_policy_params,
                )
                .with_item_spec_params::<IamRoleItemSpec<WebAppFileId>>(
                    item_spec_id!("iam_role"),
                    iam_role_params,
                )
                .with_item_spec_params::<InstanceProfileItemSpec<WebAppFileId>>(
                    item_spec_id!("instance_profile"),
                    instance_profile_params,
                )
                .with_item_spec_params::<S3BucketItemSpec<WebAppFileId>>(
                    item_spec_id!("s3_bucket"),
                    s3_bucket_params,
                )
                .with_item_spec_params::<S3ObjectItemSpec<WebAppFileId>>(
                    item_spec_id!("s3_object"),
                    s3_object_params,
                )
                .await?
        };

        let (_states_current, _states_desired) =
            StatesDiscoverCmd::current_and_desired(&mut cmd_ctx).await?;
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
