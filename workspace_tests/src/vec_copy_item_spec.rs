use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use diff::{Diff, VecDiff, VecDiffType};
use peace::{
    cfg::{
        async_trait, item_spec_id, state::Nothing, CleanOpSpec, EnsureOpSpec, ItemSpec, ItemSpecId,
        OpCheckStatus, ProgressLimit, State, StateDiffFnSpec, TryFnSpec,
    },
    data::{Data, RMaybe, R, W},
    resources::{resources::ts::Empty, states::StatesSaved, Resources},
    rt_model::ItemSpecWrapper,
};
use serde::{Deserialize, Serialize};

/// Type alias for `ItemSpecWrapper` with all of VecCopyItemSpec's parameters.
pub type VecCopyItemSpecWrapper = ItemSpecWrapper<
    VecCopyItemSpec,
    VecCopyError,
    VecCopyState,
    Nothing,
    VecCopyDiff,
    VecCopyStateCurrentFnSpec,
    VecCopyStateDesiredFnSpec,
    VecCopyStateDiffFnSpec,
    VecCopyEnsureOpSpec,
    VecCopyCleanOpSpec,
>;

/// Copies bytes from `VecA` to `VecB`.
#[derive(Debug)]
pub struct VecCopyItemSpec;

#[async_trait(?Send)]
impl ItemSpec for VecCopyItemSpec {
    type CleanOpSpec = VecCopyCleanOpSpec;
    type EnsureOpSpec = VecCopyEnsureOpSpec;
    type Error = VecCopyError;
    type StateCurrentFnSpec = VecCopyStateCurrentFnSpec;
    type StateDesiredFnSpec = VecCopyStateDesiredFnSpec;
    type StateDiff = VecCopyDiff;
    type StateDiffFnSpec = VecCopyStateDiffFnSpec;
    type StateLogical = VecCopyState;
    type StatePhysical = Nothing;

    fn id(&self) -> ItemSpecId {
        item_spec_id!("vec_copy")
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), VecCopyError> {
        resources.insert(VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]));

        let vec_b = {
            let states_saved = <RMaybe<'_, StatesSaved> as Data>::borrow(resources);
            let vec_copy_state_saved: Option<&'_ State<VecCopyState, Nothing>> = states_saved
                .as_ref()
                .and_then(|states_saved| states_saved.get(&self.id()));
            if let Some(vec_copy_state) = vec_copy_state_saved {
                VecB((*vec_copy_state.logical).clone())
            } else {
                VecB::default()
            }
        };
        resources.insert(vec_b);
        Ok(())
    }
}

/// `CleanOpSpec` for the vec to copy.
#[derive(Debug)]
pub struct VecCopyCleanOpSpec;

#[async_trait(?Send)]
impl CleanOpSpec for VecCopyCleanOpSpec {
    type Data<'op> = W<'op, VecB>;
    type Error = VecCopyError;
    type StateLogical = VecCopyState;
    type StatePhysical = Nothing;

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
impl EnsureOpSpec for VecCopyEnsureOpSpec {
    type Data<'op> = VecCopyParams<'op>;
    type Error = VecCopyError;
    type StateDiff = VecCopyDiff;
    type StateLogical = VecCopyState;
    type StatePhysical = Nothing;

    async fn check(
        _vec_copy_params: VecCopyParams<'_>,
        _state_current: &State<Self::StateLogical, Self::StatePhysical>,
        state_desired: &VecCopyState,
        diff: &VecCopyDiff,
    ) -> Result<OpCheckStatus, VecCopyError> {
        let op_check_status = if diff.0.0.is_empty() {
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
        _state_desired: &VecCopyState,
        _diff: &VecCopyDiff,
    ) -> Result<Self::StatePhysical, Self::Error> {
        // Would replace vec_b's contents with vec_a's
        Ok(Nothing)
    }

    async fn exec(
        mut vec_copy_params: VecCopyParams<'_>,
        _state_current: &State<Self::StateLogical, Self::StatePhysical>,
        state_desired: &VecCopyState,
        _diff: &VecCopyDiff,
    ) -> Result<Self::StatePhysical, VecCopyError> {
        let dest = vec_copy_params.dest_mut();
        dest.0.clear();
        dest.0.extend_from_slice(state_desired.as_slice());
        Ok(Nothing)
    }
}

#[cfg(feature = "error_reporting")]
use peace::miette;

/// Error while managing a file download.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum VecCopyError {
    /// A `peace` runtime error occurred.
    #[error("A `peace` runtime error occurred.")]
    PeaceRtError(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace::rt_model::Error,
    ),
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
impl TryFnSpec for VecCopyStateCurrentFnSpec {
    type Data<'op> = R<'op, VecB>;
    type Error = VecCopyError;
    type Output = Option<State<VecCopyState, Nothing>>;

    async fn exec(vec_b: R<'_, VecB>) -> Result<Self::Output, VecCopyError> {
        Ok(Some(State::new(
            VecCopyState::from(vec_b.0.clone()),
            Nothing,
        )))
    }
}

/// `StateCurrentFnSpec` for the vector to copy.
#[derive(Debug)]
pub struct VecCopyStateDesiredFnSpec;

#[async_trait(?Send)]
impl TryFnSpec for VecCopyStateDesiredFnSpec {
    type Data<'op> = R<'op, VecA>;
    type Error = VecCopyError;
    type Output = Option<VecCopyState>;

    async fn exec(vec_a: R<'_, VecA>) -> Result<Self::Output, VecCopyError> {
        Ok(vec_a.0.clone()).map(VecCopyState::from).map(Some)
    }
}

/// `StateDiffFnSpec` to compute the diff between two vectors.
#[derive(Debug)]
pub struct VecCopyStateDiffFnSpec;

#[async_trait(?Send)]
impl StateDiffFnSpec for VecCopyStateDiffFnSpec {
    type Data<'op> = &'op ();
    type Error = VecCopyError;
    type StateDiff = VecCopyDiff;
    type StateLogical = VecCopyState;
    type StatePhysical = Nothing;

    async fn exec(
        _: &(),
        state_current: &State<VecCopyState, Nothing>,
        state_desired: &VecCopyState,
    ) -> Result<Self::StateDiff, VecCopyError> {
        Ok(state_current.logical.diff(&state_desired)).map(VecCopyDiff::from)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct VecA(pub Vec<u8>);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct VecB(pub Vec<u8>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VecCopyState(Vec<u8>);

impl VecCopyState {
    /// Returns an empty `VecCopyState`.
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl From<Vec<u8>> for VecCopyState {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl Deref for VecCopyState {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VecCopyState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for VecCopyState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VecCopyDiff(VecDiff<u8>);

impl From<VecDiff<u8>> for VecCopyDiff {
    fn from(v: VecDiff<u8>) -> Self {
        Self(v)
    }
}

impl Deref for VecCopyDiff {
    type Target = VecDiff<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VecCopyDiff {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for VecCopyDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        self.0
            .0
            .iter()
            .try_for_each(|vec_diff_type| match vec_diff_type {
                VecDiffType::Removed { index, len } => {
                    let index_end = index + len;
                    write!(f, "(-){index}..{index_end}, ")
                }
                VecDiffType::Altered { index, changes } => {
                    write!(f, "(~){index};")?;
                    changes.iter().try_for_each(|value| write!(f, "{value}, "))
                }
                VecDiffType::Inserted { index, changes } => {
                    write!(f, "(+){index};")?;
                    changes.iter().try_for_each(|value| write!(f, "{value}, "))
                }
            })?;
        write!(f, "]")
    }
}
