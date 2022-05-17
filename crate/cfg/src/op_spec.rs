use async_trait::async_trait;

use crate::{OpCheckStatus, ProgressLimit};

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
pub trait OpSpec {
    /// State that the [`WorkSpec`] manages.
    ///
    /// This is the type returned by the [`StatusSpec`], and is used by
    /// [`EnsureOpSpec`] and [`CleanOpSpec`] to determine if their [`exec`]
    /// function needs to be run.
    ///
    /// [`WorkSpec`]: crate::WorkSpec
    /// [`StatusSpec`]: crate::WorkSpec::StatusSpec
    /// [`EnsureOpSpec`]: crate::WorkSpec::EnsureOpSpec
    /// [`CleanOpSpec`]: crate::WorkSpec::CleanOpSpec
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
    /// [ensure operation]: crate::WorkSpec::EnsureOpSpec
    /// [clean operation]: crate::WorkSpec::CleanOpSpec
    /// [resource IDs]: crate::WorkSpec::ResIds
    type Output;

    /// Data that the operation reads from, or writes to.
    ///
    /// These may be parameters to the function, or information calculated from
    /// previous operations.
    type Data;

    /// Error returned when any of the functions of this operation err.
    type Error: std::error::Error;

    /// Initializes data for the operation's check and `exec` functions.
    async fn setup(data: &Self::Data) -> Result<ProgressLimit, Self::Error>;

    /// Checks if the operation needs to be executed.
    ///
    /// If the current state is already the desired state, then the operation
    /// does not have to be executed.
    ///
    /// For example, for a file download operation, if the file is already
    /// downloaded, then it does not need to be downloaded again.
    ///
    /// # Implementors
    ///
    /// This function call is intended to be cheap and fast.
    async fn check(data: &Self::Data, state: &Self::State) -> Result<OpCheckStatus, Self::Error>;

    /// Actual execution to do the work.
    async fn exec(data: &Self::Data) -> Result<Self::Output, Self::Error>;
}
