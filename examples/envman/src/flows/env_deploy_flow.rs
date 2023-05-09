use std::path::PathBuf;

use peace::{
    cfg::{
        app_name, flow_id, item_spec_id, state::Generated, AppName, FlowId, ItemSpecId, Profile,
    },
    data::marker::Current,
    params::{ParamsSpec, ValueSpec},
    rt_model::{Flow, ItemSpecGraphBuilder},
};
use peace_item_specs::{
    file_download::{FileDownloadItemSpec, FileDownloadParams},
    tar_x::{TarXItemSpec, TarXParams},
};
use semver::Version;
use url::Url;

use crate::{
    item_specs::{
        peace_aws_iam_policy::{IamPolicyItemSpec, IamPolicyParams, IamPolicyState},
        peace_aws_iam_role::{IamRoleItemSpec, IamRoleParams, IamRoleParamsFieldWise},
        peace_aws_instance_profile::{InstanceProfileItemSpec, InstanceProfileParams},
        peace_aws_s3_bucket::{S3BucketItemSpec, S3BucketParams},
        peace_aws_s3_object::{S3ObjectItemSpec, S3ObjectParams},
    },
    model::{EnvManError, RepoSlug, WebAppFileId},
};

/// Flow to deploy a web application.
#[derive(Debug)]
pub struct EnvDeployFlow;

impl EnvDeployFlow {
    /// Returns the `Flow` graph.
    pub async fn flow() -> Result<Flow<EnvManError>, EnvManError> {
        let flow = {
            let flow_id = flow_id!("env_deploy");
            let graph = {
                let mut graph_builder = ItemSpecGraphBuilder::<EnvManError>::new();

                let app_download_id = graph_builder.add_fn(
                    FileDownloadItemSpec::<WebAppFileId>::new(item_spec_id!("app_download")).into(),
                );
                let app_extract_id = graph_builder
                    .add_fn(TarXItemSpec::<WebAppFileId>::new(item_spec_id!("app_extract")).into());

                let iam_policy_item_spec_id = graph_builder.add_fn(
                    IamPolicyItemSpec::<WebAppFileId>::new(item_spec_id!("iam_policy")).into(),
                );

                let iam_role_item_spec_id = graph_builder
                    .add_fn(IamRoleItemSpec::<WebAppFileId>::new(item_spec_id!("iam_role")).into());

                let instance_profile_item_spec_id = graph_builder.add_fn(
                    InstanceProfileItemSpec::<WebAppFileId>::new(item_spec_id!("instance_profile"))
                        .into(),
                );

                let s3_bucket_id = graph_builder.add_fn(
                    S3BucketItemSpec::<WebAppFileId>::new(item_spec_id!("s3_bucket")).into(),
                );
                let s3_object_id = graph_builder.add_fn(
                    S3ObjectItemSpec::<WebAppFileId>::new(item_spec_id!("s3_object")).into(),
                );

                graph_builder.add_edges([
                    (app_download_id, app_extract_id),
                    (iam_policy_item_spec_id, iam_role_item_spec_id),
                    (iam_role_item_spec_id, instance_profile_item_spec_id),
                    // Download the file before uploading it.
                    (app_download_id, s3_object_id),
                    // Create the bucket before uploading to it.
                    (s3_bucket_id, s3_object_id),
                ])?;
                graph_builder.build()
            };

            Flow::new(flow_id, graph)
        };

        Ok(flow)
    }

    /// Returns the params needed for this flow.
    pub fn params(
        profile: &Profile,
        slug: &RepoSlug,
        version: &Version,
        url: Option<Url>,
    ) -> Result<EnvDeployFlowParamsSpecs, EnvManError> {
        let account = slug.account();
        let repo_name = slug.repo_name();
        let app_download_dir = PathBuf::from_iter([account, repo_name, &format!("{version}")]);

        #[cfg(target_family = "windows")]
        let file_ext = "zip";
        #[cfg(any(target_family = "unix", target_family = "wasm"))]
        let file_ext = "tar";
        // windows:
        // https://github.com/azriel91/web_app/releases/download/0.1.0/web_app.zip
        //
        // linux:
        // https://github.com/azriel91/web_app/releases/download/0.1.0/web_app.tar
        let web_app_file_url = {
            match url {
                Some(url) => url,
                None => {
                    let url_candidate = format!(
                        "https://github.com/{account}/{repo_name}/releases/download/{version}/{repo_name}.{file_ext}"
                    );
                    Url::parse(&url_candidate).map_err(|error| EnvManError::EnvManUrlBuild {
                        url_candidate,
                        error,
                    })?
                }
            }
        };
        let web_app_path_local = app_download_dir.join(format!("{repo_name}.{file_ext}"));
        let app_download_params_spec = ParamsSpec::Value(FileDownloadParams::new(
            web_app_file_url,
            web_app_path_local.clone(),
            #[cfg(target_arch = "wasm32")]
            peace_item_specs::file_download::StorageForm::Base64,
        ));
        let app_extract_params_spec = {
            let tar_path = web_app_path_local.clone();
            let dest = app_download_dir.join("extracted");

            ParamsSpec::Value(TarXParams::<WebAppFileId>::new(tar_path, dest))
        };

        let iam_policy_name = profile.to_string();
        let iam_role_name = profile.to_string();
        let instance_profile_name = profile.to_string();
        let bucket_name = {
            let username = whoami::username();
            let app_name = app_name!();
            format!("{username}-peace-{app_name}-{profile}").replace('_', "-")
        };
        let path = String::from("/");

        let iam_policy_params_spec = {
            let ec2_to_s3_bucket_policy = format!(
                include_str!("ec2_to_s3_bucket_policy.json"),
                bucket_name = bucket_name
            );
            ParamsSpec::Value(IamPolicyParams::<WebAppFileId>::new(
                iam_policy_name,
                path.clone(),
                ec2_to_s3_bucket_policy,
            ))
        };

        let iam_role_params_spec =
            ParamsSpec::FieldWise(IamRoleParamsFieldWise::<WebAppFileId>::new(
                ValueSpec::Value(iam_role_name),
                ValueSpec::Value(path.clone()),
                ValueSpec::from_map(
                    Some(String::from("managed_policy_arn")),
                    |iam_policy_state: &Current<IamPolicyState>| {
                        if let Some(IamPolicyState::Some {
                            policy_id_arn_version: Generated::Value(policy_id_arn_version),
                            ..
                        }) = iam_policy_state.0.as_ref()
                        {
                            Some(policy_id_arn_version.arn().to_string())
                        } else {
                            None
                        }
                    },
                ),
            ));
        let instance_profile_params_spec = ParamsSpec::Value(
            InstanceProfileParams::<WebAppFileId>::new(instance_profile_name, path, true),
        );

        let s3_bucket_params_spec =
            ParamsSpec::Value(S3BucketParams::<WebAppFileId>::new(bucket_name.clone()));
        let s3_object_params_spec = {
            let object_key = web_app_path_local
                .file_name()
                .expect("Expected web app file name to exist.")
                .to_string_lossy()
                .to_string();

            ParamsSpec::Value(S3ObjectParams::<WebAppFileId>::new(
                web_app_path_local,
                bucket_name,
                object_key,
            ))
        };

        Ok(EnvDeployFlowParamsSpecs {
            app_download_params_spec,
            app_extract_params_spec,
            iam_policy_params_spec,
            iam_role_params_spec,
            instance_profile_params_spec,
            s3_bucket_params_spec,
            s3_object_params_spec,
        })
    }
}

#[derive(Debug)]
pub struct EnvDeployFlowParamsSpecs {
    pub app_download_params_spec: ParamsSpec<FileDownloadParams<WebAppFileId>>,
    pub app_extract_params_spec: ParamsSpec<TarXParams<WebAppFileId>>,
    pub iam_policy_params_spec: ParamsSpec<IamPolicyParams<WebAppFileId>>,
    pub iam_role_params_spec: ParamsSpec<IamRoleParams<WebAppFileId>>,
    pub instance_profile_params_spec: ParamsSpec<InstanceProfileParams<WebAppFileId>>,
    pub s3_bucket_params_spec: ParamsSpec<S3BucketParams<WebAppFileId>>,
    pub s3_object_params_spec: ParamsSpec<S3ObjectParams<WebAppFileId>>,
}
