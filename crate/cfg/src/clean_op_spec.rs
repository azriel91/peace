use async_trait::async_trait;
use peace_data::Data;

use crate::OpCheckStatus;

/// Defines the logic and data to clean up resources.
///
/// This includes:
///
/// * Data that the operation reads from, or writes to.
/// * Logic to initialize that data.
/// * Logic to check if the resources have already been cleaned.
/// * Logic to do the cleaning.
#[async_trait]
pub trait CleanOpSpec<'op> {
    /// Error returned when any of the functions of this operation err.
    type Error: std::error::Error;

    /// IDs of resources produced by the operation.
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

    /// Checks if the clean operation needs to be executed.
    ///
    /// If the resources referred to by [`StatePhysical`] have already been
    /// cleaned up, then the operation does not have to be executed.
    ///
    /// # Examples
    ///
    /// * For a file download operation, if the destination file exists, then
    ///   the file needs to be deleted.
    ///
    /// * For a web application installation operation, if the web service is
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
    /// * `state_physical`: Physical state of the managed item, returned from
    ///   the [`EnsureOpSpec::exec`] function.
    ///
    /// [`EnsureOpSpec::exec`]: crate::EnsureOpSpec::exec
    async fn check(
        data: Self::Data,
        state_physical: &Self::StatePhysical,
    ) -> Result<OpCheckStatus, Self::Error>
    // Without this, we hit a similar issue to: https://github.com/dtolnay/async-trait/issues/47
    // impl has stricter requirements than trait
    where
        'op: 'async_trait;

    /// Dry-run clean up of resources referenced by `StatePhysical`.
    ///
    /// This will only be called if [`check`] returns [`ExecRequired`].
    ///
    /// This should mirror the logic in [`exec`], with the following
    /// differences:
    ///
    /// * When items will actually be removed, this would skip the logic.
    ///
    /// # Implementors
    ///
    /// This function call is intended to be cheap.
    ///
    /// # Parameters
    ///
    /// * `data`: Runtime data that the operation reads from, or writes to.
    /// * `state_physical`: Physical state of the managed item, returned from
    ///   the [`EnsureOpSpec::exec`] function.
    ///
    /// [`check`]: Self::check
    /// [`exec`]: Self::exec
    /// [`EnsureOpSpec::exec`]: crate::EnsureOpSpec::exec
    /// [`ExecRequired`]: crate::OpCheckStatus::ExecRequired
    async fn exec_dry(
        data: Self::Data,
        state_physical: &Self::StatePhysical,
    ) -> Result<(), Self::Error>
    // Without this, we hit a similar issue to: https://github.com/dtolnay/async-trait/issues/47
    // impl has stricter requirements than trait
    where
        'op: 'async_trait;

    /// Cleans up resources referenced by `StatePhysical`
    ///
    /// This will only be called if [`check`] returns [`ExecRequired`].
    ///
    /// # Parameters
    ///
    /// * `data`: Runtime data that the operation reads from, or writes to.
    /// * `state_physical`: Physical state of the managed item, returned from
    ///   the [`EnsureOpSpec::exec`] function.
    ///
    /// [`check`]: Self::check
    /// [`EnsureOpSpec::exec`]: crate::EnsureOpSpec::exec
    /// [`ExecRequired`]: crate::OpCheckStatus::ExecRequired
    async fn exec(
        data: Self::Data,
        state_physical: &Self::StatePhysical,
    ) -> Result<(), Self::Error>
    // Without this, we hit a similar issue to: https://github.com/dtolnay/async-trait/issues/47
    // impl has stricter requirements than trait
    where
        'op: 'async_trait;
}
