use std::ops::{Deref, DerefMut};

use peace_core::FullSpecId;
use serde::Serialize;
use type_reg::untagged::TypeMap;

/// `State`s for all `FullSpec`s. `TypeMap<FullSpecId>` newtype.
///
/// # Consumer Note
///
/// For `StateNowFnSpec`, [`Resources`] stores [`StatesRw`], so *if* a
/// `FullSpec` depends on the `State` of a previous `FullSpec`, then you should
/// reference [`StatesRw`] in the subsequent `FnSpec`'s [`Data`]:
///
/// ```rust
/// use peace_data::{Data, R};
/// use peace_resources::StatesRw;
///
/// /// Parameters for the `StateNowFnSpec`.
/// #[derive(Data, Debug)]
/// pub struct StatusFnParams<'op> {
///     /// Client to make web requests.
///     states: R<'op, StatesRw>,
/// }
///
/// // later
/// // let states = state_now_fn_params.states.read().await;
/// // let predecessor_state = states.get(full_spec_id!("predecessor_id"));
/// ```
///
/// For `EnsureOpSpec`, you may reference [`States`] in
/// `EnsureOpSpec::Data` for reading -- mutating `State` is not intended after
/// it has been read.
///
/// ## Rationale
///
/// [`States`] needs to be written to during `StateNowFnSpec::exec`, and a
/// `RwLock` is needed at that stage to allow for concurrent execution.
///
/// [`Data`]: peace_data::Data
/// [`StatesRw`]: crate::StatesRw
/// [`Resources`]: crate::Resources
#[derive(Debug, Default, Serialize)]
pub struct States(TypeMap<FullSpecId>);

impl States {
    /// Returns a new `States`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `States` with the specified capacity.
    ///
    /// The `States` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<FullSpecId> {
        self.0
    }
}

impl Deref for States {
    type Target = TypeMap<FullSpecId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for States {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<TypeMap<FullSpecId>> for States {
    fn from(type_map: TypeMap<FullSpecId>) -> Self {
        Self(type_map)
    }
}
