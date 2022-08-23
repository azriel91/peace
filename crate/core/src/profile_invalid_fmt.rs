use std::{borrow::Cow, fmt};

/// Error indicating `Profile` provided is not in the correct format.
#[derive(Debug, PartialEq, Eq)]
pub struct ProfileInvalidFmt<'s> {
    /// String that was provided for the `Profile`.
    value: Cow<'s, str>,
}

impl<'s> ProfileInvalidFmt<'s> {
    /// Returns a new `ProfileInvalidFmt` error.
    pub fn new(value: Cow<'s, str>) -> Self {
        Self { value }
    }

    /// Returns the value that failed to be parsed as a [`Profile`].
    ///
    /// [`Profile`]: crate::Profile
    pub fn value(&self) -> &Cow<'s, str> {
        &self.value
    }
}

impl<'s> fmt::Display for ProfileInvalidFmt<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "`{}` is not a valid `Profile`.\n\
            `Profile`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.",
            self.value
        )
    }
}

impl<'s> std::error::Error for ProfileInvalidFmt<'s> {}
