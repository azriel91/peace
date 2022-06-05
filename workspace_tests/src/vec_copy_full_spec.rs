use peace::{
    cfg::{async_trait, FnSpec, FullSpec, OpCheckStatus, OpSpec, OpSpecDry, ProgressLimit},
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
impl<'op> OpSpec<'op> for VecCopyCleanOpSpec {
    type Data = W<'op, VecB>;
    type Error = VecCopyError;
    type Output = ();
    type State = Vec<u8>;

    async fn desired(_vec_b: W<'op, VecB>) -> Result<Vec<u8>, VecCopyError> {
        Ok(Vec::new())
    }

    async fn check(vec_b: W<'op, VecB>, state: &Vec<u8>) -> Result<OpCheckStatus, VecCopyError> {
        let op_check_status = if *vec_b.0 == *state {
            let progress_limit = TryInto::<u64>::try_into(state.len())
                .map(ProgressLimit::Bytes)
                .unwrap_or(ProgressLimit::Unknown);

            OpCheckStatus::ExecRequired { progress_limit }
        } else {
            OpCheckStatus::ExecNotRequired
        };
        Ok(op_check_status)
    }

    async fn exec(mut vec_b: W<'op, VecB>) -> Result<Self::Output, VecCopyError> {
        vec_b.0.clear();
        Ok(())
    }
}

#[async_trait]
impl<'op> OpSpecDry<'op> for VecCopyCleanOpSpec {
    async fn exec_dry() -> Result<Self::Output, Self::Error> {
        todo!("should this be inferred from the Diff instead?")
    }
}

/// Ensure OpSpec for the file to download.
#[derive(Debug, Default)]
pub struct VecCopyEnsureOpSpec;

#[async_trait]
impl<'op> OpSpec<'op> for VecCopyEnsureOpSpec {
    type Data = VecCopyParamsMut<'op>;
    type Error = VecCopyError;
    type Output = ();
    type State = Vec<u8>;

    async fn desired(vec_copy_params: VecCopyParamsMut<'op>) -> Result<Vec<u8>, VecCopyError> {
        Ok(vec_copy_params.src().0.clone())
    }

    async fn check(
        vec_copy_params: VecCopyParamsMut<'op>,
        state: &Vec<u8>,
    ) -> Result<OpCheckStatus, VecCopyError> {
        let op_check_status = if *vec_copy_params.dest().0 == *state {
            let progress_limit = TryInto::<u64>::try_into(state.len())
                .map(ProgressLimit::Bytes)
                .unwrap_or(ProgressLimit::Unknown);

            OpCheckStatus::ExecRequired { progress_limit }
        } else {
            OpCheckStatus::ExecNotRequired
        };
        Ok(op_check_status)
    }

    async fn exec(
        mut vec_copy_params: VecCopyParamsMut<'op>,
    ) -> Result<Self::Output, VecCopyError> {
        let VecCopyParamsMut { src, dest } = &mut vec_copy_params;
        dest.0.clear();
        dest.0.extend_from_slice(src.0.as_slice());
        Ok(())
    }
}

#[async_trait]
impl<'op> OpSpecDry<'op> for VecCopyEnsureOpSpec {
    async fn exec_dry() -> Result<Self::Output, Self::Error> {
        todo!("should this be inferred from the Diff instead?")
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

    pub fn dest(&self) -> &VecB {
        &self.dest
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
