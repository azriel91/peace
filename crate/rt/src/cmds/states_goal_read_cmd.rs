use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{
    ctx::{CmdCtx, CmdCtxTypesConstrained},
    scopes::SingleProfileSingleFlow,
};
use peace_cmd_model::CmdOutcome;
use peace_cmd_rt::{CmdBlockWrapper, CmdExecution};
use peace_resources_rt::states::StatesGoalStored;

use crate::cmd_blocks::StatesGoalReadCmdBlock;

/// Reads [`StatesGoalStored`]s from storage.
#[derive(Debug)]
pub struct StatesGoalReadCmd<CmdCtxTypesT>(PhantomData<CmdCtxTypesT>);

impl<CmdCtxTypesT> StatesGoalReadCmd<CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypesConstrained,
{
    /// Reads [`StatesGoalStored`]s from storage.
    ///
    /// [`StatesDiscoverCmd`] must have run prior to this command to read the
    /// state.
    ///
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec<'ctx>(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'ctx, CmdCtxTypesT>>,
    ) -> Result<
        CmdOutcome<StatesGoalStored, <CmdCtxTypesT as CmdCtxTypesConstrained>::AppError>,
        <CmdCtxTypesT as CmdCtxTypesConstrained>::AppError,
    >
    where
        CmdCtxTypesT: 'ctx,
    {
        let cmd_execution_builder = CmdExecution::<StatesGoalStored, _>::builder().with_cmd_block(
            CmdBlockWrapper::new(StatesGoalReadCmdBlock::new(), std::convert::identity),
        );

        #[cfg(feature = "output_progress")]
        let cmd_execution_builder = cmd_execution_builder.with_progress_render_enabled(false);

        cmd_execution_builder.build().exec(cmd_ctx).await
    }
}

impl<CmdCtxTypesT> Default for StatesGoalReadCmd<CmdCtxTypesT> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
