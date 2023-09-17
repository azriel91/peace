use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlow};
use peace_resources::{resources::ts::SetUp, states::StatesGoalStored};
use peace_rt_model::{params::ParamsKeys, Error};
use peace_rt_model_core::output::OutputWrite;

use crate::cmds::StatesGoalReadCmd;

/// Displays [`StatesGoal`]s from storage.
#[derive(Debug)]
pub struct StatesGoalDisplayCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> StatesGoalDisplayCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + Sync + Unpin + 'static,
    PKeys: ParamsKeys + 'static,
    O: OutputWrite<E>,
{
    /// Displays [`StatesGoal`]s from storage.
    ///
    /// [`StatesDiscoverCmd`] must have run prior to this command to read the
    /// state.
    ///
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StatesGoalStored, E> {
        let states_goal_stored_result = StatesGoalReadCmd::exec(cmd_ctx).await;
        let output = cmd_ctx.output_mut();

        match states_goal_stored_result {
            Ok(states_goal) => {
                output.present(&states_goal).await?;
                Ok(states_goal)
            }
            Err(e) => {
                output.write_err(&e).await?;
                Err(e)
            }
        }
    }
}

impl<E, O, PKeys> Default for StatesGoalDisplayCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
