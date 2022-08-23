use std::{borrow::Cow, fmt};

/// Error indicating `ItemSpecId` provided is not in the correct format.
#[derive(Debug, PartialEq, Eq)]
pub struct ItemSpecIdInvalidFmt<'s> {
    /// String that was provided for the `ItemSpecId`.
    value: Cow<'s, str>,
}

impl<'s> ItemSpecIdInvalidFmt<'s> {
    /// Returns a new `ItemSpecIdInvalidFmt` error.
    pub fn new(value: Cow<'s, str>) -> Self {
        Self { value }
    }

    /// Returns the value that failed to be parsed as an [`ItemSpecId`].
    ///
    /// [`ItemSpecId`]: crate::ItemSpecId
    pub fn value(&self) -> &Cow<'s, str> {
        &self.value
    }
}

impl<'s> fmt::Display for ItemSpecIdInvalidFmt<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "`{}` is not a valid `ItemSpecId`.\n\
            `ItemSpecId`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.",
            self.value
        )
    }
}

impl<'s> std::error::Error for ItemSpecIdInvalidFmt<'s> {}
