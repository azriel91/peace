use async_trait::async_trait;
use peace_data::Data;

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
#[async_trait]
#[nougat::gat]
pub trait EnsureOpSpec {
    /// Error returned when any of the functions of this operation err.
    type Error: std::error::Error;

    /// Logical state of the managed item.
    ///
    /// This is the type returned by the [`StatusFnSpec`], and is used by
    /// [`EnsureOpSpec`] to determine if [`exec`] needs to be run.
    ///
    /// See [`FullSpec::StateLogical`] for more detail.
    ///
    /// [`StatusFnSpec`]: crate::FullSpec::StatusFnSpec
    /// [`EnsureOpSpec`]: crate::FullSpec::EnsureOpSpec
    /// [`exec`]: Self::exec
    /// [`FullSpec::StateLogical`]: crate::FullSpec::StateLogical
    type StateLogical;

    /// Physical state produced by the operation.
    ///
    /// See [`FullSpec::StatePhysical`] for more detail.
    ///
    /// [`FullSpec::StatePhysical`]: crate::FullSpec::StatePhysical
    type StatePhysical;

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

    /// Returns the desired state of the managed item.
    ///
    /// # Examples
    ///
    /// * For a file download operation, the desired state could be the
    ///   destination path and a content hash.
    ///
    /// * For a web application service operation, the desired state could be
    ///   the web service is running on the latest version.
    ///
    /// # Implementors
    ///
    /// This function call is intended to be cheap and fast.
    async fn desired(data: Self::Data<'_>) -> Result<Self::StateLogical, Self::Error>;

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
    ///   [`StatusFnSpec`].
    /// * `state_desired`: Desired [`StateLogical`] of the managed item,
    ///   returned from [`Self::desired`].
    ///
    /// [`State`]: crate::State
    /// [`StateLogical`]: Self::StateLogical
    /// [`StatusFnSpec`]: crate::FullSpec::StatusFnSpec
    async fn check(
        data: Self::Data<'_>,
        state_current: &State<Self::StateLogical, Self::StatePhysical>,
        state_desired: &Self::StateLogical,
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
    ///   should allow subsequent `FullSpec`s that rely on this one to use those
    ///   placeholders in their logic.
    ///
    /// # Implementors
    ///
    /// This function call is intended to be cheap.
    ///
    /// [`check`]: Self::check
    /// [`exec`]: Self::exec
    /// [`ExecRequired`]: crate::OpCheckStatus::ExecRequired
    async fn exec_dry(
        data: Self::Data<'_>,
        state_current: &State<Self::StateLogical, Self::StatePhysical>,
        state_desired: &Self::StateLogical,
    ) -> Result<Self::StatePhysical, Self::Error>;

    /// Transforms the current state to the desired state.
    ///
    /// This will only be called if [`check`] returns [`ExecRequired`].
    ///
    /// [`check`]: Self::check
    /// [`ExecRequired`]: crate::OpCheckStatus::ExecRequired
    async fn exec(
        data: Self::Data<'_>,
        state_current: &State<Self::StateLogical, Self::StatePhysical>,
        state_desired: &Self::StateLogical,
    ) -> Result<Self::StatePhysical, Self::Error>;
}
