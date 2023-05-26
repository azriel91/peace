#[cfg(feature = "error_reporting")]
use peace::miette;

/// Error while running a workspace test.
///
/// Error type for tests that need an error for their item graphs and
/// command contexts.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum PeaceTestError {
    /// A VecCopy item error occurred.
    #[error("A VecCopy item error occurred.")]
    VecCopyError(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        crate::VecCopyError,
    ),

    /// A `peace` runtime error occurred.
    #[error("A `peace` runtime error occurred.")]
    PeaceRtError(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace::rt_model::Error,
    ),
}
