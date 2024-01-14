use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlow};
use peace_cmd_model::CmdOutcome;
use peace_cmd_rt::{CmdBlockWrapper, CmdExecution};
use peace_resources::{resources::ts::SetUp, states::StatesCurrentStored};
use peace_rt_model::{params::ParamsKeys, Error};
use peace_rt_model_core::output::OutputWrite;

use crate::cmd_blocks::StatesCurrentReadCmdBlock;

/// Reads [`StatesCurrentStored`]s from storage.
#[derive(Debug)]
pub struct StatesCurrentReadCmd<CmdCtxTypeParamsT>(PhantomData<(CmdCtxTypeParamsT)>);

impl<CmdCtxTypeParamsT> StatesCurrentReadCmd<CmdCtxTypeParamsT>
where
    E: std::error::Error + From<Error> + Send + Sync + Unpin + 'static,
    O: OutputWrite<E>,
    PKeys: ParamsKeys + 'static,
{
    /// Reads [`StatesCurrentStored`]s from storage.
    ///
    /// Either [`StatesCurrentStoredDiscoverCmd`] or [`StatesDiscoverCmd`] must
    /// have run prior to this command to read the state.
    ///
    /// [`StatesCurrentStoredDiscoverCmd`]: crate::StatesCurrentStoredDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec<'ctx>(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'ctx, CmdCtxTypeParamsT, SetUp>>,
    ) -> Result<CmdOutcome<StatesCurrentStored, E>, E> {
        let cmd_execution_builder =
            CmdExecution::<StatesCurrentStored, _, _>::builder().with_cmd_block(
                CmdBlockWrapper::new(StatesCurrentReadCmdBlock::new(), std::convert::identity),
            );

        #[cfg(feature = "output_progress")]
        let cmd_execution_builder = cmd_execution_builder.with_progress_render_enabled(false);

        cmd_execution_builder.build().exec(cmd_ctx).await
    }
}

impl<CmdCtxTypeParamsT> Default for StatesCurrentReadCmd<CmdCtxTypeParamsT> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
