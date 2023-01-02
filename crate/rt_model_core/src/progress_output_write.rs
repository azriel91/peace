use async_trait::async_trait;

/// Transforms progress information into a suitable output format.
///
/// # Examples
///
/// * A CLI implementation renders the progress as a textual progress bar.
/// * A REST implementation serializes the values as JSON for a response body.
/// * A frontend implementation renders the progress as an graphical progress
///   bar.
///
/// # Design
///
/// The write functions currently take `&mut self`. From an API consumer
/// perspective, this should not be difficult to use as the return value / error
/// value is intended to be returned at the end of a command.
#[async_trait(?Send)]
pub trait ProgressOutputWrite {
    //
}
