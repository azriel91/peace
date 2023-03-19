use std::time::Duration;

use chrono::{DateTime, Utc};
use indicatif::ProgressBar;

use crate::progress::{ProgressLimit, ProgressStatus};

/// Tracks progress for an item spec's `ApplyOpSpec::exec` method.
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

    /// Increments the progress by the given unit count.
    pub fn inc(&mut self, unit_count: u64) {
        self.progress_bar.inc(unit_count);
        self.last_update_dt_update();
    }

    /// Ticks the tracker without incrementing its progress.
    ///
    /// This is useful for spinners -- progress trackers where there is an
    /// unknown.
    ///
    /// Note, this also updates the `last_update_dt`, so in the case of a
    /// spinner, this should only be called when there is actually a detected
    /// change.
    pub fn tick(&mut self) {
        self.progress_bar.tick();
        self.last_update_dt_update();
    }

    /// Returns a reference to the progress status.
    pub fn progress_status(&self) -> &ProgressStatus {
        &self.progress_status
    }

    /// Sets the progress status.
    pub fn set_progress_status(&mut self, progress_status: ProgressStatus) {
        self.progress_status = progress_status;
        self.last_update_dt_update();
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
    pub fn units_current(&self) -> u64 {
        self.progress_bar.position()
    }

    /// Returns the total number of progress units.
    pub fn units_total(&self) -> Option<u64> {
        self.progress_limit
            .and_then(|progress_limit| match progress_limit {
                ProgressLimit::Unknown => None,
                ProgressLimit::Steps(units_total) | ProgressLimit::Bytes(units_total) => {
                    Some(units_total)
                }
            })
    }

    /// Returns the progress limit for the execution, if known.
    pub fn progress_limit(&self) -> Option<ProgressLimit> {
        self.progress_limit
    }

    /// Sets the progress limit of the execution.
    pub fn set_progress_limit(&mut self, progress_limit: ProgressLimit) {
        // Update units total on `ProgressBar`.
        match progress_limit {
            ProgressLimit::Unknown => {
                // Do nothing -- this keeps the `indicatif` internal `State` to
                // be `None`.
            }
            ProgressLimit::Steps(units_total) | ProgressLimit::Bytes(units_total) => {
                self.progress_bar.set_length(units_total);
            }
        }
        self.progress_limit = Some(progress_limit);
        self.last_update_dt_update();
    }

    /// Returns the timestamp a progress update was last made.
    pub fn last_update_dt(&self) -> DateTime<Utc> {
        self.last_update_dt
    }

    /// Returns the timestamp a progress update was last made.
    #[inline]
    fn last_update_dt_update(&mut self) {
        self.last_update_dt = Utc::now();
    }
}
