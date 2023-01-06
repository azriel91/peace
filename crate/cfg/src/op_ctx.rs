use std::marker::PhantomData;

use peace_core::ItemSpecId;

#[cfg(feature = "output_progress")]
use peace_core::progress::ProgressTracker;

/// References to pass information between the Peace framework and an item spec.
#[derive(Debug)]
pub struct OpCtx<'op> {
    /// ID of the item spec this belongs to.
    pub item_spec_id: &'op ItemSpecId,
    /// `ProgressTracker` for item specs to send progress to.
    #[cfg(feature = "output_progress")]
    pub progress_tracker: &'op mut ProgressTracker,
    /// Marker.
    pub marker: PhantomData<&'op ()>,
}

impl<'op> OpCtx<'op> {
    /// Returns a new `OpCtx`.
    pub fn new(
        item_spec_id: &'op ItemSpecId,
        #[cfg(feature = "output_progress")] progress_tracker: &'op mut ProgressTracker,
    ) -> Self {
        Self {
            item_spec_id,
            #[cfg(feature = "output_progress")]
            progress_tracker,
            marker: PhantomData,
        }
    }

    /// Returns the `ProgressTracker` for item specs to send progress to.
    #[cfg(feature = "output_progress")]
    pub fn progress_tracker(&mut self) -> &mut ProgressTracker {
        self.progress_tracker
    }
}
