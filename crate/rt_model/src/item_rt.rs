use std::{any::Any, fmt::Debug};

use dyn_clone::DynClone;
use peace_cfg::{async_trait, FnCtx, ItemId};
use peace_data::fn_graph::{DataAccess, DataAccessDyn};
use peace_params::ParamsSpecs;
use peace_resources::{
    resources::ts::{Empty, SetUp},
    states::StatesCurrent,
    type_reg::untagged::{BoxDtDisplay, TypeMap},
    Resources,
};

use crate::{
    outcomes::{ItemApplyBoxed, ItemApplyPartialBoxed},
    ItemParamsTypeReg, ParamsSpecsTypeReg, StatesTypeReg,
};

/// Internal trait that erases the types from [`Item`]
///
/// This exists so that different implementations of [`Item`] can be held
/// under the same boxed trait.
///
/// [`Item`]: peace_cfg::Item
#[async_trait(?Send)]
pub trait ItemRt<E>:
    Any + Debug + DataAccess + DataAccessDyn + DynClone + Send + Sync + 'static
{
    /// Returns the ID of this item.
    ///
    /// See [`Item::id`];
    ///
    /// [`Item::id`]: peace_cfg::Item::id
    fn id(&self) -> &ItemId;

    /// Returns whether this item is equal to the other.
    fn eq(&self, other: &dyn ItemRt<E>) -> bool;

    /// Returns `&self` as `&dyn Any`.
    ///
    /// This is needed to upcast to `&dyn Any` and satisfy the upcast lifetime
    /// requirement.
    fn as_any(&self) -> &dyn Any;

    /// Initializes data for the item's functions.
    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), E>
    where
        E: Debug + std::error::Error;

    /// Registers params and state types with the type registries for
    /// deserializing from disk.
    ///
    /// This is necessary to deserialize `ItemParamsFile`,
    /// `ParamsSpecsFile`, `StatesCurrentFile`, and `StatesGoalFile`.
    fn params_and_state_register(
        &self,
        item_params_type_reg: &mut ItemParamsTypeReg,
        params_specs_type_reg: &mut ParamsSpecsTypeReg,
        states_type_reg: &mut StatesTypeReg,
    );

    /// Returns if the given two states equal.
    ///
    /// This returns an error if the boxed states could not be downcasted to
    /// this item's state, which indicates one of the following:
    ///
    /// * Peace contains a bug, and passed an incorrect box to this item.
    /// * Item IDs were swapped, such that `ItemA`'s state is passed to `ItemB`.
    ///
    ///     This needs some rework on how item IDs are implemented -- as in,
    ///     whether we should use a string newtype for `ItemId`s, or redesign
    ///     how `Item`s or related types are keyed.
    ///
    /// Note: it is impossible to call this method if an `Item`'s state type has
    /// changed -- it would have failed on deserialization.
    fn state_eq(&self, state_a: &BoxDtDisplay, state_b: &BoxDtDisplay) -> Result<bool, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`Item::state_clean`].
    ///
    /// [`Item::state_clean`]: peace_cfg::Item::state_clean
    async fn state_clean(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
    ) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`Item::state_current`]`::`[`try_exec`].
    ///
    /// [`Item::state_current`]: peace_cfg::Item::state_current
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    async fn state_current_try_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<Option<BoxDtDisplay>, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`Item::state_current`]`::`[`exec`].
    ///
    /// [`Item::state_current`]: peace_cfg::Item::state_current
    /// [`exec`]: peace_cfg::TryFnSpec::exec
    async fn state_current_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`Item::state_goal`]`::`[`try_exec`].
    ///
    /// [`Item::state_goal`]: peace_cfg::Item::state_goal
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    async fn state_goal_try_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<Option<BoxDtDisplay>, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`Item::state_goal`]`::`[`exec`].
    ///
    /// [`Item::state_goal`]: peace_cfg::Item::state_goal
    /// [`exec`]: peace_cfg::TryFnSpec::exec
    async fn state_goal_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Returns the diff between the previous and goal [`State`]s.
    ///
    /// [`State`]: peace_cfg::State
    async fn state_diff_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        states_a: &TypeMap<ItemId, BoxDtDisplay>,
        states_b: &TypeMap<ItemId, BoxDtDisplay>,
    ) -> Result<Option<BoxDtDisplay>, E>
    where
        E: Debug + std::error::Error;

    /// Discovers the information needed for an ensure execution.
    ///
    /// This runs the following functions in order:
    ///
    /// * [`Item::state_current`]
    /// * [`Item::state_goal`]
    /// * [`Item::state_diff`]
    /// * [`ApplyFns::check`]
    ///
    /// [`Item::state_current`]: peace_cfg::Item::state_current
    /// [`Item::state_goal`]: peace_cfg::Item::state_goal
    /// [`Item::state_diff`]: peace_cfg::Item::state_diff
    /// [`ApplyFns::check`]: peace_cfg::Item::ApplyFns
    async fn ensure_prepare(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<ItemApplyBoxed, (E, ItemApplyPartialBoxed)>
    where
        E: Debug + std::error::Error;

    /// Discovers the information needed for a clean execution.
    ///
    /// This runs the following functions in order:
    ///
    /// * [`Item::state_current`]
    /// * [`Item::state_clean`]
    /// * [`Item::state_diff`]
    /// * [`ApplyFns::check`]
    ///
    /// [`Item::state_current`]: peace_cfg::Item::state_current
    /// [`Item::state_clean`]: peace_cfg::Item::state_clean
    /// [`Item::state_diff`]: peace_cfg::Item::state_diff
    /// [`ApplyFns::check`]: peace_cfg::Item::ApplyFns
    async fn clean_prepare(
        &self,
        states_current: &StatesCurrent,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
    ) -> Result<ItemApplyBoxed, (E, ItemApplyPartialBoxed)>
    where
        E: Debug + std::error::Error;

    /// Dry applies the item from its current state to its goal state.
    ///
    /// This runs the following function in order, passing in the information
    /// collected from [`ensure_prepare`] or [`clean_prepare`]:
    ///
    /// * [`ApplyFns::exec_dry`]
    ///
    /// # Parameters
    ///
    /// * `resources`: The resources in the current execution.
    /// * `item_apply`: The information collected in `self.ensure_prepare`.
    ///
    /// [`ApplyFns::exec_dry`]: peace_cfg::Item::ApplyFns
    async fn apply_exec_dry(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
        item_apply: &mut ItemApplyBoxed,
    ) -> Result<(), E>
    where
        E: Debug + std::error::Error;

    /// Applies the item from its current state to its goal state.
    ///
    /// This runs the following function in order, passing in the information
    /// collected from [`ensure_prepare`] or [`clean_prepare`]:
    ///
    /// * [`ApplyFns::exec`]
    ///
    /// # Parameters
    ///
    /// * `resources`: The resources in the current execution.
    /// * `item_apply`: The information collected in `self.ensure_prepare`.
    ///
    /// [`ApplyFns::exec`]: peace_cfg::Item::ApplyFns
    async fn apply_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
        item_apply: &mut ItemApplyBoxed,
    ) -> Result<(), E>
    where
        E: Debug + std::error::Error;
}
