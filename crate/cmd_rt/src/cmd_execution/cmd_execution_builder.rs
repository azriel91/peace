use std::{collections::VecDeque, fmt::Debug, marker::PhantomData};

use peace_resources::Resource;
use peace_rt_model::params::ParamsKeys;

use crate::{CmdBlock, CmdBlockRtBox, CmdBlockWrapper, CmdExecution};

/// Collects the [`CmdBlock`]s to run in a `*Cmd` to build a [`CmdExecution`].
///
/// [`CmdBlock`]: crate::CmdBlock
/// [`CmdExecution`]: crate::CmdExecution
#[derive(Debug)]
pub struct CmdExecutionBuilder<E, PKeys, Outcome>
where
    E: 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Blocks of commands to run.
    cmd_blocks: VecDeque<CmdBlockRtBox<E, PKeys>>,
    /// Marker for return type.
    marker: PhantomData<Outcome>,
}

impl<E, PKeys> CmdExecutionBuilder<E, PKeys, ()>
where
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: Debug + ParamsKeys + Unpin + 'static,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<E, PKeys, Outcome> CmdExecutionBuilder<E, PKeys, Outcome>
where
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: Debug + ParamsKeys + Unpin + 'static,
    Outcome: Debug + Resource + Unpin + 'static,
{
    pub fn with_cmd_block<CB, OutcomeNext, OutcomeAcc, OutcomePartial, InputT>(
        self,
        cmd_block: CmdBlockWrapper<CB, E, PKeys, OutcomeNext, OutcomeAcc, OutcomePartial, InputT>,
    ) -> CmdExecutionBuilder<E, PKeys, OutcomeNext>
    where
        CB: CmdBlock<
                Error = E,
                PKeys = PKeys,
                Outcome = OutcomeNext,
                OutcomeAcc = OutcomeAcc,
                OutcomePartial = OutcomePartial,
                InputT = InputT,
            > + Unpin
            + 'static,
        OutcomeNext: Debug + Resource + Unpin + 'static,
        OutcomeAcc: Debug + Resource + Unpin + 'static,
        OutcomePartial: Debug + Unpin + 'static,
        InputT: Debug + Resource + Unpin + 'static,
    {
        let CmdExecutionBuilder {
            mut cmd_blocks,
            marker: _,
        } = self;

        cmd_blocks.push_back(Box::pin(cmd_block));

        CmdExecutionBuilder {
            cmd_blocks,
            marker: PhantomData,
        }
    }

    pub fn build(self) -> CmdExecution<E, PKeys, Outcome> {
        let CmdExecutionBuilder {
            cmd_blocks,
            marker: _,
        } = self;

        CmdExecution {
            cmd_blocks,
            marker: PhantomData,
        }
    }
}

impl<E, PKeys, Outcome> Default for CmdExecutionBuilder<E, PKeys, Outcome>
where
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: Debug + ParamsKeys + Unpin + 'static,
    Outcome: Debug + Resource + 'static,
{
    fn default() -> Self {
        Self {
            cmd_blocks: VecDeque::new(),
            marker: PhantomData,
        }
    }
}
