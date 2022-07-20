use peace_cfg::async_trait;
use peace_resources::{
    resources_type_state::{SetUp, WithStates},
    Resources,
};

/// Type-erased trait corresponding to [`FullSpec::EnsureOpSpec`].
///
/// Implementation of this will fetch Data from [`Resources`], then call the
/// corresponding [`OpSpec`] method.
///
/// The exec method returns different output values, but per Clean / Ensure /
/// Status, it should be the same type, so we should know what to do with it.
/// e.g. for Status, `StatusFnSpec::exec` should store it in a status resource.
///
/// [`FullSpec::EnsureOpSpec`]: peace_cfg::FullSpec::EnsureOpSpec
/// [`OpSpec`]: peace_cfg::OpSpec
#[async_trait]
pub trait EnsureOpSpecRt {
    /// Error returned when any of the functions of this operation err.
    type Error: std::error::Error;

    /// Returns the desired state of the managed item.
    ///
    /// See [`EnsureOpSpec::desired`] for more information.
    ///
    /// [`EnsureOpSpec::desired`]: peace_cfg::EnsureOpSpec::desired
    async fn desired(&self, resources: &Resources<SetUp>) -> Result<(), Self::Error>;
    /// Checks if the operation needs to be executed.
    ///
    /// See [`EnsureOpSpec::check`] for more information.
    ///
    /// [`EnsureOpSpec::check`]: peace_cfg::EnsureOpSpec::check
    async fn check(&self, resources: &Resources<SetUp>) -> Result<(), Self::Error>;
    /// Transforms the current state to the desired state.
    ///
    /// See [`EnsureOpSpec::exec_dry`] for more information.
    ///
    /// [`EnsureOpSpec::exec_dry`]: peace_cfg::EnsureOpSpec::exec_dry
    async fn exec_dry(&self, resources: &Resources<WithStates>) -> Result<(), Self::Error>;
    /// Transforms the current state to the desired state.
    ///
    /// See [`EnsureOpSpec::exec`] for more information.
    ///
    /// [`EnsureOpSpec::exec`]: peace_cfg::EnsureOpSpec::exec
    async fn exec(&self, resources: &Resources<WithStates>) -> Result<(), Self::Error>;
}
