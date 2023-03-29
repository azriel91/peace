use std::{fmt::Debug, marker::PhantomData};

use futures::stream::{StreamExt, TryStreamExt};
use peace_cfg::OpCtx;
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    internal::StatesMut,
    paths::{FlowDir, StatesSavedFile},
    resources::ts::SetUp,
    states::{ts::Current, StatesCurrent},
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
pub struct StatesCurrentDiscoverCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> StatesCurrentDiscoverCmd<E, O, PKeys>
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
    pub async fn exec(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StatesCurrent, E> {
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
                let (progress_close_tx, progress_close_rx) =
                    tokio::sync::oneshot::channel::<()>();
            }
        }

        let resources_ref = &*resources;
        let execution_task = async move {
            #[cfg(feature = "output_progress")]
            let progress_tx = &progress_tx;

            let states_mut = flow
                .graph()
                .stream()
                .map(Result::<_, E>::Ok)
                .try_filter_map(|item_spec| async move {
                    let item_spec_id = item_spec.id();
                    let op_ctx = OpCtx::new(
                        item_spec_id,
                        #[cfg(feature = "output_progress")]
                        ProgressSender::new(item_spec_id, progress_tx),
                    );

                    let state = item_spec
                        .state_current_try_exec(op_ctx, resources_ref)
                        .await;

                    #[cfg(feature = "output_progress")]
                    {
                        let (progress_complete, msg_update) = match &state {
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

                    let state = state?;

                    Ok(state.map(|state| (item_spec.id().clone(), state)))
                })
                .try_collect::<StatesMut<Current>>()
                .await?;

            // `progress_tx` is dropped here, so `progress_rx` will safely end.

            // However, the last progress message is somehow rendered *after*
            // errors are rendered, causing any failed progress bar messages to
            // overwrite error messages, so we explicitly tell it to stop.
            #[cfg(feature = "output_progress")]
            let (Ok(()) | Err(())) = progress_close_tx.send(());

            let states_current = StatesCurrent::from(states_mut);
            Result::<_, E>::Ok(states_current)
        };

        #[cfg(feature = "output_progress")]
        let progress_render_task = crate::progress::Progress::progress_render(
            output,
            progress_trackers,
            progress_rx,
            progress_close_rx,
        );

        cfg_if::cfg_if! {
            if #[cfg(feature = "output_progress")] {
                let (states_current, _) = futures::join!(execution_task, progress_render_task);

                output.progress_end(cmd_progress_tracker).await;
            } else {
                let (states_current,) = futures::join!(execution_task);
            }
        }
        let states_current = states_current?;

        Self::serialize_internal(resources, &states_current).await?;

        Ok(states_current)
    }

    // TODO: This duplicates a bit of code with `ApplyCmd`.
    async fn serialize_internal(
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
}

impl<E, O, PKeys> Default for StatesCurrentDiscoverCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
