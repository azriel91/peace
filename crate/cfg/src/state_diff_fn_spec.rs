use async_trait::async_trait;
use peace_data::Data;
use serde::{de::DeserializeOwned, Serialize};

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
    /// [`ApplyOpSpec`] to determine if [`exec`] needs to be run.
    ///
    /// See [`ItemSpec::State`] for more detail.
    ///
    /// [`StateCurrentFnSpec`]: crate::ItemSpec::StateCurrentFnSpec
    /// [`ApplyOpSpec`]: crate::ItemSpec::ApplyOpSpec
    /// [`exec`]: Self::exec
    /// [`ItemSpec::State`]: crate::ItemSpec::State
    type State: Clone + Serialize + DeserializeOwned;

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
        state_current: &Self::State,
        state_desired: &Self::State,
    ) -> Result<Self::StateDiff, Self::Error>;
}
