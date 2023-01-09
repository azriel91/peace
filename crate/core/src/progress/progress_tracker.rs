use std::time::Duration;

use chrono::{DateTime, Utc};
use indicatif::ProgressBar;

use crate::progress::{ProgressLimit, ProgressStatus};

/// Tracks progress for an item spec's `EnsureOpSpec::exec` method.
#[derive(Debug)]
pub struct ProgressTracker {
    /// Status of the item spec's execution progress.
    progress_status: ProgressStatus,
    /// Internal progress bar to update.
    progress_bar: ProgressBar,
    /// Progress limit for the execution, if known.
    progress_limit: Option<ProgressLimit>,
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
            progress_status: ProgressStatus::Initialized,
            progress_bar,
            progress_limit: None,
            last_update_dt,
        }
    }

    /// Returns a reference to the progress status.
    pub fn progress_status(&self) -> &ProgressStatus {
        &self.progress_status
    }

    /// Sets the progress status.
    pub fn set_progress_status(&mut self, progress_status: ProgressStatus) {
        self.progress_status = progress_status;
    }

    /// Returns a reference to the progress bar.
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

    /// Returns the progress limit for the execution, if known.
    pub fn progress_limit(&self) -> Option<ProgressLimit> {
        self.progress_limit
    }

    /// Sets the progress limit of the execution.
    pub fn set_progress_limit(&mut self, progress_limit: Option<ProgressLimit>) {
        self.progress_limit = progress_limit;
    }

    /// Returns the timestamp a progress update was last made.
    pub fn last_update_dt(&self) -> DateTime<Utc> {
        self.last_update_dt
    }

    /// Returns the timestamp a progress update was last made.
    pub fn last_update_dt_update(&mut self) {
        self.last_update_dt = Utc::now();
    }
}
