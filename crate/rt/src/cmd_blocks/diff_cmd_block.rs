use std::{fmt::Debug, marker::PhantomData};

use futures::{StreamExt, TryStreamExt};
use peace_cfg::ItemId;
use peace_cmd::scopes::SingleProfileSingleFlowView;
use peace_cmd_rt::{async_trait, CmdBlock};
use peace_params::ParamsSpecs;
use peace_resources::{
    internal::StateDiffsMut,
    resources::ts::SetUp,
    states::{StateDiffs, States},
    type_reg::untagged::{BoxDtDisplay, TypeMap},
    Resources,
};
use peace_rt_model::{outcomes::CmdOutcome, params::ParamsKeys, Error, Flow};
use tokio::sync::mpsc::UnboundedSender;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::progress::ProgressUpdateAndId;
        use tokio::sync::mpsc::Sender;
    }
}

pub struct DiffCmdBlock<E, PKeys, StatesTs0, StatesTs1>(
    PhantomData<(E, PKeys, StatesTs0, StatesTs1)>,
);

impl<E, PKeys, StatesTs0, StatesTs1> Debug for DiffCmdBlock<E, PKeys, StatesTs0, StatesTs1> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DiffCmdBlock").field(&self.0).finish()
    }
}

impl<E, PKeys, StatesTs0, StatesTs1> DiffCmdBlock<E, PKeys, StatesTs0, StatesTs1> {
    /// Returns a new `DiffCmdBlock`.
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<E, PKeys, StatesTs0, StatesTs1> DiffCmdBlock<E, PKeys, StatesTs0, StatesTs1>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Returns the [`state_diff`]` for each [`Item`].
    ///
    /// This does not take in `CmdCtx` as it may be used by both
    /// `SingleProfileSingleFlow` and `MultiProfileSingleFlow`
    /// commands.
    ///
    /// [`Item`]: peace_cfg::Item
    /// [`state_diff`]: peace_cfg::Item::state_diff
    pub async fn diff_any(
        flow: &Flow<E>,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        states_a: &TypeMap<ItemId, BoxDtDisplay>,
        states_b: &TypeMap<ItemId, BoxDtDisplay>,
    ) -> Result<StateDiffs, E> {
        let state_diffs = {
            let state_diffs_mut = flow
                .graph()
                .stream()
                .map(Result::<_, E>::Ok)
                .try_filter_map(|item| async move {
                    let state_diff_opt = item
                        .state_diff_exec(params_specs, resources, states_a, states_b)
                        .await?;

                    Ok(state_diff_opt.map(|state_diff| (item.id().clone(), state_diff)))
                })
                .try_collect::<StateDiffsMut>()
                .await?;

            StateDiffs::from(state_diffs_mut)
        };

        Ok(state_diffs)
    }
}

impl<E, PKeys, StatesTs0, StatesTs1> Default for DiffCmdBlock<E, PKeys, StatesTs0, StatesTs1> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[async_trait(?Send)]
impl<E, PKeys, StatesTs0, StatesTs1> CmdBlock for DiffCmdBlock<E, PKeys, StatesTs0, StatesTs1>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
    StatesTs0: Debug + Send + Sync + 'static,
    StatesTs1: Debug + Send + Sync + 'static,
{
    type Error = E;
    type InputT = (States<StatesTs0>, States<StatesTs1>);
    type Outcome = StateDiffs;
    type OutcomeAcc = StateDiffs;
    type OutcomePartial = Result<StateDiffs, E>;
    type PKeys = PKeys;

    fn input_fetch(&self, resources: &mut Resources<SetUp>) -> Self::InputT {
        let states_ts0 = resources.remove::<States<StatesTs0>>().unwrap_or_else(|| {
            let states_ts0 = tynm::type_name::<States<StatesTs0>>();
            panic!("Expected `{states_ts0}` to exist in `Resources`");
        });
        let states_ts1 = resources.remove::<States<StatesTs1>>().unwrap_or_else(|| {
            let states_ts1 = tynm::type_name::<States<StatesTs1>>();
            panic!("Expected `{states_ts1}` to exist in `Resources`");
        });

        (states_ts0, states_ts1)
    }

    fn outcome_acc_init(&self, _input: &Self::InputT) -> Self::OutcomeAcc {
        StateDiffs::new()
    }

    fn outcome_from_acc(&self, outcome_acc: Self::OutcomeAcc) -> Self::Outcome {
        outcome_acc
    }

    async fn exec(
        &self,
        input: Self::InputT,
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

        let (states_a, states_b) = input;

        let state_diffs = Self::diff_any(flow, params_specs, resources, &states_a, &states_b).await;
        outcomes_tx
            .send(state_diffs)
            .expect("Failed to send `state_diffs`.");
    }

    fn outcome_collate(
        &self,
        block_outcome: &mut CmdOutcome<Self::OutcomeAcc, Self::Error>,
        outcome_partial: Self::OutcomePartial,
    ) -> Result<(), Self::Error> {
        let state_diffs = outcome_partial?;
        block_outcome.value = state_diffs;
        Ok(())
    }
}
