use serde::{Deserialize, Serialize};

/// Message to update the `OutputProgressWriter`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ProgressUpdate {
    /// Tick the progress bar.
    Tick,
}
