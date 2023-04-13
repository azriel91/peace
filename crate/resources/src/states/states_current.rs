use crate::states::{ts::Current, States};

/// Current `State`s for all `ItemSpec`s.
///
/// This is strictly only present when the [`States`] are discovered in the
/// current execution. `States` read from the [`StatesSavedFile`] are
/// inserted into [`Resources`] as [`StatesSaved`], as those discovered
/// states may be out of date with the actual.
///
/// # Implementors
///
/// If an `ItemSpec`'s state discovery depends on the `State` of a previous
/// `ItemSpec`, then you should insert the predecessor's state into
/// [`Resources`], and reference that in the subsequent `TryFnSpec`'s [`Data`]:
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
/// You may reference [`StatesCurrent`] in `ApplyFns::Data` for reading. It
/// is not mutable as `StatesCurrent` must remain unchanged so that all
/// `ItemSpec`s operate over consistent data.
///
/// [`Data`]: peace_data::Data
/// [`Resources`]: crate::Resources
/// [`StatesSavedFile`] crate::paths::StatesSavedFile
/// [`StatesSaved`]: crate::states::StatesSaved
pub type StatesCurrent = States<Current>;
