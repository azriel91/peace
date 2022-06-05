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
    /// State that the [`FullSpec`] manages.
    ///
    /// This is the type returned by the [`StatusFnSpec`], and is used by
    /// [`EnsureOpSpec`] and [`CleanOpSpec`] to determine if their [`exec`]
    /// function needs to be run.
    ///
    /// [`FullSpec`]: crate::FullSpec
    /// [`StatusFnSpec`]: crate::FullSpec::StatusFnSpec
    /// [`EnsureOpSpec`]: crate::FullSpec::EnsureOpSpec
    /// [`CleanOpSpec`]: crate::FullSpec::CleanOpSpec
    /// [`exec`]: crate::OpSpec::exec
    type State;

    /// Return type of the operation.
    ///
    /// This varies depending on the type of the operation:
    ///
    /// * For an [ensure operation], these are the [resource IDs] produced by
    ///   the operation.
    /// * For a [clean operation], these are the [resource IDs] cleaned by the
    ///   operation.
    ///
    /// [ensure operation]: crate::FullSpec::EnsureOpSpec
    /// [clean operation]: crate::FullSpec::CleanOpSpec
    /// [resource IDs]: crate::FullSpec::ResIds
    type Output;

    /// Data that the operation reads from, or writes to.
    ///
    /// These may be parameters to the function, or information calculated from
    /// previous operations.
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
    async fn check(data: Self::Data, state: &Self::State) -> Result<OpCheckStatus, Self::Error>
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
    async fn exec(data: Self::Data) -> Result<Self::Output, Self::Error>
    // Without this, we hit a similar issue to: https://github.com/dtolnay/async-trait/issues/47
    // impl has stricter requirements than trait
    where
        'op: 'async_trait;
}
