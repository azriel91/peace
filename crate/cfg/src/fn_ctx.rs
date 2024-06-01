use std::marker::PhantomData;

use peace_core::StepId;

#[cfg(feature = "output_progress")]
use peace_core::progress::ProgressSender;

/// References to pass information between the Peace framework and a step.
#[derive(Clone, Copy, Debug)]
pub struct FnCtx<'exec> {
    /// ID of the step this belongs to.
    pub step_id: &'exec StepId,
    /// For steps to submit progress updates.
    #[cfg(feature = "output_progress")]
    pub progress_sender: ProgressSender<'exec>,
    /// Marker.
    pub marker: PhantomData<&'exec ()>,
}

impl<'exec> FnCtx<'exec> {
    /// Returns a new `OpCtx`.
    pub fn new(
        step_id: &'exec StepId,
        #[cfg(feature = "output_progress")] progress_sender: ProgressSender<'exec>,
    ) -> Self {
        Self {
            step_id,
            #[cfg(feature = "output_progress")]
            progress_sender,
            marker: PhantomData,
        }
    }

    /// Returns the `ProgressTracker` for steps to send progress to.
    #[cfg(feature = "output_progress")]
    pub fn progress_sender(&self) -> &ProgressSender<'exec> {
        &self.progress_sender
    }
}
