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
            tokio::select! {
                Some(progress_update_and_id) = progress_rx.recv() => {
                    Self::handle_progress_update_and_id(
                        output,
                        progress_trackers,
                        progress_update_and_id
                    ).await;
                }
                Some(cmd_progress_update) = cmd_progress_rx.recv() => {
                    Self::handle_cmd_progress_update(
                        output,
                        progress_trackers,
                        cmd_progress_update
                    ).await;
                }
                else => break,
            }
        }
    }

    async fn handle_progress_update_and_id<E, O>(
        output: &mut O,
        progress_trackers: &mut IndexMap<ItemId, ProgressTracker>,
        progress_update_and_id: ProgressUpdateAndId,
    ) where
        O: OutputWrite<E>,
    {
        let ProgressUpdateAndId {
            item_id,
            progress_update,
            msg_update,
        } = &progress_update_and_id;

        let Some(progress_tracker) = progress_trackers.get_mut(item_id) else {
            panic!("Expected `progress_tracker` to exist for item: `{item_id}`.");
        };
        match progress_update {
            ProgressUpdate::Reset => {
                progress_tracker.reset();
            }
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

    async fn handle_cmd_progress_update<E, O>(
        output: &mut O,
        progress_trackers: &mut IndexMap<ItemId, ProgressTracker>,
        cmd_progress_update: CmdProgressUpdate,
    ) where
        O: OutputWrite<E>,
    {
    }
}
