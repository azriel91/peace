use std::{fmt, marker::PhantomData};

use serde::{Deserialize, Serialize};

/// Placeholder for physical state to be computed.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Placeholder {
    /// Placeholder indicating this value is calculated.
    ///
    /// Using a newtype enum has the benefit of having a `!Tbd` tag in
    /// the serialized YAML form.
    Tbd(PhantomData<()>),
}

impl Placeholder {
    /// Returns the `Tbd` variant.
    ///
    /// Convenience function so consumers don't have to import `PhantomData`.
    pub fn tbd() -> Self {
        Self::Tbd(PhantomData)
    }
}

impl fmt::Display for Placeholder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "placeholder".fmt(f)
    }
}
