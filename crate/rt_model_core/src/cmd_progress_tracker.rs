use std::collections::HashMap;

use indicatif::MultiProgress;
use peace_core::{
    progress::{ProgressTracker, ProgressUpdate},
    ItemSpecId,
};
use tokio::sync::mpsc::Receiver;

/// Tracks command execution progress for all item specs.
///
/// The Peace framework initializes the `multi_progress` and `progress_trackers`
/// and manages updating the `ProgressBar` values.
///
/// By default, the `MultiProgress` will use [`ProgressDrawTarget::hidden()`].
/// However, by default [`CliOutput`] sets the draw target to `stdout` if an
/// executable built using Peace is run interactively.
///
/// [`ProgressDrawTarget::hidden()`]: indicatif::ProgressDrawTarget::hidden
/// [`CliOutput`]: https://docs.rs/peace_rt_model_native/latest/peace_rt_model_native/struct.CliOutput.html
#[derive(Debug)]
pub struct CmdProgressTracker {
    /// `MultiProgress` that tracks the remaining progress bars.
    pub multi_progress: MultiProgress,
    /// Channel receiver for progress updates.
    pub progress_rx: Receiver<ProgressUpdate>,
    /// Tracks progress for each item spec.
    pub progress_trackers: HashMap<ItemSpecId, ProgressTracker>,
}

impl CmdProgressTracker {
    /// Returns a new `CmdProgressTracker`.
    pub fn new(
        multi_progress: MultiProgress,
        progress_rx: Receiver<ProgressUpdate>,
        progress_trackers: HashMap<ItemSpecId, ProgressTracker>,
    ) -> Self {
        Self {
            multi_progress,
            progress_rx,
            progress_trackers,
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

    /// Returns the channel receiver for progress updates.
    pub fn progress_rx(&self) -> &Receiver<ProgressUpdate> {
        &self.progress_rx
    }

    /// Returns a mutable reference to the channel receiver for progress
    /// updates.
    pub fn progress_rx_mut(&mut self) -> &mut Receiver<ProgressUpdate> {
        &mut self.progress_rx
    }

    /// Returns the `ProgressTracker`s for each item spec.
    pub fn progress_trackers(&self) -> &HashMap<ItemSpecId, ProgressTracker> {
        &self.progress_trackers
    }

    /// Returns a mutable reference to the `ProgressTracker`s for each item
    /// spec.
    pub fn progress_trackers_mut(&mut self) -> &mut HashMap<ItemSpecId, ProgressTracker> {
        &mut self.progress_trackers
    }
}
