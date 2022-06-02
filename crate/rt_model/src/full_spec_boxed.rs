use diff::Diff;
use fn_graph::{DataAccessDyn, TypeIds};
use peace_cfg::{FullSpec, OpSpec, OpSpecDry};
use serde::{de::DeserializeOwned, Serialize};

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
pub struct FullSpecBoxed<'op>(Box<dyn FullSpecRt<'op>>);

impl<'op, FS, ResIds, State, StatusOpSpec, EnsureOpSpec, CleanOpSpec> From<FS>
    for FullSpecBoxed<'op>
where
    FS: FullSpec<
            'op,
            ResIds = ResIds,
            State = State,
            StatusOpSpec = StatusOpSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + FullSpecRt<'op>
        + 'static,
    ResIds: Serialize + DeserializeOwned,
    State: Diff + Serialize + DeserializeOwned,
    StatusOpSpec: OpSpec<'op, State = (), Output = State>,
    EnsureOpSpec: OpSpecDry<'op, State = State, Output = ResIds>,
    CleanOpSpec: OpSpecDry<'op, State = State, Output = ResIds>,
{
    fn from(full_spec: FS) -> Self {
        Self(Box::new(full_spec))
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
