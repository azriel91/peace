use crate::states::{ts::Current, States};

/// Current `State`s for all `ItemSpec`s.
///
/// This is strictly only present when the [`States`] are discovered in the
/// current execution. `States` read from the [`StatesCurrentFile`] are
/// inserted into [`Resources`] as [`StatesPrevious`], as those discovered
/// states may be out of date with the actual.
///
/// # Implementors
///
/// If an `ItemSpec`'s state discovery depends on the `State` of a previous
/// `ItemSpec`, then you should insert the predecessor's state into
/// [`Resources`], and reference that in the subsequent `FnSpec`'s [`Data`]:
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
/// You may reference [`StatesCurrent`] in `EnsureOpSpec::Data` for reading. It
/// is not mutable as `StatesCurrent` must remain unchanged so that all
/// `ItemSpec`s operate over consistent data.
///
/// [`Data`]: peace_data::Data
/// [`Resources`]: crate::Resources
/// [`StatesCurrentFile`] crate::paths::StatesCurrentFile
/// [`StatesPrevious`]: crate::states::StatesPrevious
pub type StatesCurrent = States<Current>;
