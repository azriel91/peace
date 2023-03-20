use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use fn_graph::{DataAccess, DataAccessDyn, TypeIds};
use peace_cfg::{async_trait, ItemSpec, ItemSpecId, OpCheckStatus, OpCtx, TryFnSpec};
use peace_data::{
    marker::{ApplyDry, Clean, Current, Desired},
    Data,
};
use peace_resources::{
    resources::ts::{Empty, SetUp},
    states::{States, StatesCurrent, StatesDesired, StatesSaved},
    type_reg::untagged::BoxDtDisplay,
    Resources,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    outcomes::{ItemApply, ItemApplyBoxed, ItemApplyPartial, ItemApplyPartialBoxed},
    ItemSpecRt, StatesTypeRegs,
};

/// Wraps a type implementing [`ItemSpec`].
#[allow(clippy::type_complexity)]
pub struct ItemSpecWrapper<
    IS,
    E,
    State,
    StateDiff,
    StateCurrentFnSpec,
    StateDesiredFnSpec,
    StateDiffFnSpec,
    ApplyOpSpec,
>(
    IS,
    PhantomData<(
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        ApplyOpSpec,
    )>,
);

impl<IS, E, State, StateDiff, StateCurrentFnSpec, StateDesiredFnSpec, StateDiffFnSpec, ApplyOpSpec>
    Clone
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        ApplyOpSpec,
    >
where
    IS: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<IS, E, State, StateDiff, StateCurrentFnSpec, StateDesiredFnSpec, StateDiffFnSpec, ApplyOpSpec>
    ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        ApplyOpSpec,
    >
where
    IS: Debug
        + ItemSpec<
            State = State,
            StateDiff = StateDiff,
            StateCurrentFnSpec = StateCurrentFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            StateDiffFnSpec = StateDiffFnSpec,
            ApplyOpSpec = ApplyOpSpec,
        > + Send
        + Sync,
    E: Debug
        + Send
        + Sync
        + std::error::Error
        + From<<IS as ItemSpec>::Error>
        + From<crate::Error>
        + 'static,
    State: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateCurrentFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State> + Send + Sync,
    StateDesiredFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State> + Send + Sync,
    StateDiffFnSpec: Debug
        + peace_cfg::StateDiffFnSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    ApplyOpSpec: Debug
        + peace_cfg::ApplyOpSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
{
    async fn state_clean<ResourcesTs>(
        &self,
        resources: &Resources<ResourcesTs>,
    ) -> Result<State, E> {
        let state_clean = {
            let data =
                <<IS as peace_cfg::ItemSpec>::Data<'_> as Data>::borrow(self.id(), resources);
            <IS as peace_cfg::ItemSpec>::state_clean(data).await?
        };
        resources.borrow_mut::<Clean<State>>().0 = Some(state_clean.clone());

        Ok(state_clean)
    }

    async fn state_current_try_exec<ResourcesTs>(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<ResourcesTs>,
    ) -> Result<Option<State>, E> {
        let state_current = {
            let data = <<StateCurrentFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(
                self.id(),
                resources,
            );
            <StateCurrentFnSpec as TryFnSpec>::try_exec(op_ctx, data).await?
        };
        if let Some(state_current) = state_current.as_ref() {
            resources.borrow_mut::<Current<State>>().0 = Some(state_current.clone());
        }

        Ok(state_current)
    }

    async fn state_current_exec<ResourcesTs>(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<ResourcesTs>,
    ) -> Result<State, E> {
        let state_current = {
            let data = <<StateCurrentFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(
                self.id(),
                resources,
            );
            <StateCurrentFnSpec as TryFnSpec>::exec(op_ctx, data).await?
        };
        resources.borrow_mut::<Current<State>>().0 = Some(state_current.clone());

        Ok(state_current)
    }

    async fn state_desired_try_exec(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<SetUp>,
    ) -> Result<Option<State>, E> {
        let data = <<StateDesiredFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(
            self.id(),
            resources,
        );
        let state_desired =
            <StateDesiredFnSpec as peace_cfg::TryFnSpec>::try_exec(op_ctx, data).await?;
        if let Some(state_desired) = state_desired.as_ref() {
            resources.borrow_mut::<Desired<State>>().0 = Some(state_desired.clone());
        }

        Ok(state_desired)
    }

    async fn state_desired_exec(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<SetUp>,
    ) -> Result<State, E> {
        let data = <<StateDesiredFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(
            self.id(),
            resources,
        );
        let state_desired =
            <StateDesiredFnSpec as peace_cfg::TryFnSpec>::exec(op_ctx, data).await?;
        resources.borrow_mut::<Desired<State>>().0 = Some(state_desired.clone());

        Ok(state_desired)
    }

    async fn state_diff_exec<ResourcesTs, StatesTs>(
        &self,
        resources: &Resources<ResourcesTs>,
        states_base: &States<StatesTs>,
        states_desired: &StatesDesired,
    ) -> Result<Option<StateDiff>, E>
    where
        StatesTs: Debug + Send + Sync + 'static,
    {
        let item_spec_id = <IS as ItemSpec>::id(self);
        let state_base = states_base.get::<State, _>(item_spec_id);
        let state_desired = states_desired.get::<State, _>(item_spec_id);

        if let Some((state_base, state_desired)) = state_base.zip(state_desired) {
            let state_diff: StateDiff = self
                .state_diff_exec_with(resources, state_base, state_desired)
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

    async fn state_diff_exec_with<ResourcesTs>(
        &self,
        resources: &Resources<ResourcesTs>,
        state_base: &State,
        state_desired: &State,
    ) -> Result<StateDiff, E> {
        let state_diff: StateDiff = {
            let data = <<StateDiffFnSpec as peace_cfg::StateDiffFnSpec>::Data<'_> as Data>::borrow(
                self.id(),
                resources,
            );
            <StateDiffFnSpec as peace_cfg::StateDiffFnSpec>::exec(data, state_base, state_desired)
                .await
                .map_err(Into::<E>::into)?
        };

        Ok(state_diff)
    }

    async fn apply_op_check<ResourcesTs>(
        &self,
        resources: &Resources<ResourcesTs>,
        state_current: &State,
        state_desired: &State,
        state_diff: &StateDiff,
    ) -> Result<OpCheckStatus, E> {
        let data = <<ApplyOpSpec as peace_cfg::ApplyOpSpec>::Data<'_> as Data>::borrow(
            self.id(),
            resources,
        );
        <ApplyOpSpec as peace_cfg::ApplyOpSpec>::check(
            data,
            state_current,
            state_desired,
            state_diff,
        )
        .await
        .map_err(Into::<E>::into)
    }

    async fn apply_op_exec_dry<ResourcesTs>(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<ResourcesTs>,
        state_current: &State,
        state_desired: &State,
        state_diff: &StateDiff,
    ) -> Result<State, E> {
        let data = <<ApplyOpSpec as peace_cfg::ApplyOpSpec>::Data<'_> as Data>::borrow(
            self.id(),
            resources,
        );
        let state_ensured_dry = <ApplyOpSpec as peace_cfg::ApplyOpSpec>::exec_dry(
            op_ctx,
            data,
            state_current,
            state_desired,
            state_diff,
        )
        .await
        .map_err(Into::<E>::into)?;

        resources.borrow_mut::<ApplyDry<State>>().0 = Some(state_ensured_dry.clone());

        Ok(state_ensured_dry)
    }

    async fn apply_op_exec<ResourcesTs>(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<ResourcesTs>,
        state_current: &State,
        state_desired: &State,
        state_diff: &StateDiff,
    ) -> Result<State, E> {
        let data = <<ApplyOpSpec as peace_cfg::ApplyOpSpec>::Data<'_> as Data>::borrow(
            self.id(),
            resources,
        );
        let state_ensured = <ApplyOpSpec as peace_cfg::ApplyOpSpec>::exec(
            op_ctx,
            data,
            state_current,
            state_desired,
            state_diff,
        )
        .await
        .map_err(Into::<E>::into)?;

        resources.borrow_mut::<Current<State>>().0 = Some(state_ensured.clone());

        Ok(state_ensured)
    }
}

impl<IS, E, State, StateDiff, StateCurrentFnSpec, StateDesiredFnSpec, StateDiffFnSpec, ApplyOpSpec>
    Debug
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        ApplyOpSpec,
    >
where
    IS: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<IS, E, State, StateDiff, StateCurrentFnSpec, StateDesiredFnSpec, StateDiffFnSpec, ApplyOpSpec>
    Deref
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        ApplyOpSpec,
    >
{
    type Target = IS;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<IS, E, State, StateDiff, StateCurrentFnSpec, StateDesiredFnSpec, StateDiffFnSpec, ApplyOpSpec>
    DerefMut
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        ApplyOpSpec,
    >
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<IS, E, State, StateDiff, StateCurrentFnSpec, StateDesiredFnSpec, StateDiffFnSpec, ApplyOpSpec>
    From<IS>
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        ApplyOpSpec,
    >
where
    IS: Debug
        + ItemSpec<
            State = State,
            StateDiff = StateDiff,
            StateCurrentFnSpec = StateCurrentFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            StateDiffFnSpec = StateDiffFnSpec,
            ApplyOpSpec = ApplyOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + From<<IS as ItemSpec>::Error> + 'static,
    State: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateCurrentFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State> + Send + Sync,
    StateDesiredFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State> + Send + Sync,
    StateDiffFnSpec: Debug
        + peace_cfg::StateDiffFnSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    ApplyOpSpec: Debug
        + peace_cfg::ApplyOpSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
{
    fn from(item_spec: IS) -> Self {
        Self(item_spec, PhantomData)
    }
}

impl<IS, E, State, StateDiff, StateCurrentFnSpec, StateDesiredFnSpec, StateDiffFnSpec, ApplyOpSpec>
    DataAccess
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        ApplyOpSpec,
    >
where
    IS: Debug
        + ItemSpec<
            State = State,
            StateDiff = StateDiff,
            StateCurrentFnSpec = StateCurrentFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            StateDiffFnSpec = StateDiffFnSpec,
            ApplyOpSpec = ApplyOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + From<<IS as ItemSpec>::Error> + 'static,
    State: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateCurrentFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State> + Send + Sync,
    StateDesiredFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State> + Send + Sync,
    StateDiffFnSpec: Debug
        + peace_cfg::StateDiffFnSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    ApplyOpSpec: Debug
        + peace_cfg::ApplyOpSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
{
    fn borrows() -> TypeIds {
        <<ApplyOpSpec as peace_cfg::ApplyOpSpec>::Data<'_> as DataAccess>::borrows()
    }

    fn borrow_muts() -> TypeIds {
        <<ApplyOpSpec as peace_cfg::ApplyOpSpec>::Data<'_> as DataAccess>::borrow_muts()
    }
}

impl<IS, E, State, StateDiff, StateCurrentFnSpec, StateDesiredFnSpec, StateDiffFnSpec, ApplyOpSpec>
    DataAccessDyn
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        ApplyOpSpec,
    >
where
    IS: Debug
        + ItemSpec<
            State = State,
            StateDiff = StateDiff,
            StateCurrentFnSpec = StateCurrentFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            StateDiffFnSpec = StateDiffFnSpec,
            ApplyOpSpec = ApplyOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + From<<IS as ItemSpec>::Error> + 'static,
    State: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateCurrentFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State> + Send + Sync,
    StateDesiredFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State> + Send + Sync,
    StateDiffFnSpec: Debug
        + peace_cfg::StateDiffFnSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    ApplyOpSpec: Debug
        + peace_cfg::ApplyOpSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
{
    fn borrows(&self) -> TypeIds {
        <<ApplyOpSpec as peace_cfg::ApplyOpSpec>::Data<'_> as DataAccess>::borrows()
    }

    fn borrow_muts(&self) -> TypeIds {
        <<ApplyOpSpec as peace_cfg::ApplyOpSpec>::Data<'_> as DataAccess>::borrow_muts()
    }
}

#[async_trait(?Send)]
impl<IS, E, State, StateDiff, StateCurrentFnSpec, StateDesiredFnSpec, StateDiffFnSpec, ApplyOpSpec>
    ItemSpecRt<E>
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        ApplyOpSpec,
    >
where
    IS: Clone
        + Debug
        + ItemSpec<
            State = State,
            StateDiff = StateDiff,
            StateCurrentFnSpec = StateCurrentFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            StateDiffFnSpec = StateDiffFnSpec,
            ApplyOpSpec = ApplyOpSpec,
        > + Send
        + Sync,
    E: Debug
        + Send
        + Sync
        + std::error::Error
        + From<<IS as ItemSpec>::Error>
        + From<crate::Error>
        + 'static,
    State: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateCurrentFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State> + Send + Sync,
    StateDesiredFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State> + Send + Sync,
    StateDiffFnSpec: Debug
        + peace_cfg::StateDiffFnSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    ApplyOpSpec: Debug
        + peace_cfg::ApplyOpSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
{
    fn id(&self) -> &ItemSpecId {
        <IS as ItemSpec>::id(self)
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), E> {
        // Insert `XMarker<State>` to create entries in `Resources`.
        // This is used for referential param values (#94)
        resources.insert(Clean::<State>(None));
        resources.insert(Current::<State>(None));
        resources.insert(Desired::<State>(None));
        resources.insert(ApplyDry::<State>(None));

        // Run user defined setup.
        <IS as ItemSpec>::setup(self, resources)
            .await
            .map_err(Into::<E>::into)
    }

    fn state_register(&self, states_type_regs: &mut StatesTypeRegs) {
        states_type_regs
            .states_current_type_reg_mut()
            .register::<State>(<IS as ItemSpec>::id(self).clone());

        states_type_regs
            .states_desired_type_reg_mut()
            .register::<State>(<IS as ItemSpec>::id(self).clone());
    }

    async fn state_clean(&self, resources: &Resources<SetUp>) -> Result<BoxDtDisplay, E> {
        self.state_clean(resources)
            .await
            .map(BoxDtDisplay::new)
            .map_err(Into::<E>::into)
    }

    async fn state_current_try_exec(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<SetUp>,
    ) -> Result<Option<BoxDtDisplay>, E> {
        self.state_current_try_exec(op_ctx, resources)
            .await
            .map(|state_current| state_current.map(BoxDtDisplay::new))
            .map_err(Into::<E>::into)
    }

    async fn state_current_exec(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<SetUp>,
    ) -> Result<BoxDtDisplay, E> {
        self.state_current_exec(op_ctx, resources)
            .await
            .map(BoxDtDisplay::new)
            .map_err(Into::<E>::into)
    }

    async fn state_desired_try_exec(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<SetUp>,
    ) -> Result<Option<BoxDtDisplay>, E> {
        self.state_desired_try_exec(op_ctx, resources)
            .await
            .map(|state_desired| state_desired.map(BoxDtDisplay::new))
            .map_err(Into::<E>::into)
    }

    async fn state_desired_exec(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<SetUp>,
    ) -> Result<BoxDtDisplay, E> {
        self.state_desired_exec(op_ctx, resources)
            .await
            .map(BoxDtDisplay::new)
            .map_err(Into::<E>::into)
    }

    async fn state_diff_exec_with_states_saved(
        &self,
        resources: &Resources<SetUp>,
        states_saved: &StatesSaved,
        states_desired: &StatesDesired,
    ) -> Result<Option<BoxDtDisplay>, E> {
        self.state_diff_exec(resources, states_saved, states_desired)
            .await
            .map(|state_diff_opt| state_diff_opt.map(BoxDtDisplay::new))
            .map_err(Into::<E>::into)
    }

    async fn state_diff_exec_with_states_current(
        &self,
        resources: &Resources<SetUp>,
        states_current: &StatesCurrent,
        states_desired: &StatesDesired,
    ) -> Result<Option<BoxDtDisplay>, E> {
        self.state_diff_exec(resources, states_current, states_desired)
            .await
            .map(|state_diff_opt| state_diff_opt.map(BoxDtDisplay::new))
            .map_err(Into::<E>::into)
    }

    async fn ensure_prepare(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<SetUp>,
    ) -> Result<ItemApplyBoxed, (E, ItemApplyPartialBoxed)> {
        let mut item_apply_partial = ItemApplyPartial::<State, StateDiff>::new();

        match self.state_current_exec(op_ctx, resources).await {
            Ok(state_current) => item_apply_partial.state_current = Some(state_current),
            Err(error) => return Err((error, item_apply_partial.into())),
        }
        match self.state_desired_exec(op_ctx, resources).await {
            Ok(state_desired) => item_apply_partial.state_target = Some(state_desired),
            Err(error) => return Err((error, item_apply_partial.into())),
        }
        match self
            .state_diff_exec_with(
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

        match self
            .apply_op_check(resources, state_current, state_target, state_diff)
            .await
        {
            Ok(op_check_status) => item_apply_partial.op_check_status = Some(op_check_status),
            Err(error) => return Err((error, item_apply_partial.into())),
        }

        Ok(ItemApply::try_from((item_apply_partial, None))
            .expect("unreachable: All the fields are set above.")
            .into())
    }

    async fn apply_exec_dry(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<SetUp>,
        item_apply_boxed: &mut ItemApplyBoxed,
    ) -> Result<(), E> {
        let Some(item_apply) =
            item_apply_boxed.as_data_type_mut().downcast_mut::<ItemApply<State, StateDiff>>() else {
                panic!("Failed to downcast `ItemApplyBoxed` to `{concrete_type}`.\n\
                    This is a bug in the Peace framework.",
                    concrete_type = std::any::type_name::<ItemApply<State, StateDiff>>())
            };

        let ItemApply {
            state_saved: _,
            state_current,
            state_target,
            state_diff,
            op_check_status,
            state_applied,
        } = item_apply;

        match op_check_status {
            #[cfg(not(feature = "output_progress"))]
            OpCheckStatus::ExecRequired => {
                let state_applied_dry = self
                    .apply_op_exec_dry(op_ctx, resources, state_current, state_target, state_diff)
                    .await?;

                *state_applied = Some(state_applied_dry);
            }
            #[cfg(feature = "output_progress")]
            OpCheckStatus::ExecRequired { progress_limit: _ } => {
                let state_applied_dry = self
                    .apply_op_exec_dry(op_ctx, resources, state_current, state_target, state_diff)
                    .await?;

                *state_applied = Some(state_applied_dry);
            }
            OpCheckStatus::ExecNotRequired => {}
        }

        Ok(())
    }

    async fn clean_prepare(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<SetUp>,
    ) -> Result<ItemApplyBoxed, (E, ItemApplyPartialBoxed)> {
        let mut item_apply_partial = ItemApplyPartial::<State, StateDiff>::new();

        match self.state_current_try_exec(op_ctx, resources).await {
            Ok(state_current) => {
                // Hack: Setting ItemApplyPartial state_current to state_clean is a hack.
                if let Some(state_current) = state_current {
                    item_apply_partial.state_current = Some(state_current);
                } else {
                    match self.state_clean(resources).await {
                        Ok(state_clean) => item_apply_partial.state_current = Some(state_clean),
                        Err(error) => return Err((error, item_apply_partial.into())),
                    }
                }
            }
            Err(error) => return Err((error, item_apply_partial.into())),
        }
        match self.state_clean(resources).await {
            Ok(state_clean) => item_apply_partial.state_target = Some(state_clean),
            Err(error) => return Err((error, item_apply_partial.into())),
        }

        match self
            .state_diff_exec_with(
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

        match self
            .apply_op_check(resources, state_current, state_target, state_diff)
            .await
        {
            Ok(op_check_status) => item_apply_partial.op_check_status = Some(op_check_status),
            Err(error) => return Err((error, item_apply_partial.into())),
        }

        Ok(ItemApply::try_from((item_apply_partial, None))
            .expect("unreachable: All the fields are set above.")
            .into())
    }

    async fn apply_exec(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<SetUp>,
        item_apply_boxed: &mut ItemApplyBoxed,
    ) -> Result<(), E> {
        let Some(item_apply) =
            item_apply_boxed.as_data_type_mut().downcast_mut::<ItemApply<State, StateDiff>>() else {
                panic!("Failed to downcast `ItemApplyBoxed` to `{concrete_type}`.\n\
                    This is a bug in the Peace framework.",
                    concrete_type = std::any::type_name::<ItemApply<State, StateDiff>>())
            };

        let ItemApply {
            state_saved: _,
            state_current,
            state_target,
            state_diff,
            op_check_status,
            state_applied,
        } = item_apply;

        match op_check_status {
            #[cfg(not(feature = "output_progress"))]
            OpCheckStatus::ExecRequired => {
                let state_applied_next = self
                    .apply_op_exec(op_ctx, resources, state_current, state_target, state_diff)
                    .await?;

                *state_applied = Some(state_applied_next);
            }
            #[cfg(feature = "output_progress")]
            OpCheckStatus::ExecRequired { progress_limit: _ } => {
                let state_applied_next = self
                    .apply_op_exec(op_ctx, resources, state_current, state_target, state_diff)
                    .await?;

                *state_applied = Some(state_applied_next);
            }
            OpCheckStatus::ExecNotRequired => {}
        }

        Ok(())
    }
}
