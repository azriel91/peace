use std::path::PathBuf;

use peace::{
    cfg::{app_name, flow_id, item_spec_id, AppName, FlowId, ItemSpecId, Profile},
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
        peace_aws_iam_policy::{IamPolicyItemSpec, IamPolicyParams},
        peace_aws_iam_role::{IamRoleItemSpec, IamRoleParams},
        peace_aws_instance_profile::{InstanceProfileItemSpec, InstanceProfileParams},
        peace_aws_s3_bucket::{S3BucketItemSpec, S3BucketParams},
        peace_aws_s3_object::S3ObjectParams,
    },
    model::{AppCycleError, RepoSlug, WebAppFileId},
};

/// Flow to deploy a web application.
#[derive(Debug)]
pub struct EnvDeployFlow;

impl EnvDeployFlow {
    /// Returns the `Flow` graph.
    pub async fn flow() -> Result<Flow<AppCycleError>, AppCycleError> {
        let flow = {
            let flow_id = flow_id!("env_deploy");
            let graph = {
                let mut graph_builder = ItemSpecGraphBuilder::<AppCycleError>::new();

                let web_app_download_id = graph_builder.add_fn(
                    FileDownloadItemSpec::<WebAppFileId>::new(item_spec_id!("web_app_download"))
                        .into(),
                );
                let web_app_extract_id = graph_builder.add_fn(
                    TarXItemSpec::<WebAppFileId>::new(item_spec_id!("web_app_extract")).into(),
                );

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
                let web_app_s3_object_id = graph_builder.add_fn(
                    S3BucketItemSpec::<WebAppFileId>::new(item_spec_id!("web_app_s3_object"))
                        .into(),
                );

                graph_builder.add_edges([
                    (web_app_download_id, web_app_extract_id),
                    (iam_policy_item_spec_id, iam_role_item_spec_id),
                    (iam_role_item_spec_id, instance_profile_item_spec_id),
                    // Download the file before uploading it.
                    (web_app_download_id, web_app_s3_object_id),
                    // Create the bucket before uploading to it.
                    (s3_bucket_id, web_app_s3_object_id),
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
    ) -> Result<EnvDeployFlowParams, AppCycleError> {
        let account = slug.account();
        let repo_name = slug.repo_name();
        let web_app_download_dir = PathBuf::from_iter([account, repo_name, &format!("{version}")]);

        let web_app_file_download_params = {
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
                        Url::parse(&url_candidate).map_err(|error| {
                            AppCycleError::AppCycleUrlBuild {
                                url_candidate,
                                error,
                            }
                        })?
                    }
                }
            };
            let web_app_path_local = web_app_download_dir.join(format!("{repo_name}.{file_ext}"));
            FileDownloadParams::new(
                web_app_file_url,
                web_app_path_local,
                #[cfg(target_arch = "wasm32")]
                peace_item_specs::file_download::StorageForm::Base64,
            )
        };
        let web_app_tar_x_params = {
            let tar_path = web_app_file_download_params.dest().to_path_buf();
            let dest = web_app_download_dir.join("extracted");

            TarXParams::<WebAppFileId>::new(tar_path, dest)
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

        let iam_policy_params = {
            let ec2_to_s3_bucket_policy = format!(
                include_str!("ec2_to_s3_bucket_policy.json"),
                bucket_name = bucket_name
            );
            IamPolicyParams::<WebAppFileId>::new(
                iam_policy_name,
                path.clone(),
                ec2_to_s3_bucket_policy,
            )
        };

        let iam_role_params = IamRoleParams::<WebAppFileId>::new(iam_role_name, path.clone());
        let instance_profile_params =
            InstanceProfileParams::<WebAppFileId>::new(instance_profile_name, path, true);

        let s3_bucket_params = S3BucketParams::<WebAppFileId>::new(bucket_name.clone());
        let web_app_s3_object_params = {
            let web_app_path_local = web_app_file_download_params.dest().to_path_buf();
            let object_key = web_app_path_local
                .file_name()
                .expect("Expected web app file name to exist.")
                .to_string_lossy()
                .to_string();

            S3ObjectParams::<WebAppFileId>::new(web_app_path_local, bucket_name, object_key)
        };

        Ok(EnvDeployFlowParams {
            web_app_file_download_params,
            web_app_tar_x_params,
            iam_policy_params,
            iam_role_params,
            instance_profile_params,
            s3_bucket_params,
            web_app_s3_object_params,
        })
    }
}

#[derive(Debug)]
pub struct EnvDeployFlowParams {
    pub web_app_file_download_params: FileDownloadParams<WebAppFileId>,
    pub web_app_tar_x_params: TarXParams<WebAppFileId>,
    pub iam_policy_params: IamPolicyParams<WebAppFileId>,
    pub iam_role_params: IamRoleParams<WebAppFileId>,
    pub instance_profile_params: InstanceProfileParams<WebAppFileId>,
    pub s3_bucket_params: S3BucketParams<WebAppFileId>,
    pub web_app_s3_object_params: S3ObjectParams<WebAppFileId>,
}
