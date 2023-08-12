use std::{fmt::Debug, marker::PhantomData, pin::Pin};

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
pub struct CmdBlockWrapper<CB, E, PKeys, Outcome, OutcomeAcc, OutcomePartial, InputT> {
    /// Underlying `CmdBlock` implementation.
    ///
    /// The trait constraints are applied on impl blocks.
    cmd_block: CB,
    /// Seed function for `OutcomeAcc`.
    fn_outcome_acc_init: fn() -> OutcomeAcc,
    /// Marker.
    marker: PhantomData<(E, PKeys, Outcome, OutcomePartial, InputT)>,
}

impl<CB, E, PKeys, Outcome, OutcomeAcc, OutcomePartial, InputT>
    CmdBlockWrapper<CB, E, PKeys, Outcome, OutcomeAcc, OutcomePartial, InputT>
where
    CB: CmdBlock<
            Error = E,
            PKeys = PKeys,
            OutcomeAcc = OutcomeAcc,
            OutcomePartial = OutcomePartial,
            InputT = InputT,
        >,
{
    pub fn new(cmd_block: CB, fn_outcome_acc_init: fn() -> OutcomeAcc) -> Self {
        Self {
            cmd_block,
            fn_outcome_acc_init,
            marker: PhantomData,
        }
    }
}

impl<CB, E, PKeys, Outcome, OutcomeAcc, OutcomePartial, InputT> From<(CB, fn() -> OutcomeAcc)>
    for CmdBlockWrapper<CB, E, PKeys, Outcome, OutcomeAcc, OutcomePartial, InputT>
where
    CB: CmdBlock<
            Error = E,
            PKeys = PKeys,
            OutcomeAcc = OutcomeAcc,
            OutcomePartial = OutcomePartial,
            InputT = InputT,
        >,
{
    fn from((cmd_block, fn_outcome_acc_init): (CB, fn() -> OutcomeAcc)) -> Self {
        Self::new(cmd_block, fn_outcome_acc_init)
    }
}

#[async_trait(?Send)]
impl<CB, E, PKeys, Outcome, OutcomeAcc, OutcomePartial, InputT> CmdBlockRt
    for CmdBlockWrapper<CB, E, PKeys, Outcome, OutcomeAcc, OutcomePartial, InputT>
where
    CB: CmdBlock<
            Error = E,
            PKeys = PKeys,
            Outcome = Outcome,
            OutcomeAcc = OutcomeAcc,
            OutcomePartial = OutcomePartial,
        > + Unpin,
    E: Debug + std::error::Error + From<peace_rt_model::Error> + Send + Unpin + 'static,
    PKeys: Debug + ParamsKeys + Unpin + 'static,
    Outcome: Debug + Unpin + 'static,
    OutcomeAcc: Debug + Resource + Unpin + 'static,
    OutcomePartial: Debug + Unpin + 'static,
    InputT: Debug + Resource + Unpin + 'static,
{
    type Error = E;
    type Outcome = Outcome;
    type PKeys = PKeys;

    async fn exec(
        self: Pin<Box<Self>>,
        cmd_view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        #[cfg(feature = "output_progress")] progress_tx: Sender<ProgressUpdateAndId>,
        input: Box<dyn Resource>,
    ) -> Result<CmdOutcome<Self::Outcome, Self::Error>, Self::Error> {
        let input = input.downcast().unwrap_or_else(|input| {
            let input_type_name = tynm::type_name::<InputT>();
            let actual_type_name = Resource::type_name(&*input);
            panic!(
                "Expected to downcast input to `{input_type_name}`.\n\
                The actual type name is `{actual_type_name:?}`\n\
                This is a bug in the Peace framework."
            );
        });
        let cmd_block = &self.cmd_block;

        let (outcomes_tx, mut outcomes_rx) = mpsc::unbounded_channel::<OutcomePartial>();
        let mut cmd_outcome = {
            let outcome = (self.fn_outcome_acc_init)();
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

        outcome_result.map(|()| cmd_outcome.map(|outcome_acc| cmd_block.outcome_map(outcome_acc)))
    }
}
