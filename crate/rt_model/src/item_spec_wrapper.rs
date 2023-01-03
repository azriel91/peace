use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use fn_graph::{DataAccess, DataAccessDyn, TypeIds};
use peace_cfg::{
    async_trait, state::Placeholder, ItemSpec, ItemSpecId, OpCheckStatus, OpCtx, State, TryFnSpec,
};
use peace_data::Data;
use peace_resources::{
    resources::ts::{
        Empty, SetUp, WithStatesCurrent, WithStatesCurrentAndDesired, WithStatesCurrentDiffs,
        WithStatesSavedAndDesired,
    },
    states::{self, States, StatesCurrent, StatesDesired},
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
    StateLogical,
    StatePhysical,
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
        StateLogical,
        StatePhysical,
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
    StateLogical,
    StatePhysical,
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
        StateLogical,
        StatePhysical,
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
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
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
    StateLogical:
        Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical:
        Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateCurrentFnSpec: Debug
        + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State<StateLogical, StatePhysical>>
        + Send
        + Sync,
    StateDesiredFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = StateLogical> + Send + Sync,
    StateDiffFnSpec: Debug
        + peace_cfg::StateDiffFnSpec<
            Error = <IS as ItemSpec>::Error,
            StatePhysical = StatePhysical,
            StateLogical = StateLogical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            Error = <IS as ItemSpec>::Error,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<
            Error = <IS as ItemSpec>::Error,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync,
{
    async fn state_current_try_exec<ResourcesTs>(
        &self,
        resources: &Resources<ResourcesTs>,
    ) -> Result<Option<State<StateLogical, StatePhysical>>, E> {
        let state_current = {
            let data =
                <<StateCurrentFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(resources);
            <StateCurrentFnSpec as TryFnSpec>::try_exec(data).await?
        };

        Ok(state_current)
    }

    async fn state_current_exec<ResourcesTs>(
        &self,
        resources: &Resources<ResourcesTs>,
    ) -> Result<State<StateLogical, StatePhysical>, E> {
        let state_current = {
            let data =
                <<StateCurrentFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(resources);
            <StateCurrentFnSpec as TryFnSpec>::exec(data).await?
        };

        Ok(state_current)
    }

    async fn state_desired_try_exec(
        &self,
        resources: &Resources<SetUp>,
    ) -> Result<Option<State<StateLogical, Placeholder>>, E> {
        let data =
            <<StateDesiredFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(resources);
        let state_desired_logical =
            <StateDesiredFnSpec as peace_cfg::TryFnSpec>::try_exec(data).await?;

        Ok(state_desired_logical.map(|state_desired_logical| {
            State::new(state_desired_logical, Placeholder::calculated())
        }))
    }

    async fn state_desired_exec(
        &self,
        resources: &Resources<SetUp>,
    ) -> Result<State<StateLogical, Placeholder>, E> {
        let data =
            <<StateDesiredFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(resources);
        let state_desired_logical =
            <StateDesiredFnSpec as peace_cfg::TryFnSpec>::exec(data).await?;

        Ok(State::new(state_desired_logical, Placeholder::calculated()))
    }

    async fn state_diff_exec<ResourcesTs, StatesTs>(
        &self,
        resources: &Resources<ResourcesTs>,
    ) -> Result<StateDiff, E>
    where
        StatesTs: Debug + Send + Sync + 'static,
    {
        let state_diff: StateDiff = {
            let item_spec_id = <IS as ItemSpec>::id(self);
            let states_base = resources.borrow::<States<StatesTs>>();
            let state_base =
                states_base.get::<State<StateLogical, StatePhysical>, _>(&item_spec_id);
            let states_desired = resources.borrow::<StatesDesired>();
            let state_desired =
                states_desired.get::<State<StateLogical, Placeholder>, _>(&item_spec_id);

            if let (Some(state_base), Some(state_desired)) = (state_base, state_desired) {
                self.state_diff_exec_with(resources, state_base, state_desired)
                    .await?
            } else {
                panic!(
                    "`ItemSpecWrapper::state_diff_exec<{StatesTs}>` must be called after \
                    `States<{StatesTs}>` and `StatesDesired` are populated, e.g. using `StatesSavedReadCmd` and \
                    `StatesDesiredDiscoverCmd`.",
                    StatesTs = std::any::type_name::<StatesTs>()
                );
            }
        };

        Ok(state_diff)
    }

    async fn state_diff_exec_with<ResourcesTs>(
        &self,
        resources: &Resources<ResourcesTs>,
        state_base: &State<StateLogical, StatePhysical>,
        state_desired: &State<StateLogical, Placeholder>,
    ) -> Result<StateDiff, E> {
        let state_diff: StateDiff = {
            let data = <<StateDiffFnSpec as peace_cfg::StateDiffFnSpec>::Data<'_> as Data>::borrow(
                resources,
            );
            <StateDiffFnSpec as peace_cfg::StateDiffFnSpec>::exec(
                data,
                state_base,
                &state_desired.logical,
            )
            .await
            .map_err(Into::<E>::into)?
        };

        Ok(state_diff)
    }

    async fn ensure_op_check<ResourcesTs>(
        &self,
        resources: &Resources<ResourcesTs>,
        state_current: &State<StateLogical, StatePhysical>,
        state_desired: &State<StateLogical, Placeholder>,
        state_diff: &StateDiff,
    ) -> Result<OpCheckStatus, E> {
        let data = <<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_> as Data>::borrow(resources);
        <EnsureOpSpec as peace_cfg::EnsureOpSpec>::check(
            data,
            state_current,
            &state_desired.logical,
            state_diff,
        )
        .await
        .map_err(Into::<E>::into)
    }

    async fn ensure_op_exec_dry<ResourcesTs>(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<ResourcesTs>,
        state_current: &State<StateLogical, StatePhysical>,
        state_desired: &State<StateLogical, Placeholder>,
        state_diff: &StateDiff,
    ) -> Result<StatePhysical, E> {
        let data = <<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_> as Data>::borrow(resources);
        <EnsureOpSpec as peace_cfg::EnsureOpSpec>::exec_dry(
            op_ctx,
            data,
            state_current,
            &state_desired.logical,
            state_diff,
        )
        .await
        .map_err(Into::<E>::into)
    }

    async fn ensure_op_exec<ResourcesTs>(
        &self,
        op_ctx: OpCtx<'_>,
        resources: &Resources<ResourcesTs>,
        state_current: &State<StateLogical, StatePhysical>,
        state_desired: &State<StateLogical, Placeholder>,
        state_diff: &StateDiff,
    ) -> Result<StatePhysical, E> {
        let data = <<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_> as Data>::borrow(resources);
        <EnsureOpSpec as peace_cfg::EnsureOpSpec>::exec(
            op_ctx,
            data,
            state_current,
            &state_desired.logical,
            state_diff,
        )
        .await
        .map_err(Into::<E>::into)
    }
}

impl<
    IS,
    E,
    StateLogical,
    StatePhysical,
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
        StateLogical,
        StatePhysical,
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
    StateLogical,
    StatePhysical,
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
        StateLogical,
        StatePhysical,
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
    StateLogical,
    StatePhysical,
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
        StateLogical,
        StatePhysical,
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
    StateLogical,
    StatePhysical,
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
        StateLogical,
        StatePhysical,
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
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
            StateCurrentFnSpec = StateCurrentFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            StateDiffFnSpec = StateDiffFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + From<<IS as ItemSpec>::Error> + 'static,
    StateLogical:
        Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical:
        Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateCurrentFnSpec: Debug
        + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State<StateLogical, StatePhysical>>
        + Send
        + Sync,
    StateDesiredFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = StateLogical> + Send + Sync,
    StateDiffFnSpec: Debug
        + peace_cfg::StateDiffFnSpec<
            Error = <IS as ItemSpec>::Error,
            StatePhysical = StatePhysical,
            StateLogical = StateLogical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            Error = <IS as ItemSpec>::Error,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<
            Error = <IS as ItemSpec>::Error,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync,
{
    fn from(item_spec: IS) -> Self {
        Self(item_spec, PhantomData)
    }
}

impl<
    IS,
    E,
    StateLogical,
    StatePhysical,
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
        StateLogical,
        StatePhysical,
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
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
            StateCurrentFnSpec = StateCurrentFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            StateDiffFnSpec = StateDiffFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + From<<IS as ItemSpec>::Error> + 'static,
    StateLogical:
        Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical:
        Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateCurrentFnSpec: Debug
        + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State<StateLogical, StatePhysical>>
        + Send
        + Sync,
    StateDesiredFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = StateLogical> + Send + Sync,
    StateDiffFnSpec: Debug
        + peace_cfg::StateDiffFnSpec<
            Error = <IS as ItemSpec>::Error,
            StatePhysical = StatePhysical,
            StateLogical = StateLogical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            Error = <IS as ItemSpec>::Error,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<
            Error = <IS as ItemSpec>::Error,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
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
    StateLogical,
    StatePhysical,
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
        StateLogical,
        StatePhysical,
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
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
            StateCurrentFnSpec = StateCurrentFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            StateDiffFnSpec = StateDiffFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + From<<IS as ItemSpec>::Error> + 'static,
    StateLogical:
        Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical:
        Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateCurrentFnSpec: Debug
        + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State<StateLogical, StatePhysical>>
        + Send
        + Sync,
    StateDesiredFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = StateLogical> + Send + Sync,
    StateDiffFnSpec: Debug
        + peace_cfg::StateDiffFnSpec<
            Error = <IS as ItemSpec>::Error,
            StatePhysical = StatePhysical,
            StateLogical = StateLogical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            Error = <IS as ItemSpec>::Error,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<
            Error = <IS as ItemSpec>::Error,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
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
    StateLogical,
    StatePhysical,
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
        StateLogical,
        StatePhysical,
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
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
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
    StateLogical:
        Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical:
        Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateCurrentFnSpec: Debug
        + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State<StateLogical, StatePhysical>>
        + Send
        + Sync,
    StateDesiredFnSpec:
        Debug + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = StateLogical> + Send + Sync,
    StateDiffFnSpec: Debug
        + peace_cfg::StateDiffFnSpec<
            Error = <IS as ItemSpec>::Error,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            Error = <IS as ItemSpec>::Error,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<
            Error = <IS as ItemSpec>::Error,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync,
{
    fn id(&self) -> ItemSpecId {
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
            .register::<State<StateLogical, StatePhysical>>(<IS as ItemSpec>::id(self));

        states_type_regs
            .states_desired_type_reg_mut()
            .register::<State<StateLogical, Placeholder>>(<IS as ItemSpec>::id(self));
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

    async fn state_ensured_exec(
        &self,
        resources: &Resources<WithStatesCurrentDiffs>,
    ) -> Result<BoxDtDisplay, E> {
        self.state_current_exec(resources)
            .await
            .map(BoxDtDisplay::new)
            .map_err(Into::<E>::into)
    }

    async fn state_cleaned_try_exec(
        &self,
        resources: &Resources<WithStatesCurrent>,
    ) -> Result<Option<BoxDtDisplay>, E> {
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
        resources: &Resources<WithStatesSavedAndDesired>,
    ) -> Result<BoxDtDisplay, E> {
        self.state_diff_exec::<_, states::ts::Saved>(resources)
            .await
            .map(BoxDtDisplay::new)
            .map_err(Into::<E>::into)
    }

    async fn state_diff_exec_with_states_current(
        &self,
        resources: &Resources<WithStatesCurrentAndDesired>,
    ) -> Result<BoxDtDisplay, E> {
        self.state_diff_exec::<_, states::ts::Current>(resources)
            .await
            .map(BoxDtDisplay::new)
            .map_err(Into::<E>::into)
    }

    async fn ensure_prepare(
        &self,
        resources: &Resources<SetUp>,
    ) -> Result<ItemEnsureBoxed, (E, ItemEnsurePartialBoxed)> {
        let mut item_ensure_partial =
            ItemEnsurePartial::<StateLogical, StatePhysical, StateDiff>::new();

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
            item_ensure_boxed.as_data_type_mut().downcast_mut::<ItemEnsure<StateLogical, StatePhysical, StateDiff>>() else {
                panic!("Failed to downcast `ItemEnsureBoxed` to `{concrete_type}`. This is a bug in the `peace` framework.",
                    concrete_type = std::any::type_name::<ItemEnsure<StateLogical, StatePhysical, StateDiff>>())
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
            OpCheckStatus::ExecRequired { progress_limit: _ } => {
                let state_physical = self
                    .ensure_op_exec_dry(op_ctx, resources, state_current, state_desired, state_diff)
                    .await?;

                *state_ensured = Some(State::new(state_desired.logical.clone(), state_physical));
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
            item_ensure_boxed.as_data_type_mut().downcast_mut::<ItemEnsure<StateLogical, StatePhysical, StateDiff>>() else {
                panic!("Failed to downcast `ItemEnsureBoxed` to `{concrete_type}`. This is a bug in the `peace` framework.",
                    concrete_type = std::any::type_name::<ItemEnsure<StateLogical, StatePhysical, StateDiff>>())
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
            OpCheckStatus::ExecRequired { progress_limit: _ } => {
                let state_physical = self
                    .ensure_op_exec(op_ctx, resources, state_current, state_desired, state_diff)
                    .await?;

                *state_ensured = Some(State::new(state_desired.logical.clone(), state_physical));
            }
            OpCheckStatus::ExecNotRequired => {}
        }

        Ok(())
    }

    async fn clean_op_check(
        &self,
        resources: &Resources<WithStatesCurrent>,
    ) -> Result<OpCheckStatus, E> {
        let op_check_status = {
            let data =
                <<CleanOpSpec as peace_cfg::CleanOpSpec>::Data<'_> as Data>::borrow(resources);
            let item_spec_id = <IS as ItemSpec>::id(self);
            let states = resources.borrow::<StatesCurrent>();
            let state = states.get::<State<StateLogical, StatePhysical>, _>(&item_spec_id);

            if let Some(state) = state {
                <CleanOpSpec as peace_cfg::CleanOpSpec>::check(data, state).await?
            } else {
                panic!(
                    "`ItemSpecWrapper::clean_op_check` must only be called with `StatesCurrent`, `StatesDesired`, and \
                    `StateDiffs` populated using `DiffCmd`."
                );
            }
        };

        Ok(op_check_status)
    }

    async fn clean_op_exec_dry(&self, resources: &Resources<WithStatesCurrent>) -> Result<(), E> {
        let data = <<CleanOpSpec as peace_cfg::CleanOpSpec>::Data<'_> as Data>::borrow(resources);
        let item_spec_id = <IS as ItemSpec>::id(self);
        let states = resources.borrow::<StatesCurrent>();
        let state = states.get::<State<StateLogical, StatePhysical>, _>(&item_spec_id);

        if let Some(state) = state {
            <CleanOpSpec as peace_cfg::CleanOpSpec>::exec_dry(data, state).await?;
        } else {
            panic!(
                "`ItemSpecWrapper::clean_op_exec_dry` must only be called with `StatesCurrent` populated using `StatesCurrentDiscoverCmd`."
            );
        }

        Ok(())
    }

    async fn clean_op_exec(&self, resources: &Resources<WithStatesCurrent>) -> Result<(), E> {
        let data = <<CleanOpSpec as peace_cfg::CleanOpSpec>::Data<'_> as Data>::borrow(resources);
        let item_spec_id = <IS as ItemSpec>::id(self);
        let states = resources.borrow::<StatesCurrent>();
        let state = states.get::<State<StateLogical, StatePhysical>, _>(&item_spec_id);

        if let Some(state) = state {
            <CleanOpSpec as peace_cfg::CleanOpSpec>::exec(data, state).await?;
        } else {
            panic!(
                "`ItemSpecWrapper::clean_op_exec` must only be called with `StatesCurrent` populated using `StatesCurrentDiscoverCmd`."
            );
        }

        Ok(())
    }
}
