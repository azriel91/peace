use std::fmt;

/// The error type which is returned from formatting a message into a stream.
///
/// This type does not support transmission of an error other than that an error
/// occurred. Any extra information must be arranged to be transmitted through
/// some other means.
///
/// An important thing to remember is that the type `peace::fmt::Error` should
/// not be confused with [`std::fmt::Error`] or [`std::error::Error`], which you
/// may also have in scope.
///
/// [`std::fmt::Error`]: std::fmt::Error
/// [`std::error::Error`]: std::error::Error
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error;

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Presentation Error")
    }
}
