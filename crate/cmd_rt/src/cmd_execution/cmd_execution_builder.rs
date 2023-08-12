use std::{collections::VecDeque, fmt::Debug};

use peace_resources::Resource;
use peace_rt_model::params::ParamsKeys;

use crate::{CmdBlock, CmdBlockRtBox, CmdBlockWrapper, CmdExecution};

/// Collects the [`CmdBlock`]s to run in a `*Cmd` to build a [`CmdExecution`].
///
/// [`CmdBlock`]: crate::CmdBlock
/// [`CmdExecution`]: crate::CmdExecution
#[derive(Debug)]
pub struct CmdExecutionBuilder<E, PKeys, OutcomeT>
where
    E: 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Blocks of commands to run.
    cmd_blocks: VecDeque<CmdBlockRtBox<E, PKeys, OutcomeT>>,
}

impl<E, PKeys, OutcomeT> CmdExecutionBuilder<E, PKeys, OutcomeT>
where
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: Debug + ParamsKeys + Unpin + 'static,
    OutcomeT: Debug + Resource + Unpin + 'static,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cmd_block<CB, OutcomeMutT, OutcomePartialT, InputT>(
        mut self,
        cmd_block: CmdBlockWrapper<CB, E, PKeys, OutcomeT, OutcomeMutT, OutcomePartialT, InputT>,
    ) -> Self
    where
        CB: CmdBlock<
                Error = E,
                PKeys = PKeys,
                OutcomeMutT = OutcomeMutT,
                OutcomePartialT = OutcomePartialT,
                InputT = InputT,
            > + Unpin
            + 'static,
        OutcomeMutT: Debug + Resource + Unpin + 'static,
        OutcomePartialT: Debug + Unpin + 'static,
        InputT: Debug + Resource + Unpin + 'static,
    {
        self.cmd_blocks.push_back(Box::pin(cmd_block));
        self
    }

    pub fn build(self) -> CmdExecution<E, PKeys, OutcomeT> {
        let CmdExecutionBuilder { cmd_blocks } = self;

        CmdExecution { cmd_blocks }
    }
}

impl<E, PKeys, OutcomeT> Default for CmdExecutionBuilder<E, PKeys, OutcomeT>
where
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: Debug + ParamsKeys + Unpin + 'static,
    OutcomeT: Debug + Resource + 'static,
{
    fn default() -> Self {
        Self {
            cmd_blocks: VecDeque::new(),
        }
    }
}
