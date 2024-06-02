use std::marker::PhantomData;

use crate::states::{
    ts::{Current, CurrentStored},
    States,
};

/// Current `State`s for all `Item`s.
///
/// This is strictly only present when the [`States`] are discovered in the
/// current execution. `States` read from the [`StatesCurrentFile`] are
/// inserted into [`Resources`] as [`StatesCurrentStored`], as those discovered
/// states may be out of date with the actual.
///
/// # Implementors
///
/// If an `Item`'s state discovery depends on the `State` of a previous
/// `Item`, then you should insert the predecessor's state into
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
/// `Item`s operate over consistent data.
///
/// [`Data`]: peace_data::Data
/// [`Resources`]: crate::Resources
/// [`StatesCurrentFile`] crate::paths::StatesCurrentFile
/// [`StatesCurrentStored`]: crate::states::StatesCurrentStored
pub type StatesCurrent = States<Current>;

impl From<States<CurrentStored>> for States<Current> {
    fn from(states_current_stored: States<CurrentStored>) -> Self {
        let States(type_map, PhantomData) = states_current_stored;

        Self(type_map, PhantomData)
    }
}
