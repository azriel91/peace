use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

/// Physical state that is externally defined -- computed, generated, or
/// fetched.
///
/// The following type aliases are available to semantically name the type in
/// item spec implementations:
///
/// * [`Generated`]
/// * [`Fetched`]
/// * [`Timestamped`]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum External<V> {
    /// Placeholder indicating this value is not yet defined.
    Tbd(()),
    /// Value has been recorded after execution.
    Value(V),
}

impl<V> External<V> {
    /// Returns the `Tbd` variant.
    ///
    /// Convenience function so consumers don't have to import `PhantomData`.
    pub fn tbd() -> Self {
        Self::Tbd(())
    }
}

impl<V> Display for External<V>
where
    V: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tbd(()) => "not yet determined".fmt(f),
            Self::Value(v) => v.fmt(f),
        }
    }
}

/// Physical state that is computed or generated externally, e.g. a server ID.
pub type Generated<V> = External<V>;

/// Physical state that is fetched from an external source, e.g. an ETag.
pub type Fetched<V> = External<V>;

/// Physical state that depends on time, e.g. last execution time.
pub type Timestamped<V> = External<V>;
