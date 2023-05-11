use std::{
    any::type_name,
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use fn_graph::{DataAccess, DataAccessDyn, TypeIds};
use peace_cfg::{async_trait, ApplyCheck, FnCtx, ItemSpec, ItemSpecId};
use peace_data::{
    marker::{ApplyDry, Clean, Current, Desired},
    Data,
};
use peace_params::{Params, ParamsSpec, ParamsSpecs, ValueResolutionCtx, ValueResolutionMode};
use peace_resources::{
    resources::ts::{Empty, SetUp},
    states::StatesCurrent,
    type_reg::untagged::{BoxDtDisplay, TypeMap},
    Resources,
};

use crate::{
    outcomes::{ItemApply, ItemApplyBoxed, ItemApplyPartial, ItemApplyPartialBoxed},
    ItemSpecParamsTypeReg, ItemSpecRt, ParamsSpecsTypeReg, StatesTypeReg,
};

/// Wraps a type implementing [`ItemSpec`].
///
/// # Type Parameters
///
/// * `IS`: Item spec type to wrap.
/// * `E`: Application specific error type.
///
///     Notably, `E` here should be the application's error type, which is not
///     necessarily the item spec's error type (unless you have only one item
///     spec in the application).
#[allow(clippy::type_complexity)]
pub struct ItemSpecWrapper<IS, E>(IS, PhantomData<E>);

impl<IS, E> Clone for ItemSpecWrapper<IS, E>
where
    IS: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<IS, E> ItemSpecWrapper<IS, E>
where
    IS: Debug + ItemSpec + Send + Sync,
    E: Debug
        + Send
        + Sync
        + std::error::Error
        + From<<IS as ItemSpec>::Error>
        + From<crate::Error>
        + 'static,
    for<'params> <IS as ItemSpec>::Params<'params>:
        TryFrom<<<IS as ItemSpec>::Params<'params> as Params>::Partial>,

    for<'params> <IS::Params<'params> as Params>::Partial: From<IS::Params<'params>>,
{
    async fn state_clean(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
    ) -> Result<IS::State, E> {
        let state_clean = {
            let params_partial = {
                let item_spec_id = self.id();
                let params_spec = params_specs
                    .get::<ParamsSpec<IS::Params<'_>>, _>(item_spec_id)
                    .ok_or_else(|| crate::Error::ParamsSpecNotFound {
                        item_spec_id: item_spec_id.clone(),
                    })?;
                let mut value_resolution_ctx = ValueResolutionCtx::new(
                    ValueResolutionMode::Clean,
                    item_spec_id.clone(),
                    type_name::<IS::Params<'_>>(),
                );
                params_spec
                    .resolve_partial(resources, &mut value_resolution_ctx)
                    .map_err(crate::Error::ParamsResolveError)?
            };
            let data = <IS::Data<'_> as Data>::borrow(self.id(), resources);
            IS::state_clean(&params_partial, data).await?
        };
        resources.borrow_mut::<Clean<IS::State>>().0 = Some(state_clean.clone());

        Ok(state_clean)
    }

    async fn state_current_try_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<Option<IS::State>, E> {
        let state_current = {
            let params_partial = {
                let item_spec_id = self.id();
                let params_spec = params_specs
                    .get::<ParamsSpec<IS::Params<'_>>, _>(item_spec_id)
                    .ok_or_else(|| crate::Error::ParamsSpecNotFound {
                        item_spec_id: item_spec_id.clone(),
                    })?;
                let mut value_resolution_ctx = ValueResolutionCtx::new(
                    ValueResolutionMode::Current,
                    item_spec_id.clone(),
                    type_name::<IS::Params<'_>>(),
                );
                params_spec
                    .resolve_partial(resources, &mut value_resolution_ctx)
                    .map_err(crate::Error::ParamsResolveError)?
            };
            let data = <IS::Data<'_> as Data>::borrow(self.id(), resources);
            IS::try_state_current(fn_ctx, &params_partial, data).await?
        };
        if let Some(state_current) = state_current.as_ref() {
            resources.borrow_mut::<Current<IS::State>>().0 = Some(state_current.clone());
        }

        Ok(state_current)
    }

    async fn state_current_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<IS::State, E> {
        let state_current = {
            let params = {
                let item_spec_id = self.id();
                let params_spec = params_specs
                    .get::<ParamsSpec<IS::Params<'_>>, _>(item_spec_id)
                    .ok_or_else(|| crate::Error::ParamsSpecNotFound {
                        item_spec_id: item_spec_id.clone(),
                    })?;
                let mut value_resolution_ctx = ValueResolutionCtx::new(
                    ValueResolutionMode::Current,
                    item_spec_id.clone(),
                    type_name::<IS::Params<'_>>(),
                );
                params_spec
                    .resolve(resources, &mut value_resolution_ctx)
                    .map_err(crate::Error::ParamsResolveError)?
            };
            let data = <IS::Data<'_> as Data>::borrow(self.id(), resources);
            IS::state_current(fn_ctx, &params, data).await?
        };
        resources.borrow_mut::<Current<IS::State>>().0 = Some(state_current.clone());

        Ok(state_current)
    }

    async fn state_desired_try_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<Option<IS::State>, E> {
        let params_partial = {
            let item_spec_id = self.id();
            let params_spec = params_specs
                .get::<ParamsSpec<IS::Params<'_>>, _>(item_spec_id)
                .ok_or_else(|| crate::Error::ParamsSpecNotFound {
                    item_spec_id: item_spec_id.clone(),
                })?;
            let mut value_resolution_ctx = ValueResolutionCtx::new(
                ValueResolutionMode::Desired,
                item_spec_id.clone(),
                type_name::<IS::Params<'_>>(),
            );
            params_spec
                .resolve_partial(resources, &mut value_resolution_ctx)
                .map_err(crate::Error::ParamsResolveError)?
        };
        let data = <IS::Data<'_> as Data>::borrow(self.id(), resources);
        let state_desired = IS::try_state_desired(fn_ctx, &params_partial, data).await?;
        if let Some(state_desired) = state_desired.as_ref() {
            resources.borrow_mut::<Desired<IS::State>>().0 = Some(state_desired.clone());
        }

        Ok(state_desired)
    }

    async fn state_desired_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<IS::State, E> {
        let params = {
            let item_spec_id = self.id();
            let params_spec = params_specs
                .get::<ParamsSpec<IS::Params<'_>>, _>(item_spec_id)
                .ok_or_else(|| crate::Error::ParamsSpecNotFound {
                    item_spec_id: item_spec_id.clone(),
                })?;
            let mut value_resolution_ctx = ValueResolutionCtx::new(
                ValueResolutionMode::Desired,
                item_spec_id.clone(),
                type_name::<IS::Params<'_>>(),
            );
            params_spec
                .resolve(resources, &mut value_resolution_ctx)
                .map_err(crate::Error::ParamsResolveError)?
        };
        let data = <IS::Data<'_> as Data>::borrow(self.id(), resources);
        let state_desired = IS::state_desired(fn_ctx, &params, data).await?;
        resources.borrow_mut::<Desired<IS::State>>().0 = Some(state_desired.clone());

        Ok(state_desired)
    }

    async fn state_diff_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        states_a: &TypeMap<ItemSpecId, BoxDtDisplay>,
        states_b: &TypeMap<ItemSpecId, BoxDtDisplay>,
    ) -> Result<Option<IS::StateDiff>, E> {
        let item_spec_id = <IS as ItemSpec>::id(self);
        let state_base = states_a.get::<IS::State, _>(item_spec_id);
        let state_desired = states_b.get::<IS::State, _>(item_spec_id);

        if let Some((state_base, state_desired)) = state_base.zip(state_desired) {
            let state_diff: IS::StateDiff = self
                .state_diff_exec_with(params_specs, resources, state_base, state_desired)
                .await?;
            Ok(Some(state_diff))
        } else {
            // When we reach here, one of the following is true:
            //
            // * The current state cannot be retrieved, due to a predecessor's state not
            //   existing.
            // * The desired state cannot be retrieved, due to a predecessor's state not
            //   existing.
            // * A bug exists, e.g. the state is stored against the wrong type parameter.

            Ok(None)
        }
    }

    async fn state_diff_exec_with(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        state_a: &IS::State,
        state_b: &IS::State,
    ) -> Result<IS::StateDiff, E> {
        let state_diff: IS::StateDiff = {
            let params_partial = {
                let item_spec_id = self.id();
                let params_spec = params_specs
                    .get::<ParamsSpec<IS::Params<'_>>, _>(item_spec_id)
                    .ok_or_else(|| crate::Error::ParamsSpecNotFound {
                        item_spec_id: item_spec_id.clone(),
                    })?;

                // Running `diff` for a single profile will be between the current and desired
                // states, and parameters are not really intended to be used for diffing.
                //
                // However for `ShCmdItemSpec`, the shell script for diffing's path is in
                // params, which *likely* would be provided as direct `Value`s instead of
                // mapped from predecessors' state(s). Iff the values are mapped from a
                // predecessor's state, then we would want it to be the desired state, as that
                // is closest to the correct value -- `ValueResolutionMode::ApplyDry` is used in
                // `ItemSpec::apply_dry`, and `ValueResolutionMode::Apply` is used in
                // `ItemSpec::apply`.
                //
                // Running `diff` for multiple profiles will likely be between two profiles'
                // current states.
                let mut value_resolution_ctx = ValueResolutionCtx::new(
                    ValueResolutionMode::Desired,
                    item_spec_id.clone(),
                    type_name::<IS::Params<'_>>(),
                );
                params_spec
                    .resolve_partial(resources, &mut value_resolution_ctx)
                    .map_err(crate::Error::ParamsResolveError)?
            };
            let data = <IS::Data<'_> as Data>::borrow(self.id(), resources);
            IS::state_diff(&params_partial, data, state_a, state_b)
                .await
                .map_err(Into::<E>::into)?
        };

        Ok(state_diff)
    }

    async fn apply_check(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        state_current: &IS::State,
        state_desired: &IS::State,
        state_diff: &IS::StateDiff,
    ) -> Result<ApplyCheck, E> {
        let params_partial = {
            let item_spec_id = self.id();
            let params_spec = params_specs
                .get::<ParamsSpec<IS::Params<'_>>, _>(item_spec_id)
                .ok_or_else(|| crate::Error::ParamsSpecNotFound {
                    item_spec_id: item_spec_id.clone(),
                })?;

            // Normally an `apply_check` only compares the states / state diff.
            //
            // We use `ValueResolutionMode::Desired` because an apply is between the current
            // and desired states, and when resolving values, we want the target state's
            // parameters to be used. Note that during an apply, the desired state is
            // resolved as execution happens -- values that rely on predecessors' applied
            // state will be fed into successors' desired state.
            let mut value_resolution_ctx = ValueResolutionCtx::new(
                ValueResolutionMode::Desired,
                item_spec_id.clone(),
                type_name::<IS::Params<'_>>(),
            );
            params_spec
                .resolve_partial(resources, &mut value_resolution_ctx)
                .map_err(crate::Error::ParamsResolveError)?
        };
        let data = <IS::Data<'_> as Data>::borrow(self.id(), resources);
        if let Ok(params) = params_partial.try_into() {
            IS::apply_check(&params, data, state_current, state_desired, state_diff)
                .await
                .map_err(Into::<E>::into)
        } else {
            // > If we cannot resolve parameters, then this item, and its predecessor are
            // > cleaned up.
            //
            // The above is not necessarily true -- the user may have provided an incorrect
            // type to map from. However, it is more likely to be true than false.
            Ok(ApplyCheck::ExecNotRequired)
        }
    }

    async fn apply_exec_dry(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
        state_current: &IS::State,
        state_desired: &IS::State,
        state_diff: &IS::StateDiff,
    ) -> Result<IS::State, E> {
        let params = {
            let item_spec_id = self.id();
            let params_spec = params_specs
                .get::<ParamsSpec<IS::Params<'_>>, _>(item_spec_id)
                .ok_or_else(|| crate::Error::ParamsSpecNotFound {
                    item_spec_id: item_spec_id.clone(),
                })?;
            let mut value_resolution_ctx = ValueResolutionCtx::new(
                ValueResolutionMode::ApplyDry,
                item_spec_id.clone(),
                type_name::<IS::Params<'_>>(),
            );
            params_spec
                .resolve(resources, &mut value_resolution_ctx)
                .map_err(crate::Error::ParamsResolveError)?
        };
        let data = <IS::Data<'_> as Data>::borrow(self.id(), resources);
        let state_ensured_dry = IS::apply_dry(
            fn_ctx,
            &params,
            data,
            state_current,
            state_desired,
            state_diff,
        )
        .await
        .map_err(Into::<E>::into)?;

        resources.borrow_mut::<ApplyDry<IS::State>>().0 = Some(state_ensured_dry.clone());

        Ok(state_ensured_dry)
    }

    async fn apply_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
        state_current: &IS::State,
        state_desired: &IS::State,
        state_diff: &IS::StateDiff,
    ) -> Result<IS::State, E> {
        let params = {
            let item_spec_id = self.id();
            let params_spec = params_specs
                .get::<ParamsSpec<IS::Params<'_>>, _>(item_spec_id)
                .ok_or_else(|| crate::Error::ParamsSpecNotFound {
                    item_spec_id: item_spec_id.clone(),
                })?;
            let mut value_resolution_ctx = ValueResolutionCtx::new(
                ValueResolutionMode::Current,
                item_spec_id.clone(),
                type_name::<IS::Params<'_>>(),
            );
            params_spec
                .resolve(resources, &mut value_resolution_ctx)
                .map_err(crate::Error::ParamsResolveError)?
        };
        let data = <IS::Data<'_> as Data>::borrow(self.id(), resources);
        let state_ensured = IS::apply(
            fn_ctx,
            &params,
            data,
            state_current,
            state_desired,
            state_diff,
        )
        .await
        .map_err(Into::<E>::into)?;

        resources.borrow_mut::<Current<IS::State>>().0 = Some(state_ensured.clone());

        Ok(state_ensured)
    }
}

impl<IS, E> Debug for ItemSpecWrapper<IS, E>
where
    IS: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<IS, E> Deref for ItemSpecWrapper<IS, E> {
    type Target = IS;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<IS, E> DerefMut for ItemSpecWrapper<IS, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<IS, E> From<IS> for ItemSpecWrapper<IS, E>
where
    IS: Debug + ItemSpec + Send + Sync,
    E: Debug + Send + Sync + std::error::Error + From<<IS as ItemSpec>::Error> + 'static,
{
    fn from(item_spec: IS) -> Self {
        Self(item_spec, PhantomData)
    }
}

impl<IS, E> DataAccess for ItemSpecWrapper<IS, E>
where
    IS: Debug + ItemSpec + Send + Sync,
    E: Debug + Send + Sync + std::error::Error + From<<IS as ItemSpec>::Error> + 'static,
{
    fn borrows() -> TypeIds {
        let mut type_ids = <IS::Data<'_> as DataAccess>::borrows();
        type_ids.push(std::any::TypeId::of::<IS::Params<'_>>());

        type_ids
    }

    fn borrow_muts() -> TypeIds {
        <IS::Data<'_> as DataAccess>::borrow_muts()
    }
}

impl<IS, E> DataAccessDyn for ItemSpecWrapper<IS, E>
where
    IS: Debug + ItemSpec + Send + Sync,
    E: Debug + Send + Sync + std::error::Error + From<<IS as ItemSpec>::Error> + 'static,
{
    fn borrows(&self) -> TypeIds {
        let mut type_ids = <IS::Data<'_> as DataAccess>::borrows();
        type_ids.push(std::any::TypeId::of::<IS::Params<'_>>());

        type_ids
    }

    fn borrow_muts(&self) -> TypeIds {
        <IS::Data<'_> as DataAccess>::borrow_muts()
    }
}

#[async_trait(?Send)]
impl<IS, E> ItemSpecRt<E> for ItemSpecWrapper<IS, E>
where
    IS: Clone + Debug + ItemSpec + Send + Sync,
    E: Debug
        + Send
        + Sync
        + std::error::Error
        + From<<IS as ItemSpec>::Error>
        + From<crate::Error>
        + 'static,
    for<'params> <IS as ItemSpec>::Params<'params>:
        TryFrom<<<IS as ItemSpec>::Params<'params> as Params>::Partial>,
    for<'params> <IS::Params<'params> as Params>::Partial: From<IS::Params<'params>>,
{
    fn id(&self) -> &ItemSpecId {
        <IS as ItemSpec>::id(self)
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), E> {
        // Insert `XMarker<IS::State>` to create entries in `Resources`.
        // This is used for referential param values (#94)
        resources.insert(Clean::<IS::State>(None));
        resources.insert(Current::<IS::State>(None));
        resources.insert(Desired::<IS::State>(None));
        resources.insert(ApplyDry::<IS::State>(None));

        // Run user defined setup.
        <IS as ItemSpec>::setup(self, resources)
            .await
            .map_err(Into::<E>::into)
    }

    fn params_and_state_register(
        &self,
        item_spec_params_type_reg: &mut ItemSpecParamsTypeReg,
        params_specs_type_reg: &mut ParamsSpecsTypeReg,
        states_type_reg: &mut StatesTypeReg,
    ) {
        item_spec_params_type_reg.register::<IS::Params<'_>>(IS::id(self).clone());
        params_specs_type_reg.register::<ParamsSpec<IS::Params<'_>>>(IS::id(self).clone());
        states_type_reg.register::<IS::State>(IS::id(self).clone());
    }

    async fn state_clean(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
    ) -> Result<BoxDtDisplay, E> {
        self.state_clean(params_specs, resources)
            .await
            .map(BoxDtDisplay::new)
            .map_err(Into::<E>::into)
    }

    async fn state_current_try_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<Option<BoxDtDisplay>, E> {
        self.state_current_try_exec(params_specs, resources, fn_ctx)
            .await
            .map(|state_current| state_current.map(BoxDtDisplay::new))
            .map_err(Into::<E>::into)
    }

    async fn state_current_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<BoxDtDisplay, E> {
        self.state_current_exec(params_specs, resources, fn_ctx)
            .await
            .map(BoxDtDisplay::new)
            .map_err(Into::<E>::into)
    }

    async fn state_desired_try_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<Option<BoxDtDisplay>, E> {
        self.state_desired_try_exec(params_specs, resources, fn_ctx)
            .await
            .map(|state_desired| state_desired.map(BoxDtDisplay::new))
            .map_err(Into::<E>::into)
    }

    async fn state_desired_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<BoxDtDisplay, E> {
        self.state_desired_exec(params_specs, resources, fn_ctx)
            .await
            .map(BoxDtDisplay::new)
            .map_err(Into::<E>::into)
    }

    async fn state_diff_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        states_a: &TypeMap<ItemSpecId, BoxDtDisplay>,
        states_b: &TypeMap<ItemSpecId, BoxDtDisplay>,
    ) -> Result<Option<BoxDtDisplay>, E> {
        self.state_diff_exec(params_specs, resources, states_a, states_b)
            .await
            .map(|state_diff_opt| state_diff_opt.map(BoxDtDisplay::new))
            .map_err(Into::<E>::into)
    }

    async fn ensure_prepare(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
    ) -> Result<ItemApplyBoxed, (E, ItemApplyPartialBoxed)> {
        let mut item_apply_partial = ItemApplyPartial::<IS::State, IS::StateDiff>::new();

        match self
            .state_current_exec(params_specs, resources, fn_ctx)
            .await
        {
            Ok(state_current) => item_apply_partial.state_current = Some(state_current),
            Err(error) => return Err((error, item_apply_partial.into())),
        }
        #[cfg(feature = "output_progress")]
        fn_ctx.progress_sender().reset();
        match self
            .state_desired_exec(params_specs, resources, fn_ctx)
            .await
        {
            Ok(state_desired) => item_apply_partial.state_target = Some(state_desired),
            Err(error) => return Err((error, item_apply_partial.into())),
        }
        #[cfg(feature = "output_progress")]
        fn_ctx.progress_sender().reset();
        match self
            .state_diff_exec_with(
                params_specs,
                resources,
                item_apply_partial
                    .state_current
                    .as_ref()
                    .expect("unreachable: This is set just above."),
                item_apply_partial
                    .state_target
                    .as_ref()
                    .expect("unreachable: This is set just above."),
            )
            .await
        {
            Ok(state_diff) => item_apply_partial.state_diff = Some(state_diff),
            Err(error) => return Err((error, item_apply_partial.into())),
        }

        let (Some(state_current), Some(state_target), Some(state_diff)) = (
            item_apply_partial.state_current.as_ref(),
            item_apply_partial.state_target.as_ref(),
            item_apply_partial.state_diff.as_ref(),
        ) else {
            unreachable!("These are set just above.");
        };

        let state_applied = match self
            .apply_check(
                params_specs,
                resources,
                state_current,
                state_target,
                state_diff,
            )
            .await
        {
            Ok(apply_check) => {
                item_apply_partial.apply_check = Some(apply_check);

                // TODO: write test for this case
                match apply_check {
                    #[cfg(not(feature = "output_progress"))]
                    ApplyCheck::ExecRequired => None,
                    #[cfg(feature = "output_progress")]
                    ApplyCheck::ExecRequired { .. } => None,
                    ApplyCheck::ExecNotRequired => item_apply_partial.state_current.clone(),
                }
            }
            Err(error) => return Err((error, item_apply_partial.into())),
        };

        Ok(ItemApply::try_from((item_apply_partial, state_applied))
            .expect("unreachable: All the fields are set above.")
            .into())
    }

    async fn apply_exec_dry(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
        item_apply_boxed: &mut ItemApplyBoxed,
    ) -> Result<(), E> {
        let Some(item_apply) =
            item_apply_boxed.as_data_type_mut().downcast_mut::<ItemApply<IS::State, IS::StateDiff>>() else {
                panic!("Failed to downcast `ItemApplyBoxed` to `{concrete_type}`.\n\
                    This is a bug in the Peace framework.",
                    concrete_type = std::any::type_name::<ItemApply<IS::State, IS::StateDiff>>())
            };

        let ItemApply {
            state_saved: _,
            state_current,
            state_target,
            state_diff,
            apply_check,
            state_applied,
        } = item_apply;

        match apply_check {
            #[cfg(not(feature = "output_progress"))]
            ApplyCheck::ExecRequired => {
                let state_applied_dry = self
                    .apply_exec_dry(
                        params_specs,
                        resources,
                        fn_ctx,
                        state_current,
                        state_target,
                        state_diff,
                    )
                    .await?;

                *state_applied = Some(state_applied_dry);
            }
            #[cfg(feature = "output_progress")]
            ApplyCheck::ExecRequired { progress_limit: _ } => {
                let state_applied_dry = self
                    .apply_exec_dry(
                        params_specs,
                        resources,
                        fn_ctx,
                        state_current,
                        state_target,
                        state_diff,
                    )
                    .await?;

                *state_applied = Some(state_applied_dry);
            }
            ApplyCheck::ExecNotRequired => {}
        }

        Ok(())
    }

    async fn clean_prepare(
        &self,
        states_current: &StatesCurrent,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
    ) -> Result<ItemApplyBoxed, (E, ItemApplyPartialBoxed)> {
        let mut item_apply_partial = ItemApplyPartial::<IS::State, IS::StateDiff>::new();

        // Hack: Setting ItemApplyPartial state_current to state_clean is a hack.
        if let Some(state_current) = states_current.get::<IS::State, _>(self.id()) {
            item_apply_partial.state_current = Some(state_current.clone());
        } else {
            match self.state_clean(params_specs, resources).await {
                Ok(state_clean) => item_apply_partial.state_current = Some(state_clean),
                Err(error) => return Err((error, item_apply_partial.into())),
            }
        }
        match self.state_clean(params_specs, resources).await {
            Ok(state_clean) => item_apply_partial.state_target = Some(state_clean),
            Err(error) => return Err((error, item_apply_partial.into())),
        }

        match self
            .state_diff_exec_with(
                params_specs,
                resources,
                item_apply_partial
                    .state_current
                    .as_ref()
                    .expect("unreachable: This is confirmed just above."),
                item_apply_partial
                    .state_target
                    .as_ref()
                    .expect("unreachable: This is set just above."),
            )
            .await
        {
            Ok(state_diff) => item_apply_partial.state_diff = Some(state_diff),
            Err(error) => return Err((error, item_apply_partial.into())),
        }

        let (Some(state_current), Some(state_target), Some(state_diff)) = (
            item_apply_partial.state_current.as_ref(),
            item_apply_partial.state_target.as_ref(),
            item_apply_partial.state_diff.as_ref(),
        ) else {
            unreachable!("These are set just above.");
        };

        let state_applied = match self
            .apply_check(
                params_specs,
                resources,
                state_current,
                state_target,
                state_diff,
            )
            .await
        {
            Ok(apply_check) => {
                item_apply_partial.apply_check = Some(apply_check);

                // TODO: write test for this case
                match apply_check {
                    #[cfg(not(feature = "output_progress"))]
                    ApplyCheck::ExecRequired => None,
                    #[cfg(feature = "output_progress")]
                    ApplyCheck::ExecRequired { .. } => None,
                    ApplyCheck::ExecNotRequired => item_apply_partial.state_current.clone(),
                }
            }
            Err(error) => return Err((error, item_apply_partial.into())),
        };

        Ok(ItemApply::try_from((item_apply_partial, state_applied))
            .expect("unreachable: All the fields are set above.")
            .into())
    }

    async fn apply_exec(
        &self,
        params_specs: &ParamsSpecs,
        resources: &Resources<SetUp>,
        fn_ctx: FnCtx<'_>,
        item_apply_boxed: &mut ItemApplyBoxed,
    ) -> Result<(), E> {
        let Some(item_apply) =
            item_apply_boxed.as_data_type_mut().downcast_mut::<ItemApply<IS::State, IS::StateDiff>>() else {
                panic!("Failed to downcast `ItemApplyBoxed` to `{concrete_type}`.\n\
                    This is a bug in the Peace framework.",
                    concrete_type = std::any::type_name::<ItemApply<IS::State, IS::StateDiff>>())
            };

        let ItemApply {
            state_saved: _,
            state_current,
            state_target,
            state_diff,
            apply_check,
            state_applied,
        } = item_apply;

        match apply_check {
            #[cfg(not(feature = "output_progress"))]
            ApplyCheck::ExecRequired => {
                let state_applied_next = self
                    .apply_exec(
                        params_specs,
                        resources,
                        fn_ctx,
                        state_current,
                        state_target,
                        state_diff,
                    )
                    .await?;

                *state_applied = Some(state_applied_next);
            }
            #[cfg(feature = "output_progress")]
            ApplyCheck::ExecRequired { progress_limit: _ } => {
                let state_applied_next = self
                    .apply_exec(
                        params_specs,
                        resources,
                        fn_ctx,
                        state_current,
                        state_target,
                        state_diff,
                    )
                    .await?;

                *state_applied = Some(state_applied_next);
            }
            ApplyCheck::ExecNotRequired => {}
        }

        Ok(())
    }
}
