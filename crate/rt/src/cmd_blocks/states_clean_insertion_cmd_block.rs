use std::{fmt::Debug, marker::PhantomData};

use futures::{stream, StreamExt};

use peace_cfg::ItemId;
use peace_cmd::scopes::SingleProfileSingleFlowView;
use peace_cmd_rt::{async_trait, CmdBlock};
use peace_resources::{
    internal::StatesMut,
    resources::ts::SetUp,
    states::{ts::Clean, StatesClean},
    type_reg::untagged::BoxDtDisplay,
    ResourceFetchError, Resources,
};
use peace_rt_model::{outcomes::CmdOutcome, params::ParamsKeys, Error};
use tokio::sync::mpsc::UnboundedSender;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::progress::ProgressUpdateAndId;
        use tokio::sync::mpsc::Sender;
    }
}

/// Inserts [`StatesClean`]s for each item.
///
/// This calls [`Item::state_clean`] for each item, and groups them together
/// into `StatesClean`.
#[derive(Debug)]
pub struct StatesCleanInsertionCmdBlock<E, PKeys>(PhantomData<(E, PKeys)>);

impl<E, PKeys> StatesCleanInsertionCmdBlock<E, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Returns a new `StatesCleanInsertionCmdBlock`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<E, PKeys> Default for StatesCleanInsertionCmdBlock<E, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[async_trait(?Send)]
impl<E, PKeys> CmdBlock for StatesCleanInsertionCmdBlock<E, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    type Error = E;
    type InputT = ();
    type Outcome = StatesClean;
    type OutcomeAcc = StatesMut<Clean>;
    type OutcomePartial = (ItemId, Result<BoxDtDisplay, E>);
    type PKeys = PKeys;

    fn input_fetch(&self, _resources: &mut Resources<SetUp>) -> Result<(), ResourceFetchError> {
        Ok(())
    }

    fn input_type_names(&self) -> Vec<String> {
        vec![]
    }

    fn outcome_acc_init(&self, (): &Self::InputT) -> Self::OutcomeAcc {
        StatesMut::<Clean>::new()
    }

    fn outcome_from_acc(&self, outcome_acc: Self::OutcomeAcc) -> Self::Outcome {
        StatesClean::from(outcome_acc)
    }

    async fn exec(
        &self,
        _input: Self::InputT,
        cmd_view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        outcomes_tx: &UnboundedSender<Self::OutcomePartial>,
        #[cfg(feature = "output_progress")] _progress_tx: &Sender<ProgressUpdateAndId>,
    ) {
        let SingleProfileSingleFlowView {
            flow,
            params_specs,
            resources,
            ..
        } = cmd_view;

        let params_specs = &*params_specs;
        let resources = &*resources;
        stream::iter(flow.graph().iter_insertion())
            .for_each(|item_rt| async move {
                let item_id = item_rt.id().clone();
                let state_clean_boxed_result = item_rt.state_clean(params_specs, resources).await;
                outcomes_tx
                    .send((item_id, state_clean_boxed_result))
                    .expect("Failed to send `states_clean`.");
            })
            .await;
    }

    fn outcome_collate(
        &self,
        block_outcome: &mut CmdOutcome<Self::OutcomeAcc, Self::Error>,
        outcome_partial: Self::OutcomePartial,
    ) -> Result<(), Self::Error> {
        let CmdOutcome {
            value: states_clean_mut,
            errors,
        } = block_outcome;

        let (item_id, state_clean_boxed_result) = outcome_partial;

        match state_clean_boxed_result {
            Ok(state_clean_boxed) => {
                states_clean_mut.insert_raw(item_id, state_clean_boxed);
            }
            Err(error) => {
                errors.insert(item_id, error);
            }
        }
        Ok(())
    }
}
