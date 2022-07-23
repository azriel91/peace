use std::fmt::Debug;

use fn_graph::{DataAccess, DataAccessDyn};
use peace_cfg::async_trait;
use peace_resources::{
    resources_type_state::{Empty, SetUp, WithStates},
    Resources,
};

/// Internal trait that erases the types from [`FullSpec`]
///
/// This exists so that different implementations of [`FullSpec`] can be held
/// under the same boxed trait.
///
/// [`FullSpec`]: peace_cfg::FullSpec
#[async_trait]
pub trait FullSpecRt<E>: Debug + DataAccess + DataAccessDyn
where
    E: Debug + std::error::Error,
{
    /// Initializes data for the operation's check and `exec` functions.
    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), E>;

    /// Runs [`FullSpec::StatusFnSpec`]`::`[`exec`].
    ///
    /// [`FullSpec::StatusFnSpec`]: peace_cfg::FullSpec::StatusFnSpec
    /// [`exec`]: peace_cfg::FnSpec::exec
    async fn status_fn_exec(&self, resources: &Resources<SetUp>) -> Result<(), E>;

    /// Runs [`FullSpec::StatusDesiredFnSpec`]`::`[`desired`].
    ///
    /// [`FullSpec::StatusDesiredFnSpec`]: peace_cfg::FullSpec::StatusDesiredFnSpec
    /// [`desired`]: peace_cfg::FnSpec::desired
    async fn status_desired_fn_exec(&self, resources: &Resources<SetUp>) -> Result<(), E>;

    /// Runs [`FullSpec::EnsureOpSpec`]`::`[`check`].
    ///
    /// [`FullSpec::EnsureOpSpec`]: peace_cfg::FullSpec::EnsureOpSpec
    /// [`check`]: peace_cfg::OpSpec::check
    async fn ensure_op_check(&self, resources: &Resources<WithStates>) -> Result<(), E>;

    /// Runs [`FullSpec::EnsureOpSpec`]`::`[`exec_dry`].
    ///
    /// [`FullSpec::EnsureOpSpec`]: peace_cfg::FullSpec::EnsureOpSpec
    /// [`exec_dry`]: peace_cfg::OpSpec::exec_dry
    async fn ensure_op_exec_dry(&self, resources: &Resources<WithStates>) -> Result<(), E>;

    /// Runs [`FullSpec::EnsureOpSpec`]`::`[`exec`].
    ///
    /// [`FullSpec::EnsureOpSpec`]: peace_cfg::FullSpec::EnsureOpSpec
    /// [`exec`]: peace_cfg::OpSpec::exec
    async fn ensure_op_exec(&self, resources: &Resources<WithStates>) -> Result<(), E>;
}
