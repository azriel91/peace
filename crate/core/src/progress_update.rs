use serde::{Deserialize, Serialize};

/// Message to update the `ProgressOutputWrite`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ProgressUpdate {
    /// Tick the progress bar.
    Tick,
}
