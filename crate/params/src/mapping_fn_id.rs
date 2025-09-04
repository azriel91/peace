use serde::{Deserialize, Serialize};

/// ID of a mapping function. `String` newtype.
///
/// This is a string representation of a [`MappingFns`] variant, which allows
/// `*Spec`s to be serialized and deserialized and avoid:
///
/// * a `MFns: MappingFns` type parameter on each `*Spec` type -- which would
///   propagate to `Item`, causing undesired complexity in the `Item` trait.
/// * creating an object-safe trait corresponding to `MappingFns`, increasing
///   the maintenance burden.
///
/// # Implementors
///
/// The ID is considered API, and should be stable. This means you should name
/// each variant with a version number, and never remove that variant, e.g.
/// `MappingFnId::new("ServerNameFromProfile_V1_0_0" )`.
///
/// That way, previously stored mapping function IDs can still be
/// deserialized, and tool developers can opt-in to upgrading to the newer
/// mapping functions when ready.
///
/// [`MappingFns`]: crate::MappingFns
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MappingFnId(String);

impl MappingFnId {
    /// Returns a new `MappingFnId`.
    pub fn new(name: String) -> Self {
        MappingFnId(name)
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

impl AsRef<str> for MappingFnId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for MappingFnId {
    fn from(name: String) -> Self {
        MappingFnId(name)
    }
}
