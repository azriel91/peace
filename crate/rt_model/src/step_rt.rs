use std::{any::Any, fmt::Debug};

use dyn_clone::DynClone;
use peace_cfg::{async_trait, FnCtx, StepId};
use peace_data::fn_graph::{DataAccess, DataAccessDyn};
use peace_params::ParamsSpecs;
use peace_resources::{
    resources::ts::{Empty, SetUp},
    states::StatesCurrent,
    type_reg::untagged::{BoxDtDisplay, TypeMap},
    Resources,
};

use crate::{
    outcomes::{StepApplyBoxed, StepApplyPartialBoxed},
    ParamsSpecsTypeReg, StatesTypeReg,
};

/// Internal trait that erases the types from [`Step`]
///
/// This exists so that different implementations of [`Step`] can be held
/// under the same boxed trait.
///
/// [`Step`]: peace_cfg::Step
#[async_trait(?Send)]
pub trait StepRt<E>:
    Any + Debug + DataAccess + DataAccessDyn + DynClone + Send + Sync + 'static
{
    /// Returns the ID of this step.
    ///
    /// See [`Step::id`];
    ///
    /// [`Step::id`]: peace_cfg::Step::id
    fn id(&self) -> &StepId;

    /// Returns whether this step is equal to the other.
    fn eq(&self, other: &dyn StepRt<E>) -> bool;

    /// Returns `&self` as `&dyn Any`.
    ///
    /// This is needed to upcast to `&dyn Any` and satisfy the upcast lifetime
    /// requirement.
    fn as_any(&self) -> &dyn Any;

    /// Initializes data for the step's functions.
    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), E>
    where
        E: Debug + std::error::Error;

    /// Registers params and state types with the type registries for
    /// deserializing from disk.
    ///
    /// This is necessary to deserialize `StepParamsFile`,
    /// `ParamsSpecsFile`, `StatesCurrentFile`, and `StatesGoalFile`.
    fn params_and_state_register(
        &self,
        params_specs_type_reg: &mut ParamsSpecsTypeReg,
        states_type_reg: &mut StatesTypeReg,
    );

    /// Returns if the given two states equal.
    ///
    /// This returns an error if the boxed states could not be downcasted to
    /// this step's state, which indicates one of the following:
    ///
    /// * Peace contains a bug, and passed an incorrect box to this step.
    /// * Step IDs were swapped, such that `StepA`'s state is passed to `StepB`.
    ///
    ///     This needs some rework on how step IDs are implemented -- as in,
    ///     whether we should use a string newtype for `StepId`s, or redesign
    ///     how `Step`s or related types are keyed.
    ///
    /// Note: it is impossible to call this method if a `Step`'s state type has
    /// changed -- it would have failed on deserialization.
    fn state_eq(&self, state_a: &BoxDtDisplay, state_b: &BoxDtDisplay) -> Result<bool, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`Step::state_clean`].
    ///
    /// [`Step::state_clean`]: peace_cfg::Step::state_clean
    async fn state_clean(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
    ) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`Step::state_current`]`::`[`try_exec`].
    ///
    /// [`Step::state_current`]: peace_cfg::Step::state_current
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    async fn state_current_try_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<Option<BoxDtDisplay>, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`Step::state_current`]`::`[`exec`].
    ///
    /// [`Step::state_current`]: peace_cfg::Step::state_current
    /// [`exec`]: peace_cfg::TryFnSpec::exec
    async fn state_current_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`Step::state_goal`]`::`[`try_exec`].
    ///
    /// [`Step::state_goal`]: peace_cfg::Step::state_goal
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    async fn state_goal_try_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<Option<BoxDtDisplay>, E>
    where
        E: Debug + std::error::Error;

    /// Runs [`Step::state_goal`]`::`[`exec`].
    ///
    /// [`Step::state_goal`]: peace_cfg::Step::state_goal
    /// [`exec`]: peace_cfg::TryFnSpec::exec
    async fn state_goal_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<BoxDtDisplay, E>
    where
        E: Debug + std::error::Error;

    /// Returns the diff between the given [`State`]s.
    ///
    /// Given `states_a` and `states_b` represent current states and goal
    /// states, then this method returns `None` in the following cases:
    ///
    /// * The current state cannot be retrieved, due to a predecessor's state
    ///   not existing.
    /// * The goal state cannot be retrieved, due to a predecessor's state not
    ///   existing.
    /// * A bug exists, e.g. the state is stored against the wrong type
    ///   parameter.
    ///
    /// [`State`]: peace_cfg::State
    async fn state_diff_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        states_a: &TypeMap<StepId, BoxDtDisplay>,
        states_b: &TypeMap<StepId, BoxDtDisplay>,
    ) -> Result<Option<BoxDtDisplay>, E>
    where
        E: Debug + std::error::Error;

    /// Discovers the information needed for an ensure execution.
    ///
    /// This runs the following functions in order:
    ///
    /// * [`Step::state_current`]
    /// * [`Step::state_goal`]
    /// * [`Step::state_diff`]
    /// * [`ApplyFns::check`]
    ///
    /// [`Step::state_current`]: peace_cfg::Step::state_current
    /// [`Step::state_goal`]: peace_cfg::Step::state_goal
    /// [`Step::state_diff`]: peace_cfg::Step::state_diff
    /// [`ApplyFns::check`]: peace_cfg::Step::ApplyFns
    async fn ensure_prepare(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<StepApplyBoxed, (E, StepApplyPartialBoxed)>
    where
        E: Debug + std::error::Error;

    /// Discovers the information needed for a clean execution.
    ///
    /// This runs the following functions in order:
    ///
    /// * [`Step::state_current`]
    /// * [`Step::state_clean`]
    /// * [`Step::state_diff`]
    /// * [`ApplyFns::check`]
    ///
    /// [`Step::state_current`]: peace_cfg::Step::state_current
    /// [`Step::state_clean`]: peace_cfg::Step::state_clean
    /// [`Step::state_diff`]: peace_cfg::Step::state_diff
    /// [`ApplyFns::check`]: peace_cfg::Step::ApplyFns
    async fn clean_prepare(
        &self,
        states_current: &StatesCurrent,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
    ) -> Result<StepApplyBoxed, (E, StepApplyPartialBoxed)>
    where
        E: Debug + std::error::Error;

    /// Dry applies the step from its current state to its goal state.
    ///
    /// This runs the following function in order, passing in the information
    /// collected from [`ensure_prepare`] or [`clean_prepare`]:
    ///
    /// * [`ApplyFns::exec_dry`]
    ///
    /// # Parameters
    ///
    /// * `resources`: The resources in the current execution.
    /// * `step_apply`: The information collected in `self.ensure_prepare`.
    ///
    /// [`ApplyFns::exec_dry`]: peace_cfg::Step::ApplyFns
    async fn apply_exec_dry(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
        step_apply: &mut StepApplyBoxed,
    ) -> Result<(), E>
    where
        E: Debug + std::error::Error;

    /// Applies the step from its current state to its goal state.
    ///
    /// This runs the following function in order, passing in the information
    /// collected from [`ensure_prepare`] or [`clean_prepare`]:
    ///
    /// * [`ApplyFns::exec`]
    ///
    /// # Parameters
    ///
    /// * `resources`: The resources in the current execution.
    /// * `step_apply`: The information collected in `self.ensure_prepare`.
    ///
    /// [`ApplyFns::exec`]: peace_cfg::Step::ApplyFns
    async fn apply_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
        step_apply: &mut StepApplyBoxed,
    ) -> Result<(), E>
    where
        E: Debug + std::error::Error;
}
