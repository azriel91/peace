use std::marker::PhantomData;

use peace_core::ItemSpecId;

#[cfg(feature = "output_progress")]
use peace_core::progress::ProgressSender;

/// References to pass information between the Peace framework and an item spec.
#[derive(Clone, Copy, Debug)]
pub struct FnCtx<'op> {
    /// ID of the item spec this belongs to.
    pub item_spec_id: &'op ItemSpecId,
    /// For item specs to submit progress updates.
    #[cfg(feature = "output_progress")]
    pub progress_sender: ProgressSender<'op>,
    /// Marker.
    pub marker: PhantomData<&'op ()>,
}

impl<'op> FnCtx<'op> {
    /// Returns a new `OpCtx`.
    pub fn new(
        item_spec_id: &'op ItemSpecId,
        #[cfg(feature = "output_progress")] progress_sender: ProgressSender<'op>,
    ) -> Self {
        Self {
            item_spec_id,
            #[cfg(feature = "output_progress")]
            progress_sender,
            marker: PhantomData,
        }
    }

    /// Returns the `ProgressTracker` for item specs to send progress to.
    #[cfg(feature = "output_progress")]
    pub fn progress_sender(&self) -> &ProgressSender<'op> {
        &self.progress_sender
    }
}
