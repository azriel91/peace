use std::{fmt::Debug, marker::PhantomData};

use async_trait::async_trait;
use peace_cfg::ItemId;
use peace_cmd::scopes::SingleProfileSingleFlowView;
use peace_resources::{resources::ts::SetUp, Resource};
use peace_rt_model::{outcomes::CmdOutcome, params::ParamsKeys, IndexMap};
use tokio::sync::mpsc;

use crate::{CmdBlock, CmdBlockRt};

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
    fn_error_handler: fn(Box<BlockOutcomeAcc>) -> ExecutionOutcome,
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
    pub fn new(
        cmd_block: CB,
        fn_error_handler: fn(Box<BlockOutcomeAcc>) -> ExecutionOutcome,
    ) -> Self {
        Self {
            cmd_block,
            fn_error_handler,
            marker: PhantomData,
        }
    }
}

impl<CB, E, PKeys, ExecutionOutcome, BlockOutcome, BlockOutcomeAcc, BlockOutcomePartial, InputT>
    From<(CB, fn(Box<BlockOutcomeAcc>) -> ExecutionOutcome)>
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
            OutcomeAcc = BlockOutcomeAcc,
            OutcomePartial = BlockOutcomePartial,
            InputT = InputT,
        >,
{
    fn from(
        (cmd_block, fn_error_handler): (CB, fn(Box<BlockOutcomeAcc>) -> ExecutionOutcome),
    ) -> Self {
        Self::new(cmd_block, fn_error_handler)
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
    type PKeys = PKeys;

    async fn exec(
        &self,
        cmd_view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        #[cfg(feature = "output_progress")] progress_tx: Sender<ProgressUpdateAndId>,
        input: Box<dyn Resource>,
    ) -> Result<CmdOutcome<Box<dyn Resource>, Self::Error>, Self::Error> {
        let input = input.downcast().unwrap_or_else(|input| {
            let input_type_name = tynm::type_name::<InputT>();
            let actual_type_name = Resource::type_name(&*input);
            panic!(
                "Expected to downcast `input` to `{input_type_name}`.\n\
                The actual type name is `{actual_type_name:?}`\n\
                This is a bug in the Peace framework."
            );
        });
        let cmd_block = &self.cmd_block;

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

            // `progress_tx` is dropped here, so `progress_rx` will safely end.
        };

        let ((), outcome_result) = futures::join!(execution_task, outcomes_rx_task);

        outcome_result.map(|()| {
            cmd_outcome.map(|outcome_acc| {
                let outcome = cmd_block.outcome_from_acc(outcome_acc);
                Box::new(outcome) as Box<dyn Resource>
            })
        })
    }

    fn execution_outcome_from(&self, outcome_acc: Box<dyn Resource>) -> Box<dyn Resource> {
        let outcome_acc = outcome_acc.downcast().unwrap_or_else(|outcome_acc| {
            let outcome_acc_type_name = tynm::type_name::<BlockOutcome>();
            let actual_type_name = Resource::type_name(&*outcome_acc);
            panic!(
                "Expected to downcast `outcome_acc` to `{outcome_acc_type_name}`.\n\
                The actual type name is `{actual_type_name:?}`\n\
                This is a bug in the Peace framework."
            );
        });
        let execution_outcome = (self.fn_error_handler)(outcome_acc);

        Box::new(execution_outcome) as Box<dyn Resource>
    }
}
