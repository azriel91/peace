use std::marker::PhantomData;

use peace_core::ItemSpecId;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_core::progress::ProgressUpdate;
        use tokio::sync::mpsc::Sender;
    }
}

/// References to pass information between the Peace framework and an item spec.
#[derive(Debug)]
pub struct OpCtx<'op> {
    /// ID of the item spec this belongs to.
    pub item_spec_id: &'op ItemSpecId,
    /// Channel sender for item spec implementations to send progress to.
    #[cfg(feature = "output_progress")]
    pub progress_tx: &'op Sender<ProgressUpdate>,
    /// Marker.
    pub marker: PhantomData<&'op ()>,
}

impl<'op> OpCtx<'op> {
    /// Returns a new `OpCtx`.
    pub fn new(
        item_spec_id: &'op ItemSpecId,
        #[cfg(feature = "output_progress")] progress_tx: &'op Sender<ProgressUpdate>,
    ) -> Self {
        Self {
            item_spec_id,
            #[cfg(feature = "output_progress")]
            progress_tx,
            marker: PhantomData,
        }
    }

    /// Returns the channel sender for item spec implementations to send
    /// progress to.
    #[cfg(feature = "output_progress")]
    pub fn progress_tx(&self) -> &Sender<ProgressUpdate> {
        self.progress_tx
    }
}
