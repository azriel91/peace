use std::marker::PhantomData;

use peace::{
    cmd_ctx::{type_reg::untagged::TypeReg, CmdCtxTypes},
    profile_model::Profile,
    rt_model::output::OutputWrite,
};

use crate::{
    flows::EnvmanMappingFns,
    model::{EnvManError, EnvManFlow, EnvType, ProfileParamsKey, WorkspaceParamsKey},
};

#[derive(Debug)]
pub struct EnvmanCmdCtxTypes<Output>(PhantomData<Output>);

impl<Output> CmdCtxTypes for EnvmanCmdCtxTypes<Output>
where
    Output: OutputWrite,
    EnvManError: From<<Output as OutputWrite>::Error>,
{
    type AppError = EnvManError;
    type FlowParamsKey = ();
    type MappingFns = EnvmanMappingFns;
    type Output = Output;
    type ProfileParamsKey = ProfileParamsKey;
    type WorkspaceParamsKey = WorkspaceParamsKey;

    fn workspace_params_register(type_reg: &mut TypeReg<Self::WorkspaceParamsKey>) {
        type_reg.register::<Profile>(WorkspaceParamsKey::Profile);
        type_reg.register::<EnvManFlow>(WorkspaceParamsKey::Flow);
    }

    fn profile_params_register(type_reg: &mut TypeReg<Self::ProfileParamsKey>) {
        type_reg.register::<EnvType>(ProfileParamsKey::EnvType);
    }

    fn flow_params_register(_type_reg: &mut TypeReg<Self::FlowParamsKey>) {}
}
