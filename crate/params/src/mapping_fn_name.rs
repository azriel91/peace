use serde::{Deserialize, Serialize};

/// Name of a mapping function. `String` newtype.
///
/// This is a string representation of a [`MappingFns`] variant, which allows
/// `*Spec`s to be serialized and deserialized and avoid:
///
/// * a `MFns: MappingFns` type parameter on each `*Spec` type -- which would
///   propagate to `Item`, causing undesired complexity in the `Item` trait.
/// * creating an object-safe trait corresponding to `MappingFns`, increasing
///   the maintenance burden.
///
/// [`MappingFns`]: crate::MappingFns
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MappingFnName(String);

impl MappingFnName {
    /// Returns a new `MappingFnName`.
    pub fn new(name: String) -> Self {
        MappingFnName(name)
    }

    /// Returns the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Returns a reference to the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns a mutable reference to the inner string.
    pub fn as_mut_str(&mut self) -> &mut str {
        &mut self.0
    }

    /// Returns the length of the inner string.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the inner string is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<str> for MappingFnName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for MappingFnName {
    fn from(name: String) -> Self {
        MappingFnName(name)
    }
}
