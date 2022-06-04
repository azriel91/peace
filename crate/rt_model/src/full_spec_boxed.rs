use std::fmt::Debug;

use diff::Diff;
use fn_graph::{DataAccessDyn, TypeIds};
use peace_cfg::{FullSpec, OpSpec, OpSpecDry};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Error, FullSpecWrapper};

pub use self::{
    clean_op_spec_rt::CleanOpSpecRt, ensure_op_spec_rt::EnsureOpSpecRt, full_spec_rt::FullSpecRt,
    status_op_spec_rt::StatusOpSpecRt,
};

mod clean_op_spec_rt;
mod ensure_op_spec_rt;
mod full_spec_rt;
mod status_op_spec_rt;

/// Defines all of the data and logic to manage a user defined item.
///
/// # Type Parameters
///
/// * `FS`: The [`FullSpec`]
#[derive(Debug)]
pub struct FullSpecBoxed<'op, E>(Box<dyn FullSpecRt<'op, Error<E>> + 'op>)
where
    E: std::error::Error;

impl<'op, FS, E, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec> From<FS>
    for FullSpecBoxed<'op, E>
where
    FS: Debug
        + FullSpec<
            'op,
            State = State,
            Error = E,
            ResIds = ResIds,
            StatusOpSpec = StatusOpSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync
        + 'op,
    E: Debug + Send + Sync + std::error::Error + 'op,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync + 'op,
    State: Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'op,
    StatusOpSpec: Debug + OpSpec<'op, State = (), Error = E, Output = State> + Send + Sync + 'op,
    EnsureOpSpec:
        Debug + OpSpecDry<'op, State = State, Error = E, Output = ResIds> + Send + Sync + 'op,
    CleanOpSpec:
        Debug + OpSpecDry<'op, State = State, Error = E, Output = ResIds> + Send + Sync + 'op,
{
    fn from(full_spec: FS) -> Self {
        Self(Box::new(FullSpecWrapper::from(full_spec)))
    }
}

impl<'op, E> DataAccessDyn for FullSpecBoxed<'op, E>
where
    E: std::error::Error,
{
    fn borrows(&self) -> TypeIds {
        DataAccessDyn::borrows(self.0.as_ref())
    }

    fn borrow_muts(&self) -> TypeIds {
        DataAccessDyn::borrow_muts(self.0.as_ref())
    }
}