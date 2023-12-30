use std::ops::ControlFlow;

use futures::stream::{self, StreamExt};
use peace_cfg::{
    progress::{
        CmdProgressUpdate, ProgressDelta, ProgressMsgUpdate, ProgressStatus, ProgressTracker,
        ProgressUpdate, ProgressUpdateAndId,
    },
    ItemId,
};
use peace_rt_model::{output::OutputWrite, IndexMap};
use tokio::sync::mpsc::Receiver;

pub struct Progress;

impl Progress {
    /// Receives progress updates and updates `output` to render it.
    // TODO: write test for this
    pub async fn progress_render<E, O>(
        output: &mut O,
        progress_trackers: &mut IndexMap<ItemId, ProgressTracker>,
        mut progress_rx: Receiver<ProgressUpdateAndId>,
        mut cmd_progress_rx: Receiver<CmdProgressUpdate>,
    ) where
        O: OutputWrite<E>,
    {
        loop {
            // This **MUST** be `biased;` towards the `ProgressUpdateAndId` channel.
            //
            // Without it, some of the `ProgressUpdate::Delta(ProgressDelta::Tick)` messages
            // from `StatesDiscoverCmdBlock` arrive after
            // `CmdProgressUpdate::ResetToPending`, causing some progress bars
            // to revert to `Running` even though the `CmdExecution`'s
            // message should be applied after them.
            tokio::select! {
                biased;
                progress_update_and_id_message = progress_rx.recv() => {
                    match progress_update_and_id_message {
                        Some(progress_update_and_id) => Self::handle_progress_update_and_id(
                            output,
                            progress_trackers,
                            progress_update_and_id
                        ).await,
                        None => break,
                    }
                }
                Some(cmd_progress_update) = cmd_progress_rx.recv() => {
                    let control_flow = Self::handle_cmd_progress_update(
                        output,
                        progress_trackers,
                        cmd_progress_update
                    ).await;

                    match control_flow {
                        ControlFlow::Break(()) => break,
                        ControlFlow::Continue(()) => {}
                    }
                }
                else => break,
            }
        }

        while let Some(progress_update_and_id) = progress_rx.recv().await {
            Self::handle_progress_update_and_id(output, progress_trackers, progress_update_and_id)
                .await;
        }
    }

    async fn handle_progress_update_and_id<E, O>(
        output: &mut O,
        progress_trackers: &mut IndexMap<ItemId, ProgressTracker>,
        progress_update_and_id: ProgressUpdateAndId,
    ) where
        O: OutputWrite<E>,
    {
        let item_id = &progress_update_and_id.item_id;
        let Some(progress_tracker) = progress_trackers.get_mut(item_id) else {
            panic!("Expected `progress_tracker` to exist for item: `{item_id}`.");
        };

        Self::handle_progress_tracker_progress_update(
            output,
            progress_tracker,
            progress_update_and_id,
        )
        .await;
    }

    async fn handle_cmd_progress_update<E, O>(
        output: &mut O,
        progress_trackers: &mut IndexMap<ItemId, ProgressTracker>,
        cmd_progress_update: CmdProgressUpdate,
    ) -> ControlFlow<()>
    where
        O: OutputWrite<E>,
    {
        match cmd_progress_update {
            CmdProgressUpdate::Interrupt => {
                stream::iter(progress_trackers.iter_mut())
                    .fold(output, |output, (item_id, progress_tracker)| async move {
                        let item_id = item_id.clone();
                        let progress_update_and_id = ProgressUpdateAndId {
                            item_id,
                            progress_update: ProgressUpdate::Interrupt,
                            msg_update: ProgressMsgUpdate::NoChange,
                        };

                        Self::handle_progress_tracker_progress_update(
                            output,
                            progress_tracker,
                            progress_update_and_id,
                        )
                        .await;

                        output
                    })
                    .await;

                ControlFlow::Break(())
            }
            CmdProgressUpdate::ResetToPending => {
                stream::iter(progress_trackers.iter_mut())
                    .fold(output, |output, (item_id, progress_tracker)| async move {
                        let item_id = item_id.clone();
                        let progress_update_and_id = ProgressUpdateAndId {
                            item_id,
                            progress_update: ProgressUpdate::ResetToPending,
                            msg_update: ProgressMsgUpdate::Clear,
                        };

                        Self::handle_progress_tracker_progress_update(
                            output,
                            progress_tracker,
                            progress_update_and_id,
                        )
                        .await;

                        output
                    })
                    .await;

                ControlFlow::Continue(())
            }
        }
    }

    async fn handle_progress_tracker_progress_update<E, O>(
        output: &mut O,
        progress_tracker: &mut ProgressTracker,
        progress_update_and_id: ProgressUpdateAndId,
    ) where
        O: OutputWrite<E>,
    {
        let ProgressUpdateAndId {
            item_id: _,
            progress_update,
            msg_update,
        } = &progress_update_and_id;
        match progress_update {
            ProgressUpdate::Reset => progress_tracker.reset(),
            ProgressUpdate::ResetToPending => progress_tracker.reset_to_pending(),
            ProgressUpdate::Interrupt => progress_tracker.interrupt(),
            ProgressUpdate::Limit(progress_limit) => {
                progress_tracker.set_progress_limit(*progress_limit);
                progress_tracker.set_progress_status(ProgressStatus::ExecPending);
            }
            ProgressUpdate::Delta(delta) => {
                match delta {
                    ProgressDelta::Tick => progress_tracker.tick(),
                    ProgressDelta::Inc(unit_count) => progress_tracker.inc(*unit_count),
                }
                progress_tracker.set_progress_status(ProgressStatus::Running);
            }
            ProgressUpdate::Complete(progress_complete) => {
                progress_tracker
                    .set_progress_status(ProgressStatus::Complete(progress_complete.clone()));
            }
        }

        match msg_update {
            ProgressMsgUpdate::Clear => progress_tracker.set_message(None),
            ProgressMsgUpdate::NoChange => {}
            ProgressMsgUpdate::Set(message) => progress_tracker.set_message(Some(message.clone())),
        }

        output
            .progress_update(progress_tracker, &progress_update_and_id)
            .await;
    }
}
