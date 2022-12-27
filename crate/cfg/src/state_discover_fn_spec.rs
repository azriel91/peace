use async_trait::async_trait;
use peace_data::Data;

/// Defines the logic and data of a state discovery function.
#[async_trait(?Send)]
pub trait StateDiscoverFnSpec {
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

    /// Executes this function.
    async fn exec(data: Self::Data<'_>) -> Result<Self::Output, Self::Error>;
}
