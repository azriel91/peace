use std::fmt::Debug;

use diff::Diff;
use fn_graph::{DataAccessDyn, TypeIds};
use peace_cfg::{FullSpec, OpSpec, OpSpecDry};
use serde::{de::DeserializeOwned, Serialize};

use crate::FullSpecWrapper;

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
pub struct FullSpecBoxed<'op>(Box<dyn FullSpecRt<'op> + 'op>);

impl<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec> From<FS>
    for FullSpecBoxed<'op>
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
        + Sync
        + 'op,
    ResIds: Debug + Serialize + DeserializeOwned + Send + Sync + 'op,
    State: Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'op,
    StatusOpSpec: Debug + OpSpec<'op, State = (), Output = State> + Send + Sync + 'op,
    EnsureOpSpec: Debug + OpSpecDry<'op, State = State, Output = ResIds> + Send + Sync + 'op,
    CleanOpSpec: Debug + OpSpecDry<'op, State = State, Output = ResIds> + Send + Sync + 'op,
{
    fn from(full_spec: FS) -> Self {
        Self(Box::new(FullSpecWrapper::from(full_spec)))
    }
}

impl<'op> DataAccessDyn for FullSpecBoxed<'op> {
    fn borrows(&self) -> TypeIds {
        DataAccessDyn::borrows(self.0.as_ref())
    }

    fn borrow_muts(&self) -> TypeIds {
        DataAccessDyn::borrow_muts(self.0.as_ref())
    }
}
