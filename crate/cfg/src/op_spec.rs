use async_trait::async_trait;
use peace_data::Data;

use crate::OpCheckStatus;

/// Defines the logic and data of an operation.
///
/// This includes:
///
/// * Data that the operation reads from, or writes to.
/// * Logic to initialize that data.
/// * Logic to check if the operation is already done.
/// * Logic to do the operation.
/// * Return type of the operation, depending on its purpose.
#[async_trait]
pub trait OpSpec<'op> {
    /// State of the managed item.
    ///
    /// This is the type returned by the [`StatusFnSpec`], and is used by
    /// [`EnsureOpSpec`] to determine if [`OpSpec::exec`] needs to be run.
    ///
    /// See [`FullSpec::State`] for more detail.
    ///
    /// [`StatusFnSpec`]: crate::FullSpec::StatusFnSpec
    /// [`EnsureOpSpec`]: crate::FullSpec::EnsureOpSpec
    /// [`OpSpec::exec`]: crate::OpSpec::exec
    /// [`FullSpec::State`]: crate::FullSpec::State
    type State;

    /// IDs of resources produced by the operation.
    ///
    /// This is provided to the clean up logic to determine what to clean up.
    ///
    /// These should be physical IDs, not logical IDs. A logical resource ID is
    /// defined by code, and does not change. A physical resource ID is one
    /// generated during execution, which generally is random or computed.
    ///
    /// # Examples
    ///
    /// The following are examples of logical IDs and corresponding physical
    /// IDs:
    ///
    /// * If the operation creates a file, the ID *may* be the full file path,
    ///   or it may be the file name, assuming the file path may be deduced by
    ///   the clean up logic from [`Data`].
    ///
    /// * If the operation instantiates a virtual machine on a cloud platform,
    ///   this may be the ID of the instance so that it may be terminated.
    ///
    /// | Logical ID               | Physical ID                            |
    /// | ------------------------ | -------------------------------------- |
    /// | `app.file_path`          | `/mnt/data/app.zip`                    |
    /// | `app_server_instance_id` | `ef34a9a4-0c02-45a6-96ec-a4db06d4980c` |
    /// | `app_server.address`     | `10.0.0.1`                             |
    ///
    /// # Notes
    ///
    /// `ResIds` is separate from [`State`] because when computing the [`State`]
    /// in [`OpSpec::desired`], it may be impossible to know the physical ID
    /// of resources produced, such as virtual machine instance IDs.
    ///
    /// [`Data`]: crate::OpSpec::Data
    /// [`State`]: Self::State
    /// [`OpSpec::desired`]: crate::OpSpec::desired
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
    async fn desired(data: Self::Data) -> Result<Self::State, Self::Error>
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
    /// * `state_current`: Current [`State`] of the managed item, returned from
    ///   [`StatusFnSpec`].
    /// * `state_desired`: Desired [`State`] of the managed item, returned from
    ///   [`Self::desired`].
    ///
    /// [`State`]: Self::State
    /// [`StatusFnSpec`]: crate::FullSpec::StatusFnSpec
    async fn check(
        data: Self::Data,
        state_current: &Self::State,
        state_desired: &Self::State,
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
    /// [`check`]: crate::OpSpec::check
    /// [`exec`]: crate::OpSpec::exec
    /// [`ExecRequired`]: crate::OpCheckStatus::ExecRequired
    async fn exec_dry(
        data: Self::Data,
        state_current: &Self::State,
        state_desired: &Self::State,
    ) -> Result<Self::ResIds, Self::Error>
    // Without this, we hit a similar issue to: https://github.com/dtolnay/async-trait/issues/47
    // impl has stricter requirements than trait
    where
        'op: 'async_trait;

    /// Transforms the current state to the desired state.
    ///
    /// This will only be called if [`check`] returns [`ExecRequired`].
    ///
    /// [`check`]: crate::OpSpec::check
    /// [`ExecRequired`]: crate::OpCheckStatus::ExecRequired
    async fn exec(
        data: Self::Data,
        state_current: &Self::State,
        state_desired: &Self::State,
    ) -> Result<Self::ResIds, Self::Error>
    // Without this, we hit a similar issue to: https://github.com/dtolnay/async-trait/issues/47
    // impl has stricter requirements than trait
    where
        'op: 'async_trait;
}
