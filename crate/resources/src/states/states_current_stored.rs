use std::marker::PhantomData;

use crate::states::{
    ts::{Current, CurrentStored},
    States,
};

/// Stored current `State`s for all `Item`s.
///
/// This is loaded into [`Resources`] at the beginning of any command execution,
/// from the [`StatesCurrentFile`].
///
/// This is distinct from [`StatesCurrent`] to address the following use cases:
///
/// * Discovering current state from what is recorded in the
///   [`StatesCurrentFile`].
/// * Discovering current state and comparing it with previous state within the
///   same execution.
///
/// # Implementors
///
/// If a `Step`'s state discovery depends on the `State` of a previous
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
/// You may reference [`StatesCurrentStored`] in `ApplyFns::Data` for reading.
/// It is not mutable as `StatesCurrentStored` must remain unchanged so that all
/// `Item`s operate over consistent data.
///
/// [`StatesCurrentFile`]: crate::paths::StatesCurrentFile
/// [`Data`]: peace_data::Data
/// [`Resources`]: crate::Resources
pub type StatesCurrentStored = States<CurrentStored>;

impl From<States<Current>> for States<CurrentStored> {
    fn from(states_current: States<Current>) -> Self {
        let States(type_map, PhantomData) = states_current;

        Self(type_map, PhantomData)
    }
}
