use peace::{
    cfg::{
        async_trait, full_spec_id, CleanOpSpec, EnsureOpSpec, FnSpec, FullSpec, FullSpecId,
        OpCheckStatus, ProgressLimit, State,
    },
    data::{Data, Resources, R, W},
};
use serde::{Deserialize, Serialize};

/// Copies bytes from one `Vec` to another.
#[derive(Debug)]
pub struct VecCopyFullSpec;

#[async_trait]
impl<'op> FullSpec<'op> for VecCopyFullSpec {
    type CleanOpSpec = VecCopyCleanOpSpec;
    type EnsureOpSpec = VecCopyEnsureOpSpec;
    type Error = VecCopyError;
    type StateLogical = Vec<u8>;
    type StatePhysical = ();
    type StatusFnSpec = VecCopyStatusFnSpec;

    fn id(&self) -> FullSpecId {
        full_spec_id!("vec_copy_full_spec")
    }

    async fn setup(resources: &mut Resources) -> Result<(), VecCopyError> {
        resources.insert(VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]));
        Ok(())
    }
}

/// Clean OpSpec for the file to download.
#[derive(Debug)]
pub struct VecCopyCleanOpSpec;

#[async_trait]
impl<'op> CleanOpSpec<'op> for VecCopyCleanOpSpec {
    type Data = W<'op, VecB>;
    type Error = VecCopyError;
    type StateLogical = Vec<u8>;
    type StatePhysical = ();

    async fn check(
        vec_b: W<'op, VecB>,
        _state: &State<Self::StateLogical, Self::StatePhysical>,
    ) -> Result<OpCheckStatus, VecCopyError> {
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

    async fn exec_dry(
        _vec_b: W<'op, VecB>,
        _state: &State<Self::StateLogical, Self::StatePhysical>,
    ) -> Result<(), VecCopyError> {
        // Would erase vec_b
        Ok(())
    }

    async fn exec(
        mut vec_b: W<'op, VecB>,
        _state: &State<Self::StateLogical, Self::StatePhysical>,
    ) -> Result<(), VecCopyError> {
        vec_b.0.clear();
        Ok(())
    }
}

/// Ensure OpSpec for the file to download.
#[derive(Debug)]
pub struct VecCopyEnsureOpSpec;

#[async_trait]
impl<'op> EnsureOpSpec<'op> for VecCopyEnsureOpSpec {
    type Data = VecCopyParamsMut<'op>;
    type Error = VecCopyError;
    type StateLogical = Vec<u8>;
    type StatePhysical = ();

    async fn desired(vec_copy_params: VecCopyParamsMut<'op>) -> Result<Vec<u8>, VecCopyError> {
        Ok(vec_copy_params.src().0.clone())
    }

    async fn check(
        _vec_copy_params: VecCopyParamsMut<'op>,
        State {
            logical: file_state_current,
            ..
        }: &State<Self::StateLogical, Self::StatePhysical>,
        state_desired: &Vec<u8>,
    ) -> Result<OpCheckStatus, VecCopyError> {
        let op_check_status = if file_state_current != state_desired {
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
        _state_current: &State<Self::StateLogical, Self::StatePhysical>,
        _state_desired: &Vec<u8>,
    ) -> Result<Self::StatePhysical, Self::Error> {
        // Would replace vec_b's contents with vec_a's
        Ok(())
    }

    async fn exec(
        mut vec_copy_params: VecCopyParamsMut<'op>,
        _state_current: &State<Self::StateLogical, Self::StatePhysical>,
        state_desired: &Vec<u8>,
    ) -> Result<Self::StatePhysical, VecCopyError> {
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
#[derive(Debug)]
pub struct VecCopyStatusFnSpec;

#[async_trait]
impl<'op> FnSpec<'op> for VecCopyStatusFnSpec {
    type Data = R<'op, VecA>;
    type Error = VecCopyError;
    type Output = State<Vec<u8>, ()>;

    async fn exec(vec_a: R<'op, VecA>) -> Result<Self::Output, VecCopyError> {
        Ok(State::new(vec_a.0.clone(), ()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecA(Vec<u8>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecB(Vec<u8>);
