use std::marker::PhantomData;

use peace::{cmd_ctx::CmdCtxTypes, rt_model::output::OutputWrite};

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
}
