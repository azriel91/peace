use std::ops::Deref;

use peace_core::ItemSpecId;
use serde::Serialize;
use type_reg::untagged::TypeMap;

use crate::StatesDesiredMut;

/// Desired `State`s for all `ItemSpec`s. `TypeMap<ItemSpecId>` newtype.
///
/// # Implementors
///
/// If an `ItemSpec`'s desired state discovery depends on the desired `State` of
/// a previous `ItemSpec`, then you should insert the predecessor's desired
/// state into [`Resources`], and reference that in the subsequent `FnSpec`'s
/// [`Data`]:
///
/// ```rust
/// # use std::path::PathBuf;
/// #
/// # use peace_data::{Data, R};
/// #
/// /// Predecessor `FnSpec::Data`.
/// #[derive(Data, Debug)]
/// pub struct AppUploadParams<'op> {
///     /// Path to the application directory.
///     app_dir: W<'op, PathBuf>,
/// }
///
/// /// Successor `FnSpec::Data`.
/// #[derive(Data, Debug)]
/// pub struct AppInstallParams<'op> {
///     /// Path to the application directory.
///     app_dir: R<'op, PathBuf>,
///     /// Configuration to use.
///     config: W<'op, String>,
/// }
/// ```
///
/// You may reference [`StatesDesired`] in `EnsureOpSpec::Data` for reading. It
/// is not mutable as `StatesDesired` must remain unchanged so that all
/// `ItemSpec`s operate over consistent data.
///
/// [`Data`]: peace_data::Data
/// [`Resources`]: crate::Resources
#[derive(Debug, Default, Serialize)]
pub struct StatesDesired(TypeMap<ItemSpecId>);

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
    pub fn into_inner(self) -> TypeMap<ItemSpecId> {
        self.0
    }
}

impl Deref for StatesDesired {
    type Target = TypeMap<ItemSpecId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TypeMap<ItemSpecId>> for StatesDesired {
    fn from(type_map: TypeMap<ItemSpecId>) -> Self {
        Self(type_map)
    }
}

impl From<StatesDesiredMut> for StatesDesired {
    fn from(states_desired_mut: StatesDesiredMut) -> Self {
        Self(states_desired_mut.into_inner())
    }
}
