use std::{borrow::Cow, fmt};

/// Error indicating `FullSpecId` provided is not in the correct format.
#[derive(Debug, PartialEq, Eq)]
pub struct FullSpecIdInvalidFmt<'s> {
    /// String that was provided for the `FullSpecId`.
    value: Cow<'s, str>,
}

impl<'s> FullSpecIdInvalidFmt<'s> {
    /// Returns a new `FullSpecIdInvalidFmt`.
    pub fn new(value: Cow<'s, str>) -> Self {
        Self { value }
    }

    /// Returns the value that failed to be parsed as a [`FullSpecId`].
    ///
    /// [`FullSpecId`]: crate::FullSpecId
    pub fn value(&self) -> &Cow<'s, str> {
        &self.value
    }
}

impl<'s> fmt::Display for FullSpecIdInvalidFmt<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "`{}` is not a valid `FullSpecId`.\n\
            `FullSpecId`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.",
            self.value
        )
    }
}

impl<'s> std::error::Error for FullSpecIdInvalidFmt<'s> {}
