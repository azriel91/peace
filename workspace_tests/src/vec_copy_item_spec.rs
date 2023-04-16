use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use diff::{Diff, VecDiff, VecDiffType};
#[cfg(feature = "output_progress")]
use peace::cfg::progress::{ProgressLimit, ProgressMsgUpdate};
use peace::{
    cfg::{async_trait, item_spec_id, ApplyCheck, FnCtx, ItemSpec, ItemSpecId},
    data::{
        accessors::{RMaybe, R, W},
        Data,
    },
    resources::{resources::ts::Empty, states::StatesSaved, Resources},
    rt_model::ItemSpecWrapper,
};
use serde::{Deserialize, Serialize};

/// Type alias for `ItemSpecWrapper` with all of VecCopyItemSpec's parameters.
pub type VecCopyItemSpecWrapper = ItemSpecWrapper<VecCopyItemSpec, VecCopyError>;

/// Copies bytes from `VecA` to `VecB`.
#[derive(Clone, Debug)]
pub struct VecCopyItemSpec;

impl VecCopyItemSpec {
    pub const ID: &ItemSpecId = &item_spec_id!("vec_copy");
}

#[async_trait(?Send)]
impl ItemSpec for VecCopyItemSpec {
    type Data<'exec> = VecCopyData<'exec>;
    type Error = VecCopyError;
    type Params<'exec> = VecA;
    type State = VecCopyState;
    type StateDiff = VecCopyDiff;

    fn id(&self) -> &ItemSpecId {
        Self::ID
    }

    async fn try_state_current(
        fn_ctx: FnCtx<'_>,
        params_partial: Option<&Self::Params<'_>>,
        data: Self::Data<'_>,
    ) -> Result<Option<Self::State>, VecCopyError> {
        if let Some(params) = params_partial {
            Self::state_current(fn_ctx, params, data).await.map(Some)
        } else {
            Ok(None)
        }
    }

    async fn state_current(
        fn_ctx: FnCtx<'_>,
        _params: &Self::Params<'_>,
        data: Self::Data<'_>,
    ) -> Result<Self::State, VecCopyError> {
        #[cfg(not(feature = "output_progress"))]
        let _fn_ctx = fn_ctx;

        let vec_copy_state = VecCopyState::from(data.dest().0.clone());

        #[cfg(feature = "output_progress")]
        {
            if let Ok(len) = u64::try_from(vec_copy_state.len()) {
                fn_ctx.progress_sender.inc(len, ProgressMsgUpdate::NoChange);
            }
        }

        Ok(vec_copy_state)
    }

    async fn try_state_desired(
        fn_ctx: FnCtx<'_>,
        params_partial: Option<&Self::Params<'_>>,
        data: Self::Data<'_>,
    ) -> Result<Option<Self::State>, VecCopyError> {
        if let Some(params) = params_partial {
            Self::state_desired(fn_ctx, params, data).await.map(Some)
        } else {
            Ok(None)
        }
    }

    async fn state_desired(
        fn_ctx: FnCtx<'_>,
        _params: &Self::Params<'_>,
        data: Self::Data<'_>,
    ) -> Result<Self::State, VecCopyError> {
        #[cfg(not(feature = "output_progress"))]
        let _fn_ctx = fn_ctx;
        let vec_copy_state = VecCopyState::from(data.src().0.clone());

        #[cfg(feature = "output_progress")]
        {
            if let Ok(len) = u64::try_from(vec_copy_state.len()) {
                fn_ctx.progress_sender.inc(len, ProgressMsgUpdate::NoChange);
            }
        }

        Ok(vec_copy_state)
    }

    async fn state_diff(
        _params_partial: Option<&Self::Params<'_>>,
        _data: VecCopyData<'_>,
        state_current: &VecCopyState,
        state_desired: &VecCopyState,
    ) -> Result<Self::StateDiff, VecCopyError> {
        Ok(state_current.diff(state_desired)).map(VecCopyDiff::from)
    }

    async fn state_clean(
        _params_partial: Option<&Self::Params<'_>>,
        _data: Self::Data<'_>,
    ) -> Result<Self::State, VecCopyError> {
        Ok(VecCopyState::new())
    }

    async fn apply_check(
        _params: &Self::Params<'_>,
        _data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<ApplyCheck, Self::Error> {
        let apply_check = if diff.0.0.is_empty() {
            ApplyCheck::ExecNotRequired
        } else {
            #[cfg(not(feature = "output_progress"))]
            {
                let _state_current = state_current;
                let _state_target = state_target;
                ApplyCheck::ExecRequired
            }
            #[cfg(feature = "output_progress")]
            {
                let progress_limit =
                    TryInto::<u64>::try_into(state_current.len() + state_target.len())
                        .map(ProgressLimit::Bytes)
                        .unwrap_or(ProgressLimit::Unknown);

                ApplyCheck::ExecRequired { progress_limit }
            }
        };
        Ok(apply_check)
    }

    async fn apply_dry(
        _fn_ctx: FnCtx<'_>,
        _params: &Self::Params<'_>,
        _data: Self::Data<'_>,
        _state_current: &Self::State,
        state_target: &Self::State,
        _diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        // Would replace vec_b's contents with vec_a's
        Ok(state_target.clone())
    }

    async fn apply(
        fn_ctx: FnCtx<'_>,
        _params: &Self::Params<'_>,
        mut data: Self::Data<'_>,
        _state_current: &Self::State,
        state_target: &Self::State,
        _diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        let dest = data.dest_mut();
        dest.0.clear();
        dest.0.extend_from_slice(state_target.as_slice());

        #[cfg(not(feature = "output_progress"))]
        let _fn_ctx = fn_ctx;
        #[cfg(feature = "output_progress")]
        if let Ok(n) = state_target.len().try_into() {
            fn_ctx.progress_sender().inc(n, ProgressMsgUpdate::NoChange);
        }

        Ok(state_target.clone())
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), VecCopyError> {
        resources.insert(VecA(vec![0, 1, 2, 3, 4, 5, 6, 7]));

        let vec_b = {
            let states_saved = <RMaybe<'_, StatesSaved> as Data>::borrow(Self::ID, resources);
            let vec_copy_state_saved: Option<&'_ VecCopyState> = states_saved
                .as_ref()
                .and_then(|states_saved| states_saved.get(self.id()));
            if let Some(vec_copy_state) = vec_copy_state_saved {
                VecB(vec_copy_state.to_vec())
            } else {
                VecB::default()
            }
        };
        resources.insert(vec_b);
        Ok(())
    }
}

#[cfg(feature = "error_reporting")]
use peace::miette;

/// Error while executing a VecCopy.
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
pub struct VecCopyData<'exec> {
    /// Source `Vec` to read from.
    src: R<'exec, VecA>,
    /// Destination `Vec` to write to.
    dest: W<'exec, VecB>,
}

impl<'exec> VecCopyData<'exec> {
    pub fn src(&self) -> &VecA {
        &self.src
    }

    pub fn dest(&self) -> &VecB {
        &self.dest
    }

    pub fn dest_mut(&mut self) -> &mut VecB {
        &mut self.dest
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
