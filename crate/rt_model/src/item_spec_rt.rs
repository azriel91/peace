use std::fmt::Debug;

use dyn_clone::DynClone;
use fn_graph::{DataAccess, DataAccessDyn};
use peace_cfg::{async_trait, FnCtx, ItemSpecId};
use peace_resources::{
    resources::ts::{Empty, SetUp},
    type_reg::untagged::{BoxDtDisplay, TypeMap},
    Resources,
};

use crate::{
    outcomes::{ItemApplyBoxed, ItemApplyPartialBoxed},
    ItemSpecParamsTypeReg, ParamsSpecsTypeReg, StatesTypeReg,
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

    /// Initializes data for the item spec's functions.
    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), E>
    where
        E: Debug + std::error::Error;

    /// Registers params and state types with the type registries for
    /// deserializing from disk.
    ///
    /// This is necessary to deserialize `ItemSpecParamsFile`,
    /// `ParamsSpecsFile`, `StatesSavedFile`, and `StatesDesiredFile`.
    fn params_and_state_register(
        &self,
        item_spec_params_type_reg: &mut ItemSpecParamsTypeReg,
        params_specs_type_reg: &mut ParamsSpecsTypeReg,
        states_type_reg: &mut StatesTypeReg,
    );

    /// Runs [`ItemSpec::state_clean`].
    ///
    /// [`ItemSpec::state_clean`]: peace_cfg::ItemSpec::state_clean
    async fn state_clean(&self, resources: &Resources<SetUp>) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::state_current`]`::`[`try_exec`].
    ///
    /// [`ItemSpec::state_current`]: peace_cfg::ItemSpec::state_current
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    async fn state_current_try_exec(
        &self,
        fn_ctx: FnCtx<'_>,
        resources: &Resources<SetUp>,
    ) -> Result<Option<BoxDtDisplay>, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::state_current`]`::`[`exec`].
    ///
    /// [`ItemSpec::state_current`]: peace_cfg::ItemSpec::state_current
    /// [`exec`]: peace_cfg::TryFnSpec::exec
    async fn state_current_exec(
        &self,
        fn_ctx: FnCtx<'_>,
        resources: &Resources<SetUp>,
    ) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::state_desired`]`::`[`try_exec`].
    ///
    /// [`ItemSpec::state_desired`]: peace_cfg::ItemSpec::state_desired
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    async fn state_desired_try_exec(
        &self,
        fn_ctx: FnCtx<'_>,
        resources: &Resources<SetUp>,
    ) -> Result<Option<BoxDtDisplay>, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`ItemSpec::state_desired`]`::`[`exec`].
    ///
    /// [`ItemSpec::state_desired`]: peace_cfg::ItemSpec::state_desired
    /// [`exec`]: peace_cfg::TryFnSpec::exec
    async fn state_desired_exec(
        &self,
        fn_ctx: FnCtx<'_>,
        resources: &Resources<SetUp>,
    ) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Returns the diff between the previous and desired [`State`]s.
    ///
    /// [`State`]: peace_cfg::State
    async fn state_diff_exec(
        &self,
        resources: &Resources<SetUp>,
        states_a: &TypeMap<ItemSpecId, BoxDtDisplay>,
        states_b: &TypeMap<ItemSpecId, BoxDtDisplay>,
    ) -> Result<Option<BoxDtDisplay>, E>
    where
        E: Debug + std::error::Error;

    /// Discovers the information needed for an ensure execution.
    ///
    /// This runs the following functions in order:
    ///
    /// * [`ItemSpec::state_current`]
    /// * [`ItemSpec::state_desired`]
    /// * [`ItemSpec::state_diff`]
    /// * [`ApplyFns::check`]
    ///
    /// [`ItemSpec::state_current`]: peace_cfg::ItemSpec::state_current
    /// [`ItemSpec::state_desired`]: peace_cfg::ItemSpec::state_desired
    /// [`ItemSpec::state_diff`]: peace_cfg::ItemSpec::state_diff
    /// [`ApplyFns::check`]: peace_cfg::ItemSpec::ApplyFns
    async fn ensure_prepare(
        &self,
        fn_ctx: FnCtx<'_>,
        resources: &Resources<SetUp>,
    ) -> Result<ItemApplyBoxed, (E, ItemApplyPartialBoxed)>
    where
        E: Debug + std::error::Error;

    /// Discovers the information needed for a clean execution.
    ///
    /// This runs the following functions in order:
    ///
    /// * [`ItemSpec::state_current`]
    /// * [`ItemSpec::state_clean`]
    /// * [`ItemSpec::state_diff`]
    /// * [`ApplyFns::check`]
    ///
    /// [`ItemSpec::state_current`]: peace_cfg::ItemSpec::state_current
    /// [`ItemSpec::state_clean`]: peace_cfg::ItemSpec::state_clean
    /// [`ItemSpec::state_diff`]: peace_cfg::ItemSpec::state_diff
    /// [`ApplyFns::check`]: peace_cfg::ItemSpec::ApplyFns
    async fn clean_prepare(
        &self,
        fn_ctx: FnCtx<'_>,
        resources: &Resources<SetUp>,
    ) -> Result<ItemApplyBoxed, (E, ItemApplyPartialBoxed)>
    where
        E: Debug + std::error::Error;

    /// Dry applies the item from its current state to its desired state.
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
    /// [`ApplyFns::exec_dry`]: peace_cfg::ItemSpec::ApplyFns
    async fn apply_exec_dry(
        &self,
        fn_ctx: FnCtx<'_>,
        resources: &Resources<SetUp>,
        item_apply: &mut ItemApplyBoxed,
    ) -> Result<(), E>
    where
        E: Debug + std::error::Error;

    /// Applies the item from its current state to its desired state.
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
    /// [`ApplyFns::exec`]: peace_cfg::ItemSpec::ApplyFns
    async fn apply_exec(
        &self,
        fn_ctx: FnCtx<'_>,
        resources: &Resources<SetUp>,
        item_apply: &mut ItemApplyBoxed,
    ) -> Result<(), E>
    where
        E: Debug + std::error::Error;
}
