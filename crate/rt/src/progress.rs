use peace_cfg::{
    progress::{
        ProgressDelta, ProgressMsgUpdate, ProgressStatus, ProgressTracker, ProgressUpdate,
        ProgressUpdateAndId,
    },
    ItemSpecId,
};
use peace_rt_model::{output::OutputWrite, IndexMap};
use tokio::sync::mpsc::Receiver;

pub struct Progress;

impl Progress {
    /// Receives progress updates and updates `output` to render it.
    // TODO: write test for this
    pub async fn progress_render<E, O>(
        output: &mut O,
        progress_trackers: &mut IndexMap<ItemSpecId, ProgressTracker>,
        mut progress_rx: Receiver<ProgressUpdateAndId>,
    ) where
        O: OutputWrite<E>,
    {
        while let Some(progress_update_and_id) = progress_rx.recv().await {
            let ProgressUpdateAndId {
                item_spec_id,
                progress_update,
                msg_update,
            } = &progress_update_and_id;

            let Some(progress_tracker) = progress_trackers.get_mut(item_spec_id) else {
                panic!("Expected `progress_tracker` to exist for item spec: `{item_spec_id}`.");
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
                ProgressMsgUpdate::Set(message) => {
                    progress_tracker.set_message(Some(message.clone()))
                }
            }

            output
                .progress_update(progress_tracker, &progress_update_and_id)
                .await
        }
    }
}
