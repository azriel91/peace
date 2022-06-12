use async_trait::async_trait;
use peace_data::Data;

use crate::OpCheckStatus;

/// Defines the logic and data of an ensure operation.
///
/// This includes:
///
/// * Data that the operation reads from, or writes to.
/// * Logic to initialize that data.
/// * Logic to check if the operation is already done.
/// * Logic to do the operation.
/// * Physical state returned by the `exec` function.
#[async_trait]
pub trait EnsureOpSpec<'op> {
    /// Error returned when any of the functions of this operation err.
    type Error: std::error::Error;

    /// Logical state of the managed item.
    ///
    /// This is the type returned by the [`StatusFnSpec`], and is used by
    /// [`EnsureOpSpec`] to determine if [`exec`] needs to be run.
    ///
    /// See [`FullSpec::State`] for more detail.
    ///
    /// [`StatusFnSpec`]: crate::FullSpec::StatusFnSpec
    /// [`EnsureOpSpec`]: crate::FullSpec::EnsureOpSpec
    /// [`exec`]: Self::exec
    /// [`FullSpec::State`]: crate::FullSpec::State
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
    /// This differs from [`StateLogical`] whereby `StateLogical` is the state
    /// of the managed item, whereas `Data` is information computed at
    /// runtime to manage that state.
    ///
    /// [`StateLogical`]: Self::State
    type Data: Data<'op>;

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
    async fn desired(data: Self::Data) -> Result<Self::StateLogical, Self::Error>
    // Without this, we hit a similar issue to: https://github.com/dtolnay/async-trait/issues/47
    // impl has stricter requirements than trait
    where
        'op: 'async_trait;

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
    /// * `state_current`: Current [`StateLogical`] of the managed item,
    ///   returned from [`StatusFnSpec`].
    /// * `state_desired`: Desired [`StateLogical`] of the managed item,
    ///   returned from [`Self::desired`].
    ///
    /// [`StateLogical`]: Self::State
    /// [`StatusFnSpec`]: crate::FullSpec::StatusFnSpec
    async fn check(
        data: Self::Data,
        state_current: &Self::StateLogical,
        state_desired: &Self::StateLogical,
    ) -> Result<OpCheckStatus, Self::Error>
    // Without this, we hit a similar issue to: https://github.com/dtolnay/async-trait/issues/47
    // impl has stricter requirements than trait
    where
        'op: 'async_trait;

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
    /// [`check`]: crate::EnsureOpSpec::check
    /// [`exec`]: crate::EnsureOpSpec::exec
    /// [`ExecRequired`]: crate::OpCheckStatus::ExecRequired
    async fn exec_dry(
        data: Self::Data,
        state_current: &Self::StateLogical,
        state_desired: &Self::StateLogical,
    ) -> Result<Self::StatePhysical, Self::Error>
    // Without this, we hit a similar issue to: https://github.com/dtolnay/async-trait/issues/47
    // impl has stricter requirements than trait
    where
        'op: 'async_trait;

    /// Transforms the current state to the desired state.
    ///
    /// This will only be called if [`check`] returns [`ExecRequired`].
    ///
    /// [`check`]: crate::EnsureOpSpec::check
    /// [`ExecRequired`]: crate::OpCheckStatus::ExecRequired
    async fn exec(
        data: Self::Data,
        state_current: &Self::StateLogical,
        state_desired: &Self::StateLogical,
    ) -> Result<Self::StatePhysical, Self::Error>
    // Without this, we hit a similar issue to: https://github.com/dtolnay/async-trait/issues/47
    // impl has stricter requirements than trait
    where
        'op: 'async_trait;
}
