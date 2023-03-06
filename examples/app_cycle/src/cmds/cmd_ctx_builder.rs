#[rustfmt::skip] // https://github.com/rust-lang/rustfmt/issues/4609
#[macro_export]
macro_rules! ws_params_augment {
    ($cmd_ctx_builder:ident) => {
        let $cmd_ctx_builder = $cmd_ctx_builder
            .with_workspace_params_k::<String>()
            .with_workspace_param::<peace::cfg::Profile>(String::from("profile"))
            .with_workspace_param::<
                peace_item_specs::file_download::FileDownloadParams<
                    $crate::model::WebAppFileId
                >
            >(String::from("web_app_file_download_params"))
            .with_workspace_param::<
                peace_item_specs::tar_x::TarXParams<
                    $crate::model::WebAppFileId
                >
            >(String::from("web_app_tar_x_params"));
    };
}

#[rustfmt::skip] // https://github.com/rust-lang/rustfmt/issues/4609
#[macro_export]
macro_rules! ws_and_profile_params_augment {
    ($cmd_ctx_builder:ident) => {
        let $cmd_ctx_builder = $cmd_ctx_builder
            .with_workspace_params_k::<String>()
            .with_workspace_param::<peace::cfg::Profile>(String::from("profile"))
            .with_workspace_param::<
                peace_item_specs::file_download::FileDownloadParams<
                    $crate::model::WebAppFileId
                >
            >(String::from("web_app_file_download_params"))
            .with_workspace_param::<
                peace_item_specs::tar_x::TarXParams<
                    $crate::model::WebAppFileId
                >
            >(String::from("web_app_tar_x_params"))
            .with_profile_params_k::<String>()
            .with_profile_param::<$crate::model::EnvType>(String::from("env_type"));
    };
}

pub use ws_and_profile_params_augment;
pub use ws_params_augment;
