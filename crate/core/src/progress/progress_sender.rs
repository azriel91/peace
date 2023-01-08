use std::time::Duration;

use indicatif::ProgressBar;
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
    /// Progress bar to update.
    progress_bar: ProgressBar,
    /// Channel sender to send progress updates to.
    progress_tx: &'op Sender<ProgressUpdateAndId>,
}

impl<'op> ProgressSender<'op> {
    /// Returns a new `ProgressSender`.
    pub fn new(
        item_spec_id: &'op ItemSpecId,
        progress_bar: ProgressBar,
        progress_tx: &'op Sender<ProgressUpdateAndId>,
    ) -> Self {
        Self {
            item_spec_id,
            progress_bar,
            progress_tx,
        }
    }

    /// Increments the progress by the given delta.
    pub async fn inc(&self, delta: u64) {
        self.progress_bar.inc(delta);

        let _progress_send_unused = self
            .progress_tx
            .send(ProgressUpdateAndId {
                item_spec_id: self.item_spec_id.clone(),
                progress_update: ProgressUpdate::Delta {
                    delta: ProgressDelta::Inc(delta),
                },
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
        self.progress_bar.tick();

        let _progress_send_unused = self
            .progress_tx
            .send(ProgressUpdateAndId {
                item_spec_id: self.item_spec_id.clone(),
                progress_update: ProgressUpdate::Delta {
                    delta: ProgressDelta::Tick,
                },
            })
            .await;
    }

    /// Returns the estimated remaining duration to completion.
    pub fn eta(&self) -> Duration {
        self.progress_bar.eta()
    }

    /// Returns the elapsed duration.
    pub fn elapsed(&self) -> Duration {
        self.progress_bar.elapsed()
    }
}
