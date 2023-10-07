use std::{collections::VecDeque, fmt::Debug};

use interruptible::InterruptSignal;
use peace_resources::{resources::ts::SetUp, Resource, Resources};
use peace_rt_model::params::ParamsKeys;
use tokio::sync::oneshot;

use crate::{
    cmd_execution::{Interruptible, InterruptibleT, NonInterruptible},
    CmdBlock, CmdBlockRtBox, CmdBlockWrapper, CmdExecution,
};

/// Collects the [`CmdBlock`]s to run in a `*Cmd` to build a [`CmdExecution`].
///
/// [`CmdBlock`]: crate::CmdBlock
/// [`CmdExecution`]: crate::CmdExecution
#[derive(Debug)]
pub struct CmdExecutionBuilder<ExecutionOutcome, E, PKeys, Interruptibility>
where
    ExecutionOutcome: Debug + Send + Sync + 'static,
    E: 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Blocks of commands to run.
    cmd_blocks: VecDeque<CmdBlockRtBox<E, PKeys, ExecutionOutcome>>,
    /// Logic to extract the `ExecutionOutcome` from `Resources`.
    execution_outcome_fetch: fn(&mut Resources<SetUp>) -> ExecutionOutcome,
    /// Whether the `CmdExecution` is interruptible.
    ///
    /// If it is, this holds the interrupt channel receiver.
    interruptibility: Interruptibility,
    /// Whether or not to render progress.
    ///
    /// This is intended for `*Cmd`s that do not have meaningful progress to
    /// render, such as deserializing a single file on disk, and there is no
    /// benefit to presenting empty progress bars for each item to the user
    ///
    /// Defaults to `true`.
    #[cfg(feature = "output_progress")]
    progress_render_enabled: bool,
}

impl<ExecutionOutcome, E, PKeys> CmdExecutionBuilder<ExecutionOutcome, E, PKeys, NonInterruptible>
where
    ExecutionOutcome: Debug + Send + Sync + 'static,
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: ParamsKeys + 'static,
{
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables interruptibility for this CmdExecution.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let (interrupt_tx, interrupt_rx) = oneshot::channel::<InterruptSignal>();
    ///
    /// let cmd_execution = CmdExecutionBuilder::new()
    ///     .interruptible(interrupt_rx)
    ///     .build();
    ///
    /// interrupt_tx
    ///     .send(InterruptSignal).expect("Expected to send `InterruptSignal`.");;
    /// ```
    pub fn interruptible(
        self,
        interrupt_rx: oneshot::Receiver<InterruptSignal>,
    ) -> CmdExecutionBuilder<ExecutionOutcome, E, PKeys, Interruptible> {
        let Self {
            cmd_blocks,
            execution_outcome_fetch,
            interruptibility: NonInterruptible,
            #[cfg(feature = "output_progress")]
            progress_render_enabled,
        } = self;

        CmdExecutionBuilder {
            cmd_blocks,
            execution_outcome_fetch,
            interruptibility: Interruptible(interrupt_rx),
            #[cfg(feature = "output_progress")]
            progress_render_enabled,
        }
    }

    /// Returns the `CmdExecution` to execute.
    pub fn build(self) -> CmdExecution<ExecutionOutcome, E, PKeys, NonInterruptible> {
        let CmdExecutionBuilder {
            cmd_blocks,
            execution_outcome_fetch,
            interruptibility,
            #[cfg(feature = "output_progress")]
            progress_render_enabled,
        } = self;

        CmdExecution {
            cmd_blocks,
            execution_outcome_fetch,
            interruptibility,
            #[cfg(feature = "output_progress")]
            progress_render_enabled,
        }
    }
}

impl<ExecutionOutcome, E, PKeys> CmdExecutionBuilder<ExecutionOutcome, E, PKeys, Interruptible>
where
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: ParamsKeys + 'static,
    ExecutionOutcome: Debug + Resource + Unpin + 'static,
{
    /// Returns the `CmdExecution` to execute.
    pub fn build(self) -> CmdExecution<ExecutionOutcome, E, PKeys, Interruptible> {
        let CmdExecutionBuilder {
            cmd_blocks,
            execution_outcome_fetch,
            interruptibility,
            #[cfg(feature = "output_progress")]
            progress_render_enabled,
        } = self;

        CmdExecution {
            cmd_blocks,
            execution_outcome_fetch,
            interruptibility,
            #[cfg(feature = "output_progress")]
            progress_render_enabled,
        }
    }
}

impl<ExecutionOutcome, E, PKeys, Interruptibility>
    CmdExecutionBuilder<ExecutionOutcome, E, PKeys, Interruptibility>
where
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: ParamsKeys + 'static,
    ExecutionOutcome: Debug + Resource + Unpin + 'static,
    Interruptibility: InterruptibleT,
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
    ) -> CmdExecutionBuilder<ExecutionOutcome, E, PKeys, Interruptibility>
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
            interruptibility,
            #[cfg(feature = "output_progress")]
            progress_render_enabled,
        } = self;

        cmd_blocks.push_back(Box::pin(cmd_block));

        CmdExecutionBuilder {
            cmd_blocks,
            execution_outcome_fetch,
            interruptibility,
            #[cfg(feature = "output_progress")]
            progress_render_enabled,
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

    /// Specifies whether or not to render progress.
    ///
    /// This is `true` by default, so usually this would be called with `false`.
    ///
    /// This is intended for `*Cmd`s that do not have meaningful progress to
    /// render, such as deserializing a single file on disk, and there is no
    /// benefit to presenting empty progress bars for each item to the user.
    ///
    /// When this method is called multiple times, the last call wins.
    #[cfg(feature = "output_progress")]
    pub fn with_progress_render_enabled(mut self, progress_render_enabled: bool) -> Self {
        self.progress_render_enabled = progress_render_enabled;
        self
    }
}

impl<ExecutionOutcome, E, PKeys> Default
    for CmdExecutionBuilder<ExecutionOutcome, E, PKeys, NonInterruptible>
where
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: ParamsKeys + 'static,
    ExecutionOutcome: Debug + Resource + 'static,
{
    fn default() -> Self {
        Self {
            cmd_blocks: VecDeque::new(),
            execution_outcome_fetch,
            interruptibility: NonInterruptible,
            #[cfg(feature = "output_progress")]
            progress_render_enabled: true,
        }
    }
}

fn execution_outcome_fetch<ExecutionOutcome>(resources: &mut Resources<SetUp>) -> ExecutionOutcome
where
    ExecutionOutcome: Debug + Send + Sync + 'static,
{
    resources
        .try_remove::<ExecutionOutcome>()
        .unwrap_or_else(|_error| {
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
