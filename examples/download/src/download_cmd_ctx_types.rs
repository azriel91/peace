use std::marker::PhantomData;

use peace::{
    cmd_ctx::{type_reg::untagged::TypeReg, CmdCtxTypes},
    rt_model::output::OutputWrite,
};

use crate::DownloadError;

#[derive(Debug)]
pub struct DownloadCmdCtxTypes<Output>(PhantomData<Output>);

impl<Output> CmdCtxTypes for DownloadCmdCtxTypes<Output>
where
    Output: OutputWrite,
    DownloadError: From<<Output as OutputWrite>::Error>,
{
    type AppError = DownloadError;
    type FlowParamsKey = ();
    type MappingFns = ();
    type Output = Output;
    type ProfileParamsKey = ();
    type WorkspaceParamsKey = ();

    fn workspace_params_register(_type_reg: &mut TypeReg<Self::WorkspaceParamsKey>) {}

    fn profile_params_register(_type_reg: &mut TypeReg<Self::ProfileParamsKey>) {}

    fn flow_params_register(_type_reg: &mut TypeReg<Self::FlowParamsKey>) {}
}
