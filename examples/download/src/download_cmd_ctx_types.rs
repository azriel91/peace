use peace::{cmd::ctx::CmdCtxTypesCollector, rt_model::params::ParamsKeysUnknown};

use crate::DownloadError;

pub type DownloadCmdCtxTypes<Output> =
    CmdCtxTypesCollector<DownloadError, Output, ParamsKeysUnknown>;
