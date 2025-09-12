use std::marker::PhantomData;

use peace::{cmd_ctx::CmdCtxTypes, rt_model::output::OutputWrite};

use crate::{
    flows::EnvmanMappingFns,
    model::{EnvManError, ProfileParamsKey, WorkspaceParamsKey},
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
}
