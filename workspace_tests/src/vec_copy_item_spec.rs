use diff::{Diff, VecDiff};
#[nougat::gat(Data)]
use peace::cfg::CleanOpSpec;
#[nougat::gat(Data)]
use peace::cfg::EnsureOpSpec;
#[nougat::gat(Data)]
use peace::cfg::FnSpec;
#[nougat::gat(Data)]
use peace::cfg::StateDiffFnSpec;
use peace::{
    cfg::{
        async_trait, item_spec_id, nougat, ItemSpec, ItemSpecId, OpCheckStatus, ProgressLimit,
        State,
    },
    data::{Data, R, W},
    resources::{resources_type_state::Empty, Resources},
    rt_model::ItemSpecWrapper,
};
use serde::{Deserialize, Serialize};

/// Type alias for `ItemSpecWrapper` with all of VecCopyItemSpec's parameters.
pub type VecCopyItemSpecWrapper = ItemSpecWrapper<
    VecCopyItemSpec,
    VecCopyError,
    Vec<u8>,
    (),
    VecDiff<u8>,
    VecCopyStateCurrentFnSpec,
    VecCopyStateDesiredFnSpec,
    VecCopyStateDiffFnSpec,
    VecCopyEnsureOpSpec,
    VecCopyCleanOpSpec,
>;

/// Copies bytes from one `Vec` to another.
#[derive(Debug)]
pub struct VecCopyItemSpec;

#[async_trait(?Send)]
impl ItemSpec for VecCopyItemSpec {
    type CleanOpSpec = VecCopyCleanOpSpec;
    type EnsureOpSpec = VecCopyEnsureOpSpec;
    type Error = VecCopyError;
    type StateCurrentFnSpec = VecCopyStateCurrentFnSpec;
    type StateDesiredFnSpec = VecCopyStateDesiredFnSpec;
    type StateDiff = VecDiff<u8>;
    type StateDiffFnSpec = VecCopyStateDiffFnSpec;
    type StateLogical = Vec<u8>;
    type StatePhysical = ();

    fn id(&self) -> ItemSpecId {
        item_spec_id!("vec_copy_item_spec")
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

#[async_trait(?Send)]
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

#[async_trait(?Send)]
#[nougat::gat]
impl EnsureOpSpec for VecCopyEnsureOpSpec {
    type Data<'op> = VecCopyParams<'op>
        where Self: 'op;
    type Error = VecCopyError;
    type StateDiff = VecDiff<u8>;
    type StateLogical = Vec<u8>;
    type StatePhysical = ();

    async fn check(
        _vec_copy_params: VecCopyParams<'_>,
        _state_current: &State<Self::StateLogical, Self::StatePhysical>,
        state_desired: &Vec<u8>,
        diff: &VecDiff<u8>,
    ) -> Result<OpCheckStatus, VecCopyError> {
        let op_check_status = if diff.0.is_empty() {
            OpCheckStatus::ExecNotRequired
        } else {
            let progress_limit = TryInto::<u64>::try_into(state_desired.len())
                .map(ProgressLimit::Bytes)
                .unwrap_or(ProgressLimit::Unknown);

            OpCheckStatus::ExecRequired { progress_limit }
        };
        Ok(op_check_status)
    }

    async fn exec_dry(
        _vec_copy_params: VecCopyParams<'_>,
        _state_current: &State<Self::StateLogical, Self::StatePhysical>,
        _state_desired: &Vec<u8>,
        _diff: &VecDiff<u8>,
    ) -> Result<Self::StatePhysical, Self::Error> {
        // Would replace vec_b's contents with vec_a's
        Ok(())
    }

    async fn exec(
        mut vec_copy_params: VecCopyParams<'_>,
        _state_current: &State<Self::StateLogical, Self::StatePhysical>,
        state_desired: &Vec<u8>,
        _diff: &VecDiff<u8>,
    ) -> Result<Self::StatePhysical, VecCopyError> {
        let dest = vec_copy_params.dest_mut();
        dest.0.clear();
        dest.0.extend_from_slice(state_desired.as_slice());
        Ok(())
    }
}

/// Error while managing a file download.
#[derive(Debug, thiserror::Error)]
pub enum VecCopyError {
    /// A `peace` runtime error occurred.
    #[error("A `peace` runtime error occurred.")]
    PeaceRtError(#[from] peace::rt_model::Error),
}

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

/// `StateCurrentFnSpec` for the vector to copy.
#[derive(Debug)]
pub struct VecCopyStateCurrentFnSpec;

#[async_trait(?Send)]
#[nougat::gat]
impl FnSpec for VecCopyStateCurrentFnSpec {
    type Data<'op> = R<'op, VecB>
        where Self: 'op;
    type Error = VecCopyError;
    type Output = State<Vec<u8>, ()>;

    async fn exec(vec_b: R<'_, VecB>) -> Result<Self::Output, VecCopyError> {
        Ok(State::new(vec_b.0.clone(), ()))
    }
}

/// `StateCurrentFnSpec` for the vector to copy.
#[derive(Debug)]
pub struct VecCopyStateDesiredFnSpec;

#[async_trait(?Send)]
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

/// `StateDiffFnSpec` to compute the diff between two vectors.
#[derive(Debug)]
pub struct VecCopyStateDiffFnSpec;

#[async_trait(?Send)]
#[nougat::gat]
impl StateDiffFnSpec for VecCopyStateDiffFnSpec {
    type Data<'op> = &'op ()
        where Self: 'op;
    type Error = VecCopyError;
    type StateDiff = VecDiff<u8>;
    type StateLogical = Vec<u8>;
    type StatePhysical = ();

    async fn exec(
        _: &(),
        state_current: &State<Vec<u8>, ()>,
        state_desired: &Vec<u8>,
    ) -> Result<Self::StateDiff, VecCopyError> {
        Ok(state_current.logical.diff(state_desired))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecA(pub Vec<u8>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecB(pub Vec<u8>);
