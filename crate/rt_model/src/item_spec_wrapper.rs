use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use fn_graph::{DataAccess, DataAccessDyn, TypeIds};
use peace_cfg::{
    async_trait, state::Placeholder, ItemSpec, ItemSpecId, OpCheckStatus, State, TryFnSpec,
};
use peace_data::Data;
use peace_resources::{
    resources::ts::{
        Empty, SetUp, WithStatesCurrent, WithStatesCurrentAndDesired, WithStatesCurrentDiffs,
        WithStatesSavedAndDesired,
    },
    states::{StateDiffs, StatesCurrent, StatesDesired, StatesSaved},
    type_reg::untagged::BoxDtDisplay,
    Resources,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{ItemSpecRt, StatesTypeRegs};

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
    StateCurrentFnSpec:
        Debug + TryFnSpec<Output = Option<State<StateLogical, StatePhysical>>> + Send + Sync,
    StateDesiredFnSpec: Debug + TryFnSpec<Output = Option<StateLogical>> + Send + Sync,
    StateDiffFnSpec: Debug
        + peace_cfg::StateDiffFnSpec<
            StatePhysical = StatePhysical,
            StateLogical = StateLogical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<StateLogical = StateLogical, StatePhysical = StatePhysical>
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
    StateCurrentFnSpec:
        Debug + TryFnSpec<Output = Option<State<StateLogical, StatePhysical>>> + Send + Sync,
    StateDesiredFnSpec: Debug + TryFnSpec<Output = Option<StateLogical>> + Send + Sync,
    StateDiffFnSpec: Debug
        + peace_cfg::StateDiffFnSpec<
            StatePhysical = StatePhysical,
            StateLogical = StateLogical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<StateLogical = StateLogical, StatePhysical = StatePhysical>
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
    StateCurrentFnSpec:
        Debug + TryFnSpec<Output = Option<State<StateLogical, StatePhysical>>> + Send + Sync,
    StateDesiredFnSpec: Debug + TryFnSpec<Output = Option<StateLogical>> + Send + Sync,
    StateDiffFnSpec: Debug
        + peace_cfg::StateDiffFnSpec<
            StatePhysical = StatePhysical,
            StateLogical = StateLogical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<StateLogical = StateLogical, StatePhysical = StatePhysical>
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
        + TryFnSpec<
            Error = <IS as ItemSpec>::Error,
            Output = Option<State<StateLogical, StatePhysical>>,
        > + Send
        + Sync,
    StateDesiredFnSpec: Debug
        + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = Option<StateLogical>>
        + Send
        + Sync,
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
        let state: Option<State<StateLogical, StatePhysical>> = {
            let data =
                <<StateCurrentFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(resources);
            <StateCurrentFnSpec as TryFnSpec>::try_exec(data).await?
        };

        Ok(state.map(BoxDtDisplay::new))
    }

    async fn state_ensured_try_exec(
        &self,
        resources: &Resources<WithStatesCurrentDiffs>,
    ) -> Result<BoxDtDisplay, E> {
        let state: State<StateLogical, StatePhysical> = {
            let data =
                <<StateCurrentFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(resources);
            <StateCurrentFnSpec as TryFnSpec>::try_exec(data)
                .await?
                .ok_or_else(|| {
                    let item_spec_id = self.id();
                    crate::Error::StateCurrentDiscoverNone { item_spec_id }
                })?
        };

        Ok(BoxDtDisplay::new(state))
    }

    async fn state_cleaned_try_exec(
        &self,
        resources: &Resources<WithStatesCurrent>,
    ) -> Result<Option<BoxDtDisplay>, E> {
        let state: Option<State<StateLogical, StatePhysical>> = {
            let data =
                <<StateCurrentFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(resources);
            <StateCurrentFnSpec as TryFnSpec>::try_exec(data).await?
        };

        Ok(state.map(BoxDtDisplay::new))
    }

    async fn state_desired_try_exec(
        &self,
        resources: &Resources<SetUp>,
    ) -> Result<Option<BoxDtDisplay>, E> {
        let state_desired = {
            let data =
                <<StateDesiredFnSpec as peace_cfg::TryFnSpec>::Data<'_> as Data>::borrow(resources);
            let state_desired_logical =
                <StateDesiredFnSpec as peace_cfg::TryFnSpec>::try_exec(data).await?;

            state_desired_logical.map(|state_desired_logical| {
                State::new(state_desired_logical, Placeholder::calculated())
            })
        };

        Ok(state_desired.map(BoxDtDisplay::new))
    }

    async fn state_diff_exec_with_states_saved(
        &self,
        resources: &Resources<WithStatesSavedAndDesired>,
    ) -> Result<BoxDtDisplay, E> {
        let state_diff: StateDiff = {
            let data = <<StateDiffFnSpec as peace_cfg::StateDiffFnSpec>::Data<'_> as Data>::borrow(
                resources,
            );
            let item_spec_id = <IS as ItemSpec>::id(self);
            let states_saved = resources.borrow::<StatesSaved>();
            let state = states_saved.get::<State<StateLogical, StatePhysical>, _>(&item_spec_id);
            let states_desired = resources.borrow::<StatesDesired>();
            let state_desired =
                states_desired.get::<State<StateLogical, Placeholder>, _>(&item_spec_id);

            if let (Some(state), Some(state_desired)) = (state, state_desired) {
                <StateDiffFnSpec as peace_cfg::StateDiffFnSpec>::exec(
                    data,
                    state,
                    &state_desired.logical,
                )
                .await
                .map_err(Into::<E>::into)?
            } else {
                panic!(
                    "`ItemSpecWrapper::state_diff_exec_with_states_saved` must only be called with \
                    `StatesSaved` and `StatesDesired` populated using `StatesSavedReadCmd` and \
                    `StatesDesiredDiscoverCmd`."
                );
            }
        };

        Ok(BoxDtDisplay::new(state_diff))
    }

    async fn state_diff_exec_with_states_current(
        &self,
        resources: &Resources<WithStatesCurrentAndDesired>,
    ) -> Result<BoxDtDisplay, E> {
        let state_diff: StateDiff = {
            let data = <<StateDiffFnSpec as peace_cfg::StateDiffFnSpec>::Data<'_> as Data>::borrow(
                resources,
            );
            let item_spec_id = <IS as ItemSpec>::id(self);
            let states_current = resources.borrow::<StatesCurrent>();
            let state = states_current.get::<State<StateLogical, StatePhysical>, _>(&item_spec_id);
            let states_desired = resources.borrow::<StatesDesired>();
            let state_desired =
                states_desired.get::<State<StateLogical, Placeholder>, _>(&item_spec_id);

            if let (Some(state), Some(state_desired)) = (state, state_desired) {
                <StateDiffFnSpec as peace_cfg::StateDiffFnSpec>::exec(
                    data,
                    state,
                    &state_desired.logical,
                )
                .await
                .map_err(Into::<E>::into)?
            } else {
                panic!(
                    "`ItemSpecWrapper::state_diff_exec_with_states_current` must only be called with \
                    `StatesCurrent` and `StatesDesired` populated using `StatesCurrentDiscoverCmd` and \
                    `StatesDesiredDiscoverCmd`."
                );
            }
        };

        Ok(BoxDtDisplay::new(state_diff))
    }

    async fn ensure_op_check(
        &self,
        resources: &Resources<WithStatesCurrentDiffs>,
    ) -> Result<OpCheckStatus, E> {
        let op_check_status = {
            let data =
                <<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_> as Data>::borrow(resources);
            let item_spec_id = <IS as ItemSpec>::id(self);
            let states = resources.borrow::<StatesCurrent>();
            let state = states.get::<State<StateLogical, StatePhysical>, _>(&item_spec_id);
            let states_desired = resources.borrow::<StatesDesired>();
            let state_desired =
                states_desired.get::<State<StateLogical, Placeholder>, _>(&item_spec_id);
            let state_diffs = resources.borrow::<StateDiffs>();
            let state_diff = state_diffs.get::<StateDiff, _>(&item_spec_id);

            if let (Some(state), Some(state_desired), Some(state_diff)) =
                (state, state_desired, state_diff)
            {
                <EnsureOpSpec as peace_cfg::EnsureOpSpec>::check(
                    data,
                    state,
                    &state_desired.logical,
                    state_diff,
                )
                .await?
            } else {
                panic!(
                    "`ItemSpecWrapper::ensure_op_check` must only be called with `StatesCurrent`, `StatesDesired`, and \
                    `StateDiffs` populated using `DiffCmd`."
                );
            }
        };

        Ok(op_check_status)
    }

    async fn ensure_op_exec_dry(
        &self,
        resources: &Resources<WithStatesCurrentDiffs>,
    ) -> Result<(), E> {
        let data = <<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_> as Data>::borrow(resources);
        let item_spec_id = <IS as ItemSpec>::id(self);
        let states = resources.borrow::<StatesCurrent>();
        let state = states.get::<State<StateLogical, StatePhysical>, _>(&item_spec_id);
        let states_desired = resources.borrow::<StatesDesired>();
        let state_desired =
            states_desired.get::<State<StateLogical, Placeholder>, _>(&item_spec_id);
        let state_diffs = resources.borrow::<StateDiffs>();
        let state_diff = state_diffs.get::<StateDiff, _>(&item_spec_id);

        if let (Some(state), Some(state_desired), Some(state_diff)) =
            (state, state_desired, state_diff)
        {
            <EnsureOpSpec as peace_cfg::EnsureOpSpec>::exec_dry(
                data,
                state,
                &state_desired.logical,
                state_diff,
            )
            .await?;
        } else {
            panic!(
                "`ItemSpecWrapper::ensure_op_exec_dry` must only be called with `StatesCurrent`, `StatesDesired`, and \
                `StateDiffs` populated using `DiffCmd`."
            );
        }

        Ok(())
    }

    async fn ensure_op_exec(&self, resources: &Resources<WithStatesCurrentDiffs>) -> Result<(), E> {
        let data = <<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_> as Data>::borrow(resources);
        let item_spec_id = <IS as ItemSpec>::id(self);
        let states = resources.borrow::<StatesCurrent>();
        let state = states.get::<State<StateLogical, StatePhysical>, _>(&item_spec_id);
        let states_desired = resources.borrow::<StatesDesired>();
        let state_desired =
            states_desired.get::<State<StateLogical, Placeholder>, _>(&item_spec_id);
        let state_diffs = resources.borrow::<StateDiffs>();
        let state_diff = state_diffs.get::<StateDiff, _>(&item_spec_id);

        if let (Some(state), Some(state_desired), Some(state_diff)) =
            (state, state_desired, state_diff)
        {
            <EnsureOpSpec as peace_cfg::EnsureOpSpec>::exec(
                data,
                state,
                &state_desired.logical,
                state_diff,
            )
            .await?;
        } else {
            panic!(
                "`ItemSpecWrapper::ensure_op_exec` must only be called with `StatesCurrent`, `StatesDesired`, and \
                `StateDiffs` populated using `DiffCmd`."
            );
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
