#[rustfmt::skip] // https://github.com/rust-lang/rustfmt/issues/4609
#[macro_export]
macro_rules! ws_params_augment {
    ($cmd_ctx_builder:ident) => {
        let $cmd_ctx_builder = $cmd_ctx_builder
            .with_workspace_params_k::<crate::model::WorkspaceParamsKey>()
            .with_workspace_param::<peace::cfg::Profile>(crate::model::WorkspaceParamsKey::Profile);
    };
}

#[rustfmt::skip] // https://github.com/rust-lang/rustfmt/issues/4609
#[macro_export]
macro_rules! ws_and_profile_params_augment {
    ($cmd_ctx_builder:ident) => {
        let $cmd_ctx_builder = $cmd_ctx_builder
            .with_workspace_params_k::<crate::model::WorkspaceParamsKey>()
            .with_workspace_param::<peace::cfg::Profile>(crate::model::WorkspaceParamsKey::Profile)
            .with_profile_params_k::<crate::model::ProfileParamsKey>()
            .with_profile_param::<$crate::model::EnvType>(crate::model::ProfileParamsKey::EnvType);
    };
}

#[rustfmt::skip] // https://github.com/rust-lang/rustfmt/issues/4609
#[macro_export]
macro_rules! ws_profile_and_flow_params_augment {
    ($cmd_ctx_builder:ident) => {
        let $cmd_ctx_builder = $cmd_ctx_builder
            .with_workspace_params_k::<crate::model::WorkspaceParamsKey>()
            .with_workspace_param::<peace::cfg::Profile>(crate::model::WorkspaceParamsKey::Profile)
            .with_profile_params_k::<crate::model::ProfileParamsKey>()
            .with_profile_param::<$crate::model::EnvType>(crate::model::ProfileParamsKey::EnvType)
            .with_flow_params_k::<crate::model::EnvDeployFlowParamsKey>()
            .with_flow_param::<
                peace_item_specs::file_download::FileDownloadParams<
                    $crate::model::WebAppFileId
                >
            >(crate::model::EnvDeployFlowParamsKey::AppDownloadParams)
            .with_flow_param::<
                peace_item_specs::tar_x::TarXParams<
                    $crate::model::WebAppFileId
                >
            >(crate::model::EnvDeployFlowParamsKey::AppExtractParams)
            .with_flow_param::<
                $crate::item_specs::peace_aws_iam_policy::IamPolicyParams<
                    $crate::model::WebAppFileId
                >
            >(crate::model::EnvDeployFlowParamsKey::IamPolicyParams)
            .with_flow_param::<
                $crate::item_specs::peace_aws_iam_role::IamRoleParams<
                    $crate::model::WebAppFileId
                >
            >(crate::model::EnvDeployFlowParamsKey::IamRoleParams)
            .with_flow_param::<
                $crate::item_specs::peace_aws_instance_profile::InstanceProfileParams<
                    $crate::model::WebAppFileId
                >
            >(crate::model::EnvDeployFlowParamsKey::InstanceProfileParams)
            .with_flow_param::<
                $crate::item_specs::peace_aws_s3_bucket::S3BucketParams<
                    $crate::model::WebAppFileId
                >
            >(crate::model::EnvDeployFlowParamsKey::S3BucketParams)
            .with_flow_param::<
                $crate::item_specs::peace_aws_s3_object::S3ObjectParams<
                    $crate::model::WebAppFileId
                >
            >(crate::model::EnvDeployFlowParamsKey::S3ObjectParams)
            ;
    };
}

pub use ws_and_profile_params_augment;
pub use ws_params_augment;
pub use ws_profile_and_flow_params_augment;
