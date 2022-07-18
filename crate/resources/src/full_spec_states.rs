use std::ops::{Deref, DerefMut};

use peace_core::FullSpecId;
use serde::Serialize;
use type_reg::untagged::TypeMap;

/// `State`s for all `FullSpec`s. `TypeMap<FullSpecId>` newtype.
///
/// # Note
///
/// `Resources` stores [`FullSpecStatesRw`], so you should use that in `FnSpec`
/// or `OpSpec` [`Data`]:
///
/// ```rust
/// use peace_data::{Data, R};
/// use peace_resources::FullSpecStatesRw;
///
/// /// Parameters for a `FnSpec` or an `OpSpec`.
/// #[derive(Data, Debug)]
/// pub struct StatusFnParams<'op> {
///     /// Client to make web requests.
///     states: R<'op, FullSpecStatesRw>,
/// }
/// ```
///
/// [`Data`]: peace_data::Data
/// [`FullSpecStatesRw`]: crate::FullSpecStatesRw
#[derive(Debug, Default, Serialize)]
pub struct FullSpecStates(TypeMap<FullSpecId>);

impl FullSpecStates {
    /// Returns a new `FullSpecStates`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `FullSpecStates` with the specified capacity.
    ///
    /// The `FullSpecStates` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<FullSpecId> {
        self.0
    }
}

impl Deref for FullSpecStates {
    type Target = TypeMap<FullSpecId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FullSpecStates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<TypeMap<FullSpecId>> for FullSpecStates {
    fn from(type_map: TypeMap<FullSpecId>) -> Self {
        Self(type_map)
    }
}
