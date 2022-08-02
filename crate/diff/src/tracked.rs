use std::{
    cmp::PartialEq,
    hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};

use crate::{Equality, MaybeEq};

/// Tracks the known state of a value.
#[derive(Clone, Debug, Deserialize, Serialize, Eq)]
pub enum Tracked<T> {
    /// Value does not exist.
    None,
    /// Value exists, but its content is not known.
    Unknown,
    /// Value exists.
    Known(T),
}

impl<T> PartialEq for Tracked<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Both non-existent values.
            (Self::None, Self::None) => true,

            // Both known values with info.
            (Self::Known(t_self), Self::Known(t_other)) => t_self.eq(t_other),

            // One known value and one non-existent, or
            // Any unknown value.
            (Self::Known(_), Self::None)
            | (Self::None, Self::Known(_))
            | (_, Self::Unknown)
            | (Self::Unknown, _) => false,
        }
    }
}

impl<T> Hash for Tracked<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::None => 0.hash(state),
            Self::Known(t) => t.hash(state),
            Self::Unknown => 2.hash(state),
        }
    }
}

impl<T> MaybeEq for Tracked<T>
where
    T: MaybeEq,
{
    fn maybe_eq(&self, other: &Self) -> Equality {
        match (self, other) {
            // Both non-existent values.
            (Self::None, Self::None) => Equality::Equal,

            // Both known values with info.
            (Self::Known(t_self), Self::Known(t_other)) => t_self.maybe_eq(t_other),

            // One known value and one non-existent.
            (Self::Known(_), Self::None) | (Self::None, Self::Known(_)) => Equality::NotEqual,

            // Any unknown value.
            (_, Self::Unknown) | (Self::Unknown, _) => Equality::Unknown,
        }
    }
}
