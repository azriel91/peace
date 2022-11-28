use async_trait::async_trait;
use peace_data::Data;
use serde::{de::DeserializeOwned, Serialize};

use crate::State;

/// Defines the logic and data of the `State` diffing function.
///
/// # Design Note
///
/// There was personal internal debate whether this should just be a plain
/// `async fn` in `ItemSpec`, instead of its own trait.
///
/// If it's a plain async function:
///
/// * Simpler function interface.
/// * Can run everything concurrently and in parallel, as state is immutable,
///   and no other resources are accessed.
///
/// Questions:
///
/// * Will users want to have access to other resources when computing a diff?
///
///     Possibly -- application parameters such as command line arguments,
///     environmental variables, and configuration files may be used to alter
///     what is returned in the diff. `peace` only requires that it is
///
/// * Should we recommend they store all the information they need in the
///   `State`?
///
///     No, from the previous point, other information that may affect the diff
///     may not belong in the `State`.
#[async_trait(?Send)]
pub trait StateDiffFnSpec {
    /// Logical state of the managed item.
    ///
    /// This is the type returned by the [`StateCurrentFnSpec`], and is used by
    /// [`EnsureOpSpec`] to determine if [`exec`] needs to be run.
    ///
    /// See [`ItemSpec::StateLogical`] for more detail.
    ///
    /// [`StateCurrentFnSpec`]: crate::ItemSpec::StateCurrentFnSpec
    /// [`EnsureOpSpec`]: crate::ItemSpec::EnsureOpSpec
    /// [`exec`]: Self::exec
    /// [`ItemSpec::StateLogical`]: crate::ItemSpec::StateLogical
    type StateLogical: Clone + Serialize + DeserializeOwned;

    /// Physical state produced by the operation.
    ///
    /// See [`ItemSpec::StatePhysical`] for more detail.
    ///
    /// [`ItemSpec::StatePhysical`]: crate::ItemSpec::StatePhysical
    type StatePhysical: Clone + Serialize + DeserializeOwned;

    /// State difference returned by this function.
    ///
    /// See [`ItemSpec::StateDiff`] for more detail.
    ///
    /// [`StateDiffFnSpec`]: crate::ItemSpec::StateDiffFnSpec
    /// [`ItemSpec::StateDiff`]: crate::ItemSpec::StateDiff
    type StateDiff;

    /// Data that the function reads from, or writes to.
    ///
    /// These may be parameters to the function, or information calculated from
    /// previous functions.
    type Data<'op>: Data<'op>
    where
        Self: 'op;

    /// Error returned when this function errs.
    type Error: std::error::Error;

    /// Executes this function.
    async fn exec(
        data: Self::Data<'_>,
        state_current: &State<Self::StateLogical, Self::StatePhysical>,
        state_desired: &Self::StateLogical,
    ) -> Result<Self::StateDiff, Self::Error>;
}
