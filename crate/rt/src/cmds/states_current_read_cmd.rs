use std::{fmt::Debug, marker::PhantomData};

use peace_cmd_ctx::{CmdCtxSpsf, CmdCtxTypes};
use peace_cmd_model::CmdOutcome;
use peace_cmd_rt::{CmdBlockWrapper, CmdExecution};
use peace_resource_rt::states::StatesCurrentStored;

use crate::cmd_blocks::StatesCurrentReadCmdBlock;

/// Reads [`StatesCurrentStored`]s from storage.
#[derive(Debug)]
pub struct StatesCurrentReadCmd<CmdCtxTypesT>(PhantomData<CmdCtxTypesT>);

impl<CmdCtxTypesT> StatesCurrentReadCmd<CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Reads [`StatesCurrentStored`]s from storage.
    ///
    /// Either [`StatesCurrentStoredDiscoverCmd`] or [`StatesDiscoverCmd`] must
    /// have run prior to this command to read the state.
    ///
    /// [`StatesCurrentStoredDiscoverCmd`]: crate::StatesCurrentStoredDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec<'ctx>(
        cmd_ctx: &mut CmdCtxSpsf<'ctx, CmdCtxTypesT>,
    ) -> Result<
        CmdOutcome<StatesCurrentStored, <CmdCtxTypesT as CmdCtxTypes>::AppError>,
        <CmdCtxTypesT as CmdCtxTypes>::AppError,
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
