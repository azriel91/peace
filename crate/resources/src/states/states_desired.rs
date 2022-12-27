use crate::states::{ts::Desired, States};

/// Desired `State`s for all `ItemSpec`s.
///
/// This is typically `TypeMap<ItemSpecId, State<StateLogical, Placeholder>>`,
/// where [`Placeholder`] is not used in `StateDiff` computations.
///
/// # Implementors
///
/// If an `ItemSpec`'s desired state discovery depends on the desired `State` of
/// a previous `ItemSpec`, then you should insert the predecessor's desired
/// state into [`Resources`], and reference that in the subsequent
/// `StateDiscoverFnSpec`'s [`Data`]:
///
/// ```rust
/// # use std::path::PathBuf;
/// #
/// # use peace_data::{Data, R};
/// #
/// /// Predecessor `StateDiscoverFnSpec::Data`.
/// #[derive(Data, Debug)]
/// pub struct AppUploadParams<'op> {
///     /// Path to the application directory.
///     app_dir: W<'op, PathBuf>,
/// }
///
/// /// Successor `StateDiscoverFnSpec::Data`.
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
/// [`Placeholder`]: peace_cfg::state::Placeholder
/// [`Data`]: peace_data::Data
/// [`Resources`]: crate::Resources
pub type StatesDesired = States<Desired>;
