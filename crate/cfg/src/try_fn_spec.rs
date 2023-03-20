use async_trait::async_trait;
use peace_data::Data;

use crate::OpCtx;

/// Defines the logic and data of a state discovery function.
#[async_trait(?Send)]
pub trait TryFnSpec {
    /// Return type of the function.
    ///
    /// * For [`StateCurrentFnSpec`], this is the current [`State`] of the
    ///   managed item.
    /// * For [`StateDesiredFnSpec`], this is the desired [`StateLogical`] of
    ///   the managed item.
    ///
    /// [`StateCurrentFnSpec`]: crate::ItemSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec`]: crate::ItemSpec::StateDesiredFnSpec
    /// [`State`]: crate::State
    /// [`StateLogical`]: crate::StateLogical
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

    /// Executes the function, returning `Ok(None)` if the output is not ready
    /// to be queried.
    async fn try_exec(
        op_ctx: OpCtx<'_>,
        data: Self::Data<'_>,
    ) -> Result<Option<Self::Output>, Self::Error>;

    /// Executes the function.
    async fn exec(op_ctx: OpCtx<'_>, data: Self::Data<'_>) -> Result<Self::Output, Self::Error>;
}
