use std::{fmt::Debug, marker::PhantomData};

use diff::Diff;
use fn_graph::{DataAccess, DataAccessDyn, TypeIds};
use peace_cfg::{async_trait, FullSpec, OpSpec, OpSpecDry};
use peace_data::Resources;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    full_spec_boxed::{CleanOpSpecRt, EnsureOpSpecRt, FullSpecRt, StatusOpSpecRt},
    Error,
};

/// Wraps a type implementing [`FullSpec`].
#[derive(Debug)]
pub struct FullSpecWrapper<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec>(
    FS,
    PhantomData<&'op (ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec)>,
);

impl<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec> From<FS>
    for FullSpecWrapper<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            'op,
            ResIds = ResIds,
            State = State,
            StatusOpSpec = StatusOpSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync,
    State: Debug + Diff + Serialize + DeserializeOwned + Send + Sync,
    StatusOpSpec: Debug + OpSpec<'op, State = (), Output = State> + Send + Sync,
    EnsureOpSpec: Debug + OpSpecDry<'op, State = State, Output = ResIds> + Send + Sync,
    CleanOpSpec: Debug + OpSpecDry<'op, State = State, Output = ResIds> + Send + Sync,
{
    fn from(full_spec: FS) -> Self {
        Self(full_spec, PhantomData)
    }
}

impl<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec> DataAccessDyn
    for FullSpecWrapper<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            'op,
            ResIds = ResIds,
            State = State,
            StatusOpSpec = StatusOpSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync,
    State: Debug + Diff + Serialize + DeserializeOwned + Send + Sync,
    StatusOpSpec: Debug + OpSpec<'op, State = (), Output = State> + Send + Sync,
    EnsureOpSpec: Debug + OpSpecDry<'op, State = State, Output = ResIds> + Send + Sync,
    CleanOpSpec: Debug + OpSpecDry<'op, State = State, Output = ResIds> + Send + Sync,
{
    fn borrows(&self) -> TypeIds {
        <EnsureOpSpec::Data as DataAccess>::borrows()
    }

    fn borrow_muts(&self) -> TypeIds {
        <EnsureOpSpec::Data as DataAccess>::borrow_muts()
    }
}

impl<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec> FullSpecRt<'op>
    for FullSpecWrapper<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            'op,
            ResIds = ResIds,
            State = State,
            StatusOpSpec = StatusOpSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync,
    State: Debug + Diff + Serialize + DeserializeOwned + Send + Sync,
    StatusOpSpec: Debug + OpSpec<'op, State = (), Output = State> + Send + Sync,
    EnsureOpSpec: Debug + OpSpecDry<'op, State = State, Output = ResIds> + Send + Sync,
    CleanOpSpec: Debug + OpSpecDry<'op, State = State, Output = ResIds> + Send + Sync,
{
}

#[async_trait]
impl<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec> StatusOpSpecRt<'op>
    for FullSpecWrapper<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            'op,
            ResIds = ResIds,
            State = State,
            StatusOpSpec = StatusOpSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync,
    State: Debug + Diff + Serialize + DeserializeOwned + Send + Sync,
    StatusOpSpec: Debug + OpSpec<'op, State = (), Output = State> + Send + Sync,
    EnsureOpSpec: Debug + OpSpecDry<'op, State = State, Output = ResIds> + Send + Sync,
    CleanOpSpec: Debug + OpSpecDry<'op, State = State, Output = ResIds> + Send + Sync,
{
    async fn setup(&self, _resources: &Resources) -> Result<(), Error> {
        Ok(())
    }

    async fn check(&self, _resources: &Resources) -> Result<(), Error> {
        Ok(())
    }

    async fn exec(&self, _resources: &Resources) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
impl<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec> EnsureOpSpecRt<'op>
    for FullSpecWrapper<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            'op,
            ResIds = ResIds,
            State = State,
            StatusOpSpec = StatusOpSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync,
    State: Debug + Diff + Serialize + DeserializeOwned + Send + Sync,
    StatusOpSpec: Debug + OpSpec<'op, State = (), Output = State> + Send + Sync,
    EnsureOpSpec: Debug + OpSpecDry<'op, State = State, Output = ResIds> + Send + Sync,
    CleanOpSpec: Debug + OpSpecDry<'op, State = State, Output = ResIds> + Send + Sync,
{
    async fn setup(&self, _resources: &Resources) -> Result<(), Error> {
        Ok(())
    }

    async fn check(&self, _resources: &Resources) -> Result<(), Error> {
        Ok(())
    }

    async fn exec(&self, _resources: &Resources) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
impl<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec> CleanOpSpecRt<'op>
    for FullSpecWrapper<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec>
where
    FS: Debug
        + FullSpec<
            'op,
            ResIds = ResIds,
            State = State,
            StatusOpSpec = StatusOpSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync,
    State: Debug + Diff + Serialize + DeserializeOwned + Send + Sync,
    StatusOpSpec: Debug + OpSpec<'op, State = (), Output = State> + Send + Sync,
    EnsureOpSpec: Debug + OpSpecDry<'op, State = State, Output = ResIds> + Send + Sync,
    CleanOpSpec: Debug + OpSpecDry<'op, State = State, Output = ResIds> + Send + Sync,
{
    async fn setup(&self, _resources: &Resources) -> Result<(), Error> {
        Ok(())
    }

    async fn check(&self, _resources: &Resources) -> Result<(), Error> {
        Ok(())
    }

    async fn exec(&self, _resources: &Resources) -> Result<(), Error> {
        Ok(())
    }
}
