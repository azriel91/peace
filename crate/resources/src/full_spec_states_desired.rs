use std::ops::{Deref, DerefMut};

use peace_core::FullSpecId;
use serde::Serialize;
use type_reg::untagged::TypeMap;

/// `State`s for all `FullSpec`s. `TypeMap<FullSpecId>` newtype.
///
/// # Consumer Note
///
/// For `StatusDesiredFnSpec`, [`Resources`] stores [`FullSpecStatesDesiredRw`], so *if* a
/// `FullSpec` depends on the `State` of a previous `FullSpec`, then you should
/// reference [`FullSpecStatesDesiredRw`] in the subsequent `FnSpec`'s [`Data`]:
///
/// ```rust
/// use peace_data::{Data, R};
/// use peace_resources::FullSpecStatesDesiredRw;
///
/// /// Parameters for the `StatusDesiredFnSpec`.
/// #[derive(Data, Debug)]
/// pub struct EnsureOpSpecParams<'op> {
///     /// Client to make web requests.
///     states: R<'op, FullSpecStatesDesiredRw>,
/// }
///
/// // later
/// // let states = status_fn_params.states.read().await;
/// // let predecessor_state = states.get(full_spec_id!("predecessor_id"));
/// ```
///
/// For `EnsureOpSpec`, you may reference [`FullSpecStatesDesired`] in
/// `EnsureOpSpec::Data` for reading -- mutating desired `State` is not intended after
/// it has been determined.
///
/// ## Rationale
///
/// [`FullSpecStatesDesired`] needs to be written to during `StatusDesiredFnSpec::exec`, and a
/// `RwLock` is needed at that stage to allow for concurrent execution.
///
/// [`Data`]: peace_data::Data
/// [`FullSpecStatesDesiredRw`]: crate::FullSpecStatesDesiredRw
/// [`Resources`]: crate::Resources
#[derive(Debug, Default, Serialize)]
pub struct FullSpecStatesDesired(TypeMap<FullSpecId>);

impl FullSpecStatesDesired {
    /// Returns a new `FullSpecStatesDesired`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `FullSpecStatesDesired` with the specified capacity.
    ///
    /// The `FullSpecStatesDesired` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<FullSpecId> {
        self.0
    }
}

impl Deref for FullSpecStatesDesired {
    type Target = TypeMap<FullSpecId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FullSpecStatesDesired {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<TypeMap<FullSpecId>> for FullSpecStatesDesired {
    fn from(type_map: TypeMap<FullSpecId>) -> Self {
        Self(type_map)
    }
}
