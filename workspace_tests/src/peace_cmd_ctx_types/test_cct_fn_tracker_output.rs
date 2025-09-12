use peace::cmd_ctx::CmdCtxTypes;

use crate::{FnTrackerOutput, PeaceTestError};

#[derive(Debug)]
pub struct TestCctFnTrackerOutput;

impl CmdCtxTypes for TestCctFnTrackerOutput {
    type AppError = PeaceTestError;
    type FlowParamsKey = ();
    type MappingFns = ();
    type Output = FnTrackerOutput;
    type ProfileParamsKey = ();
    type WorkspaceParamsKey = ();
}
