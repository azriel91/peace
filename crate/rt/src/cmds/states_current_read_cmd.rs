use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{
    ctx::{CmdCtx, CmdCtxTypesConstrained},
    scopes::SingleProfileSingleFlow,
};
use peace_cmd_model::CmdOutcome;
use peace_cmd_rt::{CmdBlockWrapper, CmdExecution};
use peace_resources::states::StatesCurrentStored;

use crate::cmd_blocks::StatesCurrentReadCmdBlock;

/// Reads [`StatesCurrentStored`]s from storage.
#[derive(Debug)]
pub struct StatesCurrentReadCmd<CmdCtxTypesT>(PhantomData<CmdCtxTypesT>);

impl<CmdCtxTypesT> StatesCurrentReadCmd<CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypesConstrained,
{
    /// Reads [`StatesCurrentStored`]s from storage.
    ///
    /// Either [`StatesCurrentStoredDiscoverCmd`] or [`StatesDiscoverCmd`] must
    /// have run prior to this command to read the state.
    ///
    /// [`StatesCurrentStoredDiscoverCmd`]: crate::StatesCurrentStoredDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec<'ctx>(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'ctx, CmdCtxTypesT>>,
    ) -> Result<
        CmdOutcome<StatesCurrentStored, <CmdCtxTypesT as CmdCtxTypesConstrained>::AppError>,
        <CmdCtxTypesT as CmdCtxTypesConstrained>::AppError,
    >
    where
        CmdCtxTypesT: 'ctx,
    {
        let cmd_execution_builder = CmdExecution::<StatesCurrentStored, _>::builder()
            .with_cmd_block(CmdBlockWrapper::new(
                StatesCurrentReadCmdBlock::new(),
                std::convert::identity,
            ));

        #[cfg(feature = "output_progress")]
        let cmd_execution_builder = cmd_execution_builder.with_progress_render_enabled(false);

        cmd_execution_builder.build().exec(cmd_ctx).await
    }
}

impl<CmdCtxTypesT> Default for StatesCurrentReadCmd<CmdCtxTypesT> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
