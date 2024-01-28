use peace::{cmd::ctx::CmdCtxTypes, rt_model::params::ParamsKeysUnknown};

use crate::{NoOpOutput, PeaceTestError};

#[derive(Debug)]
pub(crate) struct PeaceCmdCtxTypes;

impl CmdCtxTypes for PeaceCmdCtxTypes {
    type AppError = PeaceTestError;
    type Output = NoOpOutput;
    type ParamsKeys = ParamsKeysUnknown;
}
