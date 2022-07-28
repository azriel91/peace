#[nougat::gat(Data)]
use peace::cfg::CleanOpSpec;
#[nougat::gat(Data)]
use peace::cfg::EnsureOpSpec;
#[nougat::gat(Data)]
use peace::cfg::FnSpec;
use peace::{
    cfg::{
        async_trait, full_spec_id, nougat, FullSpec, FullSpecId, OpCheckStatus, ProgressLimit,
        State,
    },
    data::{Data, R, W},
    resources::{resources_type_state::Empty, Resources},
};
use serde::{Deserialize, Serialize};

/// Copies bytes from one `Vec` to another.
#[derive(Debug)]
pub struct VecCopyFullSpec;

#[async_trait]
impl FullSpec for VecCopyFullSpec {
    type CleanOpSpec = VecCopyCleanOpSpec;
    type EnsureOpSpec = VecCopyEnsureOpSpec;
    type Error = VecCopyError;
    type StateDesiredFnSpec = VecCopyStateDesiredFnSpec;
    type StateLogical = Vec<u8>;
    type StateNowFnSpec = VecCopyStateNowFnSpec;
    type StatePhysical = ();

    fn id(&self) -> FullSpecId {
        full_spec_id!("vec_copy_full_spec")
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), VecCopyError> {
        resources.insert(VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]));
        resources.insert(VecB(vec![]));
        Ok(())
    }
}

/// `CleanOpSpec` for the vec to copy.
#[derive(Debug)]
pub struct VecCopyCleanOpSpec;

#[async_trait]
#[nougat::gat]
impl CleanOpSpec for VecCopyCleanOpSpec {
    type Data<'op> = W<'op, VecB>
        where Self: 'op;
    type Error = VecCopyError;
    type StateLogical = Vec<u8>;
    type StatePhysical = ();

    async fn check(
        vec_b: W<'_, VecB>,
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
        _vec_b: W<'_, VecB>,
        _state: &State<Self::StateLogical, Self::StatePhysical>,
    ) -> Result<(), VecCopyError> {
        // Would erase vec_b
        Ok(())
    }

    async fn exec(
        mut vec_b: W<'_, VecB>,
        _state: &State<Self::StateLogical, Self::StatePhysical>,
    ) -> Result<(), VecCopyError> {
        vec_b.0.clear();
        Ok(())
    }
}

/// `EnsureOpSpec` for the vec to copy.
#[derive(Debug)]
pub struct VecCopyEnsureOpSpec;

#[async_trait]
#[nougat::gat]
impl EnsureOpSpec for VecCopyEnsureOpSpec {
    type Data<'op> = VecCopyParams<'op>
        where Self: 'op;
    type Error = VecCopyError;
    type StateLogical = Vec<u8>;
    type StatePhysical = ();

    async fn check(
        _vec_copy_params: VecCopyParams<'_>,
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
        _vec_copy_params: VecCopyParams<'_>,
        _state_current: &State<Self::StateLogical, Self::StatePhysical>,
        _state_desired: &Vec<u8>,
    ) -> Result<Self::StatePhysical, Self::Error> {
        // Would replace vec_b's contents with vec_a's
        Ok(())
    }

    async fn exec(
        mut vec_copy_params: VecCopyParams<'_>,
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
pub struct VecCopyParams<'op> {
    /// Destination `Vec` to write to.
    dest: W<'op, VecB>,
}

impl<'op> VecCopyParams<'op> {
    pub fn dest_mut(&mut self) -> &mut VecB {
        &mut *self.dest
    }
}

/// `StateNowFnSpec` for the vector to copy.
#[derive(Debug)]
pub struct VecCopyStateNowFnSpec;

#[async_trait]
#[nougat::gat]
impl FnSpec for VecCopyStateNowFnSpec {
    type Data<'op> = R<'op, VecB>
        where Self: 'op;
    type Error = VecCopyError;
    type Output = State<Vec<u8>, ()>;

    async fn exec(vec_b: R<'_, VecB>) -> Result<Self::Output, VecCopyError> {
        Ok(State::new(vec_b.0.clone(), ()))
    }
}

/// `StateNowFnSpec` for the vector to copy.
#[derive(Debug)]
pub struct VecCopyStateDesiredFnSpec;

#[async_trait]
#[nougat::gat]
impl FnSpec for VecCopyStateDesiredFnSpec {
    type Data<'op> = R<'op, VecA>
        where Self: 'op;
    type Error = VecCopyError;
    type Output = Vec<u8>;

    async fn exec(vec_a: R<'_, VecA>) -> Result<Self::Output, VecCopyError> {
        Ok(vec_a.0.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecA(pub Vec<u8>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecB(pub Vec<u8>);
