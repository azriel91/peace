use std::{fmt::Debug, marker::PhantomData};

use futures::{StreamExt, TryStreamExt};
use peace_cfg::ItemId;
use peace_cmd::scopes::SingleProfileSingleFlowView;
use peace_cmd_rt::{async_trait, CmdBlock};
use peace_params::ParamsSpecs;
use peace_resources::{
    internal::StateDiffsMut,
    resources::ts::SetUp,
    states::{
        ts::{Current, CurrentStored, Goal, GoalStored},
        StateDiffs, States,
    },
    type_reg::untagged::{BoxDtDisplay, TypeMap},
    ResourceFetchError, Resources,
};
use peace_rt_model::{outcomes::CmdOutcome, params::ParamsKeys, Error, Flow};
use tokio::sync::mpsc::UnboundedSender;

use crate::cmds::DiffStateSpec;

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
    type Outcome = (StateDiffs, Self::InputT);
    type OutcomeAcc = Option<(StateDiffs, Self::InputT)>;
    type OutcomePartial = Result<(StateDiffs, Self::InputT), E>;
    type PKeys = PKeys;

    fn input_fetch(
        &self,
        resources: &mut Resources<SetUp>,
    ) -> Result<Self::InputT, ResourceFetchError> {
        let states_ts0 = resources.try_remove::<States<StatesTs0>>()?;
        let states_ts1 = resources.try_remove::<States<StatesTs1>>()?;

        Ok((states_ts0, states_ts1))
    }

    fn input_type_names(&self) -> Vec<String> {
        vec![
            tynm::type_name::<States<StatesTs0>>(),
            tynm::type_name::<States<StatesTs1>>(),
        ]
    }

    fn outcome_acc_init(&self, _input: &Self::InputT) -> Self::OutcomeAcc {
        None
    }

    fn outcome_from_acc(&self, outcome_acc: Self::OutcomeAcc) -> Self::Outcome {
        outcome_acc.unwrap_or_else(|| {
            panic!(
                "Expected `state_diffs_ts0_and_ts1` for diffing to be sent in `DiffCmdBlock::exec`."
            );
        })
    }

    fn outcome_insert(&self, resources: &mut Resources<SetUp>, outcome: Self::Outcome) {
        let (state_diffs, (states_ts0, states_ts1)) = outcome;
        resources.insert(state_diffs);
        resources.insert(states_ts0);
        resources.insert(states_ts1);
    }

    fn outcome_type_names(&self) -> Vec<String> {
        vec![
            tynm::type_name::<StateDiffs>(),
            tynm::type_name::<States<StatesTs0>>(),
            tynm::type_name::<States<StatesTs1>>(),
        ]
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

        let (states_ts0, states_ts1) = input;

        let state_diffs_ts0_ts1 =
            Self::diff_any(flow, params_specs, resources, &states_ts0, &states_ts1)
                .await
                .map(|state_diffs| (state_diffs, (states_ts0, states_ts1)));
        outcomes_tx
            .send(state_diffs_ts0_ts1)
            .expect("Failed to send `state_diffs`.");
    }

    fn outcome_collate(
        &self,
        block_outcome: &mut CmdOutcome<Self::OutcomeAcc, Self::Error>,
        outcome_partial: Self::OutcomePartial,
    ) -> Result<(), Self::Error> {
        let state_diffs_and_ts0_ts1 = outcome_partial?;
        block_outcome.value = Some(state_diffs_and_ts0_ts1);
        Ok(())
    }
}

/// Infers the states to use in diffing, from a `StateTs`.
pub trait DiffCmdBlockStatesTsExt {
    /// Returns the `ApplyFor` this `StatesTs` is meant for.
    fn diff_state_spec() -> DiffStateSpec;
}

impl DiffCmdBlockStatesTsExt for Current {
    fn diff_state_spec() -> DiffStateSpec {
        DiffStateSpec::Current
    }
}

impl DiffCmdBlockStatesTsExt for CurrentStored {
    fn diff_state_spec() -> DiffStateSpec {
        DiffStateSpec::CurrentStored
    }
}
impl DiffCmdBlockStatesTsExt for Goal {
    fn diff_state_spec() -> DiffStateSpec {
        DiffStateSpec::Goal
    }
}
impl DiffCmdBlockStatesTsExt for GoalStored {
    fn diff_state_spec() -> DiffStateSpec {
        DiffStateSpec::GoalStored
    }
}
