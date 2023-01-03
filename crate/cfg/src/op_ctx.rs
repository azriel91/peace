use std::marker::PhantomData;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_core::ProgressUpdate;
        use tokio::sync::mpsc::Sender;
    }
}

/// References to pass information between the Peace framework and an item spec.
#[derive(Debug)]
pub struct OpCtx<'op> {
    /// Channel sender for item spec implementations to send progress to.
    #[cfg(feature = "output_progress")]
    pub progress_tx: &'op Sender<ProgressUpdate>,
    /// Marker.
    pub marker: PhantomData<&'op ()>,
}

impl<'op> OpCtx<'op> {
    /// Returns a new `OpCtx`.
    pub fn new(
        #[cfg(feature = "output_progress")] progress_tx: &'op Sender<ProgressUpdate>,
    ) -> Self {
        Self {
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
