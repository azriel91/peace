use std::{
    cmp::{Ordering, PartialOrd},
    fmt,
};

/// Represents whether a value is equal to another.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Equality {
    /// Values are not equal.
    NotEqual,
    /// Values are equal.
    Equal,
    /// Cannot determine equality of values.
    ///
    /// This is when either or both [`Tracked`] values are [`Tracked::Unknown`].
    ///
    /// [`Tracked`]: crate::Tracked
    /// [`Tracked::Unknown`]: crate::Tracked::Unknown
    Unknown,
}

impl fmt::Display for Equality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotEqual => "!=".fmt(f),
            Self::Equal => "==".fmt(f),
            Self::Unknown => "?=".fmt(f),
        }
    }
}

impl PartialOrd for Equality {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Currently using this order:
        //
        // ```
        // NotEqual < Equal < Unknown
        // ```
        match (self, other) {
            (Self::NotEqual, Self::NotEqual) => Some(Ordering::Equal),
            (Self::NotEqual, Self::Equal) => Some(Ordering::Less),
            (Self::NotEqual, Self::Unknown) => Some(Ordering::Less),
            (Self::Equal, Self::NotEqual) => Some(Ordering::Greater),
            (Self::Equal, Self::Equal) => Some(Ordering::Equal),
            (Self::Equal, Self::Unknown) => Some(Ordering::Less),
            (Self::Unknown, Self::NotEqual) => Some(Ordering::Greater),
            (Self::Unknown, Self::Equal) => Some(Ordering::Greater),
            (Self::Unknown, Self::Unknown) => None,
        }
    }
}

impl From<bool> for Equality {
    fn from(eq: bool) -> Self {
        if eq {
            Equality::Equal
        } else {
            Equality::NotEqual
        }
    }
}

impl From<Equality> for bool {
    fn from(equality: Equality) -> bool {
        match equality {
            Equality::NotEqual => false,
            Equality::Equal => true,
            Equality::Unknown => false,
        }
    }
}
