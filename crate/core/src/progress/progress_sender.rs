use tokio::sync::mpsc::Sender;

use crate::{
    progress::{ProgressDelta, ProgressUpdate, ProgressUpdateAndId},
    ItemSpecId,
};

/// Submits progress for an item spec's `EnsureOpSpec::exec` method.
#[derive(Debug)]
pub struct ProgressSender<'op> {
    /// ID of the item spec this belongs to.
    item_spec_id: &'op ItemSpecId,
    /// Channel sender to send progress updates to.
    progress_tx: &'op Sender<ProgressUpdateAndId>,
}

impl<'op> ProgressSender<'op> {
    /// Returns a new `ProgressSender`.
    pub fn new(
        item_spec_id: &'op ItemSpecId,
        progress_tx: &'op Sender<ProgressUpdateAndId>,
    ) -> Self {
        Self {
            item_spec_id,
            progress_tx,
        }
    }

    /// Increments the progress by the given unit count.
    pub async fn inc(&self, unit_count: u64) {
        let _progress_send_unused = self
            .progress_tx
            .send(ProgressUpdateAndId {
                item_spec_id: self.item_spec_id.clone(),
                progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(unit_count)),
            })
            .await;
    }

    /// Ticks the tracker without incrementing its progress.
    ///
    /// This is useful for spinners -- progress trackers where there is an
    /// unknown.
    ///
    /// Note, this also updates the `last_update_dt`, so in the case of a
    /// spinner, this should only be called when there is actually a detected
    /// change.
    pub async fn tick(&self) {
        let _progress_send_unused = self
            .progress_tx
            .send(ProgressUpdateAndId {
                item_spec_id: self.item_spec_id.clone(),
                progress_update: ProgressUpdate::Delta(ProgressDelta::Tick),
            })
            .await;
    }
}
