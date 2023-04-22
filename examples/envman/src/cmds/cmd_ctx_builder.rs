#[rustfmt::skip] // https://github.com/rust-lang/rustfmt/issues/4609
#[macro_export]
macro_rules! ws_params_augment {
    ($cmd_ctx_builder:ident) => {
        let $cmd_ctx_builder = $cmd_ctx_builder
            .with_workspace_params_k::<$crate::model::WorkspaceParamsKey>()
            .with_workspace_param::<peace::cfg::Profile>($crate::model::WorkspaceParamsKey::Profile);
    };
}

#[rustfmt::skip] // https://github.com/rust-lang/rustfmt/issues/4609
#[macro_export]
macro_rules! ws_and_profile_params_augment {
    ($cmd_ctx_builder:ident) => {
        let $cmd_ctx_builder = $cmd_ctx_builder
            .with_workspace_params_k::<$crate::model::WorkspaceParamsKey>()
            .with_workspace_param::<peace::cfg::Profile>($crate::model::WorkspaceParamsKey::Profile)
            .with_profile_params_k::<$crate::model::ProfileParamsKey>()
            .with_profile_param::<$crate::model::EnvType>($crate::model::ProfileParamsKey::EnvType);
    };
}

pub use ws_and_profile_params_augment;
pub use ws_params_augment;
