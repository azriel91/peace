use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use fn_graph::{DataAccess, DataAccessDyn, TypeIds};
use peace_cfg::{async_trait, ItemSpec, ItemSpecId, OpCheckStatus, OpCtx, TryFnSpec};
use peace_data::Data;
use peace_resources::{
    resources::ts::{Empty, SetUp},
    states::{StateDiffs, States, StatesCurrent, StatesDesired, StatesSaved},
    type_reg::untagged::BoxDtDisplay,
    Resources,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    outcomes::{ItemEnsure, ItemEnsureBoxed, ItemEnsurePartial, ItemEnsurePartialBoxed},
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
    EnsureOpSpec,
    CleanOpSpec,
>(
    IS,
    PhantomData<(
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    )>,
);

impl<
    IS,
    E,
    State,
    StateDiff,
    StateCurrentFnSpec,
    StateDesiredFnSpec,
    StateDiffFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> Clone
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    IS: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<
    IS,
    E,
    State,
    StateDiff,
    StateCurrentFnSpec,
    StateDesiredFnSpec,
    StateDiffFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
>
    ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    IS: Debug
        + ItemSpec<
            State = State,
            StateDiff = StateDiff,
            StateCurrentFnSpec = StateCurrentFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            StateDiffFnSpec = StateDiffFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
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
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<Error = <IS as ItemSpec>::Error, State = State>
        + Send
        + Sync,
{
    async fn state_current_try_exec<ResourcesTs>(
        &self,
        resources: &Resources<ResourcesTs>,
    ) -> Result<Option<State>, E> {
        let state_current = {
            let data = <<StateCurrentFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(
                self.id(),
                resources,
            );
            <StateCurrentFnSpec as TryFnSpec>::try_exec(data).await?
        };

        Ok(state_current)
    }

    async fn state_current_exec<ResourcesTs>(
        &self,
        resources: &Resources<ResourcesTs>,
    ) -> Result<State, E> {
        let state_current = {
            let data = <<StateCurrentFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(
                self.id(),
                resources,
            );
            <StateCurrentFnSpec as TryFnSpec>::exec(data).await?
        };

        Ok(state_current)
    }

    async fn state_desired_try_exec(
        &self,
        resources: &Resources<SetUp>,
    ) -> Result<Option<State>, E> {
        let data = <<StateDesiredFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(
            self.id(),
            resources,
        );
        let state_desired = <StateDesiredFnSpec as peace_cfg::TryFnSpec>::try_exec(data).await?;

        Ok(state_desired)
    }

    async fn state_desired_exec(&self, resources: &Resources<SetUp>) -> Result<State, E> {
        let data = <<StateDesiredFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(
            self.id(),
            resources,
        );
        let state_desired = <StateDesiredFnSpec as peace_cfg::TryFnSpec>::exec(data).await?;

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

    async fn ensure_op_check<ResourcesTs>(
        &self,
        resources: &Resources<ResourcesTs>,
        state_current: &State,
        state_desired: &State,
        state_diff: &StateDiff,
    ) -> Result<OpCheckStatus, E> {
        let data = <<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_> as Data>::borrow(
            self.id(),
            resources,
        );
        <EnsureOpSpec as peace_cfg::EnsureOpSpec>::check(
            data,
            state_current,
            state_desired,
            state_diff,
        )
        .await
        .map_err(Into::<E>::into)
    }

    async fn ensure_op_exec_dry<ResourcesTs>(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<ResourcesTs>,
        state_current: &State,
        state_desired: &State,
        state_diff: &StateDiff,
    ) -> Result<State, E> {
        let data = <<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_> as Data>::borrow(
            self.id(),
            resources,
        );
        <EnsureOpSpec as peace_cfg::EnsureOpSpec>::exec_dry(
            op_ctx,
            data,
            state_current,
            state_desired,
            state_diff,
        )
        .await
        .map_err(Into::<E>::into)
    }

    async fn ensure_op_exec<ResourcesTs>(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<ResourcesTs>,
        state_current: &State,
        state_desired: &State,
        state_diff: &StateDiff,
    ) -> Result<State, E> {
        let data = <<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_> as Data>::borrow(
            self.id(),
            resources,
        );
        <EnsureOpSpec as peace_cfg::EnsureOpSpec>::exec(
            op_ctx,
            data,
            state_current,
            state_desired,
            state_diff,
        )
        .await
        .map_err(Into::<E>::into)
    }
}

impl<
    IS,
    E,
    State,
    StateDiff,
    StateCurrentFnSpec,
    StateDesiredFnSpec,
    StateDiffFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> Debug
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    IS: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<
    IS,
    E,
    State,
    StateDiff,
    StateCurrentFnSpec,
    StateDesiredFnSpec,
    StateDiffFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> Deref
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
{
    type Target = IS;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<
    IS,
    E,
    State,
    StateDiff,
    StateCurrentFnSpec,
    StateDesiredFnSpec,
    StateDiffFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> DerefMut
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<
    IS,
    E,
    State,
    StateDiff,
    StateCurrentFnSpec,
    StateDesiredFnSpec,
    StateDiffFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> From<IS>
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    IS: Debug
        + ItemSpec<
            State = State,
            StateDiff = StateDiff,
            StateCurrentFnSpec = StateCurrentFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            StateDiffFnSpec = StateDiffFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
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
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<Error = <IS as ItemSpec>::Error, State = State>
        + Send
        + Sync,
{
    fn from(item_spec: IS) -> Self {
        Self(item_spec, PhantomData)
    }
}

impl<
    IS,
    E,
    State,
    StateDiff,
    StateCurrentFnSpec,
    StateDesiredFnSpec,
    StateDiffFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> DataAccess
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    IS: Debug
        + ItemSpec<
            State = State,
            StateDiff = StateDiff,
            StateCurrentFnSpec = StateCurrentFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            StateDiffFnSpec = StateDiffFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
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
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<Error = <IS as ItemSpec>::Error, State = State>
        + Send
        + Sync,
{
    fn borrows() -> TypeIds {
        <<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_> as DataAccess>::borrows()
    }

    fn borrow_muts() -> TypeIds {
        <<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_> as DataAccess>::borrow_muts()
    }
}

impl<
    IS,
    E,
    State,
    StateDiff,
    StateCurrentFnSpec,
    StateDesiredFnSpec,
    StateDiffFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> DataAccessDyn
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    IS: Debug
        + ItemSpec<
            State = State,
            StateDiff = StateDiff,
            StateCurrentFnSpec = StateCurrentFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            StateDiffFnSpec = StateDiffFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
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
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<Error = <IS as ItemSpec>::Error, State = State>
        + Send
        + Sync,
{
    fn borrows(&self) -> TypeIds {
        <<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_> as DataAccess>::borrows()
    }

    fn borrow_muts(&self) -> TypeIds {
        <<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_> as DataAccess>::borrow_muts()
    }
}

#[async_trait(?Send)]
impl<
    IS,
    E,
    State,
    StateDiff,
    StateCurrentFnSpec,
    StateDesiredFnSpec,
    StateDiffFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> ItemSpecRt<E>
    for ItemSpecWrapper<
        IS,
        E,
        State,
        StateDiff,
        StateCurrentFnSpec,
        StateDesiredFnSpec,
        StateDiffFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
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
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
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
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            Error = <IS as ItemSpec>::Error,
            State = State,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<Error = <IS as ItemSpec>::Error, State = State>
        + Send
        + Sync,
{
    fn id(&self) -> &ItemSpecId {
        <IS as ItemSpec>::id(self)
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), E> {
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

    async fn state_current_try_exec(
        &self,
        resources: &Resources<SetUp>,
    ) -> Result<Option<BoxDtDisplay>, E> {
        self.state_current_try_exec(resources)
            .await
            .map(|state_current| state_current.map(BoxDtDisplay::new))
            .map_err(Into::<E>::into)
    }

    async fn state_current_exec(&self, resources: &Resources<SetUp>) -> Result<BoxDtDisplay, E> {
        self.state_current_exec(resources)
            .await
            .map(BoxDtDisplay::new)
            .map_err(Into::<E>::into)
    }

    /// `states_current` and `state_diffs` are not needed by the discovery, but
    /// are here as markers that this method should be called after the caller
    /// has previously diffed the desired states to states discovered in the
    /// current execution.
    async fn state_ensured_exec(
        &self,
        resources: &Resources<SetUp>,
        _states_current: &StatesCurrent,
        _state_diffs: &StateDiffs,
    ) -> Result<BoxDtDisplay, E> {
        // The ensured state is the current state re-discovered
        // after `EnsureOpSpec::exec` has run.
        self.state_current_exec(resources)
            .await
            .map(BoxDtDisplay::new)
            .map_err(Into::<E>::into)
    }

    /// `states_current` is not needed by the discovery, but is here as a marker
    /// that this method should be called after the caller has previously saved
    /// the state of the item.
    async fn state_cleaned_try_exec(
        &self,
        resources: &Resources<SetUp>,
        _states_current: &StatesCurrent,
    ) -> Result<Option<BoxDtDisplay>, E> {
        // The cleaned state is the current state re-discovered
        // after `CleanOpSpec::exec` has run.
        self.state_current_try_exec(resources)
            .await
            .map(|state_current| state_current.map(BoxDtDisplay::new))
            .map_err(Into::<E>::into)
    }

    async fn state_desired_try_exec(
        &self,
        resources: &Resources<SetUp>,
    ) -> Result<Option<BoxDtDisplay>, E> {
        self.state_desired_try_exec(resources)
            .await
            .map(|state_desired| state_desired.map(BoxDtDisplay::new))
            .map_err(Into::<E>::into)
    }

    async fn state_desired_exec(&self, resources: &Resources<SetUp>) -> Result<BoxDtDisplay, E> {
        self.state_desired_exec(resources)
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
        resources: &Resources<SetUp>,
    ) -> Result<ItemEnsureBoxed, (E, ItemEnsurePartialBoxed)> {
        let mut item_ensure_partial = ItemEnsurePartial::<State, StateDiff>::new();

        match self.state_current_exec(resources).await {
            Ok(state_current) => item_ensure_partial.state_current = Some(state_current),
            Err(error) => return Err((error, item_ensure_partial.into())),
        }
        match self.state_desired_exec(resources).await {
            Ok(state_desired) => item_ensure_partial.state_desired = Some(state_desired),
            Err(error) => return Err((error, item_ensure_partial.into())),
        }
        match self
            .state_diff_exec_with(
                resources,
                item_ensure_partial
                    .state_current
                    .as_ref()
                    .expect("unreachable: This is set just above."),
                item_ensure_partial
                    .state_desired
                    .as_ref()
                    .expect("unreachable: This is set just above."),
            )
            .await
        {
            Ok(state_diff) => item_ensure_partial.state_diff = Some(state_diff),
            Err(error) => return Err((error, item_ensure_partial.into())),
        }

        let (Some(state_current), Some(state_desired), Some(state_diff)) = (
            item_ensure_partial.state_current.as_ref(),
            item_ensure_partial.state_desired.as_ref(),
            item_ensure_partial.state_diff.as_ref(),
        ) else {
            unreachable!("These are set just above.");
        };

        match self
            .ensure_op_check(resources, state_current, state_desired, state_diff)
            .await
        {
            Ok(op_check_status) => item_ensure_partial.op_check_status = Some(op_check_status),
            Err(error) => return Err((error, item_ensure_partial.into())),
        }

        Ok(ItemEnsure::try_from((item_ensure_partial, None))
            .expect("unreachable: All the fields are set above.")
            .into())
    }

    async fn ensure_exec_dry(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<SetUp>,
        item_ensure_boxed: &mut ItemEnsureBoxed,
    ) -> Result<(), E> {
        let Some(item_ensure) =
            item_ensure_boxed.as_data_type_mut().downcast_mut::<ItemEnsure<State, StateDiff>>() else {
                panic!("Failed to downcast `ItemEnsureBoxed` to `{concrete_type}`.\n\
                    This is a bug in the Peace framework.",
                    concrete_type = std::any::type_name::<ItemEnsure<State, StateDiff>>())
            };

        let ItemEnsure {
            state_saved: _,
            state_current,
            state_desired,
            state_diff,
            op_check_status,
            state_ensured,
        } = item_ensure;

        match op_check_status {
            #[cfg(not(feature = "output_progress"))]
            OpCheckStatus::ExecRequired => {
                let state_ensured_dry = self
                    .ensure_op_exec_dry(op_ctx, resources, state_current, state_desired, state_diff)
                    .await?;

                *state_ensured = Some(state_ensured_dry);
            }
            #[cfg(feature = "output_progress")]
            OpCheckStatus::ExecRequired { progress_limit: _ } => {
                let state_ensured_dry = self
                    .ensure_op_exec_dry(op_ctx, resources, state_current, state_desired, state_diff)
                    .await?;

                *state_ensured = Some(state_ensured_dry);
            }
            OpCheckStatus::ExecNotRequired => {}
        }

        Ok(())
    }

    async fn ensure_exec(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<SetUp>,
        item_ensure_boxed: &mut ItemEnsureBoxed,
    ) -> Result<(), E> {
        let Some(item_ensure) =
            item_ensure_boxed.as_data_type_mut().downcast_mut::<ItemEnsure<State, StateDiff>>() else {
                panic!("Failed to downcast `ItemEnsureBoxed` to `{concrete_type}`.\n\
                    This is a bug in the Peace framework.",
                    concrete_type = std::any::type_name::<ItemEnsure<State, StateDiff>>())
            };

        let ItemEnsure {
            state_saved: _,
            state_current,
            state_desired,
            state_diff,
            op_check_status,
            state_ensured,
        } = item_ensure;

        match op_check_status {
            #[cfg(not(feature = "output_progress"))]
            OpCheckStatus::ExecRequired => {
                let state_ensured_next = self
                    .ensure_op_exec(op_ctx, resources, state_current, state_desired, state_diff)
                    .await?;

                *state_ensured = Some(state_ensured_next);
            }
            #[cfg(feature = "output_progress")]
            OpCheckStatus::ExecRequired { progress_limit: _ } => {
                let state_ensured_next = self
                    .ensure_op_exec(op_ctx, resources, state_current, state_desired, state_diff)
                    .await?;

                *state_ensured = Some(state_ensured_next);
            }
            OpCheckStatus::ExecNotRequired => {}
        }

        Ok(())
    }

    async fn clean_op_check(
        &self,
        resources: &Resources<SetUp>,
        states_current: &StatesCurrent,
    ) -> Result<OpCheckStatus, E> {
        let op_check_status = {
            let data = <<CleanOpSpec as peace_cfg::CleanOpSpec>::Data<'_> as Data>::borrow(
                self.id(),
                resources,
            );
            let item_spec_id = <IS as ItemSpec>::id(self);
            let state = states_current.get::<State, _>(item_spec_id);

            if let Some(state) = state {
                <CleanOpSpec as peace_cfg::CleanOpSpec>::check(data, state).await?
            } else {
                // When we reach here, one of the following is true:
                //
                // * The current state cannot be retrieved, due to a predecessor's state not
                //   existing.
                // * A bug exists, e.g. the state is stored against the wrong type parameter.

                OpCheckStatus::ExecNotRequired
            }
        };

        Ok(op_check_status)
    }

    async fn clean_op_exec_dry(
        &self,
        resources: &Resources<SetUp>,
        states_current: &StatesCurrent,
    ) -> Result<(), E> {
        let data = <<CleanOpSpec as peace_cfg::CleanOpSpec>::Data<'_> as Data>::borrow(
            self.id(),
            resources,
        );
        let item_spec_id = <IS as ItemSpec>::id(self);
        let state = states_current.get::<State, _>(item_spec_id);

        if let Some(state) = state {
            <CleanOpSpec as peace_cfg::CleanOpSpec>::exec_dry(data, state).await?;
        } else {
            // When we reach here, one of the following is true:
            //
            // * The current state cannot be retrieved, due to a predecessor's
            //   state not existing.
            // * A bug exists, e.g. the state is stored against the wrong type
            //   parameter.
        }

        Ok(())
    }

    async fn clean_op_exec(
        &self,
        resources: &Resources<SetUp>,
        states_current: &StatesCurrent,
    ) -> Result<(), E> {
        let data = <<CleanOpSpec as peace_cfg::CleanOpSpec>::Data<'_> as Data>::borrow(
            self.id(),
            resources,
        );
        let item_spec_id = <IS as ItemSpec>::id(self);
        let state = states_current.get::<State, _>(item_spec_id);

        if let Some(state) = state {
            <CleanOpSpec as peace_cfg::CleanOpSpec>::exec(data, state).await?;
        } else {
            // When we reach here, one of the following is true:
            //
            // * The current state cannot be retrieved, due to a predecessor's
            //   state not existing.
            // * A bug exists, e.g. the state is stored against the wrong type
            //   parameter.
        }

        Ok(())
    }
}
