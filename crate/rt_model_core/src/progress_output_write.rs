use async_trait::async_trait;
use peace_core::ProgressUpdate;

/// Transforms progress information into a suitable output format.
///
/// # Use cases
///
/// * A CLI implementation renders the progress as a textual progress bar.
/// * A REST implementation serializes the values as JSON for a response body.
/// * A frontend implementation renders the progress as an graphical progress
///   bar.
///
/// # Design
///
/// The write functions currently take `&mut self`. From an API implementer
/// perspective, this should make it easier to implement, as progress
/// information should be rendered in real time, and [`Receiver::recv`] takes
/// `&mut self`.
///
/// [`Receiver`]: tokio::sync::mpsc::Receiver::recv
#[async_trait(?Send)]
pub trait ProgressOutputWrite {
    /// Renders the progress information, and returns when no more progress
    /// information is available to write.
    ///
    /// This function is infallible as progress information is considered
    /// transient, and loss of progress information is not considered as
    /// something worth stopping an operation.
    ///
    /// # Implementors
    ///
    /// This should create a new channel, and return the channel sender.
    ///
    /// The sender will be passed to each of the `EnsureOpSpec::exec` functions
    /// so that progress information can be sent within them.
    async fn render(&mut self, progress_update: ProgressUpdate);
}
