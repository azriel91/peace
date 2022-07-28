use std::ops::Deref;

use peace_core::FullSpecId;
use serde::Serialize;
use type_reg::untagged::TypeMap;

use crate::StatesDesiredMut;

/// Desired `State`s for all `FullSpec`s. `TypeMap<FullSpecId>` newtype.
///
/// # Implementors
///
/// For `StateDesiredFnSpec`, [`Resources`] stores [`StatesDesiredRw`], so *if*
/// a `FullSpec` depends on the `State` of a previous `FullSpec`, then you
/// should reference [`StatesDesiredRw`] in the subsequent `FnSpec`'s [`Data`]:
///
/// ```rust
/// use peace_data::{Data, R};
/// use peace_resources::StatesDesiredRw;
///
/// /// Parameters for the `StateDesiredFnSpec`.
/// #[derive(Data, Debug)]
/// pub struct EnsureOpSpecParams<'op> {
///     /// Client to make web requests.
///     states: R<'op, StatesDesiredRw>,
/// }
///
/// // later
/// // let states = state_now_fn_params.states.read().await;
/// // let predecessor_state = states.get(full_spec_id!("predecessor_id"));
/// ```
///
/// You may reference [`StatesDesired`] in `EnsureOpSpec::Data` for reading. It
/// is not mutable as `StatesDesired` must remain unchanged so that all
/// `FullSpec`s operate over consistent data.
///
/// ## Rationale
///
/// [`StatesDesired`] needs to be written to during `StateDesiredFnSpec::exec`,
/// and a `RwLock` is needed at that stage to allow for concurrent execution.
///
/// [`Data`]: peace_data::Data
/// [`StatesDesiredRw`]: crate::StatesDesiredRw
/// [`Resources`]: crate::Resources
#[derive(Debug, Default, Serialize)]
pub struct StatesDesired(TypeMap<FullSpecId>);

impl StatesDesired {
    /// Returns a new `StatesDesired` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `StatesDesired` map with the specified capacity.
    ///
    /// The `StatesDesired` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<FullSpecId> {
        self.0
    }
}

impl Deref for StatesDesired {
    type Target = TypeMap<FullSpecId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TypeMap<FullSpecId>> for StatesDesired {
    fn from(type_map: TypeMap<FullSpecId>) -> Self {
        Self(type_map)
    }
}

impl From<StatesDesiredMut> for StatesDesired {
    fn from(states_desired_mut: StatesDesiredMut) -> Self {
        Self(states_desired_mut.into_inner())
    }
}
