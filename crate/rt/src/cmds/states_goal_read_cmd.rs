use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlow};
use peace_cmd_rt::{CmdBlockWrapper, CmdExecution};
use peace_resources::{resources::ts::SetUp, states::StatesGoalStored};
use peace_rt_model::{params::ParamsKeys, Error};
use peace_rt_model_core::output::OutputWrite;

use crate::cmd_blocks::StatesGoalReadCmdBlock;

/// Reads [`StatesGoalStored`]s from storage.
#[derive(Debug)]
pub struct StatesGoalReadCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> StatesGoalReadCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + Sync + Unpin + 'static,
    O: OutputWrite<E>,
    PKeys: ParamsKeys + 'static,
{
    /// Reads [`StatesGoalStored`]s from storage.
    ///
    /// [`StatesDiscoverCmd`] must have run prior to this command to read the
    /// state.
    ///
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec<'ctx>(
        cmd_ctx: &mut CmdCtx<'ctx, SingleProfileSingleFlow<'ctx, E, O, PKeys, SetUp>>,
    ) -> Result<StatesGoalStored, E> {
        CmdExecution::<StatesGoalStored, _, _>::builder()
            .with_cmd_block(CmdBlockWrapper::new(
                StatesGoalReadCmdBlock::new(),
                std::convert::identity,
            ))
            .build()
            .exec(cmd_ctx)
            .await
            .map(|cmd_outcome| cmd_outcome.value)
    }
}

impl<E, O, PKeys> Default for StatesGoalReadCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
