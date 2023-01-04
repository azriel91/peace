use std::collections::HashMap;

use indicatif::{MultiProgress, ProgressBar};
use peace_cfg::ItemSpecId;

/// Tracks progress for each item spec's command execution.
#[derive(Debug)]
pub struct CmdProgressTracker {
    /// `MultiProgress` that tracks the remaining progress bars.
    multi_progress: MultiProgress,
    /// `ProgressBar`s for each item spec.
    progress_bars: HashMap<ItemSpecId, ProgressBar>,
}

impl CmdProgressTracker {
    /// Returns a new `CmdProgressTracker`.
    pub(crate) fn new(
        multi_progress: MultiProgress,
        progress_bars: HashMap<ItemSpecId, ProgressBar>,
    ) -> Self {
        Self {
            multi_progress,
            progress_bars,
        }
    }

    /// Returns the `MultiProgress` that tracks the remaining progress bars.
    pub fn multi_progress(&self) -> &MultiProgress {
        &self.multi_progress
    }

    /// Returns a mutable reference to the `MultiProgress` that tracks the
    /// remaining progress bars.
    pub fn multi_progress_mut(&mut self) -> &mut MultiProgress {
        &mut self.multi_progress
    }

    /// Returns the `ProgressBar`s for each item spec.
    pub fn progress_bars(&self) -> &HashMap<ItemSpecId, ProgressBar> {
        &self.progress_bars
    }

    /// Returns a mutable reference to the `ProgressBar`s for each item spec.
    pub fn progress_bars_mut(&mut self) -> &mut HashMap<ItemSpecId, ProgressBar> {
        &mut self.progress_bars
    }
}
