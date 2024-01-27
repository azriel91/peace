use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{
    ctx::{CmdCtx, CmdCtxTypeParamsConstrained},
    scopes::SingleProfileSingleFlow,
};
use peace_cmd_model::CmdOutcome;
use peace_cmd_rt::{CmdBlockWrapper, CmdExecution};
use peace_resources::{resources::ts::SetUp, states::StatesGoalStored};

use crate::cmd_blocks::StatesGoalReadCmdBlock;

/// Reads [`StatesGoalStored`]s from storage.
#[derive(Debug)]
pub struct StatesGoalReadCmd<CmdCtxTypeParamsT>(PhantomData<CmdCtxTypeParamsT>);

impl<CmdCtxTypeParamsT> StatesGoalReadCmd<CmdCtxTypeParamsT>
where
    CmdCtxTypeParamsT: CmdCtxTypeParamsConstrained,
{
    /// Reads [`StatesGoalStored`]s from storage.
    ///
    /// [`StatesDiscoverCmd`] must have run prior to this command to read the
    /// state.
    ///
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec<'ctx>(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'ctx, CmdCtxTypeParamsT, SetUp>>,
    ) -> Result<
        CmdOutcome<StatesGoalStored, <CmdCtxTypeParamsT as CmdCtxTypeParamsConstrained>::AppError>,
        <CmdCtxTypeParamsT as CmdCtxTypeParamsConstrained>::AppError,
    >
    where
        CmdCtxTypeParamsT: 'ctx,
    {
        let cmd_execution_builder = CmdExecution::<StatesGoalStored, _>::builder().with_cmd_block(
            CmdBlockWrapper::new(StatesGoalReadCmdBlock::new(), std::convert::identity),
        );

        #[cfg(feature = "output_progress")]
        let cmd_execution_builder = cmd_execution_builder.with_progress_render_enabled(false);

        cmd_execution_builder.build().exec(cmd_ctx).await
    }
}

impl<CmdCtxTypeParamsT> Default for StatesGoalReadCmd<CmdCtxTypeParamsT> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
