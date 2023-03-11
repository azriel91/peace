use std::fmt::Debug;

use dyn_clone::DynClone;
use fn_graph::{DataAccess, DataAccessDyn};
use peace_cfg::{async_trait, ItemSpecId, OpCheckStatus, OpCtx};
use peace_resources::{
    resources::ts::{Empty, SetUp},
    states::{StateDiffs, StatesCurrent, StatesDesired, StatesSaved},
    type_reg::untagged::BoxDtDisplay,
    Resources,
};

use crate::{
    outcomes::{ItemEnsureBoxed, ItemEnsurePartialBoxed},
    StatesTypeRegs,
};

/// Internal trait that erases the types from [`ItemSpec`]
///
/// This exists so that different implementations of [`ItemSpec`] can be held
/// under the same boxed trait.
///
/// [`ItemSpec`]: peace_cfg::ItemSpec
#[async_trait(?Send)]
pub trait ItemSpecRt<E>: Debug + DataAccess + DataAccessDyn + DynClone {
    /// Returns the ID of this item spec.
    ///
    /// See [`ItemSpec::id`];
    ///
    /// [`ItemSpec::id`]: peace_cfg::ItemSpec::id
    fn id(&self) -> &ItemSpecId;

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
    async fn state_current_exec(&self, resources: &Resources<SetUp>) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::StateCurrentFnSpec`]`::`[`exec`].
    ///
    /// `states_current` and `state_diffs` are not needed by the discovery, but
    /// are here as markers that this method should be called after the caller
    /// has previously diffed the desired states to states discovered in the
    /// current execution.
    ///
    /// [`ItemSpec::StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`exec`]: peace_cfg::TryFnSpec::exec
    async fn state_ensured_exec(
        &self,
        resources: &Resources<SetUp>,
        states_current: &StatesCurrent,
        state_diffs: &StateDiffs,
    ) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::StateCurrentFnSpec`]`::`[`try_exec`].
    ///
    /// `states_current` is not needed by the discovery, but is here as a marker
    /// that this method should be called after the caller has previously saved
    /// the state of the item.
    ///
    /// [`ItemSpec::StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    async fn state_cleaned_try_exec(
        &self,
        resources: &Resources<SetUp>,
        states_current: &StatesCurrent,
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
        resources: &Resources<SetUp>,
        states_saved: &StatesSaved,
        states_desired: &StatesDesired,
    ) -> Result<Option<BoxDtDisplay>, E>
    where
        E: Debug + std::error::Error;

    /// Returns the diff between the current and desired [`State`]s.
    ///
    /// [`State`]: peace_cfg::State
    async fn state_diff_exec_with_states_current(
        &self,
        resources: &Resources<SetUp>,
        states_current: &StatesCurrent,
        states_desired: &StatesDesired,
    ) -> Result<Option<BoxDtDisplay>, E>
    where
        E: Debug + std::error::Error;

    /// Discovers the information needed for an ensure execution.
    ///
    /// This runs the following functions in order:
    ///
    /// * [`StateCurrentFnSpec::try_exec`]
    /// * [`StateDesiredFnSpec::try_exec`]
    /// * [`StateDiffFnSpec::exec`]
    /// * [`EnsureOpSpec::check`]
    ///
    /// [`StateCurrentFnSpec::try_exec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec::try_exec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    /// [`StateDiffFnSpec::exec`]: peace_cfg::ItemSpec::StateDiffFnSpec
    /// [`EnsureOpSpec::check`]: peace_cfg::ItemSpec::EnsureOpSpec
    async fn ensure_prepare(
        &self,
        resources: &Resources<SetUp>,
    ) -> Result<ItemEnsureBoxed, (E, ItemEnsurePartialBoxed)>
    where
        E: Debug + std::error::Error;

    /// Dry ensures the item from its current state to its desired state.
    ///
    /// This runs the following functions in order:
    ///
    /// * [`StateCurrentFnSpec::try_exec`]
    /// * [`StateDesiredFnSpec::try_exec`]
    /// * [`StateDiffFnSpec::exec`]
    /// * [`EnsureOpSpec::check`]
    /// * [`EnsureOpSpec::exec_dry`]
    ///
    /// # Parameters
    ///
    /// * `resources`: The resources in the current execution.
    /// * `item_ensure`: The information collected in `self.ensure_prepare`.
    ///
    /// [`StateCurrentFnSpec::try_exec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec::try_exec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    /// [`StateDiffFnSpec::exec`]: peace_cfg::ItemSpec::StateDiffFnSpec
    /// [`EnsureOpSpec::check`]: peace_cfg::ItemSpec::EnsureOpSpec
    /// [`EnsureOpSpec::exec_dry`]: peace_cfg::ItemSpec::EnsureOpSpec
    async fn ensure_exec_dry(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<SetUp>,
        item_ensure: &mut ItemEnsureBoxed,
    ) -> Result<(), E>
    where
        E: Debug + std::error::Error;

    /// Ensures the item from its current state to its desired state.
    ///
    /// This runs the following functions in order:
    ///
    /// * [`StateCurrentFnSpec::exec`]
    /// * [`StateDesiredFnSpec::exec`]
    /// * [`StateDiffFnSpec::exec`]
    /// * [`EnsureOpSpec::check`]
    /// * [`EnsureOpSpec::exec`]
    ///
    /// [`StateCurrentFnSpec::exec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec::exec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    /// [`StateDiffFnSpec::exec`]: peace_cfg::ItemSpec::StateDiffFnSpec
    /// [`EnsureOpSpec::check`]: peace_cfg::ItemSpec::EnsureOpSpec
    /// [`EnsureOpSpec::exec`]: peace_cfg::ItemSpec::EnsureOpSpec
    async fn ensure_exec(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<SetUp>,
        item_ensure: &mut ItemEnsureBoxed,
    ) -> Result<(), E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::CleanOpSpec`]`::`[`check`].
    ///
    /// [`ItemSpec::CleanOpSpec`]: peace_cfg::ItemSpec::CleanOpSpec
    /// [`check`]: peace_cfg::OpSpec::check
    async fn clean_op_check(
        &self,
        resources: &Resources<SetUp>,
        states_current: &StatesCurrent,
    ) -> Result<OpCheckStatus, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::CleanOpSpec`]`::`[`exec_dry`].
    ///
    /// [`ItemSpec::CleanOpSpec`]: peace_cfg::ItemSpec::CleanOpSpec
    /// [`exec_dry`]: peace_cfg::OpSpec::exec_dry
    async fn clean_op_exec_dry(
        &self,
        resources: &Resources<SetUp>,
        states_current: &StatesCurrent,
    ) -> Result<(), E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::CleanOpSpec`]`::`[`exec`].
    ///
    /// [`ItemSpec::CleanOpSpec`]: peace_cfg::ItemSpec::CleanOpSpec
    /// [`exec`]: peace_cfg::OpSpec::exec
    async fn clean_op_exec(
        &self,
        resources: &Resources<SetUp>,
        states_current: &StatesCurrent,
    ) -> Result<(), E>
    where
        E: Debug + std::error::Error;
}
