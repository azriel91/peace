use async_trait::async_trait;
use peace_data::Data;
use serde::{de::DeserializeOwned, Serialize};

use crate::{OpCheckStatus, State};

/// Defines the logic and data of an ensure operation.
///
/// This includes:
///
/// * Data that the operation reads from, or writes to.
/// * Logic to initialize that data.
/// * Logic to check if the operation is already done.
/// * Logic to do the operation.
/// * Physical state returned by the `exec` function.
///
/// Note that for the [`check`], [`exec_dry`], and [`exec`] functions, the
/// current state passed in includes both logical and physical state, as a
/// previous execution may have generated physical resources.
///
/// The desired state that is passed in is only the logical state, as this is
/// the part that can be managed.
///
/// This design is chosen so that multiple executions can be written to be
/// idempotent, which is the intended way this trait is to be implemented.
///
/// [`check`]: Self::check
/// [`exec_dry`]: Self::exec_dry
/// [`exec`]: Self::exec
#[async_trait(?Send)]
#[nougat::gat]
pub trait EnsureOpSpec {
    /// Error returned when any of the functions of this operation err.
    type Error: std::error::Error;

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

    /// State difference produced by [`StateDiffFnSpec`].
    ///
    /// See [`ItemSpec::StateDiff`] for more detail.
    ///
    /// [`StateDiffFnSpec`]: crate::ItemSpec::StateDiffFnSpec
    /// [`ItemSpec::StateDiff`]: crate::ItemSpec::StateDiff
    type StateDiff: Clone + Serialize + DeserializeOwned;

    /// Data that the operation reads from, or writes to.
    ///
    /// This may include:
    ///
    /// * Information calculated from previous operations.
    /// * Information written for subsequent operations.
    ///
    /// This differs from [`State`] (both physical and logical) whereby `State`
    /// is the state of the managed item, whereas `Data` is information
    /// computed at runtime to manage that state.
    type Data<'op>: Data<'op>
    where
        Self: 'op;

    /// Checks if the operation needs to be executed.
    ///
    /// If the current state is already the desired state, then the operation
    /// does not have to be executed.
    ///
    /// # Examples
    ///
    /// * For a file download operation, if the destination file differs from
    ///   the file on the server, then the file needs to be downloaded.
    ///
    /// * For a web application service operation, if the web service is
    ///   running, but reports a previous version, then the service may need to
    ///   be restarted.
    ///
    /// # Implementors
    ///
    /// This function call is intended to be cheap and fast.
    ///
    /// # Parameters
    ///
    /// * `data`: Runtime data that the operation reads from, or writes to.
    /// * `state_current`: Current [`State`] of the managed item, returned from
    ///   [`StateCurrentFnSpec`].
    /// * `state_desired`: Desired [`StateLogical`] of the managed item,
    ///   returned from [`StateDesiredFnSpec`].
    /// * `state_diff`: Desired [`StateLogical`] of the managed item, returned
    ///   from [`StateDiffFnSpec`].
    ///
    /// [`State`]: crate::State
    /// [`StateLogical`]: Self::StateLogical
    /// [`StateCurrentFnSpec`]: crate::ItemSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec`]: crate::ItemSpec::StateDesiredFnSpec
    /// [`StateDiffFnSpec`]: crate::ItemSpec::StateDiffFnSpec
    async fn check(
        data: Self::Data<'_>,
        state_current: &State<Self::StateLogical, Self::StatePhysical>,
        state_desired: &Self::StateLogical,
        diff: &Self::StateDiff,
    ) -> Result<OpCheckStatus, Self::Error>;

    /// Dry-run transform of the current state to the desired state.
    ///
    /// This will only be called if [`check`] returns [`ExecRequired`].
    ///
    /// This should mirror the logic in [`exec`], with the following
    /// differences:
    ///
    /// * When state will actually be altered, this would skip the logic.
    ///
    /// * Where there would be IDs received from an external system, a
    ///   placeholder ID should still be inserted into the runtime data. This
    ///   should allow subsequent `ItemSpec`s that rely on this one to use those
    ///   placeholders in their logic.
    ///
    /// # Implementors
    ///
    /// This function call is intended to be cheap.
    ///
    /// # Parameters
    ///
    /// * `data`: Runtime data that the operation reads from, or writes to.
    /// * `state_current`: Current [`State`] of the managed item, returned from
    ///   [`StateCurrentFnSpec`].
    /// * `state_desired`: Desired [`StateLogical`] of the managed item,
    ///   returned from [`StateDesiredFnSpec`].
    /// * `state_diff`: Desired [`StateLogical`] of the managed item, returned
    ///   from [`StateDiffFnSpec`].
    ///
    /// [`check`]: Self::check
    /// [`ExecRequired`]: crate::OpCheckStatus::ExecRequired
    /// [`State`]: crate::State
    /// [`StateCurrentFnSpec`]: crate::ItemSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec`]: crate::ItemSpec::StateDesiredFnSpec
    /// [`StateDiffFnSpec`]: crate::ItemSpec::StateDiffFnSpec
    /// [`StateLogical`]: Self::StateLogical
    async fn exec_dry(
        data: Self::Data<'_>,
        state_current: &State<Self::StateLogical, Self::StatePhysical>,
        state_desired: &Self::StateLogical,
        diff: &Self::StateDiff,
    ) -> Result<Self::StatePhysical, Self::Error>;

    /// Transforms the current state to the desired state.
    ///
    /// This will only be called if [`check`] returns [`ExecRequired`].
    ///
    /// # Parameters
    ///
    /// * `data`: Runtime data that the operation reads from, or writes to.
    /// * `state_current`: Current [`State`] of the managed item, returned from
    ///   [`StateCurrentFnSpec`].
    /// * `state_desired`: Desired [`StateLogical`] of the managed item,
    ///   returned from [`StateDesiredFnSpec`].
    /// * `state_diff`: Desired [`StateLogical`] of the managed item, returned
    ///   from [`StateDiffFnSpec`].
    ///
    /// [`check`]: Self::check
    /// [`ExecRequired`]: crate::OpCheckStatus::ExecRequired
    /// [`State`]: crate::State
    /// [`StateCurrentFnSpec`]: crate::ItemSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec`]: crate::ItemSpec::StateDesiredFnSpec
    /// [`StateDiffFnSpec`]: crate::ItemSpec::StateDiffFnSpec
    /// [`StateLogical`]: Self::StateLogical
    async fn exec(
        data: Self::Data<'_>,
        state_current: &State<Self::StateLogical, Self::StatePhysical>,
        state_desired: &Self::StateLogical,
        diff: &Self::StateDiff,
    ) -> Result<Self::StatePhysical, Self::Error>;
}
