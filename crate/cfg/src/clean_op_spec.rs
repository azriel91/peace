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
    /// IDs of resources produced by the operation.
    ///
    /// See [`FullSpec::ResIds`] for more detail.
    ///
    /// [`FullSpec::ResIds`]: crate::FullSpec::ResIds
    type ResIds;

    /// Data that the operation reads from, or writes to.
    ///
    /// This may include:
    ///
    /// * Information calculated from previous operations.
    /// * Information written for subsequent operations.
    ///
    /// This differs from [`State`] whereby `State` is the state of the managed
    /// item, whereas `Data` is information computed at runtime to manage that
    /// state.
    ///
    /// [`State`]: Self::State
    type Data: Data<'op>;

    /// Error returned when any of the functions of this operation err.
    type Error: std::error::Error;

    /// Checks if the clean operation needs to be executed.
    ///
    /// If the resources referred to by [`ResIds`] have already been cleaned up,
    /// then the operation does not have to be executed.
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
    /// * `res_ids`: Resource IDs of the managed item, returned from the
    ///   [`EnsureOpSpec`]'s [`OpSpec::exec`] function.
    ///
    /// [`EnsureOpSpec`]: crate::FullSpec::EnsureOpSpec
    /// [`OpSpec::exec`]: crate::OpSpec::exec
    async fn check(data: Self::Data, res_ids: &Self::ResIds) -> Result<OpCheckStatus, Self::Error>
    // Without this, we hit a similar issue to: https://github.com/dtolnay/async-trait/issues/47
    // impl has stricter requirements than trait
    where
        'op: 'async_trait;

    /// Dry-run clean up of resources referenced by `ResIds`.
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
    /// * `res_ids`: Resource IDs of the managed item, returned from the
    ///   [`EnsureOpSpec`]'s [`OpSpec::exec`] function.
    ///
    /// [`check`]: Self::check
    /// [`exec`]: Self::exec
    /// [`EnsureOpSpec`]: crate::FullSpec::EnsureOpSpec
    /// [`OpSpec::exec`]: crate::OpSpec::exec
    /// [`ExecRequired`]: crate::OpCheckStatus::ExecRequired
    async fn exec_dry(data: Self::Data, res_ids: &Self::ResIds) -> Result<(), Self::Error>
    // Without this, we hit a similar issue to: https://github.com/dtolnay/async-trait/issues/47
    // impl has stricter requirements than trait
    where
        'op: 'async_trait;

    /// Cleans up resources referenced by `ResIds`
    ///
    /// This will only be called if [`check`] returns [`ExecRequired`].
    ///
    /// # Parameters
    ///
    /// * `data`: Runtime data that the operation reads from, or writes to.
    /// * `res_ids`: Resource IDs of the managed item, returned from the
    ///   [`EnsureOpSpec`]'s [`OpSpec::exec`] function.
    ///
    /// [`check`]: Self::check
    /// [`EnsureOpSpec`]: crate::FullSpec::EnsureOpSpec
    /// [`OpSpec::exec`]: crate::OpSpec::exec
    /// [`ExecRequired`]: crate::OpCheckStatus::ExecRequired
    async fn exec(data: Self::Data, res_ids: &Self::ResIds) -> Result<(), Self::Error>
    // Without this, we hit a similar issue to: https://github.com/dtolnay/async-trait/issues/47
    // impl has stricter requirements than trait
    where
        'op: 'async_trait;
}
