use async_trait::async_trait;
use peace_data::Data;

/// Defines the logic and data of a function.
#[async_trait]
#[nougat::gat]
pub trait FnSpec {
    /// Return type of the function.
    ///
    /// * For the [`status`] function, this is the current [`StateLogical`] of
    ///   the managed item.
    ///
    /// [`status`]: crate::FullSpec::StatusFnSpec
    /// [`StateLogical`]: crate::FullSpec::StateLogical
    type Output;

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
    async fn exec(data: Self::Data<'_>) -> Result<Self::Output, Self::Error>;
}
