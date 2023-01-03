use serde::{Deserialize, Serialize};

/// Message to update the `ProgressOutputWrite`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ProgressUpdate {
    /// Ticks the progress bar without incrementing its value.
    ///
    /// Generally useful for progress bars with an unknown total.
    Tick,
    /// Increments the progress bar by the specified amount.
    Inc(u64),
}
