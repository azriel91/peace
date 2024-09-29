use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use std::collections::HashMap;
        use peace_core::{progress::ProgressStatus, ItemId};
    }
}

/// How to style the outcome `InfoGraph`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutcomeInfoGraphVariant {
    /// Example `InfoGraph` diagram with no special styling.
    Example,
    /// Current `InfoGraph` diagram that shows execution progress.
    Current {
        /// Execution progress status of each item.
        #[cfg(feature = "output_progress")]
        item_progress_statuses: HashMap<ItemId, ProgressStatus>,
    },
}
