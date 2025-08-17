use std::any::TypeId;

/// Key used to identify a [`MappingFn`] in the [`MappingFnRegistry`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MappingFnRegKey {
    /// Type ID of the [`MappingFns`] type.
    pub type_id: TypeId,
    /// Hash of the discriminant of the [`MappingFns`] type.
    pub variant_hash: u64,
}
