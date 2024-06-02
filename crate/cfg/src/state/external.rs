use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

/// Physical state that is externally defined -- computed, generated, or
/// fetched.
///
/// Compared to [`ExternalOpt`], this does not have the `None` variant, to
/// indicate that the external source must return a value, and the lack of a
/// value is a bug and must be surfaced as an issue to the user.
///
/// The following type aliases are available to semantically name the type in
/// item implementations:
///
/// * [`Generated`]
/// * [`Fetched`]
/// * [`Timestamped`]
///
/// [`ExternalOpt`]: crate::state::ExternalOpt
#[enser::enser]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum External<V> {
    /// Placeholder indicating this value is not yet defined.
    Tbd,
    /// Value has been recorded after execution.
    Value(V),
}

impl<V> Display for External<V>
where
    V: Clone + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_name = tynm::type_name::<V>();
        match self {
            Self::Tbd => write!(f, "{type_name} not yet determined"),
            Self::Value(v) => write!(f, "{type_name}: {v}"),
        }
    }
}

/// Physical state that is computed or generated externally, e.g. a server ID.
pub type Generated<V> = External<V>;

/// Physical state that is fetched from an external source, e.g. an ETag.
pub type Fetched<V> = External<V>;

/// Physical state that depends on time, e.g. last execution time.
pub type Timestamped<V> = External<V>;
