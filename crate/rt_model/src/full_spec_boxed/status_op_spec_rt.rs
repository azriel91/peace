use fn_graph::Resources;
use peace_cfg::async_trait;

/// Type-erased trait corresponding to [`FullSpec::StatusOpSpec`].
///
/// Implementation of this will fetch Data from [`Resources`], then call the
/// corresponding [`OpSpec`] method.
///
/// The exec method returns different output values, but per Clean / Ensure /
/// Status, it should be the same type, so we should know what to do with it.
/// e.g. for Status, `StatusOpSpec::exec` should store it in a status resource.
///
/// [`FullSpec::StatusOpSpec`]: peace_cfg::FullSpec::StatusOpSpec
/// [`OpSpec`]: peace_cfg::OpSpec
#[async_trait]
pub trait StatusOpSpecRt<'op> {
    /// Error returned when any of the functions of this operation err.
    type Error: std::error::Error;

    /// Checks if the operation needs to be executed.
    async fn check(&self, resources: &Resources) -> Result<(), Self::Error>;
    /// Transforms the current state to the desired state.
    async fn exec(&self, resources: &Resources) -> Result<(), Self::Error>;
}
