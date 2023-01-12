use std::fmt;

use serde::{Deserialize, Serialize};

/// Physical state that is externally defined -- computed, generated, or
/// fetched.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum External {
    /// Placeholder indicating this value is not yet defined.
    Tbd(()),
}

impl External {
    /// Returns the `Tbd` variant.
    ///
    /// Convenience function so consumers don't have to import `PhantomData`.
    pub fn tbd() -> Self {
        Self::Tbd(())
    }
}

impl fmt::Display for External {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "external".fmt(f)
    }
}
