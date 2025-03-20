//! Information such as which profile or flow a command is run for for the Peace
//! framework.

// Re-exports
pub use interruptible;

pub use crate::{
    cmd_ctx_mpnf::{CmdCtxMpnf, CmdCtxMpnfFields},
    cmd_ctx_mpnf_params::{CmdCtxMpnfParams, CmdCtxMpnfParamsBuilder},
    cmd_ctx_mpsf::{CmdCtxMpsf, CmdCtxMpsfFields},
    cmd_ctx_mpsf_params::{CmdCtxMpsfParams, CmdCtxMpsfParamsBuilder},
    cmd_ctx_npnf::{CmdCtxNpnf, CmdCtxNpnfFields},
    cmd_ctx_npnf_params::{CmdCtxNpnfParams, CmdCtxNpnfParamsBuilder},
    cmd_ctx_spnf::{CmdCtxSpnf, CmdCtxSpnfFields},
    cmd_ctx_spnf_params::{CmdCtxSpnfParams, CmdCtxSpnfParamsBuilder},
    cmd_ctx_spsf::{CmdCtxSpsf, CmdCtxSpsfFields},
    cmd_ctx_spsf_params::{CmdCtxSpsfParams, CmdCtxSpsfParamsBuilder},
    cmd_ctx_types::CmdCtxTypes,
    profile_filter_fn::ProfileFilterFn,
    profile_selection::ProfileSelection,
};

pub(crate) use crate::{
    cmd_ctx_builder_support::CmdCtxBuilderSupport,
    cmd_ctx_builder_support_multi::CmdCtxBuilderSupportMulti,
};

mod cmd_ctx_builder_support;
mod cmd_ctx_builder_support_multi;
mod cmd_ctx_mpnf;
mod cmd_ctx_mpnf_params;
mod cmd_ctx_mpsf;
mod cmd_ctx_mpsf_params;
mod cmd_ctx_npnf;
mod cmd_ctx_npnf_params;
mod cmd_ctx_spnf;
mod cmd_ctx_spnf_params;
mod cmd_ctx_spsf;
mod cmd_ctx_spsf_params;
mod cmd_ctx_types;
mod profile_filter_fn;
mod profile_selection;
