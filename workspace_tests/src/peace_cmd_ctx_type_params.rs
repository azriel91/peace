use peace::{cmd::ctx::CmdCtxTypeParams, rt_model::params::ParamsKeysUnknown};

use crate::{NoOpOutput, PeaceTestError};

#[derive(Debug)]
pub(crate) struct PeaceCmdCtxTypeParams;

impl CmdCtxTypeParams for PeaceCmdCtxTypeParams {
    type AppError = PeaceTestError;
    type Output = NoOpOutput;
    type ParamsKeys = ParamsKeysUnknown;
}
