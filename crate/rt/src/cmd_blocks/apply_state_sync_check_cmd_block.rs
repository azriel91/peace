use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::scopes::SingleProfileSingleFlowView;
use peace_cmd_rt::{async_trait, CmdBlock};
use peace_resources::{
    resources::ts::SetUp,
    states::{States, StatesCurrent, StatesCurrentStored, StatesGoal, StatesGoalStored},
    Resources,
};
use peace_rt_model::{outcomes::CmdOutcome, params::ParamsKeys, Error};
use peace_rt_model_core::{ApplyCmdError, ItemsStateStoredStale, StateStoredAndDiscovered};
use tokio::sync::mpsc::UnboundedSender;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::{
            progress::{
                ProgressComplete,
                ProgressDelta,
                ProgressMsgUpdate,
                ProgressUpdate,
                ProgressUpdateAndId,
            },
        };
        use tokio::sync::mpsc::Sender;
    }
}

// Whether to block an apply operation if stored states are not in sync with
// discovered state.

/// Neither stored current states nor stored goal state need to be in sync
/// with the discovered current states and goal state.
const APPLY_STORE_STATE_SYNC_NONE: isize = 0;

/// The stored current states must be in sync with the discovered current
/// state for the apply to proceed.
///
/// The stored goal state does not need to be in sync with the discovered
/// goal state.
const APPLY_STORE_STATE_SYNC_CURRENT: isize = 1;

/// The stored goal state must be in sync with the discovered goal
/// state for the apply to proceed.
///
/// The stored current states does not need to be in sync with the
/// discovered current state.
///
/// For `CleanCmd`, this variant is equivalent to `None`.
const APPLY_STORE_STATE_SYNC_GOAL: isize = 2;

/// Both stored current states and stored goal state must be in sync with
/// the discovered current states and goal state for the apply to
/// proceed.
///
/// For `CleanCmd`, this variant is equivalent to `Current`.
const APPLY_STORE_STATE_SYNC_CURRENT_AND_GOAL: isize = 3;

/// Stops a `CmdExecution` if stored states and discovered states are not in
/// sync.
pub struct ApplyStateSyncCheckCmdBlock<E, PKeys, const APPLY_STORE_STATE_SYNC_N: isize>(
    PhantomData<(E, PKeys)>,
);

impl<E, PKeys, const APPLY_STORE_STATE_SYNC_N: isize> Debug
    for ApplyStateSyncCheckCmdBlock<E, PKeys, APPLY_STORE_STATE_SYNC_N>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ApplyStateSyncCheckCmdBlock")
            .field(&self.0)
            .finish()
    }
}

impl<E, PKeys> ApplyStateSyncCheckCmdBlock<E, PKeys, APPLY_STORE_STATE_SYNC_CURRENT>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Returns a block that discovers current states.
    pub fn current() -> Self {
        Self(PhantomData)
    }
}

impl<E, PKeys> ApplyStateSyncCheckCmdBlock<E, PKeys, APPLY_STORE_STATE_SYNC_GOAL>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Returns a block that discovers goal states.
    pub fn goal() -> Self {
        Self(PhantomData)
    }
}

impl<E, PKeys> ApplyStateSyncCheckCmdBlock<E, PKeys, APPLY_STORE_STATE_SYNC_CURRENT_AND_GOAL>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Returns a block that discovers both current and goal states.
    pub fn current_and_goal() -> Self {
        Self(PhantomData)
    }
}

impl<E, PKeys, const APPLY_STORE_STATE_SYNC_N: isize>
    ApplyStateSyncCheckCmdBlock<E, PKeys, APPLY_STORE_STATE_SYNC_N>
where
    PKeys: ParamsKeys + 'static,
{
    fn items_state_stored_stale<StatesTsStored, StatesTs>(
        cmd_view: &SingleProfileSingleFlowView<'_, E, PKeys, SetUp>,
        states_stored: &States<StatesTsStored>,
        states_discovered: &States<StatesTs>,
        #[cfg(feature = "output_progress")] progress_tx: &Sender<ProgressUpdateAndId>,
    ) -> Result<ItemsStateStoredStale, E>
    where
        E: std::error::Error + From<Error> + Send + 'static,
    {
        let items_state_stored_stale = cmd_view.flow.graph().iter_insertion().try_fold(
            ItemsStateStoredStale::new(),
            |mut items_state_stored_stale, item_rt| {
                let item_id = item_rt.id();
                let state_stored = states_stored.get_raw(item_id);
                let state_discovered = states_discovered.get_raw(item_id);

                match (state_stored, state_discovered) {
                    (None, None) => {
                        // Item not discoverable, may be dependent on
                        // predecessor
                    }
                    (None, Some(state_discovered)) => {
                        let item_id = item_id.clone();
                        let state_discovered = state_discovered.clone();
                        items_state_stored_stale.insert(
                            item_id,
                            StateStoredAndDiscovered::OnlyDiscoveredExists { state_discovered },
                        );
                    }
                    (Some(state_stored), None) => {
                        let item_id = item_id.clone();
                        let state_stored = state_stored.clone();
                        items_state_stored_stale.insert(
                            item_id,
                            StateStoredAndDiscovered::OnlyStoredExists { state_stored },
                        );
                    }
                    (Some(state_stored), Some(state_discovered)) => {
                        let state_eq = item_rt.state_eq(state_stored, state_discovered);
                        match state_eq {
                            Ok(true) => {
                                #[cfg(feature = "output_progress")]
                                {
                                    let state_type = tynm::type_name::<StatesTs>();
                                    let _progress_send_unused =
                                        progress_tx.try_send(ProgressUpdateAndId {
                                            item_id: item_id.clone(),
                                            progress_update: ProgressUpdate::Delta(
                                                ProgressDelta::Tick,
                                            ),
                                            msg_update: ProgressMsgUpdate::Set(format!(
                                                "State {state_type} in sync"
                                            )),
                                        });
                                }
                            }
                            Ok(false) => {
                                #[cfg(feature = "output_progress")]
                                {
                                    let state_type = tynm::type_name::<StatesTs>();
                                    let _progress_send_unused =
                                        progress_tx.try_send(ProgressUpdateAndId {
                                            item_id: item_id.clone(),
                                            progress_update: ProgressUpdate::Complete(
                                                ProgressComplete::Fail,
                                            ),
                                            msg_update: ProgressMsgUpdate::Set(format!(
                                                "State {state_type} out of sync"
                                            )),
                                        });
                                }

                                let item_id = item_id.clone();
                                let state_stored = state_stored.clone();
                                let state_discovered = state_discovered.clone();
                                items_state_stored_stale.insert(
                                    item_id,
                                    StateStoredAndDiscovered::ValuesDiffer {
                                        state_stored,
                                        state_discovered,
                                    },
                                );
                            }
                            Err(error) => return Err(error),
                        }
                    }
                }

                Ok(items_state_stored_stale)
            },
        )?;

        Ok(items_state_stored_stale)
    }
}

#[async_trait(?Send)]
impl<E, PKeys> CmdBlock for ApplyStateSyncCheckCmdBlock<E, PKeys, APPLY_STORE_STATE_SYNC_NONE>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    type Error = E;
    type InputT = ();
    type Outcome = Self::InputT;
    type OutcomeAcc = Option<Self::InputT>;
    type OutcomePartial = ApplyStateSyncCheckCmdBlockExecOutcome<E, Self::InputT>;
    type PKeys = PKeys;

    fn input_fetch(&self, _resources: &mut Resources<SetUp>) -> Self::InputT {}

    fn outcome_acc_init(&self, _input: &Self::InputT) -> Self::OutcomeAcc {
        None
    }

    fn outcome_from_acc(&self, _outcome_acc: Self::OutcomeAcc) -> Self::Outcome {}

    fn outcome_insert(&self, _resources: &mut Resources<SetUp>, _outcome: Self::Outcome) {}

    async fn exec(
        &self,
        input: Self::InputT,
        _cmd_view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        outcomes_tx: &UnboundedSender<Self::OutcomePartial>,
        #[cfg(feature = "output_progress")] _progress_tx: &Sender<ProgressUpdateAndId>,
    ) {
        outcomes_tx
            .send(ApplyStateSyncCheckCmdBlockExecOutcome {
                states_stored_and_discovered: input,
                outcome_result: OutcomeResult::Ok,
            })
            .expect("Failed to send `apply_state_sync_check_cmd_block_exec_outcome`.");
    }

    fn outcome_collate(
        &self,
        block_outcome: &mut CmdOutcome<Self::OutcomeAcc, Self::Error>,
        outcome_partial: Self::OutcomePartial,
    ) -> Result<(), Self::Error> {
        outcome_collate(block_outcome, outcome_partial)
    }
}

#[async_trait(?Send)]
impl<E, PKeys> CmdBlock for ApplyStateSyncCheckCmdBlock<E, PKeys, APPLY_STORE_STATE_SYNC_CURRENT>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    type Error = E;
    type InputT = (StatesCurrentStored, StatesCurrent);
    type Outcome = Self::InputT;
    type OutcomeAcc = Option<Self::InputT>;
    type OutcomePartial = ApplyStateSyncCheckCmdBlockExecOutcome<E, Self::InputT>;
    type PKeys = PKeys;

    fn input_fetch(&self, resources: &mut Resources<SetUp>) -> Self::InputT {
        input_fetch_current(resources)
    }

    fn outcome_acc_init(&self, _input: &Self::InputT) -> Self::OutcomeAcc {
        None
    }

    fn outcome_from_acc(&self, outcome_acc: Self::OutcomeAcc) -> Self::Outcome {
        outcome_acc.expect("Expected `outcome_acc` to be set in `exec`.")
    }

    fn outcome_insert(&self, resources: &mut Resources<SetUp>, outcome: Self::Outcome) {
        let (states_current_stored, states_current) = outcome;
        resources.insert(states_current_stored);
        resources.insert(states_current);
    }

    async fn exec(
        &self,
        mut input: Self::InputT,
        cmd_view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        outcomes_tx: &UnboundedSender<Self::OutcomePartial>,
        #[cfg(feature = "output_progress")] progress_tx: &Sender<ProgressUpdateAndId>,
    ) {
        let (states_current_stored, states_current) = &mut input;

        let state_current_stale_result = Self::items_state_stored_stale(
            cmd_view,
            &states_current_stored,
            &states_current,
            #[cfg(feature = "output_progress")]
            progress_tx,
        );
        match state_current_stale_result {
            Ok(items_state_stored_stale) => {
                if items_state_stored_stale.stale() {
                    outcomes_tx
                        .send(ApplyStateSyncCheckCmdBlockExecOutcome {
                            states_stored_and_discovered: input,
                            outcome_result: OutcomeResult::StatesCurrentOutOfSync {
                                items_state_stored_stale,
                            },
                        })
                        .expect("unreachable: `outcomes_rx` is in a sibling task.");
                    return;
                }
            }
            Err(error) => {
                outcomes_tx
                    .send(ApplyStateSyncCheckCmdBlockExecOutcome {
                        states_stored_and_discovered: input,
                        outcome_result: OutcomeResult::StatesDowncastError { error },
                    })
                    .expect("unreachable: `outcomes_rx` is in a sibling task.");
                return;
            }
        };

        outcomes_tx
            .send(ApplyStateSyncCheckCmdBlockExecOutcome {
                states_stored_and_discovered: input,
                outcome_result: OutcomeResult::Ok,
            })
            .expect("unreachable: `outcomes_rx` is in a sibling task.");
    }

    fn outcome_collate(
        &self,
        block_outcome: &mut CmdOutcome<Self::OutcomeAcc, Self::Error>,
        outcome_partial: Self::OutcomePartial,
    ) -> Result<(), Self::Error> {
        outcome_collate(block_outcome, outcome_partial)
    }
}

#[async_trait(?Send)]
impl<E, PKeys> CmdBlock for ApplyStateSyncCheckCmdBlock<E, PKeys, APPLY_STORE_STATE_SYNC_GOAL>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    type Error = E;
    type InputT = (StatesGoalStored, StatesGoal);
    type Outcome = Self::InputT;
    type OutcomeAcc = Option<Self::InputT>;
    type OutcomePartial = ApplyStateSyncCheckCmdBlockExecOutcome<E, Self::InputT>;
    type PKeys = PKeys;

    fn input_fetch(&self, resources: &mut Resources<SetUp>) -> Self::InputT {
        input_fetch_goal(resources)
    }

    fn outcome_acc_init(&self, _input: &Self::InputT) -> Self::OutcomeAcc {
        None
    }

    fn outcome_from_acc(&self, outcome_acc: Self::OutcomeAcc) -> Self::Outcome {
        outcome_acc.expect("Expected `outcome_acc` to be set in `exec`.")
    }

    fn outcome_insert(&self, resources: &mut Resources<SetUp>, outcome: Self::Outcome) {
        let (states_goal_stored, states_goal) = outcome;
        resources.insert(states_goal_stored);
        resources.insert(states_goal);
    }

    async fn exec(
        &self,
        mut input: Self::InputT,
        cmd_view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        outcomes_tx: &UnboundedSender<Self::OutcomePartial>,
        #[cfg(feature = "output_progress")] progress_tx: &Sender<ProgressUpdateAndId>,
    ) {
        let (states_goal_stored, states_goal) = &mut input;

        let state_goal_stale_result = Self::items_state_stored_stale(
            cmd_view,
            &states_goal_stored,
            &states_goal,
            #[cfg(feature = "output_progress")]
            progress_tx,
        );
        match state_goal_stale_result {
            Ok(items_state_stored_stale) => {
                if items_state_stored_stale.stale() {
                    outcomes_tx
                        .send(ApplyStateSyncCheckCmdBlockExecOutcome {
                            states_stored_and_discovered: input,
                            outcome_result: OutcomeResult::StatesGoalOutOfSync {
                                items_state_stored_stale,
                            },
                        })
                        .expect("unreachable: `outcomes_rx` is in a sibling task.");
                    return;
                }
            }
            Err(error) => {
                outcomes_tx
                    .send(ApplyStateSyncCheckCmdBlockExecOutcome {
                        states_stored_and_discovered: input,
                        outcome_result: OutcomeResult::StatesDowncastError { error },
                    })
                    .expect("unreachable: `outcomes_rx` is in a sibling task.");
                return;
            }
        };

        outcomes_tx
            .send(ApplyStateSyncCheckCmdBlockExecOutcome {
                states_stored_and_discovered: input,
                outcome_result: OutcomeResult::Ok,
            })
            .expect("unreachable: `outcomes_rx` is in a sibling task.");
    }

    fn outcome_collate(
        &self,
        block_outcome: &mut CmdOutcome<Self::OutcomeAcc, Self::Error>,
        outcome_partial: Self::OutcomePartial,
    ) -> Result<(), Self::Error> {
        outcome_collate(block_outcome, outcome_partial)
    }
}

#[async_trait(?Send)]
impl<E, PKeys> CmdBlock
    for ApplyStateSyncCheckCmdBlock<E, PKeys, APPLY_STORE_STATE_SYNC_CURRENT_AND_GOAL>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    type Error = E;
    type InputT = (
        StatesCurrentStored,
        StatesCurrent,
        StatesGoalStored,
        StatesGoal,
    );
    type Outcome = Self::InputT;
    type OutcomeAcc = Option<Self::InputT>;
    type OutcomePartial = ApplyStateSyncCheckCmdBlockExecOutcome<E, Self::InputT>;
    type PKeys = PKeys;

    fn input_fetch(&self, resources: &mut Resources<SetUp>) -> Self::InputT {
        let (states_current_stored, states_current) = input_fetch_current(resources);
        let (states_goal_stored, states_goal) = input_fetch_goal(resources);

        (
            states_current_stored,
            states_current,
            states_goal_stored,
            states_goal,
        )
    }

    fn outcome_acc_init(&self, _input: &Self::InputT) -> Self::OutcomeAcc {
        None
    }

    fn outcome_from_acc(&self, outcome_acc: Self::OutcomeAcc) -> Self::Outcome {
        outcome_acc.expect("Expected `outcome_acc` to be set in `exec`.")
    }

    fn outcome_insert(&self, resources: &mut Resources<SetUp>, outcome: Self::Outcome) {
        let (states_current_stored, states_current, states_goal_stored, states_goal) = outcome;
        resources.insert(states_current_stored);
        resources.insert(states_current);
        resources.insert(states_goal_stored);
        resources.insert(states_goal);
    }

    async fn exec(
        &self,
        mut input: Self::InputT,
        cmd_view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        outcomes_tx: &UnboundedSender<Self::OutcomePartial>,
        #[cfg(feature = "output_progress")] progress_tx: &Sender<ProgressUpdateAndId>,
    ) {
        let (states_current_stored, states_current, states_goal_stored, states_goal) = &mut input;

        let state_current_stale_result = Self::items_state_stored_stale(
            cmd_view,
            &states_current_stored,
            &states_current,
            #[cfg(feature = "output_progress")]
            progress_tx,
        );
        match state_current_stale_result {
            Ok(items_state_stored_stale) => {
                if items_state_stored_stale.stale() {
                    outcomes_tx
                        .send(ApplyStateSyncCheckCmdBlockExecOutcome {
                            states_stored_and_discovered: input,
                            outcome_result: OutcomeResult::StatesCurrentOutOfSync {
                                items_state_stored_stale,
                            },
                        })
                        .expect("unreachable: `outcomes_rx` is in a sibling task.");
                    return;
                }
            }
            Err(error) => {
                outcomes_tx
                    .send(ApplyStateSyncCheckCmdBlockExecOutcome {
                        states_stored_and_discovered: input,
                        outcome_result: OutcomeResult::StatesDowncastError { error },
                    })
                    .expect("unreachable: `outcomes_rx` is in a sibling task.");
                return;
            }
        };

        let state_goal_stale_result = Self::items_state_stored_stale(
            cmd_view,
            &states_goal_stored,
            &states_goal,
            #[cfg(feature = "output_progress")]
            progress_tx,
        );
        match state_goal_stale_result {
            Ok(items_state_stored_stale) => {
                if items_state_stored_stale.stale() {
                    outcomes_tx
                        .send(ApplyStateSyncCheckCmdBlockExecOutcome {
                            states_stored_and_discovered: input,
                            outcome_result: OutcomeResult::StatesGoalOutOfSync {
                                items_state_stored_stale,
                            },
                        })
                        .expect("unreachable: `outcomes_rx` is in a sibling task.");
                    return;
                }
            }
            Err(error) => {
                outcomes_tx
                    .send(ApplyStateSyncCheckCmdBlockExecOutcome {
                        states_stored_and_discovered: input,
                        outcome_result: OutcomeResult::StatesDowncastError { error },
                    })
                    .expect("unreachable: `outcomes_rx` is in a sibling task.");
                return;
            }
        };

        outcomes_tx
            .send(ApplyStateSyncCheckCmdBlockExecOutcome {
                states_stored_and_discovered: input,
                outcome_result: OutcomeResult::Ok,
            })
            .expect("unreachable: `outcomes_rx` is in a sibling task.");
    }

    fn outcome_collate(
        &self,
        block_outcome: &mut CmdOutcome<Self::OutcomeAcc, Self::Error>,
        outcome_partial: Self::OutcomePartial,
    ) -> Result<(), Self::Error> {
        outcome_collate(block_outcome, outcome_partial)
    }
}

/// Outcome of apply state sync check execution.
#[derive(Debug)]
pub struct ApplyStateSyncCheckCmdBlockExecOutcome<E, Stateses> {
    /// States compared during the state sync check.
    ///
    /// These will be inserted back into `Resources`.
    states_stored_and_discovered: Stateses,
    /// The actual result to use.
    outcome_result: OutcomeResult<E>,
}

#[derive(Debug)]
enum OutcomeResult<E> {
    /// States that are desired to be in sync are in sync.
    Ok,
    /// Stored current states are not in sync with the actual current state.
    StatesCurrentOutOfSync {
        /// Items whose stored current state is out of sync with the discovered
        /// state.
        items_state_stored_stale: ItemsStateStoredStale,
    },
    /// Stored goal states are not in sync with the actual goal state.
    StatesGoalOutOfSync {
        /// Items whose stored goal state is out of sync with the discovered
        /// state.
        items_state_stored_stale: ItemsStateStoredStale,
    },
    /// Error downcasting a boxed item state to its concrete stype.
    StatesDowncastError {
        /// The error from state downcast.
        error: E,
    },
}

// Use trampolining to decrease compiled code size..

fn input_fetch_current(resources: &mut Resources<SetUp>) -> (StatesCurrentStored, StatesCurrent) {
    let states_current_stored = resources
        .remove::<StatesCurrentStored>()
        .unwrap_or_else(|| {
            let input_type_name = tynm::type_name::<StatesCurrentStored>();
            panic!(
                "Expected `{input_type_name}` to exist in `Resources`.\n\
            Make sure a previous `CmdBlock` has that type as its `Outcome`."
            );
        });
    let states_current = resources.remove::<StatesCurrent>().unwrap_or_else(|| {
        let input_type_name = tynm::type_name::<StatesCurrent>();
        panic!(
            "Expected `{input_type_name}` to exist in `Resources`.\n\
            Make sure a previous `CmdBlock` has that type as its `Outcome`."
        );
    });

    (states_current_stored, states_current)
}

fn input_fetch_goal(resources: &mut Resources<SetUp>) -> (StatesGoalStored, StatesGoal) {
    let states_goal_stored = resources.remove::<StatesGoalStored>().unwrap_or_else(|| {
        let input_type_name = tynm::type_name::<StatesGoalStored>();
        panic!(
            "Expected `{input_type_name}` to exist in `Resources`.\n\
            Make sure a previous `CmdBlock` has that type as its `Outcome`."
        );
    });
    let states_goal = resources.remove::<StatesGoal>().unwrap_or_else(|| {
        let input_type_name = tynm::type_name::<StatesGoal>();
        panic!(
            "Expected `{input_type_name}` to exist in `Resources`.\n\
            Make sure a previous `CmdBlock` has that type as its `Outcome`."
        );
    });

    (states_goal_stored, states_goal)
}

fn outcome_collate<E, InputT>(
    block_outcome: &mut CmdOutcome<Option<InputT>, E>,
    outcome_partial: ApplyStateSyncCheckCmdBlockExecOutcome<E, InputT>,
) -> Result<(), E>
where
    E: std::error::Error + From<Error> + Send + 'static,
{
    let ApplyStateSyncCheckCmdBlockExecOutcome {
        states_stored_and_discovered,
        outcome_result,
    } = outcome_partial;

    block_outcome.value = Some(states_stored_and_discovered);

    match outcome_result {
        OutcomeResult::Ok => Ok(()),
        OutcomeResult::StatesCurrentOutOfSync {
            items_state_stored_stale,
        } => Err(E::from(Error::ApplyCmdError(
            ApplyCmdError::StatesCurrentOutOfSync {
                items_state_stored_stale,
            },
        ))),
        OutcomeResult::StatesGoalOutOfSync {
            items_state_stored_stale,
        } => Err(E::from(Error::ApplyCmdError(
            ApplyCmdError::StatesGoalOutOfSync {
                items_state_stored_stale,
            },
        ))),
        OutcomeResult::StatesDowncastError { error } => Err(error),
    }
}
