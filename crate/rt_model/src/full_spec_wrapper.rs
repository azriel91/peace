use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use fn_graph::{DataAccess, DataAccessDyn, TypeIds};
use peace_cfg::{async_trait, FnSpec, FullSpec, State};
use peace_data::{Data, Resources};
use peace_diff::Diff;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    full_spec_boxed::{CleanOpSpecRt, EnsureOpSpecRt, FullSpecRt, StatusFnSpecRt},
    Error, FullSpecResourceses, FullSpecRtId,
};

/// Wraps a type implementing [`FullSpec`].
pub struct FullSpecWrapper<
    'op,
    FS,
    E,
    StateLogical,
    StatePhysical,
    StatusFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
>(
    FS,
    PhantomData<&'op (
        E,
        StateLogical,
        StatePhysical,
        StatusFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    )>,
);

impl<'op, FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> Debug
    for FullSpecWrapper<
        'op,
        FS,
        E,
        StateLogical,
        StatePhysical,
        StatusFnSpec,
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

impl<'op, FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> Deref
    for FullSpecWrapper<
        'op,
        FS,
        E,
        StateLogical,
        StatePhysical,
        StatusFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
{
    type Target = FS;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'op, FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> DerefMut
    for FullSpecWrapper<
        'op,
        FS,
        E,
        StateLogical,
        StatePhysical,
        StatusFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'op, FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> From<FS>
    for FullSpecWrapper<
        'op,
        FS,
        E,
        StateLogical,
        StatePhysical,
        StatusFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    FS: Debug
        + FullSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec: Debug + FnSpec<'op, Output = State<StateLogical, StatePhysical>> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<'op, StateLogical = StateLogical, StatePhysical = StatePhysical>
        + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<'op, StateLogical = StateLogical, StatePhysical = StatePhysical>
        + Send
        + Sync,
{
    fn from(full_spec: FS) -> Self {
        Self(full_spec, PhantomData)
    }
}

impl<'op, FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> DataAccess
    for FullSpecWrapper<
        'op,
        FS,
        E,
        StateLogical,
        StatePhysical,
        StatusFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    FS: Debug
        + FullSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec: Debug + FnSpec<'op, Output = State<StateLogical, StatePhysical>> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<'op, StateLogical = StateLogical, StatePhysical = StatePhysical>
        + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<'op, StateLogical = StateLogical, StatePhysical = StatePhysical>
        + Send
        + Sync,
{
    fn borrows() -> TypeIds {
        <EnsureOpSpec::Data as DataAccess>::borrows()
    }

    fn borrow_muts() -> TypeIds {
        <EnsureOpSpec::Data as DataAccess>::borrow_muts()
    }
}

impl<'op, FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> DataAccessDyn
    for FullSpecWrapper<
        'op,
        FS,
        E,
        StateLogical,
        StatePhysical,
        StatusFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    FS: Debug
        + FullSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec: Debug + FnSpec<'op, Output = State<StateLogical, StatePhysical>> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<'op, StateLogical = StateLogical, StatePhysical = StatePhysical>
        + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<'op, StateLogical = StateLogical, StatePhysical = StatePhysical>
        + Send
        + Sync,
{
    fn borrows(&self) -> TypeIds {
        <EnsureOpSpec::Data as DataAccess>::borrows()
    }

    fn borrow_muts(&self) -> TypeIds {
        <EnsureOpSpec::Data as DataAccess>::borrow_muts()
    }
}

#[async_trait]
impl<'op, FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
    FullSpecRt<'op, Error<E>>
    for FullSpecWrapper<
        'op,
        FS,
        E,
        StateLogical,
        StatePhysical,
        StatusFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    FS: Debug
        + FullSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec:
        Debug + FnSpec<'op, Error = E, Output = State<StateLogical, StatePhysical>> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync,
{
    async fn setup(&self, resources: &'op mut Resources) -> Result<(), Error<E>> {
        <FS as FullSpec>::setup(resources)
            .await
            .map_err(Error::FullSpecSetup)
    }

    async fn status_fn_exec(&self, resources: &'op Resources) -> Result<(), Error<E>> {
        <Self as StatusFnSpecRt>::exec(self, resources).await
    }
}

#[async_trait]
impl<'op, FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
    StatusFnSpecRt<'op>
    for FullSpecWrapper<
        'op,
        FS,
        E,
        StateLogical,
        StatePhysical,
        StatusFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    FS: Debug
        + FullSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec:
        Debug + FnSpec<'op, Error = E, Output = State<StateLogical, StatePhysical>> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync,
{
    type Error = Error<E>;

    async fn exec(&self, resources: &'op Resources) -> Result<(), Self::Error> {
        let state = {
            let data = StatusFnSpec::Data::borrow(resources);
            <StatusFnSpec as FnSpec>::exec(data).await
        };

        // Store `state` so that we can use it in subsequent operations.
        let full_spec_rt_id = FullSpecRtId::new(fn_graph::FnId::new(0)); // Pass this into the `exec` function of each trait
        let full_spec_resourceses = resources.borrow::<FullSpecResourceses>();
        let mut full_spec_resources = full_spec_resourceses.borrow_mut(full_spec_rt_id);
        full_spec_resources.insert(state);

        Ok(())
    }
}

#[async_trait]
impl<'op, FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
    EnsureOpSpecRt<'op>
    for FullSpecWrapper<
        'op,
        FS,
        E,
        StateLogical,
        StatePhysical,
        StatusFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    FS: Debug
        + FullSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec:
        Debug + FnSpec<'op, Error = E, Output = State<StateLogical, StatePhysical>> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync,
{
    type Error = Error<E>;

    async fn check(&self, _resources: &Resources) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn exec(&self, _resources: &Resources) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[async_trait]
impl<'op, FS, E, StateLogical, StatePhysical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
    CleanOpSpecRt<'op>
    for FullSpecWrapper<
        'op,
        FS,
        E,
        StateLogical,
        StatePhysical,
        StatusFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    >
where
    FS: Debug
        + FullSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec:
        Debug + FnSpec<'op, Error = E, Output = State<StateLogical, StatePhysical>> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<
            'op,
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync,
{
    type Error = Error<E>;

    async fn check(&self, _resources: &Resources) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn exec(&self, _resources: &Resources) -> Result<(), Self::Error> {
        Ok(())
    }
}
