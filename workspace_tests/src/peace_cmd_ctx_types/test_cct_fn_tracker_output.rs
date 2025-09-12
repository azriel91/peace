use peace::cmd_ctx::{type_reg::untagged::TypeReg, CmdCtxTypes};

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

    fn workspace_params_register(_type_reg: &mut TypeReg<Self::WorkspaceParamsKey>) {}

    fn profile_params_register(_type_reg: &mut TypeReg<Self::ProfileParamsKey>) {}

    fn flow_params_register(_type_reg: &mut TypeReg<Self::FlowParamsKey>) {}
}
