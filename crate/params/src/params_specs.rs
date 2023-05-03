use std::ops::{Deref, DerefMut};

use peace_core::ItemSpecId;
use peace_resources::type_reg::untagged::{BoxDt, TypeMap};
use serde::Serialize;

/// Map of item spec ID to its params' specs. `TypeMap<ItemSpecId, BoxDt>`
/// newtype.
///
/// The concrete `*ParamsSpec` type can be obtained by calling
/// `.get(item_spec_id)` with the correct type:
///
/// ```rust,ignore
/// let item_spec_params_spec = MyItemSpecParams::spec().build();
/// let mut params_specs = ParamsSpecs::new();
/// params_specs.insert(item_spec_id!("my_item"), item_spec_params_spec);
///
/// // later
///
/// let item_spec_params_spec = params_specs.get::<MyItemSpecParams, _>(&item_spec_id!("my_item"));
/// ```
///
/// The information may not be of the same type across flows, as flows are
/// different in what they are doing.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(transparent)] // Needed to serialize as a map instead of a list.
pub struct ParamsSpecs(TypeMap<ItemSpecId, BoxDt>);

impl ParamsSpecs {
    /// Returns a new `ParamsSpecs` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `ParamsSpecs` map with the specified capacity.
    ///
    /// The `ParamsSpecs` will be able to hold at least capacity
    /// elements without reallocating. If capacity is 0, the map will not
    /// allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity_typed(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<ItemSpecId, BoxDt> {
        self.0
    }
}

impl Deref for ParamsSpecs {
    type Target = TypeMap<ItemSpecId, BoxDt>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ParamsSpecs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<TypeMap<ItemSpecId, BoxDt>> for ParamsSpecs {
    fn from(type_map: TypeMap<ItemSpecId, BoxDt>) -> Self {
        Self(type_map)
    }
}
