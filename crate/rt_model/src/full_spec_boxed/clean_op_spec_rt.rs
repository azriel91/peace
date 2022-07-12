use peace_cfg::async_trait;
use peace_resources::{resources_type_state::SetUp, Resources};

/// Type-erased trait corresponding to [`FullSpec::CleanOpSpec`].
///
/// Implementation of this will fetch Data from [`Resources`], then call the
/// corresponding [`CleanOpSpec`] method.
///
/// The exec method returns different output values, but per Clean / Ensure /
/// Status, it should be the same type, so we should know what to do with it.
/// e.g. for Status, `StatusFnSpec::exec` should store it in a status resource.
///
/// [`FullSpec::CleanOpSpec`]: peace_cfg::FullSpec::CleanOpSpec
/// [`CleanOpSpec`]: peace_cfg::CleanOpSpec
#[async_trait]
pub trait CleanOpSpecRt {
    /// Error returned when any of the functions of this operation err.
    type Error: std::error::Error;

    /// Checks if the operation needs to be executed.
    async fn check(&self, resources: &Resources<SetUp>) -> Result<(), Self::Error>;

    /// Transforms the current state to the desired state.
    async fn exec(&self, resources: &Resources<SetUp>) -> Result<(), Self::Error>;
}
