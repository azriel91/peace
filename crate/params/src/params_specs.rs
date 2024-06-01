use std::ops::{Deref, DerefMut};

use peace_core::StepId;
use peace_resources::type_reg::untagged::TypeMap;
use serde::Serialize;

use crate::AnySpecRtBoxed;

/// Map of step ID to its params' specs. `TypeMap<StepId,
/// AnySpecRtBoxed>` newtype.
///
/// The concrete `*ValueSpec` type can be obtained by calling
/// `.get(step_id)` with the correct type:
///
/// ```rust,ignore
/// let step_params_spec = MyStepParams::spec().build();
/// let mut params_specs = ParamsSpecs::new();
/// params_specs.insert(step_id!("my_step"), step_params_spec);
///
/// // later
///
/// let step_params_spec = params_specs.get::<MyStepParams, _>(&step_id!("my_step"));
/// ```
///
/// The information may not be of the same type across flows, as flows are
/// different in what they are doing.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(transparent)] // Needed to serialize as a map instead of a list.
pub struct ParamsSpecs(TypeMap<StepId, AnySpecRtBoxed>);

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
    pub fn into_inner(self) -> TypeMap<StepId, AnySpecRtBoxed> {
        self.0
    }
}

impl Deref for ParamsSpecs {
    type Target = TypeMap<StepId, AnySpecRtBoxed>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ParamsSpecs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<TypeMap<StepId, AnySpecRtBoxed>> for ParamsSpecs {
    fn from(type_map: TypeMap<StepId, AnySpecRtBoxed>) -> Self {
        Self(type_map)
    }
}
