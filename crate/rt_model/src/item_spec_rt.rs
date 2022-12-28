use std::fmt::Debug;

use fn_graph::{DataAccess, DataAccessDyn};
use peace_cfg::{async_trait, ItemSpecId, OpCheckStatus};
use peace_resources::{
    resources::ts::{
        Empty, SetUp, WithStatesCurrent, WithStatesCurrentAndDesired, WithStatesCurrentDiffs,
        WithStatesSavedAndDesired,
    },
    type_reg::untagged::BoxDtDisplay,
    Resources,
};

use crate::StatesTypeRegs;

/// Internal trait that erases the types from [`ItemSpec`]
///
/// This exists so that different implementations of [`ItemSpec`] can be held
/// under the same boxed trait.
///
/// [`ItemSpec`]: peace_cfg::ItemSpec
#[async_trait(?Send)]
pub trait ItemSpecRt<E>: Debug + DataAccess + DataAccessDyn {
    /// Returns the ID of this full spec.
    ///
    /// See [`ItemSpec::id`];
    ///
    /// [`ItemSpec::id`]: peace_cfg::ItemSpec::id
    fn id(&self) -> ItemSpecId;

    /// Initializes data for the operation's check and `exec` functions.
    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), E>
    where
        E: Debug + std::error::Error;

    /// Registers state types with type registries for deserializing from disk.
    ///
    /// This is necessary to deserialize `StatesSavedFile` and
    /// `StatesDesiredFile`.
    fn state_register(&self, states_type_regs: &mut StatesTypeRegs);

    /// Runs [`ItemSpec::StateCurrentFnSpec`]`::`[`try_exec`].
    ///
    /// [`ItemSpec::StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    async fn state_current_try_exec(
        &self,
        resources: &Resources<SetUp>,
    ) -> Result<Option<BoxDtDisplay>, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::StateCurrentFnSpec`]`::`[`exec`].
    ///
    /// [`ItemSpec::StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`exec`]: peace_cfg::TryFnSpec::exec
    async fn state_ensured_exec(
        &self,
        resources: &Resources<WithStatesCurrentDiffs>,
    ) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::StateCurrentFnSpec`]`::`[`try_exec`].
    ///
    /// [`ItemSpec::StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    async fn state_cleaned_try_exec(
        &self,
        resources: &Resources<WithStatesCurrent>,
    ) -> Result<Option<BoxDtDisplay>, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::StateDesiredFnSpec`]`::`[`try_exec`].
    ///
    /// [`ItemSpec::StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    async fn state_desired_try_exec(
        &self,
        resources: &Resources<SetUp>,
    ) -> Result<Option<BoxDtDisplay>, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::StateDesiredFnSpec`]`::`[`exec`].
    ///
    /// [`ItemSpec::StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    /// [`exec`]: peace_cfg::TryFnSpec::exec
    async fn state_desired_exec(&self, resources: &Resources<SetUp>) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Returns the diff between the previous and desired [`State`]s.
    ///
    /// [`State`]: peace_cfg::State
    async fn state_diff_exec_with_states_saved(
        &self,
        resources: &Resources<WithStatesSavedAndDesired>,
    ) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Returns the diff between the current and desired [`State`]s.
    ///
    /// [`State`]: peace_cfg::State
    async fn state_diff_exec_with_states_current(
        &self,
        resources: &Resources<WithStatesCurrentAndDesired>,
    ) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::EnsureOpSpec`]`::`[`check`].
    ///
    /// [`ItemSpec::EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    /// [`check`]: peace_cfg::OpSpec::check
    async fn ensure_op_check(
        &self,
        resources: &Resources<WithStatesCurrentDiffs>,
    ) -> Result<OpCheckStatus, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::EnsureOpSpec`]`::`[`exec_dry`].
    ///
    /// [`ItemSpec::EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    /// [`exec_dry`]: peace_cfg::OpSpec::exec_dry
    async fn ensure_op_exec_dry(
        &self,
        resources: &Resources<WithStatesCurrentDiffs>,
    ) -> Result<(), E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::EnsureOpSpec`]`::`[`exec`].
    ///
    /// [`ItemSpec::EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    /// [`exec`]: peace_cfg::OpSpec::exec
    async fn ensure_op_exec(&self, resources: &Resources<WithStatesCurrentDiffs>) -> Result<(), E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::CleanOpSpec`]`::`[`check`].
    ///
    /// [`ItemSpec::CleanOpSpec`]: peace_cfg::ItemSpec::CleanOpSpec
    /// [`check`]: peace_cfg::OpSpec::check
    async fn clean_op_check(
        &self,
        resources: &Resources<WithStatesCurrent>,
    ) -> Result<OpCheckStatus, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::CleanOpSpec`]`::`[`exec_dry`].
    ///
    /// [`ItemSpec::CleanOpSpec`]: peace_cfg::ItemSpec::CleanOpSpec
    /// [`exec_dry`]: peace_cfg::OpSpec::exec_dry
    async fn clean_op_exec_dry(&self, resources: &Resources<WithStatesCurrent>) -> Result<(), E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::CleanOpSpec`]`::`[`exec`].
    ///
    /// [`ItemSpec::CleanOpSpec`]: peace_cfg::ItemSpec::CleanOpSpec
    /// [`exec`]: peace_cfg::OpSpec::exec
    async fn clean_op_exec(&self, resources: &Resources<WithStatesCurrent>) -> Result<(), E>
    where
        E: Debug + std::error::Error;
}
