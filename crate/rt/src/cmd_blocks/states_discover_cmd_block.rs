use std::{fmt::Debug, marker::PhantomData};

use peace_cfg::{FnCtx, ItemId};
use peace_cmd::scopes::SingleProfileSingleFlowView;
use peace_cmd_rt::{async_trait, CmdBlock};
use peace_resources::{
    internal::StatesMut,
    resources::ts::SetUp,
    states::{
        ts::{Current, Goal},
        States, StatesCurrent, StatesGoal,
    },
    type_reg::untagged::BoxDtDisplay,
    Resources,
};
use peace_rt_model::{outcomes::CmdOutcome, params::ParamsKeys, Error, ItemBoxed};
use tokio::sync::mpsc::UnboundedSender;

use crate::BUFFERED_FUTURES_MAX;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::{
            progress::{
                ProgressComplete,
                ProgressDelta,
                ProgressMsgUpdate,
                ProgressSender,
                ProgressUpdate,
                ProgressUpdateAndId,
            },
        };
        use tokio::sync::mpsc::Sender;
    }
}

const DISCOVER_FOR_CURRENT: isize = 0;
const DISCOVER_FOR_GOAL: isize = 1;
const DISCOVER_FOR_CURRENT_AND_GOAL: isize = 2;

pub struct StatesDiscoverCmdBlock<E, PKeys, const DISCOVER_FOR_N: isize>(PhantomData<(E, PKeys)>);

impl<E, PKeys, const DISCOVER_FOR_N: isize> Debug
    for StatesDiscoverCmdBlock<E, PKeys, DISCOVER_FOR_N>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("StatesDiscoverCmdBlock")
            .field(&self.0)
            .finish()
    }
}

impl<E, PKeys> StatesDiscoverCmdBlock<E, PKeys, DISCOVER_FOR_CURRENT>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Returns a block that discovers current states.
    pub fn current() -> Self {
        Self(PhantomData)
    }
}

impl<E, PKeys> StatesDiscoverCmdBlock<E, PKeys, DISCOVER_FOR_GOAL>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Returns a block that discovers goal states.
    pub fn goal() -> Self {
        Self(PhantomData)
    }
}

impl<E, PKeys> StatesDiscoverCmdBlock<E, PKeys, DISCOVER_FOR_CURRENT_AND_GOAL>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Returns a block that discovers both current and goal states.
    pub fn current_and_goal() -> Self {
        Self(PhantomData)
    }
}

impl<E, PKeys, const DISCOVER_FOR_N: isize> StatesDiscoverCmdBlock<E, PKeys, DISCOVER_FOR_N>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    async fn item_states_discover(
        discover_for: DiscoverFor,
        #[cfg(feature = "output_progress")] is_sub_cmd: bool,
        #[cfg(feature = "output_progress")] progress_tx: &Sender<ProgressUpdateAndId>,
        params_specs: &&peace_params::ParamsSpecs,
        resources: &&mut Resources<SetUp>,
        outcomes_tx: &tokio::sync::mpsc::UnboundedSender<ItemDiscoverOutcome<E>>,
        item: &ItemBoxed<E>,
    ) {
        let item_id = item.id();
        let fn_ctx = FnCtx::new(
            item_id,
            #[cfg(feature = "output_progress")]
            ProgressSender::new(item_id, progress_tx),
        );

        let (state_current_result, state_goal_result) = match discover_for {
            DiscoverFor::Current => {
                let state_current_result = item
                    .state_current_try_exec(params_specs, resources, fn_ctx)
                    .await;

                (Some(state_current_result), None)
            }
            DiscoverFor::Goal => {
                let state_goal_result = item
                    .state_goal_try_exec(params_specs, resources, fn_ctx)
                    .await;

                (None, Some(state_goal_result))
            }
            DiscoverFor::CurrentAndGoal => {
                let state_current_result = item
                    .state_current_try_exec(params_specs, resources, fn_ctx)
                    .await;
                let state_goal_result = item
                    .state_goal_try_exec(params_specs, resources, fn_ctx)
                    .await;

                (Some(state_current_result), Some(state_goal_result))
            }
        };

        // Send progress update.
        #[cfg(feature = "output_progress")]
        Self::discover_progress_update(
            &state_current_result,
            &state_goal_result,
            discover_for,
            is_sub_cmd,
            progress_tx,
            item_id,
        );

        let mut item_error = None;
        let state_current = if let Some(state_current_result) = state_current_result {
            match state_current_result {
                Ok(state_current_opt) => state_current_opt,
                Err(error) => {
                    item_error = Some(error);
                    None
                }
            }
        } else {
            None
        };

        let state_goal = if let Some(state_goal_result) = state_goal_result {
            match state_goal_result {
                Ok(state_goal_opt) => state_goal_opt,
                Err(error) => {
                    // It's probably more crucial to store the
                    // `states_current`
                    // error than the states goal error, if both err.
                    if item_error.is_none() {
                        item_error = Some(error);
                    }
                    None
                }
            }
        } else {
            None
        };

        if let Some(error) = item_error {
            outcomes_tx
                .send(ItemDiscoverOutcome::Fail {
                    item_id: item_id.clone(),
                    state_current,
                    state_goal,
                    error,
                })
                .expect("unreachable: `outcomes_rx` is in a sibling task.");
        } else {
            outcomes_tx
                .send(ItemDiscoverOutcome::Success {
                    item_id: item_id.clone(),
                    state_current,
                    state_goal,
                })
                .expect("unreachable: `outcomes_rx` is in a sibling task.");
        }
    }

    #[cfg(feature = "output_progress")]
    fn discover_progress_update(
        state_current_result: &Option<Result<Option<BoxDtDisplay>, E>>,
        state_goal_result: &Option<Result<Option<BoxDtDisplay>, E>>,
        discover_for: DiscoverFor,
        is_sub_cmd: bool,
        progress_tx: &Sender<ProgressUpdateAndId>,
        item_id: &ItemId,
    ) {
        let state_current_result = state_current_result.as_ref();
        let state_goal_result = state_goal_result.as_ref();
        let (progress_update, msg_update) = match discover_for {
            DiscoverFor::Current => match state_current_result {
                Some(Ok(_)) => {
                    let progress_update = if is_sub_cmd {
                        ProgressUpdate::Delta(ProgressDelta::Tick)
                    } else {
                        ProgressUpdate::Complete(ProgressComplete::Success)
                    };

                    (progress_update, ProgressMsgUpdate::Clear)
                }
                Some(Err(error)) => (
                    ProgressUpdate::Complete(ProgressComplete::Fail),
                    ProgressMsgUpdate::Set(format!("{error}")),
                ),
                None => return,
            },
            DiscoverFor::Goal => match state_goal_result {
                Some(Ok(_)) => {
                    let progress_update = if is_sub_cmd {
                        ProgressUpdate::Delta(ProgressDelta::Tick)
                    } else {
                        ProgressUpdate::Complete(ProgressComplete::Success)
                    };

                    (progress_update, ProgressMsgUpdate::Clear)
                }
                Some(Err(error)) => (
                    ProgressUpdate::Complete(ProgressComplete::Fail),
                    ProgressMsgUpdate::Set(format!("{error}")),
                ),
                None => return,
            },
            DiscoverFor::CurrentAndGoal => match state_current_result.zip(state_goal_result) {
                Some((Ok(_), Ok(_))) => {
                    let progress_update = if is_sub_cmd {
                        ProgressUpdate::Delta(ProgressDelta::Tick)
                    } else {
                        ProgressUpdate::Complete(ProgressComplete::Success)
                    };

                    (progress_update, ProgressMsgUpdate::Clear)
                }
                Some((Ok(_), Err(error)) | (Err(error), Ok(_))) => (
                    ProgressUpdate::Complete(ProgressComplete::Fail),
                    ProgressMsgUpdate::Set(format!("{error}")),
                ),
                Some((Err(error_current), Err(error_goal))) => (
                    ProgressUpdate::Complete(ProgressComplete::Fail),
                    ProgressMsgUpdate::Set(format!("{error_current}, {error_goal}")),
                ),
                None => return,
            },
        };
        let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
            item_id: item_id.clone(),
            progress_update,
            msg_update,
        });
    }
}

#[derive(Debug)]
pub enum ItemDiscoverOutcome<E> {
    /// Discover succeeded.
    Success {
        item_id: ItemId,
        state_current: Option<BoxDtDisplay>,
        state_goal: Option<BoxDtDisplay>,
    },
    /// Discover failed.
    Fail {
        item_id: ItemId,
        state_current: Option<BoxDtDisplay>,
        state_goal: Option<BoxDtDisplay>,
        error: E,
    },
}

/// Which states to discover.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum DiscoverFor {
    /// Discover current states of each item.
    Current = DISCOVER_FOR_CURRENT,
    /// Discover goal states of each item.
    Goal = DISCOVER_FOR_GOAL,
    /// Discover both current and goal states.
    CurrentAndGoal = DISCOVER_FOR_CURRENT_AND_GOAL,
}

#[async_trait(?Send)]
impl<E, PKeys> CmdBlock for StatesDiscoverCmdBlock<E, PKeys, DISCOVER_FOR_CURRENT>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    type Error = E;
    type InputT = ();
    type Outcome = States<Current>;
    type OutcomeAcc = StatesMut<Current>;
    type OutcomePartial = ItemDiscoverOutcome<E>;
    type PKeys = PKeys;

    fn outcome_acc_init(&self) -> Self::OutcomeAcc {
        StatesMut::<Current>::new()
    }

    fn outcome_from_acc(&self, outcome_acc: Self::OutcomeAcc) -> Self::Outcome {
        let states_current_mut = outcome_acc;
        StatesCurrent::from(states_current_mut)
    }

    async fn exec(
        &self,
        _input: Box<Self::InputT>,
        cmd_view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        outcomes_tx: &UnboundedSender<Self::OutcomePartial>,
        #[cfg(feature = "output_progress")] progress_tx: &Sender<ProgressUpdateAndId>,
    ) {
        let SingleProfileSingleFlowView {
            flow,
            params_specs,
            resources,
            ..
        } = &*cmd_view;

        flow.graph()
            .for_each_concurrent(BUFFERED_FUTURES_MAX, |item| {
                Self::item_states_discover(
                    DiscoverFor::Current,
                    #[cfg(feature = "output_progress")]
                    true,
                    #[cfg(feature = "output_progress")]
                    progress_tx,
                    params_specs,
                    resources,
                    outcomes_tx,
                    item,
                )
            })
            .await;
    }

    fn outcome_collate(
        &self,
        block_outcome: &mut CmdOutcome<Self::OutcomeAcc, Self::Error>,
        outcome_partial: Self::OutcomePartial,
    ) -> Result<(), Self::Error> {
        let CmdOutcome {
            value: states_current_mut,
            errors,
        } = block_outcome;

        match outcome_partial {
            ItemDiscoverOutcome::Success {
                item_id,
                state_current,
                state_goal: _,
            } => {
                if let Some(state_current) = state_current {
                    states_current_mut.insert_raw(item_id.clone(), state_current);
                }
            }
            ItemDiscoverOutcome::Fail {
                item_id,
                state_current,
                state_goal: _,
                error,
            } => {
                errors.insert(item_id.clone(), error);

                if let Some(state_current) = state_current {
                    states_current_mut.insert_raw(item_id.clone(), state_current);
                }
            }
        }

        Ok(())
    }
}

#[async_trait(?Send)]
impl<E, PKeys> CmdBlock for StatesDiscoverCmdBlock<E, PKeys, DISCOVER_FOR_GOAL>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    type Error = E;
    type InputT = ();
    type Outcome = States<Goal>;
    type OutcomeAcc = StatesMut<Goal>;
    type OutcomePartial = ItemDiscoverOutcome<E>;
    type PKeys = PKeys;

    fn outcome_acc_init(&self) -> Self::OutcomeAcc {
        StatesMut::<Goal>::new()
    }

    fn outcome_from_acc(&self, outcome_acc: Self::OutcomeAcc) -> Self::Outcome {
        let states_goal_mut = outcome_acc;
        StatesGoal::from(states_goal_mut)
    }

    async fn exec(
        &self,
        _input: Box<Self::InputT>,
        cmd_view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        outcomes_tx: &UnboundedSender<Self::OutcomePartial>,
        #[cfg(feature = "output_progress")] progress_tx: &Sender<ProgressUpdateAndId>,
    ) {
        let SingleProfileSingleFlowView {
            flow,
            params_specs,
            resources,
            ..
        } = &*cmd_view;

        flow.graph()
            .for_each_concurrent(BUFFERED_FUTURES_MAX, |item| {
                Self::item_states_discover(
                    DiscoverFor::Goal,
                    #[cfg(feature = "output_progress")]
                    true,
                    #[cfg(feature = "output_progress")]
                    progress_tx,
                    params_specs,
                    resources,
                    outcomes_tx,
                    item,
                )
            })
            .await;
    }

    fn outcome_collate(
        &self,
        block_outcome: &mut CmdOutcome<Self::OutcomeAcc, Self::Error>,
        outcome_partial: Self::OutcomePartial,
    ) -> Result<(), Self::Error> {
        let CmdOutcome {
            value: states_goal_mut,
            errors,
        } = block_outcome;

        match outcome_partial {
            ItemDiscoverOutcome::Success {
                item_id,
                state_current: _,
                state_goal,
            } => {
                if let Some(state_goal) = state_goal {
                    states_goal_mut.insert_raw(item_id, state_goal);
                }
            }
            ItemDiscoverOutcome::Fail {
                item_id,
                state_current: _,
                state_goal,
                error,
            } => {
                errors.insert(item_id.clone(), error);

                if let Some(state_goal) = state_goal {
                    states_goal_mut.insert_raw(item_id, state_goal);
                }
            }
        }

        Ok(())
    }
}

#[async_trait(?Send)]
impl<E, PKeys> CmdBlock for StatesDiscoverCmdBlock<E, PKeys, DISCOVER_FOR_CURRENT_AND_GOAL>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    type Error = E;
    type InputT = ();
    type Outcome = (States<Current>, States<Goal>);
    type OutcomeAcc = (StatesMut<Current>, StatesMut<Goal>);
    type OutcomePartial = ItemDiscoverOutcome<E>;
    type PKeys = PKeys;

    fn outcome_acc_init(&self) -> Self::OutcomeAcc {
        let states_current_mut = StatesMut::<Current>::new();
        let states_goal_mut = StatesMut::<Goal>::new();
        (states_current_mut, states_goal_mut)
    }

    fn outcome_from_acc(&self, outcome_acc: Self::OutcomeAcc) -> Self::Outcome {
        let (states_current_mut, states_goal_mut) = outcome_acc;
        (
            StatesCurrent::from(states_current_mut),
            StatesGoal::from(states_goal_mut),
        )
    }

    async fn exec(
        &self,
        _input: Box<Self::InputT>,
        cmd_view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        outcomes_tx: &UnboundedSender<Self::OutcomePartial>,
        #[cfg(feature = "output_progress")] progress_tx: &Sender<ProgressUpdateAndId>,
    ) {
        let SingleProfileSingleFlowView {
            flow,
            params_specs,
            resources,
            ..
        } = &*cmd_view;

        flow.graph()
            .for_each_concurrent(BUFFERED_FUTURES_MAX, |item| {
                Self::item_states_discover(
                    DiscoverFor::CurrentAndGoal,
                    #[cfg(feature = "output_progress")]
                    true,
                    #[cfg(feature = "output_progress")]
                    progress_tx,
                    params_specs,
                    resources,
                    outcomes_tx,
                    item,
                )
            })
            .await;
    }

    fn outcome_collate(
        &self,
        block_outcome: &mut CmdOutcome<Self::OutcomeAcc, Self::Error>,
        outcome_partial: Self::OutcomePartial,
    ) -> Result<(), Self::Error> {
        let CmdOutcome {
            value: (states_current_mut, states_goal_mut),
            errors,
        } = block_outcome;

        match outcome_partial {
            ItemDiscoverOutcome::Success {
                item_id,
                state_current,
                state_goal,
            } => {
                if let Some(state_current) = state_current {
                    states_current_mut.insert_raw(item_id.clone(), state_current);
                }
                if let Some(state_goal) = state_goal {
                    states_goal_mut.insert_raw(item_id, state_goal);
                }
            }
            ItemDiscoverOutcome::Fail {
                item_id,
                state_current,
                state_goal,
                error,
            } => {
                errors.insert(item_id.clone(), error);

                if let Some(state_current) = state_current {
                    states_current_mut.insert_raw(item_id.clone(), state_current);
                }
                if let Some(state_goal) = state_goal {
                    states_goal_mut.insert_raw(item_id, state_goal);
                }
            }
        }

        Ok(())
    }
}
