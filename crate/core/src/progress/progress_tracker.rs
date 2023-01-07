use std::time::Duration;

use chrono::{DateTime, Utc};
use indicatif::ProgressBar;

/// Tracks progress for an item spec's `EnsureOpSpec::exec` method.
#[derive(Debug)]
pub struct ProgressTracker {
    /// Internal progress bar to update.
    progress_bar: ProgressBar,
    /// Timestamp of last progress update.
    ///
    /// This is useful to determine if execution has stalled.
    last_update_dt: DateTime<Utc>,
}

impl ProgressTracker {
    /// Returns a new `ProgressTracker`.
    pub fn new(progress_bar: ProgressBar) -> Self {
        let last_update_dt = Utc::now();

        Self {
            progress_bar,
            last_update_dt,
        }
    }

    /// Returns a reference to this `ProgressTracker`'s progress bar.
    pub fn progress_bar(&self) -> &ProgressBar {
        &self.progress_bar
    }

    /// Returns the estimated remaining duration to completion.
    pub fn eta(&self) -> Duration {
        self.progress_bar.eta()
    }

    /// Returns the elapsed duration.
    pub fn elapsed(&self) -> Duration {
        self.progress_bar.elapsed()
    }

    /// Returns the number of progress units already completed.
    pub fn units_current(&self) -> Option<u64> {
        self.progress_bar.length()
    }

    /// Returns the timestamp a progress update was last made.
    pub fn last_update_dt(&self) -> DateTime<Utc> {
        self.last_update_dt
    }
}
