use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlow};
use peace_cmd_rt::{CmdBlockWrapper, CmdExecution};
use peace_resources::{resources::ts::SetUp, states::StatesCurrentStored};
use peace_rt_model::{outcomes::CmdOutcome, params::ParamsKeys, Error};
use peace_rt_model_core::output::OutputWrite;

use crate::cmd_blocks::StatesCurrentReadCmdBlock;

/// Reads [`StatesCurrentStored`]s from storage.
#[derive(Debug)]
pub struct StatesCurrentReadCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> StatesCurrentReadCmd<E, O, PKeys>
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
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'ctx, E, O, PKeys, SetUp>>,
    ) -> Result<CmdOutcome<StatesCurrentStored, E>, E> {
        CmdExecution::<StatesCurrentStored, _, _>::builder()
            .with_cmd_block(CmdBlockWrapper::new(
                StatesCurrentReadCmdBlock::new(),
                std::convert::identity,
            ))
            .build()
            .exec(cmd_ctx)
            .await
    }
}

impl<E, O, PKeys> Default for StatesCurrentReadCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
