use fn_graph::Resources;
use peace_cfg::async_trait;

use crate::Error;

/// Type-erased trait corresponding to [`FullSpec::CleanOpSpec`].
///
/// Implementation of this will fetch Data from [`Resources`], then call the
/// corresponding [`OpSpec`] method.
///
/// The exec method returns different output values, but per Clean / Ensure /
/// Status, it should be the same type, so we should know what to do with it.
/// e.g. for Status, `StatusOpSpec::exec` should store it in a status resource.
///
/// [`FullSpec::CleanOpSpec`]: peace_cfg::FullSpec::CleanOpSpec
/// [`OpSpec`]: peace_cfg::OpSpec
#[async_trait]
pub trait CleanOpSpecRt<'op> {
    /// Initializes data for the operation's check and `exec` functions.
    async fn setup(&self, resources: &Resources) -> Result<(), Error>;
    /// Checks if the operation needs to be executed.
    async fn check(&self, resources: &Resources) -> Result<(), Error>;
    /// Transforms the current state to the desired state.
    async fn exec(&self, resources: &Resources) -> Result<(), Error>;
}
