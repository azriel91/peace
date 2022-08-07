use std::fmt::Debug;

use fn_graph::{DataAccess, DataAccessDyn};
use peace_cfg::{async_trait, ItemSpecId, OpCheckStatus};
use peace_resources::{
    resources_type_state::{Empty, SetUp, WithStateDiffs, WithStatesCurrentAndDesired},
    type_reg::untagged::DataType,
    Resources,
};

/// Internal trait that erases the types from [`ItemSpec`]
///
/// This exists so that different implementations of [`ItemSpec`] can be held
/// under the same boxed trait.
///
/// [`ItemSpec`]: peace_cfg::ItemSpec
#[async_trait(?Send)]
pub trait ItemSpecRt<E>: Debug + DataAccess + DataAccessDyn
where
    E: Debug + std::error::Error,
{
    /// Returns the ID of this full spec.
    ///
    /// See [`ItemSpec::id`];
    ///
    /// [`ItemSpec::id`]: peace_cfg::ItemSpec::id
    fn id(&self) -> ItemSpecId;

    /// Initializes data for the operation's check and `exec` functions.
    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), E>;

    /// Runs [`ItemSpec::StateCurrentFnSpec`]`::`[`exec`].
    ///
    /// [`ItemSpec::StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`exec`]: peace_cfg::FnSpec::exec
    async fn state_current_fn_exec(
        &self,
        resources: &Resources<SetUp>,
    ) -> Result<Box<dyn DataType>, E>;

    /// Runs [`ItemSpec::StateCurrentFnSpec`]`::`[`exec`].
    ///
    /// [`ItemSpec::StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`exec`]: peace_cfg::FnSpec::exec
    async fn state_ensured_fn_exec(
        &self,
        resources: &Resources<WithStateDiffs>,
    ) -> Result<Box<dyn DataType>, E>;

    /// Runs [`ItemSpec::StateDesiredFnSpec`]`::`[`desired`].
    ///
    /// [`ItemSpec::StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    /// [`desired`]: peace_cfg::FnSpec::desired
    async fn state_desired_fn_exec(
        &self,
        resources: &Resources<SetUp>,
    ) -> Result<Box<dyn DataType>, E>;

    /// Returns the diff between the current and desired [`State`]s.
    ///
    /// [`State`]: peace_cfg::State
    async fn state_diff_fn_exec(
        &self,
        resources: &Resources<WithStatesCurrentAndDesired>,
    ) -> Result<Box<dyn DataType>, E>;

    /// Runs [`ItemSpec::EnsureOpSpec`]`::`[`check`].
    ///
    /// [`ItemSpec::EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    /// [`check`]: peace_cfg::OpSpec::check
    async fn ensure_op_check(
        &self,
        resources: &Resources<WithStateDiffs>,
    ) -> Result<OpCheckStatus, E>;

    /// Runs [`ItemSpec::EnsureOpSpec`]`::`[`exec_dry`].
    ///
    /// [`ItemSpec::EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    /// [`exec_dry`]: peace_cfg::OpSpec::exec_dry
    async fn ensure_op_exec_dry(&self, resources: &Resources<WithStateDiffs>) -> Result<(), E>;

    /// Runs [`ItemSpec::EnsureOpSpec`]`::`[`exec`].
    ///
    /// [`ItemSpec::EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    /// [`exec`]: peace_cfg::OpSpec::exec
    async fn ensure_op_exec(&self, resources: &Resources<WithStateDiffs>) -> Result<(), E>;
}
