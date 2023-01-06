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

    /// Increments the progress by the given delta.
    pub fn inc(&mut self, delta: u64) {
        self.progress_bar.inc(delta);
        self.last_update_dt = Utc::now();
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
        self.last_update_dt = Utc::now();
    }

    /// Returns the estimated remaining duration to completion.
    pub fn eta(&self) -> Duration {
        self.progress_bar.eta()
    }

    /// Returns the elapsed duration .
    pub fn elapsed(&self) -> Duration {
        self.progress_bar.elapsed()
    }

    /// Returns the timestamp a progress update was last made.
    pub fn last_update_dt(&self) -> DateTime<Utc> {
        self.last_update_dt
    }
}
