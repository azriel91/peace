use std::{fmt::Debug, marker::PhantomData};

use peace_cfg::{FnCtx, ItemSpecId};
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    internal::StatesMut,
    paths::{FlowDir, StatesDesiredFile, StatesSavedFile},
    resources::ts::SetUp,
    states::{
        ts::{Current, Desired},
        StatesCurrent, StatesDesired,
    },
    type_reg::untagged::BoxDtDisplay,
    Resources,
};
use peace_rt_model::{output::OutputWrite, params::ParamsKeys, Error, IndexMap, Storage};
use tokio::sync::mpsc;

use crate::BUFFERED_FUTURES_MAX;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::{
            progress::{
                ProgressComplete,
                ProgressMsgUpdate,
                ProgressSender,
                ProgressUpdate,
                ProgressUpdateAndId,
            },
        };
        use peace_rt_model::CmdProgressTracker;
    }
}

#[derive(Debug)]
pub struct StatesDiscoverCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> StatesDiscoverCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    O: OutputWrite<E>,
    PKeys: ParamsKeys + 'static,
{
    /// Runs [`try_state_current`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`], and will be serialized to
    /// `$flow_dir/states_saved.yaml`.
    ///
    /// If any `state_current` function needs to read the `State` from a
    /// previous `ItemSpec`, it may automatically be referenced using
    /// [`Current<T>`] where `T` us the predecessor's state. Peace will have
    /// automatically inserted it into `Resources`, and the successor should
    /// references it in their [`Data`].
    ///
    /// [`Current<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Current.html
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`try_state_current`]: peace_cfg::ItemSpec::try_state_current
    pub async fn current(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StatesCurrent, E> {
        Self::exec(cmd_ctx, DiscoverFor::Current)
            .await
            .map(|(states_current, _states_desired)| states_current)
    }

    /// Runs [`try_state_desired`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesDesired`], and will be serialized to
    /// `$flow_dir/states_desired.yaml`.
    ///
    /// If any `state_desired` function needs to read the `State` from a
    /// previous `ItemSpec`, it may automatically be referenced using
    /// [`Desired<T>`] where `T` us the predecessor's state. Peace will have
    /// automatically inserted it into `Resources`, and the successor should
    /// references it in their [`Data`].
    ///
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`Desired<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Desired.html
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`try_state_desired`]: peace_cfg::ItemSpec::try_state_desired
    pub async fn desired(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StatesDesired, E> {
        Self::exec(cmd_ctx, DiscoverFor::Desired)
            .await
            .map(|(_states_current, states_desired)| states_desired)
    }

    /// Runs [`try_state_current`] and [`try_state_desired`]` for each
    /// [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`] and [`StatesDesired`], and states will be serialized
    /// to `$flow_dir/states_saved.yaml` and
    /// `$flow_dir/states_desired.yaml`.
    ///
    /// If any `state_current` function needs to read the `State` from a
    /// previous `ItemSpec`, the predecessor should insert a copy / clone of
    /// their state into `Resources`, and the successor should references it
    /// in their [`Data`].
    ///
    /// If any `state_desired` function needs to read the `State` from a
    /// previous `ItemSpec`, it may automatically be referenced using
    /// [`Desired<T>`] where `T` us the predecessor's state. Peace will have
    /// automatically inserted it into `Resources`, and the successor should
    /// references it in their [`Data`].
    ///
    /// [`Current<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Current.html
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`Desired<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Desired.html
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`try_state_current`]: peace_cfg::ItemSpec::try_state_current
    /// [`try_state_desired`]: peace_cfg::ItemSpec::try_state_desired
    pub async fn current_and_desired(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<(StatesCurrent, StatesDesired), E> {
        Self::exec(cmd_ctx, DiscoverFor::CurrentAndDesired).await
    }

    /// Actual logic to discover current and/or desired states.
    async fn exec(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        discover_for: DiscoverFor,
    ) -> Result<(StatesCurrent, StatesDesired), E> {
        let SingleProfileSingleFlowView {
            #[cfg(feature = "output_progress")]
            output,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            flow,
            resources,
            ..
        } = cmd_ctx.scope_mut().view();

        cfg_if::cfg_if! {
            if #[cfg(feature = "output_progress")] {
                output.progress_begin(cmd_progress_tracker).await;

                let CmdProgressTracker {
                    multi_progress: _,
                    progress_trackers,
                } = cmd_progress_tracker;

                let (progress_tx, progress_rx) =
                    mpsc::channel::<ProgressUpdateAndId>(crate::PROGRESS_COUNT_MAX);
            }
        }

        let (outcomes_tx, mut outcomes_rx) = mpsc::unbounded_channel::<ItemDiscoverOutcome<E>>();

        let resources_ref = &*resources;
        let execution_task = async move {
            #[cfg(feature = "output_progress")]
            let progress_tx = &progress_tx;
            let outcomes_tx = &outcomes_tx;

            flow.graph()
                .for_each_concurrent(BUFFERED_FUTURES_MAX, |item_spec| async move {
                    let item_spec_id = item_spec.id();
                    let fn_ctx = FnCtx::new(
                        item_spec_id,
                        #[cfg(feature = "output_progress")]
                        ProgressSender::new(item_spec_id, progress_tx),
                    );

                    let (state_current_result, state_desired_result) = match discover_for {
                        DiscoverFor::Current => {
                            let state_current_result = item_spec
                                .state_current_try_exec(fn_ctx, resources_ref)
                                .await;

                            (Some(state_current_result), None)
                        }
                        DiscoverFor::Desired => {
                            let state_desired_result = item_spec
                                .state_desired_try_exec(fn_ctx, resources_ref)
                                .await;

                            (None, Some(state_desired_result))
                        }
                        DiscoverFor::CurrentAndDesired => {
                            let state_current_result = item_spec
                                .state_current_try_exec(fn_ctx, resources_ref)
                                .await;
                            let state_desired_result = item_spec
                                .state_desired_try_exec(fn_ctx, resources_ref)
                                .await;

                            (Some(state_current_result), Some(state_desired_result))
                        }
                    };

                    let state_current = if let Some(state_current_result) = state_current_result {
                        #[cfg(feature = "output_progress")]
                        {
                            let (progress_complete, msg_update) = match &state_current_result {
                                Ok(_) => (ProgressComplete::Success, ProgressMsgUpdate::Clear),
                                Err(error) => (
                                    ProgressComplete::Fail,
                                    ProgressMsgUpdate::Set(format!("{error}")),
                                ),
                            };

                            let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                                item_spec_id: item_spec_id.clone(),
                                progress_update: ProgressUpdate::Complete(progress_complete),
                                msg_update,
                            });
                        }

                        match state_current_result {
                            Ok(state_current_opt) => state_current_opt,
                            Err(error) => {
                                outcomes_tx
                                    .send(ItemDiscoverOutcome::Fail {
                                        item_spec_id: item_spec_id.clone(),
                                        state_current: None,
                                        state_desired: None,
                                        error,
                                    })
                                    .expect("unreachable: `outcomes_rx` is in a sibling task.");
                                return; // short circuit
                            }
                        }
                    } else {
                        None
                    };

                    let state_desired = if let Some(state_desired_result) = state_desired_result {
                        #[cfg(feature = "output_progress")]
                        {
                            let (progress_complete, msg_update) = match &state_desired_result {
                                Ok(_) => (ProgressComplete::Success, ProgressMsgUpdate::Clear),
                                Err(error) => (
                                    ProgressComplete::Fail,
                                    ProgressMsgUpdate::Set(format!("{error}")),
                                ),
                            };

                            let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                                item_spec_id: item_spec_id.clone(),
                                progress_update: ProgressUpdate::Complete(progress_complete),
                                msg_update,
                            });
                        }

                        match state_desired_result {
                            Ok(state_desired_opt) => state_desired_opt,
                            Err(error) => {
                                outcomes_tx
                                    .send(ItemDiscoverOutcome::Fail {
                                        item_spec_id: item_spec_id.clone(),
                                        state_current,
                                        state_desired: None,
                                        error,
                                    })
                                    .expect("unreachable: `outcomes_rx` is in a sibling task.");
                                return; // short circuit
                            }
                        }
                    } else {
                        None
                    };

                    outcomes_tx
                        .send(ItemDiscoverOutcome::Success {
                            item_spec_id: item_spec_id.clone(),
                            state_current,
                            state_desired,
                        })
                        .expect("unreachable: `outcomes_rx` is in a sibling task.");
                })
                .await;

            // `progress_tx` is dropped here, so `progress_rx` will safely end.
        };

        #[cfg(feature = "output_progress")]
        let progress_render_task =
            crate::progress::Progress::progress_render(output, progress_trackers, progress_rx);

        let mut errors = IndexMap::<ItemSpecId, E>::new();
        let outcomes_rx_task = async {
            let mut states_current_mut = StatesMut::<Current>::new();
            let mut states_desired_mut = StatesMut::<Desired>::new();

            while let Some(outcome) = outcomes_rx.recv().await {
                match outcome {
                    ItemDiscoverOutcome::Success {
                        item_spec_id,
                        state_current,
                        state_desired,
                    } => {
                        if let Some(state_current) = state_current {
                            states_current_mut.insert_raw(item_spec_id.clone(), state_current);
                        }
                        if let Some(state_desired) = state_desired {
                            states_desired_mut.insert_raw(item_spec_id, state_desired);
                        }
                    }
                    ItemDiscoverOutcome::Fail {
                        item_spec_id,
                        state_current,
                        state_desired,
                        error,
                    } => {
                        errors.insert(item_spec_id.clone(), error);

                        if let Some(state_current) = state_current {
                            states_current_mut.insert_raw(item_spec_id.clone(), state_current);
                        }
                        if let Some(state_desired) = state_desired {
                            states_desired_mut.insert_raw(item_spec_id, state_desired);
                        }
                    }
                }
            }

            let states_current = StatesCurrent::from(states_current_mut);
            let states_desired = StatesDesired::from(states_desired_mut);

            (states_current, states_desired)
        };

        cfg_if::cfg_if! {
            if #[cfg(feature = "output_progress")] {
                let ((), _progress_result, outcomes_result) = futures::join!(execution_task, progress_render_task, outcomes_rx_task);

                output.progress_end(cmd_progress_tracker).await;
            } else {
                let ((), outcomes_result) = futures::join!(execution_task, outcomes_rx_task);
            }
        }
        let (states_current, states_desired) = outcomes_result;

        match discover_for {
            DiscoverFor::Current => {
                Self::serialize_current(resources, &states_current).await?;
            }
            DiscoverFor::Desired => {
                Self::serialize_desired(resources, &states_desired).await?;
            }
            DiscoverFor::CurrentAndDesired => {
                Self::serialize_current(resources, &states_current).await?;
                Self::serialize_desired(resources, &states_desired).await?;
            }
        }

        Ok((states_current, states_desired))
    }

    // TODO: This duplicates a bit of code with `ApplyCmd`.
    async fn serialize_current(
        resources: &mut Resources<SetUp>,
        states_current: &StatesCurrent,
    ) -> Result<(), E> {
        use peace_rt_model::StatesSerializer;

        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_saved_file = StatesSavedFile::from(&*flow_dir);

        StatesSerializer::serialize(&storage, states_current, &states_saved_file).await?;

        drop(flow_dir);
        drop(storage);

        resources.insert(states_saved_file);

        Ok(())
    }

    async fn serialize_desired(
        resources: &mut Resources<SetUp>,
        states_desired: &StatesDesired,
    ) -> Result<(), E> {
        use peace_rt_model::StatesSerializer;

        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_desired_file = StatesDesiredFile::from(&*flow_dir);

        StatesSerializer::serialize(&storage, states_desired, &states_desired_file).await?;

        drop(flow_dir);
        drop(storage);

        resources.insert(states_desired_file);

        Ok(())
    }
}

impl<E, O, PKeys> Default for StatesDiscoverCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[derive(Debug)]
enum ItemDiscoverOutcome<E> {
    /// Discover succeeded.
    Success {
        item_spec_id: ItemSpecId,
        state_current: Option<BoxDtDisplay>,
        state_desired: Option<BoxDtDisplay>,
    },
    /// Discover failed.
    Fail {
        item_spec_id: ItemSpecId,
        state_current: Option<BoxDtDisplay>,
        state_desired: Option<BoxDtDisplay>,
        error: E,
    },
}

/// Which states to discover.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum DiscoverFor {
    /// Discover current states of each item.
    Current,
    /// Discover desired states of each item.
    Desired,
    /// Discover both current and desired states.
    CurrentAndDesired,
}
