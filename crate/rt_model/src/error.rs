use miette::Diagnostic;

/// Errors that happen within runtime operations.
#[derive(Debug, Diagnostic, thiserror::Error)]
pub enum Error<E>
where
    E: std::error::Error,
{
    #[error(transparent)]
    CleanSetup(E),
    #[error(transparent)]
    EnsureSetup(E),
    #[error(transparent)]
    StatusSetup(E),
}
