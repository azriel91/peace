use async_trait::async_trait;
use peace_data::Data;

/// Defines the logic and data of a function.
#[async_trait]
pub trait FnSpec<'op> {
    /// Return type of the function.
    ///
    /// * For the [`status`] function, this is the current [`StateLogical`] of
    ///   the managed item.
    ///
    /// [`status`]: crate::FullSpec::StatusFnSpec
    /// [`StateLogical`]: crate::FullSpec::State
    type Output;

    /// Data that the function reads from, or writes to.
    ///
    /// These may be parameters to the function, or information calculated from
    /// previous functions.
    type Data: Data<'op>;

    /// Error returned when this function errs.
    type Error: std::error::Error;

    /// Executes this function.
    async fn exec(data: Self::Data) -> Result<Self::Output, Self::Error>
    // Without this, we hit a similar issue to: https://github.com/dtolnay/async-trait/issues/47
    // impl has stricter requirements than trait
    where
        'op: 'async_trait;
}
