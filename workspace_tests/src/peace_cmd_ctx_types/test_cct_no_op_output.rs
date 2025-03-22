use peace::cmd_ctx::CmdCtxTypes;

use crate::{NoOpOutput, PeaceTestError};

#[derive(Debug)]
pub struct TestCctNoOpOutput;

impl CmdCtxTypes for TestCctNoOpOutput {
    type AppError = PeaceTestError;
    type FlowParamsKey = String;
    type Output = NoOpOutput;
    type ProfileParamsKey = String;
    type WorkspaceParamsKey = String;
}
