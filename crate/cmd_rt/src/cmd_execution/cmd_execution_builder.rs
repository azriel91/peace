use std::{collections::VecDeque, fmt::Debug};

use peace_resources::Resource;
use peace_rt_model::params::ParamsKeys;

use crate::{CmdBlock, CmdBlockRtBox, CmdBlockWrapper, CmdExecution};

/// Collects the [`CmdBlock`]s to run in a `*Cmd` to build a [`CmdExecution`].
///
/// [`CmdBlock`]: crate::CmdBlock
/// [`CmdExecution`]: crate::CmdExecution
#[derive(Debug)]
pub struct CmdExecutionBuilder<ExecutionOutcome, E, PKeys>
where
    ExecutionOutcome: Debug + Send + Sync + 'static,
    E: 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Blocks of commands to run.
    cmd_blocks: VecDeque<CmdBlockRtBox<E, PKeys, ExecutionOutcome>>,
}

impl<ExecutionOutcome, E, PKeys> CmdExecutionBuilder<ExecutionOutcome, E, PKeys>
where
    ExecutionOutcome: Debug + Send + Sync + 'static,
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: Debug + ParamsKeys + Unpin + 'static,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<ExecutionOutcome, E, PKeys> CmdExecutionBuilder<ExecutionOutcome, E, PKeys>
where
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: Debug + ParamsKeys + Unpin + 'static,
    ExecutionOutcome: Debug + Resource + Unpin + 'static,
{
    pub fn with_cmd_block<CB, BlockOutcomeNext, BlockOutcomeAcc, BlockOutcomePartial, InputT>(
        self,
        cmd_block: CmdBlockWrapper<
            CB,
            E,
            PKeys,
            ExecutionOutcome,
            BlockOutcomeNext,
            BlockOutcomeAcc,
            BlockOutcomePartial,
            InputT,
        >,
    ) -> CmdExecutionBuilder<ExecutionOutcome, E, PKeys>
    where
        CB: CmdBlock<
                Error = E,
                PKeys = PKeys,
                Outcome = BlockOutcomeNext,
                OutcomeAcc = BlockOutcomeAcc,
                OutcomePartial = BlockOutcomePartial,
                InputT = InputT,
            > + Unpin
            + 'static,
        ExecutionOutcome: Debug + Resource + Unpin + 'static,
        BlockOutcomeNext: Debug + Resource + Unpin + 'static,
        BlockOutcomeAcc: Debug + Resource + Unpin + 'static,
        BlockOutcomePartial: Debug + Unpin + 'static,
        InputT: Debug + Resource + Unpin + 'static,
    {
        let CmdExecutionBuilder { mut cmd_blocks } = self;

        cmd_blocks.push_back(Box::pin(cmd_block));

        CmdExecutionBuilder { cmd_blocks }
    }

    pub fn build(self) -> CmdExecution<ExecutionOutcome, E, PKeys> {
        let CmdExecutionBuilder { cmd_blocks } = self;

        CmdExecution { cmd_blocks }
    }
}

impl<ExecutionOutcome, E, PKeys> Default for CmdExecutionBuilder<ExecutionOutcome, E, PKeys>
where
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: Debug + ParamsKeys + Unpin + 'static,
    ExecutionOutcome: Debug + Resource + 'static,
{
    fn default() -> Self {
        Self {
            cmd_blocks: VecDeque::new(),
        }
    }
}
