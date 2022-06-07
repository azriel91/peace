use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use diff::Diff;
use fn_graph::{DataAccess, DataAccessDyn, TypeIds};
use peace_cfg::{async_trait, FnSpec, FullSpec};
use peace_data::Resources;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    full_spec_boxed::{CleanOpSpecRt, EnsureOpSpecRt, FullSpecRt, StatusFnSpecRt},
    Error,
};

/// Wraps a type implementing [`FullSpec`].
pub struct FullSpecWrapper<
    'op,
    FS,
    E,
    ResIds,
    StateLogical,
    StatusFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
>(
    FS,
    PhantomData<&'op (
        E,
        ResIds,
        StateLogical,
        StatusFnSpec,
        EnsureOpSpec,
        CleanOpSpec,
    )>,
);

impl<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> Debug
    for FullSpecWrapper<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> Deref
    for FullSpecWrapper<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
{
    type Target = FS;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> DerefMut
    for FullSpecWrapper<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> From<FS>
    for FullSpecWrapper<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            'op,
            StateLogical = StateLogical,
            Error = E,
            ResIds = ResIds,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync,
    StateLogical: Debug + Diff + Serialize + DeserializeOwned + Send + Sync,
    StatusFnSpec: Debug + FnSpec<'op, Output = StateLogical> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<'op, StateLogical = StateLogical, ResIds = ResIds>
        + Send
        + Sync,
    CleanOpSpec: Debug + peace_cfg::CleanOpSpec<'op, ResIds = ResIds> + Send + Sync,
{
    fn from(full_spec: FS) -> Self {
        Self(full_spec, PhantomData)
    }
}

impl<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> DataAccess
    for FullSpecWrapper<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            'op,
            StateLogical = StateLogical,
            Error = E,
            ResIds = ResIds,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync,
    StateLogical: Debug + Diff + Serialize + DeserializeOwned + Send + Sync,
    StatusFnSpec: Debug + FnSpec<'op, Output = StateLogical> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<'op, StateLogical = StateLogical, ResIds = ResIds>
        + Send
        + Sync,
    CleanOpSpec: Debug + peace_cfg::CleanOpSpec<'op, ResIds = ResIds> + Send + Sync,
{
    fn borrows() -> TypeIds {
        <EnsureOpSpec::Data as DataAccess>::borrows()
    }

    fn borrow_muts() -> TypeIds {
        <EnsureOpSpec::Data as DataAccess>::borrow_muts()
    }
}

impl<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> DataAccessDyn
    for FullSpecWrapper<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            'op,
            StateLogical = StateLogical,
            Error = E,
            ResIds = ResIds,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync,
    StateLogical: Debug + Diff + Serialize + DeserializeOwned + Send + Sync,
    StatusFnSpec: Debug + FnSpec<'op, Output = StateLogical> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<'op, StateLogical = StateLogical, ResIds = ResIds>
        + Send
        + Sync,
    CleanOpSpec: Debug + peace_cfg::CleanOpSpec<'op, ResIds = ResIds> + Send + Sync,
{
    fn borrows(&self) -> TypeIds {
        <EnsureOpSpec::Data as DataAccess>::borrows()
    }

    fn borrow_muts(&self) -> TypeIds {
        <EnsureOpSpec::Data as DataAccess>::borrow_muts()
    }
}

#[async_trait]
impl<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
    FullSpecRt<'op, Error<E>>
    for FullSpecWrapper<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            'op,
            StateLogical = StateLogical,
            Error = E,
            ResIds = ResIds,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync,
    StateLogical: Debug + Diff + Serialize + DeserializeOwned + Send + Sync,
    StatusFnSpec: Debug + FnSpec<'op, Error = E, Output = StateLogical> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<'op, StateLogical = StateLogical, Error = E, ResIds = ResIds>
        + Send
        + Sync,
    CleanOpSpec: Debug + peace_cfg::CleanOpSpec<'op, Error = E, ResIds = ResIds> + Send + Sync,
{
    async fn setup(&self, resources: &mut Resources) -> Result<(), Error<E>> {
        <FS as FullSpec>::setup(resources)
            .await
            .map_err(Error::FullSpecSetup)
    }
}

#[async_trait]
impl<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> StatusFnSpecRt<'op>
    for FullSpecWrapper<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            'op,
            StateLogical = StateLogical,
            Error = E,
            ResIds = ResIds,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync,
    StateLogical: Debug + Diff + Serialize + DeserializeOwned + Send + Sync,
    StatusFnSpec: Debug + FnSpec<'op, Error = E, Output = StateLogical> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<'op, StateLogical = StateLogical, Error = E, ResIds = ResIds>
        + Send
        + Sync,
    CleanOpSpec: Debug + peace_cfg::CleanOpSpec<'op, Error = E, ResIds = ResIds> + Send + Sync,
{
    type Error = Error<E>;

    async fn exec(&self, _resources: &Resources) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[async_trait]
impl<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> EnsureOpSpecRt<'op>
    for FullSpecWrapper<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            'op,
            StateLogical = StateLogical,
            Error = E,
            ResIds = ResIds,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync,
    StateLogical: Debug + Diff + Serialize + DeserializeOwned + Send + Sync,
    StatusFnSpec: Debug + FnSpec<'op, Error = E, Output = StateLogical> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<'op, StateLogical = StateLogical, Error = E, ResIds = ResIds>
        + Send
        + Sync,
    CleanOpSpec: Debug + peace_cfg::CleanOpSpec<'op, Error = E, ResIds = ResIds> + Send + Sync,
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
impl<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec> CleanOpSpecRt<'op>
    for FullSpecWrapper<'op, FS, E, ResIds, StateLogical, StatusFnSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            'op,
            StateLogical = StateLogical,
            Error = E,
            ResIds = ResIds,
            StatusFnSpec = StatusFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    E: Debug + Send + Sync + std::error::Error,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync,
    StateLogical: Debug + Diff + Serialize + DeserializeOwned + Send + Sync,
    StatusFnSpec: Debug + FnSpec<'op, Error = E, Output = StateLogical> + Send + Sync,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<'op, StateLogical = StateLogical, Error = E, ResIds = ResIds>
        + Send
        + Sync,
    CleanOpSpec: Debug + peace_cfg::CleanOpSpec<'op, Error = E, ResIds = ResIds> + Send + Sync,
{
    type Error = Error<E>;

    async fn check(&self, _resources: &Resources) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn exec(&self, _resources: &Resources) -> Result<(), Self::Error> {
        Ok(())
    }
}
