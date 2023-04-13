use crate::states::{ts::Desired, States};

/// Desired `State`s for all `ItemSpec`s.
///
/// This is typically `TypeMap<ItemSpecId, State>`,
/// where [`External`] is not necessarily used in `StateDiff` computations.
///
/// # Implementors
///
/// If an `ItemSpec`'s desired state discovery depends on the desired `State` of
/// a previous `ItemSpec`, then you should insert the predecessor's desired
/// state into [`Resources`], and reference that in the subsequent
/// `TryFnSpec`'s [`Data`]:
///
/// ```rust
/// # use std::path::PathBuf;
/// #
/// # use peace_data::{Data, R};
/// #
/// /// Predecessor `TryFnSpec::Data`.
/// #[derive(Data, Debug)]
/// pub struct AppUploadParams<'exec> {
///     /// Path to the application directory.
///     app_dir: W<'exec, PathBuf>,
/// }
///
/// /// Successor `TryFnSpec::Data`.
/// #[derive(Data, Debug)]
/// pub struct AppInstallParams<'exec> {
///     /// Path to the application directory.
///     app_dir: R<'exec, PathBuf>,
///     /// Configuration to use.
///     config: W<'exec, String>,
/// }
/// ```
///
/// You may reference [`StatesDesired`] in `ApplyFns::Data` for reading. It
/// is not mutable as `StatesDesired` must remain unchanged so that all
/// `ItemSpec`s operate over consistent data.
///
/// [`External`]: peace_cfg::state::External
/// [`Data`]: peace_data::Data
/// [`Resources`]: crate::Resources
pub type StatesDesired = States<Desired>;
