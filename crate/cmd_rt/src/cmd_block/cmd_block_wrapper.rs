use std::{fmt::Debug, marker::PhantomData};

use async_trait::async_trait;
use peace_cfg::ItemId;
use peace_cmd::scopes::SingleProfileSingleFlowView;
use peace_resources::{resources::ts::SetUp, Resource};
use peace_rt_model::{outcomes::CmdOutcome, params::ParamsKeys, IndexMap};
use tokio::sync::mpsc;

use crate::{CmdBlock, CmdBlockError, CmdBlockRt};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::progress::ProgressUpdateAndId;
        use tokio::sync::mpsc::Sender;
    }
}

/// Wraps a [`CmdBlock`] so that the type erased [`CmdBlockRt`] trait can be
/// implemented on the type within this crate.
///
/// [`CmdBlockRt`]: crate::CmdBlockRt
#[derive(Debug)]
pub struct CmdBlockWrapper<
    CB,
    E,
    PKeys,
    ExecutionOutcome,
    BlockOutcome,
    BlockOutcomeAcc,
    BlockOutcomePartial,
    InputT,
> {
    /// Underlying `CmdBlock` implementation.
    ///
    /// The trait constraints are applied on impl blocks.
    cmd_block: CB,
    /// Function to run if an item failure happens while executing this
    /// `CmdBlock`.
    fn_error_handler: fn(BlockOutcomeAcc) -> ExecutionOutcome,
    /// Marker.
    marker: PhantomData<(E, PKeys, BlockOutcome, BlockOutcomePartial, InputT)>,
}

impl<CB, E, PKeys, ExecutionOutcome, BlockOutcome, BlockOutcomeAcc, BlockOutcomePartial, InputT>
    CmdBlockWrapper<
        CB,
        E,
        PKeys,
        ExecutionOutcome,
        BlockOutcome,
        BlockOutcomeAcc,
        BlockOutcomePartial,
        InputT,
    >
where
    CB: CmdBlock<
            Error = E,
            PKeys = PKeys,
            OutcomeAcc = BlockOutcomeAcc,
            OutcomePartial = BlockOutcomePartial,
            InputT = InputT,
        >,
{
    pub fn new(cmd_block: CB, fn_error_handler: fn(BlockOutcomeAcc) -> ExecutionOutcome) -> Self {
        Self {
            cmd_block,
            fn_error_handler,
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<CB, E, PKeys, ExecutionOutcome, BlockOutcome, BlockOutcomeAcc, BlockOutcomePartial, InputT>
    CmdBlockRt
    for CmdBlockWrapper<
        CB,
        E,
        PKeys,
        ExecutionOutcome,
        BlockOutcome,
        BlockOutcomeAcc,
        BlockOutcomePartial,
        InputT,
    >
where
    CB: CmdBlock<
            Error = E,
            PKeys = PKeys,
            Outcome = BlockOutcome,
            OutcomeAcc = BlockOutcomeAcc,
            OutcomePartial = BlockOutcomePartial,
            InputT = InputT,
        > + Unpin,
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: Debug + ParamsKeys + Unpin + 'static,
    ExecutionOutcome: Debug + Unpin + Send + Sync + 'static,
    BlockOutcome: Debug + Unpin + Send + Sync + 'static,
    BlockOutcomeAcc: Debug + Resource + Unpin + 'static,
    BlockOutcomePartial: Debug + Unpin + 'static,
    InputT: Debug + Resource + Unpin + 'static,
{
    type Error = E;
    type ExecutionOutcome = ExecutionOutcome;
    type PKeys = PKeys;

    async fn exec(
        &self,
        cmd_view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        #[cfg(feature = "output_progress")] progress_tx: Sender<ProgressUpdateAndId>,
    ) -> Result<(), CmdBlockError<ExecutionOutcome, Self::Error>> {
        let cmd_block = &self.cmd_block;
        let input = cmd_block.input_fetch(&mut cmd_view.resources);

        let (outcomes_tx, mut outcomes_rx) = mpsc::unbounded_channel::<BlockOutcomePartial>();
        let mut cmd_outcome = {
            let outcome = cmd_block.outcome_acc_init();
            let errors = IndexMap::<ItemId, E>::new();
            CmdOutcome {
                value: outcome,
                errors,
            }
        };
        let outcomes_rx_task = async {
            while let Some(item_outcome) = outcomes_rx.recv().await {
                cmd_block.outcome_collate(&mut cmd_outcome, item_outcome)?;
            }

            Result::<(), E>::Ok(())
        };

        let execution_task = async move {
            let outcomes_tx = &outcomes_tx;
            #[cfg(feature = "output_progress")]
            let progress_tx = &progress_tx;

            cmd_block
                .exec(
                    input,
                    cmd_view,
                    outcomes_tx,
                    #[cfg(feature = "output_progress")]
                    progress_tx,
                )
                .await;

            cmd_view
            // `progress_tx` is dropped here, so `progress_rx` will safely end.
        };

        let (cmd_view, outcome_result) = futures::join!(execution_task, outcomes_rx_task);
        let () = outcome_result.map_err(CmdBlockError::Block)?;

        if cmd_outcome.is_ok() {
            let cmd_outcome =
                cmd_outcome.map(|outcome_acc| cmd_block.outcome_from_acc(outcome_acc));
            let CmdOutcome {
                value: outcome_acc,
                errors: _,
            } = cmd_outcome;

            cmd_view.resources.insert(outcome_acc);

            Ok(())
        } else {
            // If possible, `CmdBlock` outcomes with item errors need to be mapped to
            // the `CmdExecution` outcome type, so we still return the item errors.
            //
            // e.g. `StatesCurrentMut` should be mapped into `StatesEnsured` when some
            // items fail to be ensured.
            //
            // Note, when discovering current and goal states for diffing, and an item
            // error occurs, mapping the partially accumulated `(StatesCurrentMut,
            // StatesGoalMut)` into `StateDiffs` may or may not be semantically
            // meaningful.

            let cmd_outcome = cmd_outcome.map(self.fn_error_handler);
            Err(CmdBlockError::Outcome(cmd_outcome))
        }
    }
}
