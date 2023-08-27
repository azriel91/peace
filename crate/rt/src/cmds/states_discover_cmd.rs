use std::{fmt::Debug, marker::PhantomData};

use futures::future::FutureExt;
use peace_cfg::{FnCtx, ItemId};
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
    CmdIndependence,
};
use peace_resources::{
    internal::StatesMut,
    paths::{FlowDir, StatesCurrentFile, StatesGoalFile},
    resources::ts::SetUp,
    states::{
        ts::{Current, Goal},
        StatesCurrent, StatesGoal,
    },
    type_reg::untagged::BoxDtDisplay,
    Resources,
};
use peace_rt_model::{
    outcomes::CmdOutcome, output::OutputWrite, params::ParamsKeys, Error, ItemBoxed, ItemGraph,
    Storage,
};

use crate::{cmds::CmdBase, BUFFERED_FUTURES_MAX};

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

pub struct StatesDiscoverCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> Debug for StatesDiscoverCmd<E, O, PKeys> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("StatesDiscoverCmd").field(&self.0).finish()
    }
}

impl<E, O, PKeys> StatesDiscoverCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    O: OutputWrite<E>,
    PKeys: ParamsKeys + 'static,
{
    /// Runs [`try_state_current`] for each [`Item`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`], and will be serialized to
    /// `$flow_dir/states_current.yaml`.
    ///
    /// If any `state_current` function needs to read the `State` from a
    /// previous `Item`, it may automatically be referenced using [`Current<T>`]
    /// where `T` us the predecessor's state. Peace will have automatically
    /// inserted it into `Resources`, and the successor should references it
    /// in their [`Data`].
    ///
    /// This function will always serialize states to storage.
    ///
    /// [`Current<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Current.html
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`Item`]: peace_cfg::Item
    /// [`try_state_current`]: peace_cfg::Item::try_state_current
    pub async fn current(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdOutcome<StatesCurrent, E>, E> {
        Self::current_with(&mut cmd_ctx.as_standalone(), true).await
    }

    /// Runs [`try_state_current`] for each [`Item`].
    ///
    /// See [`Self::current`] for full documentation.
    ///
    /// This function exists so that this command can be executed as sub
    /// functionality of another command.
    ///
    /// # Parameters
    ///
    /// * `cmd_independence`: Whether this command is run as a top level
    ///   command, or part of a subcommand.
    /// * `serialize_to_storage`: Whether to write states to storage after
    ///   discovery.
    ///
    /// [`try_state_current`]: peace_cfg::Item::try_state_current
    pub async fn current_with(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
        serialize_to_storage: bool,
    ) -> Result<CmdOutcome<StatesCurrent, E>, E> {
        Self::exec(cmd_independence, DiscoverFor::Current, serialize_to_storage)
            .await
            .map(|cmd_outcome| {
                let CmdOutcome {
                    value: (states_current, _states_goal),
                    errors,
                } = cmd_outcome;

                CmdOutcome {
                    value: states_current,
                    errors,
                }
            })
    }

    /// Runs [`try_state_goal`] for each [`Item`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesGoal`], and will be serialized to
    /// `$flow_dir/states_goal.yaml`.
    ///
    /// If any `state_goal` function needs to read the `State` from a
    /// previous `Item`, it may automatically be referenced using [`Goal<T>`]
    /// where `T` us the predecessor's state. Peace will have automatically
    /// inserted it into `Resources`, and the successor should references it
    /// in their [`Data`].
    ///
    /// This function will always serialize states to storage.
    ///
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`Goal<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Goal.html
    /// [`Item`]: peace_cfg::Item
    /// [`try_state_goal`]: peace_cfg::Item::try_state_goal
    pub async fn goal(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdOutcome<StatesGoal, E>, E> {
        Self::goal_with(&mut cmd_ctx.as_standalone(), true).await
    }

    /// Runs [`try_state_goal`] for each [`Item`].
    ///
    /// See [`Self::goal`] for full documentation.
    ///
    /// This function exists so that this command can be executed as sub
    /// functionality of another command.
    ///
    /// # Parameters
    ///
    /// * `cmd_independence`: Whether this command is run as a top level
    ///   command, or part of a subcommand.
    /// * `serialize_to_storage`: Whether to write states to storage after
    ///   discovery.
    ///
    /// [`try_state_goal`]: peace_cfg::Item::try_state_goal
    pub async fn goal_with(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
        serialize_to_storage: bool,
    ) -> Result<CmdOutcome<StatesGoal, E>, E> {
        Self::exec(cmd_independence, DiscoverFor::Goal, serialize_to_storage)
            .await
            .map(|cmd_outcome| {
                let CmdOutcome {
                    value: (_states_current, states_goal),
                    errors,
                } = cmd_outcome;

                CmdOutcome {
                    value: states_goal,
                    errors,
                }
            })
    }

    /// Runs [`try_state_current`] and [`try_state_goal`]` for each
    /// [`Item`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`] and [`StatesGoal`], and states will be serialized
    /// to `$flow_dir/states_current.yaml` and
    /// `$flow_dir/states_goal.yaml`.
    ///
    /// If any `state_current` function needs to read the `State` from a
    /// previous `Item`, the predecessor should insert a copy / clone of
    /// their state into `Resources`, and the successor should references it
    /// in their [`Data`].
    ///
    /// If any `state_goal` function needs to read the `State` from a
    /// previous `Item`, it may automatically be referenced using
    /// [`Goal<T>`] where `T` us the predecessor's state. Peace will have
    /// automatically inserted it into `Resources`, and the successor should
    /// references it in their [`Data`].
    ///
    /// This function will always serialize states to storage.
    ///
    /// [`Current<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Current.html
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`Goal<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Goal.html
    /// [`Item`]: peace_cfg::Item
    /// [`try_state_current`]: peace_cfg::Item::try_state_current
    /// [`try_state_goal`]: peace_cfg::Item::try_state_goal
    pub async fn current_and_goal(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdOutcome<(StatesCurrent, StatesGoal), E>, E> {
        Self::current_and_goal_with(&mut cmd_ctx.as_standalone(), true).await
    }

    /// Runs [`try_state_current`] and [`try_state_goal`]` for each
    /// [`Item`].
    ///
    /// See [`Self::goal`] for full documentation.
    ///
    /// This function exists so that this command can be executed as sub
    /// functionality of another command.
    ///
    /// # Parameters
    ///
    /// * `cmd_independence`: Whether this command is run as a top level
    ///   command, or part of a subcommand.
    /// * `serialize_to_storage`: Whether to write states to storage after
    ///   discovery.
    ///
    /// [`try_state_current`]: peace_cfg::Item::try_state_current
    /// [`try_state_goal`]: peace_cfg::Item::try_state_goal
    pub async fn current_and_goal_with(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
        serialize_to_storage: bool,
    ) -> Result<CmdOutcome<(StatesCurrent, StatesGoal), E>, E> {
        Self::exec(
            cmd_independence,
            DiscoverFor::CurrentAndGoal,
            serialize_to_storage,
        )
        .await
    }

    /// Discovers current and/or goal states, marking progress bars as
    /// complete when discovery finishes.
    async fn exec(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
        discover_for: DiscoverFor,
        serialize_to_storage: bool,
    ) -> Result<CmdOutcome<(StatesCurrent, StatesGoal), E>, E> {
        let outcome = {
            let states_current_mut = StatesMut::<Current>::new();
            let states_goal_mut = StatesMut::<Goal>::new();

            (states_current_mut, states_goal_mut)
        };

        #[cfg(feature = "output_progress")]
        let is_sub_cmd = matches!(
            cmd_independence,
            CmdIndependence::SubCmd { .. } | CmdIndependence::SubCmdWithProgress { .. }
        );

        let cmd_outcome = CmdBase::<E, O, PKeys>::exec(
            cmd_independence,
            outcome,
            |cmd_view, #[cfg(feature = "output_progress")] progress_tx, outcomes_tx| {
                async move {
                    let SingleProfileSingleFlowView {
                        flow,
                        params_specs,
                        resources,
                        ..
                    } = &*cmd_view;

                    flow.graph()
                        .for_each_concurrent(BUFFERED_FUTURES_MAX, |item| {
                            Self::item_states_discover(
                                discover_for,
                                #[cfg(feature = "output_progress")]
                                is_sub_cmd,
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
                .boxed_local()
            },
            |cmd_outcome, item_discover_outcome| {
                let CmdOutcome {
                    value: (states_current_mut, states_goal_mut),
                    errors,
                } = cmd_outcome;

                match item_discover_outcome {
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
            },
        )
        .await?;

        let cmd_outcome = cmd_outcome.map(|(states_current_mut, states_goal_mut)| {
            let states_current = StatesCurrent::from(states_current_mut);
            let states_goal = StatesGoal::from(states_goal_mut);

            (states_current, states_goal)
        });

        let CmdOutcome {
            value: (states_current, states_goal),
            errors: _,
        } = &cmd_outcome;

        let (item_graph, resources) = match cmd_independence {
            CmdIndependence::Standalone { cmd_ctx } => {
                let SingleProfileSingleFlowView {
                    flow, resources, ..
                } = cmd_ctx.view();
                (flow.graph(), resources)
            }
            CmdIndependence::SubCmd { cmd_view } => {
                let SingleProfileSingleFlowView {
                    flow, resources, ..
                } = cmd_view;
                (flow.graph(), &mut **resources)
            }
            #[cfg(feature = "output_progress")]
            CmdIndependence::SubCmdWithProgress { cmd_view, .. } => {
                let SingleProfileSingleFlowView {
                    flow, resources, ..
                } = cmd_view;
                (flow.graph(), &mut **resources)
            }
        };

        if serialize_to_storage {
            match discover_for {
                DiscoverFor::Current => {
                    Self::serialize_current(item_graph, resources, states_current).await?;
                }
                DiscoverFor::Goal => {
                    Self::serialize_goal(item_graph, resources, states_goal).await?;
                }
                DiscoverFor::CurrentAndGoal => {
                    Self::serialize_current(item_graph, resources, states_current).await?;
                    Self::serialize_goal(item_graph, resources, states_goal).await?;
                }
            }
        }

        Ok(cmd_outcome)
    }

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

    // TODO: This duplicates a bit of code with `ApplyCmd`.
    async fn serialize_current(
        item_graph: &ItemGraph<E>,
        resources: &mut Resources<SetUp>,
        states_current: &StatesCurrent,
    ) -> Result<(), E> {
        use peace_rt_model::StatesSerializer;

        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_current_file = StatesCurrentFile::from(&*flow_dir);

        StatesSerializer::serialize(&storage, item_graph, states_current, &states_current_file)
            .await?;

        drop(flow_dir);
        drop(storage);

        resources.insert(states_current_file);

        Ok(())
    }

    async fn serialize_goal(
        item_graph: &ItemGraph<E>,
        resources: &mut Resources<SetUp>,
        states_goal: &StatesGoal,
    ) -> Result<(), E> {
        use peace_rt_model::StatesSerializer;

        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_goal_file = StatesGoalFile::from(&*flow_dir);

        StatesSerializer::serialize(&storage, item_graph, states_goal, &states_goal_file).await?;

        drop(flow_dir);
        drop(storage);

        resources.insert(states_goal_file);

        Ok(())
    }
}

impl<E, O, PKeys> Default for StatesDiscoverCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
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
    Current,
    /// Discover goal states of each item.
    Goal,
    /// Discover both current and goal states.
    CurrentAndGoal,
}
