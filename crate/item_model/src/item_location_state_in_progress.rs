use serde::{Deserialize, Serialize};

/// Represents the state of an [`ItemLocation`].
///
/// This affects how the [`ItemLocation`] is rendered.
///
/// This is analogous to [`ItemLocationState`], with added variants for when the
/// state is being determined.
///
/// [`ItemLocation`]: crate::ItemLocation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ItemLocationStateInProgress {
    /// [`ItemLocation`] does not exist.
    ///
    /// This means it should be rendered invisible / low opacity.
    NotExists,
    /// [`ItemLocation`] may or may not exist, and we are in the process of
    /// determining that.
    ///
    /// This means it should be rendered pulsing / mid opacity.
    ///
    /// [`ItemLocation`]: crate::ItemLocation
    DiscoverInProgress,
    /// [`ItemLocation`] is being created.
    ///
    /// This means it should be rendered with full opacity and blue animated
    /// outlines.
    ///
    /// [`ItemLocation`]: crate::ItemLocation
    CreateInProgress,
    /// [`ItemLocation`] is being modified.
    ///
    /// This means it should be rendered with full opacity and blue animated
    /// outlines.
    ///
    /// [`ItemLocation`]: crate::ItemLocation
    ModificationInProgress,
    /// [`ItemLocation`] exists.
    ///
    /// This means it should be rendered with full opacity.
    ///
    /// [`ItemLocation`]: crate::ItemLocation
    ExistsOk,
    /// [`ItemLocation`] exists, but is in an erroneous state.
    ///
    /// This means it should be rendered with full opacity with a red shape
    /// colour.
    ///
    /// [`ItemLocation`]: crate::ItemLocation
    ExistsError,
}
