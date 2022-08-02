use serde::{Deserialize, Serialize};

use crate::{Equality, MaybeEq, Tracked};

/// Represents a changeable value.
///
/// The `from` or `to` values are [`Tracked`] values, so the following cases
/// mean the value is unchanged:
///
/// * Both values are [`Tracked::None`]
/// * Both values are [`Tracked::Known`]`(t)`, and both `t`s are equal.
///
/// When either value is [`Tracked::Unknown`], then its [`Equality`] is
/// [`Unknown`].
///
/// # Design Note
///
/// This is deliberately called `Changeable` instead of `Change`, because
/// `Change` implies there *is* a change, whereas `Changeable` means there may
/// be one.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Changeable<T> {
    /// Current value.
    pub from: Tracked<T>,
    /// Next value.
    pub to: Tracked<T>,
}

impl<T> Changeable<T>
where
    T: MaybeEq,
{
    /// Returns a new `Changeable` value.
    ///
    /// See [`Changeable::known`] if both values are known.
    pub fn new(from: Tracked<T>, to: Tracked<T>) -> Self {
        Self { from, to }
    }

    /// Returns a new `Changeable` value.
    ///
    /// See [`Changeable::known`] if both values are known.
    pub fn known(from: T, to: T) -> Self {
        let from = Tracked::Known(from);
        let to = Tracked::Known(to);

        Self { from, to }
    }

    /// Returns the equality of the `from` and `to` values.
    pub fn equality(&self) -> Equality {
        <Tracked<T> as MaybeEq>::maybe_eq(&self.from, &self.to)
    }
}
