use serde::{Deserialize, Serialize};

#[cfg(feature = "output_progress")]
use peace_cmd_model::CmdBlockItemInteractionType;
#[cfg(feature = "output_progress")]
use peace_core::{
    progress::{ProgressLimit, ProgressStatus},
    ItemId,
};

/// A message that carries what needs to be updated in the web UI.
///
/// This is received by the `CmdExecution` task, processed into `InfoGraph`, and
/// rendered by `leptos`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum WebUiUpdate {
    /// A `CmdBlock` has started.
    #[cfg(feature = "output_progress")]
    CmdBlockStart {
        /// The type of interactions the `CmdBlock` has with the
        /// `ItemLocation`s.
        cmd_block_item_interaction_type: CmdBlockItemInteractionType,
    },
    /// Item's execution progress status.
    #[cfg(feature = "output_progress")]
    ItemProgressStatus {
        /// ID of the item that is updated.
        item_id: ItemId,
        /// Status of the item's execution progress.
        progress_status: ProgressStatus,
        /// Progress limit for the execution, if known.
        progress_limit: Option<ProgressLimit>,
        /// Message to display.
        message: Option<String>,
    },
    /// Markdown to render.
    Markdown {
        /// The markdown source to render.
        // TODO: receiver should render this using `pulldown-cmark`.
        markdown_src: String,
    },
}
