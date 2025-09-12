use peace::cmd_ctx::{type_reg::untagged::TypeReg, CmdCtxTypes};

use crate::{NoOpOutput, PeaceTestError};

#[derive(Debug)]
pub struct TestCctNoOpOutput;

impl CmdCtxTypes for TestCctNoOpOutput {
    type AppError = PeaceTestError;
    type FlowParamsKey = ();
    type MappingFns = ();
    type Output = NoOpOutput;
    type ProfileParamsKey = ();
    type WorkspaceParamsKey = ();

    fn workspace_params_register(_type_reg: &mut TypeReg<Self::WorkspaceParamsKey>) {}

    fn profile_params_register(_type_reg: &mut TypeReg<Self::ProfileParamsKey>) {}

    fn flow_params_register(_type_reg: &mut TypeReg<Self::FlowParamsKey>) {}
}
