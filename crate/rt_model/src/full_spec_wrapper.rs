use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use fn_graph::{DataAccess, DataAccessDyn, TypeIds};
use peace_cfg::{async_trait, nougat::Gat, FnSpec, FullSpec, FullSpecId, State};
use peace_data::Data;
use peace_diff::Diff;
use peace_resources::{
    resources_type_state::{Empty, SetUp, WithStates},
    type_reg::untagged::DataType,
    Resources, States, StatesDesired,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::FullSpecRt;

/// Wraps a type implementing [`FullSpec`].
pub struct FullSpecWrapper<
    FS,
    E,
    StateLogical,
    StatePhysical,
    StateNowFnSpec,
    StateDesiredFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
>(
    FS,
    PhantomData<(
        E,
        StateLogical,
        StatePhysical,
        StateNowFnSpec,
        StateDesiredFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    )>,
);

impl<
    FS,
    E,
    StateLogical,
    StatePhysical,
    StateNowFnSpec,
    StateDesiredFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> Debug
    for FullSpecWrapper<
        FS,
        E,
        StateLogical,
        StatePhysical,
        StateNowFnSpec,
        StateDesiredFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    FS: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<
    FS,
    E,
    StateLogical,
    StatePhysical,
    StateNowFnSpec,
    StateDesiredFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> Deref
    for FullSpecWrapper<
        FS,
        E,
        StateLogical,
        StatePhysical,
        StateNowFnSpec,
        StateDesiredFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
{
    type Target = FS;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<
    FS,
    E,
    StateLogical,
    StatePhysical,
    StateNowFnSpec,
    StateDesiredFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> DerefMut
    for FullSpecWrapper<
        FS,
        E,
        StateLogical,
        StatePhysical,
        StateNowFnSpec,
        StateDesiredFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<
    FS,
    E,
    StateLogical,
    StatePhysical,
    StateNowFnSpec,
    StateDesiredFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> From<FS>
    for FullSpecWrapper<
        FS,
        E,
        StateLogical,
        StatePhysical,
        StateNowFnSpec,
        StateDesiredFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    FS: Debug
        + FullSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateNowFnSpec = StateNowFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Clone + Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateNowFnSpec: Debug + FnSpec<Output = State<StateLogical, StatePhysical>> + Send + Sync,
    StateDesiredFnSpec: Debug + FnSpec<Output = StateLogical> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<StateLogical = StateLogical, StatePhysical = StatePhysical>
        + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<StateLogical = StateLogical, StatePhysical = StatePhysical>
        + Send
        + Sync,
{
    fn from(full_spec: FS) -> Self {
        Self(full_spec, PhantomData)
    }
}

impl<
    FS,
    E,
    StateLogical,
    StatePhysical,
    StateNowFnSpec,
    StateDesiredFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> DataAccess
    for FullSpecWrapper<
        FS,
        E,
        StateLogical,
        StatePhysical,
        StateNowFnSpec,
        StateDesiredFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    FS: Debug
        + FullSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateNowFnSpec = StateNowFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Clone + Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateNowFnSpec: Debug + FnSpec<Output = State<StateLogical, StatePhysical>> + Send + Sync,
    StateDesiredFnSpec: Debug + FnSpec<Output = StateLogical> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<StateLogical = StateLogical, StatePhysical = StatePhysical>
        + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<StateLogical = StateLogical, StatePhysical = StatePhysical>
        + Send
        + Sync,
{
    fn borrows() -> TypeIds {
        <Gat!(<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_>) as DataAccess>::borrows()
    }

    fn borrow_muts() -> TypeIds {
        <Gat!(<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_>) as DataAccess>::borrow_muts()
    }
}

impl<
    FS,
    E,
    StateLogical,
    StatePhysical,
    StateNowFnSpec,
    StateDesiredFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> DataAccessDyn
    for FullSpecWrapper<
        FS,
        E,
        StateLogical,
        StatePhysical,
        StateNowFnSpec,
        StateDesiredFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    FS: Debug
        + FullSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateNowFnSpec = StateNowFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Clone + Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateNowFnSpec: Debug + FnSpec<Output = State<StateLogical, StatePhysical>> + Send + Sync,
    StateDesiredFnSpec: Debug + FnSpec<Output = StateLogical> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<StateLogical = StateLogical, StatePhysical = StatePhysical>
        + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<StateLogical = StateLogical, StatePhysical = StatePhysical>
        + Send
        + Sync,
{
    fn borrows(&self) -> TypeIds {
        <Gat!(<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_>) as DataAccess>::borrows()
    }

    fn borrow_muts(&self) -> TypeIds {
        <Gat!(<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_>) as DataAccess>::borrow_muts()
    }
}

#[async_trait]
impl<
    FS,
    E,
    StateLogical,
    StatePhysical,
    StateNowFnSpec,
    StateDesiredFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> FullSpecRt<E>
    for FullSpecWrapper<
        FS,
        E,
        StateLogical,
        StatePhysical,
        StateNowFnSpec,
        StateDesiredFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    FS: Debug
        + FullSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateNowFnSpec = StateNowFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Clone + Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    <StateLogical as Diff>::Repr: Debug + Send + Sync + Clone + Serialize,
    StatePhysical: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateNowFnSpec:
        Debug + FnSpec<Error = E, Output = State<StateLogical, StatePhysical>> + Send + Sync,
    StateDesiredFnSpec: Debug + FnSpec<Error = E, Output = StateLogical> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync,
{
    fn id(&self) -> FullSpecId {
        <FS as FullSpec>::id(self)
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), E> {
        <FS as FullSpec>::setup(self, resources).await
    }

    async fn state_now_fn_exec(
        &self,
        resources: &Resources<SetUp>,
    ) -> Result<Box<dyn DataType>, E> {
        let state: State<StateLogical, StatePhysical> = {
            let data =
                <Gat!(<StateNowFnSpec as peace_cfg::FnSpec>::Data<'_>) as Data>::borrow(resources);
            <StateNowFnSpec as FnSpec>::exec(data).await?
        };

        Ok(Box::new(state))
    }

    async fn state_desired_fn_exec(
        &self,
        resources: &Resources<SetUp>,
    ) -> Result<Box<dyn DataType>, E> {
        let state_desired = {
            let data = <Gat!(<StateDesiredFnSpec as peace_cfg::FnSpec>::Data<'_>) as Data>::borrow(
                resources,
            );
            <StateDesiredFnSpec as peace_cfg::FnSpec>::exec(data).await?
        };

        Ok(Box::new(state_desired))
    }

    fn diff(&self, states: &States, states_desired: &StatesDesired) -> Box<dyn DataType> {
        let full_spec_id = <FS as FullSpec>::id(self);
        let state = states.get::<State<StateLogical, StatePhysical>, _>(&full_spec_id);
        let state_desired = states_desired.get::<StateLogical, _>(&full_spec_id);

        if let (Some(state), Some(state_desired)) = (state, state_desired) {
            Box::new(state.logical.diff(state_desired))
        } else {
            panic!(
                "`FullSpecWrapper::diff` must only be called with `States` and `StatesDesired` \
                populated using `StateNowCmd` and `StateDesiredCmd`."
            );
        }
    }

    async fn ensure_op_check(&self, _resources: &Resources<WithStates>) -> Result<(), E> {
        todo!()
    }

    async fn ensure_op_exec_dry(&self, _resources: &Resources<WithStates>) -> Result<(), E> {
        todo!()
    }

    async fn ensure_op_exec(&self, _resources: &Resources<WithStates>) -> Result<(), E> {
        todo!()
    }
}
