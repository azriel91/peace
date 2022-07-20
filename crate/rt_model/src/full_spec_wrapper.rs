use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use fn_graph::{DataAccess, DataAccessDyn, TypeIds};
use peace_cfg::{async_trait, nougat::Gat, FnSpec, FullSpec, State};
use peace_data::Data;
use peace_diff::Diff;
use peace_resources::{
    resources_type_state::{Empty, SetUp, WithStates},
    FullSpecStatesRw, Resources,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::full_spec_boxed::{CleanOpSpecRt, EnsureOpSpecRt, FullSpecRt, StatusFnSpecRt};

/// Wraps a type implementing [`FullSpec`].
pub struct FullSpecWrapper<
    FS,
    E,
    StateLogical,
    StatePhysical,
    StatusFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
>(
    FS,
    PhantomData<(
        E,
        StateLogical,
        StatePhysical,
        StatusFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    )>,
);

impl<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> Debug
    for FullSpecWrapper<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> Deref
    for FullSpecWrapper<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
{
    type Target = FS;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> DerefMut
    for FullSpecWrapper<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> From<FS>
    for FullSpecWrapper<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Clone + Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec: Debug + FnSpec<Output = State<StateLogical, StatePhysical>> + Send + Sync,
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

impl<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> DataAccess
    for FullSpecWrapper<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Clone + Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec: Debug + FnSpec<Output = State<StateLogical, StatePhysical>> + Send + Sync,
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

impl<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> DataAccessDyn
    for FullSpecWrapper<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Clone + Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec: Debug + FnSpec<Output = State<StateLogical, StatePhysical>> + Send + Sync,
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
impl<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> FullSpecRt<E>
    for FullSpecWrapper<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Clone + Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec:
        Debug + FnSpec<Error = E, Output = State<StateLogical, StatePhysical>> + Send + Sync,
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
    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), E> {
        <FS as FullSpec>::setup(self, resources).await
    }

    async fn status_fn_exec(&self, resources: &Resources<SetUp>) -> Result<(), E> {
        <Self as StatusFnSpecRt>::exec(self, resources).await
    }

    async fn ensure_op_desired(&self, resources: &Resources<SetUp>) -> Result<(), E> {
        <Self as EnsureOpSpecRt>::desired(self, resources).await
    }
}

#[async_trait]
impl<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> StatusFnSpecRt
    for FullSpecWrapper<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Clone + Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec:
        Debug + FnSpec<Error = E, Output = State<StateLogical, StatePhysical>> + Send + Sync,
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
    type Error = E;

    async fn exec(&self, resources: &Resources<SetUp>) -> Result<(), Self::Error> {
        let state: State<StateLogical, StatePhysical> = {
            let data =
                <Gat!(<StatusFnSpec as peace_cfg::FnSpec>::Data<'_>) as Data>::borrow(resources);
            <StatusFnSpec as FnSpec>::exec(data).await?
        };

        // Store `state` so that we can use it in subsequent operations.
        let full_spec_states_rw = resources.borrow::<FullSpecStatesRw>();
        let mut full_spec_states = full_spec_states_rw.write().await;
        full_spec_states.insert(self.id(), state);

        Ok(())
    }
}

#[async_trait]
impl<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> EnsureOpSpecRt
    for FullSpecWrapper<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Clone + Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec:
        Debug + FnSpec<Error = E, Output = State<StateLogical, StatePhysical>> + Send + Sync,
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
    type Error = E;

    async fn desired(&self, resources: &Resources<SetUp>) -> Result<(), Self::Error> {
        let _state_logical = {
            let data = <Gat!(<EnsureOpSpec as peace_cfg::EnsureOpSpec>::Data<'_>) as Data>::borrow(
                resources,
            );
            <EnsureOpSpec as peace_cfg::EnsureOpSpec>::desired(data).await?
        };

        Ok(())
    }

    async fn check(&self, _resources: &Resources<SetUp>) -> Result<(), Self::Error> {
        todo!()
    }

    async fn exec_dry(&self, _resources: &Resources<WithStates>) -> Result<(), Self::Error> {
        todo!()
    }

    async fn exec(&self, _resources: &Resources<WithStates>) -> Result<(), Self::Error> {
        todo!()
    }
}

#[async_trait]
impl<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> CleanOpSpecRt
    for FullSpecWrapper<FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Clone + Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec:
        Debug + FnSpec<Error = E, Output = State<StateLogical, StatePhysical>> + Send + Sync,
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
    type Error = E;

    async fn check(&self, _resources: &Resources<SetUp>) -> Result<(), Self::Error> {
        todo!()
    }

    async fn exec_dry(&self, _resources: &Resources<WithStates>) -> Result<(), Self::Error> {
        todo!()
    }

    async fn exec(&self, _resources: &Resources<WithStates>) -> Result<(), Self::Error> {
        todo!()
    }
}
