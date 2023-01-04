use std::collections::HashMap;

use indicatif::{MultiProgress, ProgressBar};
use peace_core::ItemSpecId;

/// Tracks progress for each item spec's command execution.
///
/// The Peace framework initializes the `multi_progress` and `progress_bars` and
/// manage updating the `ProgressBar` values.
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
    multi_progress: MultiProgress,
    /// `ProgressBar`s for each item spec.
    progress_bars: HashMap<ItemSpecId, ProgressBar>,
}

impl CmdProgressTracker {
    /// Returns a new `CmdProgressTracker`.
    pub fn new(
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
