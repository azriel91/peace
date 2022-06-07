use peace::{
    cfg::{async_trait, CleanOpSpec, EnsureOpSpec, FnSpec, FullSpec, OpCheckStatus, ProgressLimit},
    data::{Data, Resources, R, W},
};
use serde::{Deserialize, Serialize};

/// Copies bytes from one `Vec` to another.
#[derive(Debug, Default)]
pub struct VecCopyFullSpec {
    status_fn_spec: VecCopyStatusFnSpec,
    ensure_op_spec: VecCopyEnsureOpSpec,
    clean_op_spec: VecCopyCleanOpSpec,
}

#[async_trait]
impl<'op> FullSpec<'op> for VecCopyFullSpec {
    type CleanOpSpec = VecCopyCleanOpSpec;
    type EnsureOpSpec = VecCopyEnsureOpSpec;
    type Error = VecCopyError;
    type ResIds = ();
    type State = Vec<u8>;
    type StatusFnSpec = VecCopyStatusFnSpec;

    fn status_fn_spec(&self) -> &Self::StatusFnSpec {
        &self.status_fn_spec
    }

    fn ensure_op_spec(&self) -> &Self::EnsureOpSpec {
        &self.ensure_op_spec
    }

    fn clean_op_spec(&self) -> &Self::CleanOpSpec {
        &self.clean_op_spec
    }

    async fn setup(resources: &mut Resources) -> Result<(), VecCopyError> {
        resources.insert(VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]));
        Ok(())
    }
}

/// Clean OpSpec for the file to download.
#[derive(Debug, Default)]
pub struct VecCopyCleanOpSpec;

#[async_trait]
impl<'op> CleanOpSpec<'op> for VecCopyCleanOpSpec {
    type Data = W<'op, VecB>;
    type Error = VecCopyError;
    type ResIds = ();

    async fn check(vec_b: W<'op, VecB>, _res_ids: &()) -> Result<OpCheckStatus, VecCopyError> {
        let op_check_status = if vec_b.0.is_empty() {
            OpCheckStatus::ExecNotRequired
        } else {
            let progress_limit = TryInto::<u64>::try_into(vec_b.0.len())
                .map(ProgressLimit::Bytes)
                .unwrap_or(ProgressLimit::Unknown);

            OpCheckStatus::ExecRequired { progress_limit }
        };
        Ok(op_check_status)
    }

    async fn exec_dry(_vec_b: W<'op, VecB>, _res_ids: &()) -> Result<(), VecCopyError> {
        // Would erase vec_b
        Ok(())
    }

    async fn exec(mut vec_b: W<'op, VecB>, _res_ids: &()) -> Result<(), VecCopyError> {
        vec_b.0.clear();
        Ok(())
    }
}

/// Ensure OpSpec for the file to download.
#[derive(Debug, Default)]
pub struct VecCopyEnsureOpSpec;

#[async_trait]
impl<'op> EnsureOpSpec<'op> for VecCopyEnsureOpSpec {
    type Data = VecCopyParamsMut<'op>;
    type Error = VecCopyError;
    type ResIds = ();
    type State = Vec<u8>;

    async fn desired(vec_copy_params: VecCopyParamsMut<'op>) -> Result<Vec<u8>, VecCopyError> {
        Ok(vec_copy_params.src().0.clone())
    }

    async fn check(
        _vec_copy_params: VecCopyParamsMut<'op>,
        state_current: &Vec<u8>,
        state_desired: &Vec<u8>,
    ) -> Result<OpCheckStatus, VecCopyError> {
        let op_check_status = if state_current != state_desired {
            let progress_limit = TryInto::<u64>::try_into(state_desired.len())
                .map(ProgressLimit::Bytes)
                .unwrap_or(ProgressLimit::Unknown);

            OpCheckStatus::ExecRequired { progress_limit }
        } else {
            OpCheckStatus::ExecNotRequired
        };
        Ok(op_check_status)
    }

    async fn exec_dry(
        _vec_copy_params: VecCopyParamsMut<'op>,
        _state_current: &Vec<u8>,
        _state_desired: &Vec<u8>,
    ) -> Result<Self::ResIds, Self::Error> {
        // Would replace vec_b's contents with vec_a's
        Ok(())
    }

    async fn exec(
        mut vec_copy_params: VecCopyParamsMut<'op>,
        _state_current: &Vec<u8>,
        state_desired: &Vec<u8>,
    ) -> Result<Self::ResIds, VecCopyError> {
        let dest = vec_copy_params.dest_mut();
        dest.0.clear();
        dest.0.extend_from_slice(state_desired.as_slice());
        Ok(())
    }
}

/// Error while managing a file download.
#[derive(Debug, thiserror::Error)]
pub enum VecCopyError {}

#[derive(Data, Debug)]
pub struct VecCopyParamsMut<'op> {
    /// Source `Vec` to read from.
    src: R<'op, VecA>,
    /// Destination `Vec` to write to.
    dest: W<'op, VecB>,
}

impl<'op> VecCopyParamsMut<'op> {
    pub fn src(&self) -> &VecA {
        &self.src
    }

    pub fn dest_mut(&mut self) -> &mut VecB {
        &mut *self.dest
    }
}

/// Status OpSpec for the file to download.
#[derive(Debug, Default)]
pub struct VecCopyStatusFnSpec;

#[async_trait]
impl<'op> FnSpec<'op> for VecCopyStatusFnSpec {
    type Data = R<'op, VecA>;
    type Error = VecCopyError;
    type Output = Vec<u8>;

    async fn exec(vec_a: R<'op, VecA>) -> Result<Vec<u8>, VecCopyError> {
        Ok(vec_a.0.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecA(Vec<u8>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecB(Vec<u8>);
