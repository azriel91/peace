use std::{fmt::Debug, marker::PhantomData};

use futures::FutureExt;
use peace_cfg::{FnCtx, ItemId};
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
use peace_rt_model::{
    outcomes::CmdOutcome, output::OutputWrite, params::ParamsKeys, Error, Storage,
};

use crate::{
    cmds::{cmd_ctx_internal::CmdIndependence, CmdBase},
    BUFFERED_FUTURES_MAX,
};

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
    /// Runs [`try_state_current`] for each [`Item`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`], and will be serialized to
    /// `$flow_dir/states_saved.yaml`.
    ///
    /// If any `state_current` function needs to read the `State` from a
    /// previous `Item`, it may automatically be referenced using [`Current<T>`]
    /// where `T` us the predecessor's state. Peace will have automatically
    /// inserted it into `Resources`, and the successor should references it
    /// in their [`Data`].
    ///
    /// [`Current<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Current.html
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`Item`]: peace_cfg::Item
    /// [`try_state_current`]: peace_cfg::Item::try_state_current
    pub async fn current(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdOutcome<StatesCurrent, E>, E> {
        Self::current_with(&mut CmdIndependence::Standalone { cmd_ctx }).await
    }

    /// Runs [`try_state_current`] for each [`Item`].
    ///
    /// See [`Self::current`] for full documentation.
    ///
    /// This function exists so that this command can be executed as sub
    /// functionality of another command.
    ///
    /// [`try_state_current`]: peace_cfg::Item::try_state_current
    pub async fn current_with(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
    ) -> Result<CmdOutcome<StatesCurrent, E>, E> {
        Self::exec(cmd_independence, DiscoverFor::Current)
            .await
            .map(|cmd_outcome| {
                let CmdOutcome {
                    value: (states_current, _states_desired),
                    errors,
                } = cmd_outcome;

                CmdOutcome {
                    value: states_current,
                    errors,
                }
            })
    }

    /// Runs [`try_state_desired`] for each [`Item`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesDesired`], and will be serialized to
    /// `$flow_dir/states_desired.yaml`.
    ///
    /// If any `state_desired` function needs to read the `State` from a
    /// previous `Item`, it may automatically be referenced using [`Desired<T>`]
    /// where `T` us the predecessor's state. Peace will have automatically
    /// inserted it into `Resources`, and the successor should references it
    /// in their [`Data`].
    ///
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`Desired<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Desired.html
    /// [`Item`]: peace_cfg::Item
    /// [`try_state_desired`]: peace_cfg::Item::try_state_desired
    pub async fn desired(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdOutcome<StatesDesired, E>, E> {
        Self::desired_with(&mut CmdIndependence::Standalone { cmd_ctx }).await
    }

    /// Runs [`try_state_desired`] for each [`Item`].
    ///
    /// See [`Self::desired`] for full documentation.
    ///
    /// This function exists so that this command can be executed as sub
    /// functionality of another command.
    ///
    /// [`try_state_desired`]: peace_cfg::Item::try_state_desired
    pub async fn desired_with(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
    ) -> Result<CmdOutcome<StatesDesired, E>, E> {
        Self::exec(cmd_independence, DiscoverFor::Desired)
            .await
            .map(|cmd_outcome| {
                let CmdOutcome {
                    value: (_states_current, states_desired),
                    errors,
                } = cmd_outcome;

                CmdOutcome {
                    value: states_desired,
                    errors,
                }
            })
    }

    /// Runs [`try_state_current`] and [`try_state_desired`]` for each
    /// [`Item`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`] and [`StatesDesired`], and states will be serialized
    /// to `$flow_dir/states_saved.yaml` and
    /// `$flow_dir/states_desired.yaml`.
    ///
    /// If any `state_current` function needs to read the `State` from a
    /// previous `Item`, the predecessor should insert a copy / clone of
    /// their state into `Resources`, and the successor should references it
    /// in their [`Data`].
    ///
    /// If any `state_desired` function needs to read the `State` from a
    /// previous `Item`, it may automatically be referenced using
    /// [`Desired<T>`] where `T` us the predecessor's state. Peace will have
    /// automatically inserted it into `Resources`, and the successor should
    /// references it in their [`Data`].
    ///
    /// [`Current<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Current.html
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`Desired<T>`]: https://docs.rs/peace_data/latest/peace_data/marker/struct.Desired.html
    /// [`Item`]: peace_cfg::Item
    /// [`try_state_current`]: peace_cfg::Item::try_state_current
    /// [`try_state_desired`]: peace_cfg::Item::try_state_desired
    pub async fn current_and_desired(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdOutcome<(StatesCurrent, StatesDesired), E>, E> {
        Self::exec(
            &mut CmdIndependence::Standalone { cmd_ctx },
            DiscoverFor::CurrentAndDesired,
        )
        .await
    }

    /// Discovers current and/or desired states, marking progress bars as
    /// complete when discovery finishes.
    async fn exec(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
        discover_for: DiscoverFor,
    ) -> Result<CmdOutcome<(StatesCurrent, StatesDesired), E>, E> {
        let outcome = {
            let states_current_mut = StatesMut::<Current>::new();
            let states_desired_mut = StatesMut::<Desired>::new();

            (states_current_mut, states_desired_mut)
        };

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
                        .for_each_concurrent(BUFFERED_FUTURES_MAX, |item| async move {
                            let item_id = item.id();
                            let fn_ctx = FnCtx::new(
                                item_id,
                                #[cfg(feature = "output_progress")]
                                ProgressSender::new(item_id, progress_tx),
                            );

                            let (state_current_result, state_desired_result) = match discover_for {
                                DiscoverFor::Current => {
                                    let state_current_result = item
                                        .state_current_try_exec(params_specs, resources, fn_ctx)
                                        .await;

                                    (Some(state_current_result), None)
                                }
                                DiscoverFor::Desired => {
                                    let state_desired_result = item
                                        .state_desired_try_exec(params_specs, resources, fn_ctx)
                                        .await;

                                    (None, Some(state_desired_result))
                                }
                                DiscoverFor::CurrentAndDesired => {
                                    let state_current_result = item
                                        .state_current_try_exec(params_specs, resources, fn_ctx)
                                        .await;
                                    let state_desired_result = item
                                        .state_desired_try_exec(params_specs, resources, fn_ctx)
                                        .await;

                                    (Some(state_current_result), Some(state_desired_result))
                                }
                            };

                            let state_current =
                                if let Some(state_current_result) = state_current_result {
                                    #[cfg(feature = "output_progress")]
                                    {
                                        let (progress_complete, msg_update) =
                                            match &state_current_result {
                                                Ok(_) => (
                                                    ProgressComplete::Success,
                                                    ProgressMsgUpdate::Clear,
                                                ),
                                                Err(error) => (
                                                    ProgressComplete::Fail,
                                                    ProgressMsgUpdate::Set(format!("{error}")),
                                                ),
                                            };

                                        let _progress_send_unused =
                                            progress_tx.try_send(ProgressUpdateAndId {
                                                item_id: item_id.clone(),
                                                progress_update: ProgressUpdate::Complete(
                                                    progress_complete,
                                                ),
                                                msg_update,
                                            });
                                    }

                                    match state_current_result {
                                        Ok(state_current_opt) => state_current_opt,
                                        Err(error) => {
                                            outcomes_tx
                                        .send(ItemDiscoverOutcome::Fail {
                                            item_id: item_id.clone(),
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

                            let state_desired =
                                if let Some(state_desired_result) = state_desired_result {
                                    #[cfg(feature = "output_progress")]
                                    {
                                        let (progress_complete, msg_update) =
                                            match &state_desired_result {
                                                Ok(_) => (
                                                    ProgressComplete::Success,
                                                    ProgressMsgUpdate::Clear,
                                                ),
                                                Err(error) => (
                                                    ProgressComplete::Fail,
                                                    ProgressMsgUpdate::Set(format!("{error}")),
                                                ),
                                            };

                                        let _progress_send_unused =
                                            progress_tx.try_send(ProgressUpdateAndId {
                                                item_id: item_id.clone(),
                                                progress_update: ProgressUpdate::Complete(
                                                    progress_complete,
                                                ),
                                                msg_update,
                                            });
                                    }

                                    match state_desired_result {
                                        Ok(state_desired_opt) => state_desired_opt,
                                        Err(error) => {
                                            outcomes_tx
                                        .send(ItemDiscoverOutcome::Fail {
                                            item_id: item_id.clone(),
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
                                    item_id: item_id.clone(),
                                    state_current,
                                    state_desired,
                                })
                                .expect("unreachable: `outcomes_rx` is in a sibling task.");
                        })
                        .await;
                }
                .boxed_local()
            },
            |cmd_outcome, item_discover_outcome| {
                let CmdOutcome {
                    value: (states_current_mut, states_desired_mut),
                    errors,
                } = cmd_outcome;

                match item_discover_outcome {
                    ItemDiscoverOutcome::Success {
                        item_id,
                        state_current,
                        state_desired,
                    } => {
                        if let Some(state_current) = state_current {
                            states_current_mut.insert_raw(item_id.clone(), state_current);
                        }
                        if let Some(state_desired) = state_desired {
                            states_desired_mut.insert_raw(item_id, state_desired);
                        }
                    }
                    ItemDiscoverOutcome::Fail {
                        item_id,
                        state_current,
                        state_desired,
                        error,
                    } => {
                        errors.insert(item_id.clone(), error);

                        if let Some(state_current) = state_current {
                            states_current_mut.insert_raw(item_id.clone(), state_current);
                        }
                        if let Some(state_desired) = state_desired {
                            states_desired_mut.insert_raw(item_id, state_desired);
                        }
                    }
                }

                Ok(())
            },
        )
        .await?;

        let cmd_outcome = cmd_outcome.map(|(states_current_mut, states_desired_mut)| {
            let states_current = StatesCurrent::from(states_current_mut);
            let states_desired = StatesDesired::from(states_desired_mut);

            (states_current, states_desired)
        });

        let CmdOutcome {
            value: (states_current, states_desired),
            errors: _,
        } = &cmd_outcome;

        let resources = match cmd_independence {
            CmdIndependence::Standalone { cmd_ctx } => cmd_ctx.view().resources,
            CmdIndependence::SubCmd { cmd_view } => cmd_view.resources,
            #[cfg(feature = "output_progress")]
            CmdIndependence::SubCmdWithProgress { cmd_view, .. } => cmd_view.resources,
        };

        match discover_for {
            DiscoverFor::Current => {
                Self::serialize_current(resources, states_current).await?;
            }
            DiscoverFor::Desired => {
                Self::serialize_desired(resources, states_desired).await?;
            }
            DiscoverFor::CurrentAndDesired => {
                Self::serialize_current(resources, states_current).await?;
                Self::serialize_desired(resources, states_desired).await?;
            }
        }

        Ok(cmd_outcome)
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
        item_id: ItemId,
        state_current: Option<BoxDtDisplay>,
        state_desired: Option<BoxDtDisplay>,
    },
    /// Discover failed.
    Fail {
        item_id: ItemId,
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
