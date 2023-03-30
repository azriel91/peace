use std::{fmt::Debug, marker::PhantomData};

use futures::stream::{StreamExt, TryStreamExt};
use peace_cfg::OpCtx;
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
    Resources,
};
use peace_rt_model::{output::OutputWrite, params::ParamsKeys, Error, Storage};

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
        use tokio::sync::mpsc;
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
    /// Runs [`StateCurrentFnSpec`]`::`[`try_exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`], and will be serialized to
    /// `$flow_dir/states_saved.yaml`.
    ///
    /// If any `StateCurrentFnSpec` needs to read the `State` from a previous
    /// `ItemSpec`, the predecessor should insert a copy / clone of their state
    /// into `Resources`, and the successor should references it in their
    /// [`Data`].
    ///
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    pub async fn current(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StatesCurrent, E> {
        Self::exec(cmd_ctx, DiscoverFor::Current)
            .await
            .map(|(states_current, _states_desired)| states_current)
    }

    /// Runs [`StateDesiredFnSpec`]`::`[`try_exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesDesired`], and will be serialized to
    /// `$flow_dir/states_desired.yaml`.
    ///
    /// If any `StateDesiredFnSpec` needs to read the `State` from a previous
    /// `ItemSpec`, the predecessor should insert a copy / clone of their state
    /// into `Resources`, and the successor should references it in their
    /// [`Data`].
    ///
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    pub async fn desired(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StatesDesired, E> {
        Self::exec(cmd_ctx, DiscoverFor::Desired)
            .await
            .map(|(_states_current, states_desired)| states_desired)
    }

    /// Runs [`StateCurrentFnSpec`] and [`StateDesiredFnSpec`]`::`[`try_exec`]
    /// for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`] and [`StatesDesired`], and states will be serialized
    /// to `$flow_dir/states_saved.yaml` and
    /// `$flow_dir/states_desired.yaml`.
    ///
    /// If any `StateCurrentFnSpec` needs to read the `State` from a previous
    /// `ItemSpec`, the predecessor should insert a copy / clone of their state
    /// into `Resources`, and the successor should references it in their
    /// [`Data`].
    ///
    /// If any `StateDesiredFnSpec` needs to read the `State` from a previous
    /// `ItemSpec`, the predecessor should insert a copy / clone of their state
    /// into `Resources`, and the successor should references it in their
    /// [`Data`].
    ///
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    pub async fn current_and_desired(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<(StatesCurrent, StatesDesired), E> {
        Self::exec(cmd_ctx, DiscoverFor::CurrentAndDesired).await
    }

    /// Runs [`StateCurrentFnSpec`]`::`[`try_exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`], and will be serialized to
    /// `$flow_dir/states_saved.yaml`.
    ///
    /// If any `StateCurrentFnSpec` needs to read the `State` from a previous
    /// `ItemSpec`, the predecessor should insert a copy / clone of their state
    /// into `Resources`, and the successor should references it in their
    /// [`Data`].
    ///
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    /// [`Data`]: peace_cfg::TryFnSpec::Data
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
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

        let resources_ref = &*resources;
        let execution_task = async move {
            #[cfg(feature = "output_progress")]
            let progress_tx = &progress_tx;

            let (states_current_mut, states_desired_mut) = flow
                .graph()
                .stream()
                .map(Result::<_, E>::Ok)
                .try_fold(
                    (StatesMut::<Current>::new(), StatesMut::<Desired>::new()),
                    |(mut states_current_mut, mut states_desired_mut), item_spec| async move {
                        let item_spec_id = item_spec.id();
                        let op_ctx = OpCtx::new(
                            item_spec_id,
                            #[cfg(feature = "output_progress")]
                            ProgressSender::new(item_spec_id, progress_tx),
                        );

                        let (state_current_result, state_desired_result) = match discover_for {
                            DiscoverFor::Current => {
                                let state_current_result = item_spec
                                    .state_current_try_exec(op_ctx, resources_ref)
                                    .await;

                                (Some(state_current_result), None)
                            }
                            DiscoverFor::Desired => {
                                let state_desired_result = item_spec
                                    .state_desired_try_exec(op_ctx, resources_ref)
                                    .await;

                                (None, Some(state_desired_result))
                            }
                            DiscoverFor::CurrentAndDesired => {
                                let state_current_result = item_spec
                                    .state_current_try_exec(op_ctx, resources_ref)
                                    .await;
                                let state_desired_result = item_spec
                                    .state_desired_try_exec(op_ctx, resources_ref)
                                    .await;

                                (Some(state_current_result), Some(state_desired_result))
                            }
                        };

                        if let Some(state_current_result) = state_current_result {
                            #[cfg(feature = "output_progress")]
                            {
                                let (progress_complete, msg_update) = match &state_current_result {
                                    Ok(_) => (ProgressComplete::Success, ProgressMsgUpdate::Clear),
                                    Err(error) => (
                                        ProgressComplete::Fail,
                                        ProgressMsgUpdate::Set(format!("{error}")),
                                    ),
                                };

                                let _progress_send_unused =
                                    progress_tx.try_send(ProgressUpdateAndId {
                                        item_spec_id: item_spec_id.clone(),
                                        progress_update: ProgressUpdate::Complete(
                                            progress_complete,
                                        ),
                                        msg_update,
                                    });
                            }

                            let state_current = state_current_result?;
                            if let Some(state_current) = state_current {
                                states_current_mut
                                    .insert_raw(item_spec.id().clone(), state_current);
                            }
                        }

                        if let Some(state_desired_result) = state_desired_result {
                            #[cfg(feature = "output_progress")]
                            {
                                let (progress_complete, msg_update) = match &state_desired_result {
                                    Ok(_) => (ProgressComplete::Success, ProgressMsgUpdate::Clear),
                                    Err(error) => (
                                        ProgressComplete::Fail,
                                        ProgressMsgUpdate::Set(format!("{error}")),
                                    ),
                                };

                                let _progress_send_unused =
                                    progress_tx.try_send(ProgressUpdateAndId {
                                        item_spec_id: item_spec_id.clone(),
                                        progress_update: ProgressUpdate::Complete(
                                            progress_complete,
                                        ),
                                        msg_update,
                                    });
                            }

                            let state_desired = state_desired_result?;
                            if let Some(state_desired) = state_desired {
                                states_desired_mut
                                    .insert_raw(item_spec.id().clone(), state_desired);
                            }
                        }

                        Ok((states_current_mut, states_desired_mut))
                    },
                )
                .await?;

            let states_current = StatesCurrent::from(states_current_mut);
            let states_desired = StatesDesired::from(states_desired_mut);
            Result::<_, E>::Ok((states_current, states_desired))

            // `progress_tx` is dropped here, so `progress_rx` will safely end.
        };

        #[cfg(feature = "output_progress")]
        let progress_render_task =
            crate::progress::Progress::progress_render(output, progress_trackers, progress_rx);

        cfg_if::cfg_if! {
            if #[cfg(feature = "output_progress")] {
                let (states_current_and_desired, _) = futures::join!(execution_task, progress_render_task);

                output.progress_end(cmd_progress_tracker).await;
            } else {
                let (states_current_and_desired,) = futures::join!(execution_task);
            }
        }
        let (states_current, states_desired) = states_current_and_desired?;

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
