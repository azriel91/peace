use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

/// Physical state that is externally defined -- computed, generated, or
/// fetched.
///
/// Compared to [`External`], this also has a `None` variant, to indicate that
/// the external source has been queried, but it did not return a value.
///
/// The following type aliases are available to semantically name the type in
/// item spec implementations:
///
/// * [`GeneratedOpt`]
/// * [`FetchedOpt`]
/// * [`TimestampedOpt`]
///
/// [`External`]: crate::state::External
#[enser::enser]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ExternalOpt<V> {
    /// Placeholder indicating this value is not yet defined.
    Tbd,
    /// The external source did not return a value.
    None,
    /// Value has been recorded after execution.
    Value(V),
}

impl<V> Display for ExternalOpt<V>
where
    V: Clone + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tbd => "not yet determined".fmt(f),
            Self::None => "not existent".fmt(f),
            Self::Value(v) => v.fmt(f),
        }
    }
}

/// Physical state that is computed or generated externally, e.g. a server ID.
pub type GeneratedOpt<V> = ExternalOpt<V>;

/// Physical state that is fetched from an external source, e.g. an ETag.
pub type FetchedOpt<V> = ExternalOpt<V>;

/// Physical state that depends on time, e.g. last execution time.
pub type TimestampedOpt<V> = ExternalOpt<V>;
