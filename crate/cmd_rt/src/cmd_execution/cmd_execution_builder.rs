use std::{collections::VecDeque, fmt::Debug};

use peace_resources::{resources::ts::SetUp, Resource, Resources};
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
    /// Logic to extract the `ExecutionOutcome` from `Resources`.
    execution_outcome_fetch: fn(&mut Resources<SetUp>) -> ExecutionOutcome,
}

impl<ExecutionOutcome, E, PKeys> CmdExecutionBuilder<ExecutionOutcome, E, PKeys>
where
    ExecutionOutcome: Debug + Send + Sync + 'static,
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: ParamsKeys + 'static,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<ExecutionOutcome, E, PKeys> CmdExecutionBuilder<ExecutionOutcome, E, PKeys>
where
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: ParamsKeys + 'static,
    ExecutionOutcome: Debug + Resource + Unpin + 'static,
{
    /// Adds a `CmdBlock` to this execution.
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
        let CmdExecutionBuilder {
            mut cmd_blocks,
            execution_outcome_fetch,
        } = self;

        cmd_blocks.push_back(Box::pin(cmd_block));

        CmdExecutionBuilder {
            cmd_blocks,
            execution_outcome_fetch,
        }
    }

    /// Specifies the logic to fetch the `ExecutionOutcome` from `Resources`.
    ///
    /// By default, the `CmdExecution` will run
    /// `resources.remove::<ExecutionOutcome>()`. However, if the
    /// `ExecutionOutcome` is not inserted as a single type, this allows
    /// consumers to specify which types to remove from `resources` and return
    /// as the `ExecutionOutcome`.
    pub fn with_execution_outcome_fetch(
        mut self,
        execution_outcome_fetch: fn(&mut Resources<SetUp>) -> ExecutionOutcome,
    ) -> Self {
        self.execution_outcome_fetch = execution_outcome_fetch;
        self
    }

    /// Returns the `CmdExecution` to execute.
    pub fn build(self) -> CmdExecution<ExecutionOutcome, E, PKeys> {
        let CmdExecutionBuilder {
            cmd_blocks,
            execution_outcome_fetch,
        } = self;

        CmdExecution {
            cmd_blocks,
            execution_outcome_fetch,
        }
    }
}

impl<ExecutionOutcome, E, PKeys> Default for CmdExecutionBuilder<ExecutionOutcome, E, PKeys>
where
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: ParamsKeys + 'static,
    ExecutionOutcome: Debug + Resource + 'static,
{
    fn default() -> Self {
        Self {
            cmd_blocks: VecDeque::new(),
            execution_outcome_fetch,
        }
    }
}

fn execution_outcome_fetch<ExecutionOutcome>(resources: &mut Resources<SetUp>) -> ExecutionOutcome
where
    ExecutionOutcome: Debug + Send + Sync + 'static,
{
    resources.remove::<ExecutionOutcome>().unwrap_or_else(|| {
        let execution_outcome_type_name = tynm::type_name::<ExecutionOutcome>();
        panic!(
            "Expected `{execution_outcome_type_name}` to exist in `Resources`.\n\
            Make sure the final `CmdBlock` has that type as its `Outcome`.\n\
            \n\
            You may wish to call `CmdExecutionBuilder::with_execution_outcome_fetch`\n\
            to specify how to fetch the `ExecutionOutcome`."
        );
    })
}
