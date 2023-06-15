use std::{
    fmt,
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "output_progress")]
use peace::cfg::progress::{ProgressLimit, ProgressMsgUpdate};
use peace::{
    cfg::{async_trait, item_id, ApplyCheck, FnCtx, Item, ItemId},
    data::{
        accessors::{RMaybe, R, W},
        Data,
    },
    params::Params,
    resources::{resources::ts::Empty, states::StatesCurrentStored, Resources},
};
use serde::{Deserialize, Serialize};

/// Copies a number from `MockSrc` to `MockDest`.
///
/// This also allows each item function to be overridden.
#[derive(Clone, Debug)]
pub struct MockItem<Id>
where
    Id: Clone + Debug + Default + Send + Sync + 'static,
{
    /// ID of the item.
    id: ItemId,
    /// Marker.
    mock_fns: MockFns<Id>,
}

type FnTryState<Id> = fn(
    FnCtx<'_>,
    &<MockSrc as Params>::Partial,
    MockData<'_, Id>,
) -> Result<Option<MockState>, MockItemError>;

/// Copies bytes from `MockSrc` to `MockDest`.
#[derive(Clone, Debug, Default)]
pub struct MockFns<Id>
where
    Id: Clone + Debug + Default + Send + Sync + 'static,
{
    /// Override for `try_state_current` function.
    try_state_current: Option<FnTryState<Id>>,
    /// Override for `try_state_goal` function.
    try_state_goal: Option<FnTryState<Id>>,
    /// Marker.
    marker: PhantomData<Id>,
}

impl<Id> MockItem<Id>
where
    Id: Clone + Debug + Default + Send + Sync + 'static,
{
    pub const ID_DEFAULT: &ItemId = &item_id!("mock");

    pub fn new(id: ItemId) -> Self {
        Self {
            id,
            mock_fns: MockFns::<Id>::default(),
        }
    }

    pub fn with_try_state_current(mut self, f: FnTryState<Id>) -> Self {
        self.mock_fns.try_state_current = Some(f);
        self
    }

    pub fn with_try_state_goal(mut self, f: FnTryState<Id>) -> Self {
        self.mock_fns.try_state_goal = Some(f);
        self
    }

    async fn state_current_internal(
        fn_ctx: FnCtx<'_>,
        data: MockData<'_, Id>,
    ) -> Result<MockState, MockItemError> {
        #[cfg(not(feature = "output_progress"))]
        let _fn_ctx = fn_ctx;

        let mock_state = MockState(data.dest().0);

        #[cfg(feature = "output_progress")]
        {
            if let Ok(n) = u64::try_from(mock_state.0) {
                fn_ctx.progress_sender.inc(n, ProgressMsgUpdate::NoChange);
            }
        }

        Ok(mock_state)
    }

    async fn state_goal_internal(
        fn_ctx: FnCtx<'_>,
        mock_src: &u8,
    ) -> Result<MockState, MockItemError> {
        #[cfg(not(feature = "output_progress"))]
        let _fn_ctx = fn_ctx;
        let mock_state = MockState(*mock_src);

        #[cfg(feature = "output_progress")]
        {
            if let Ok(n) = u64::try_from(mock_state.0) {
                fn_ctx.progress_sender.inc(n, ProgressMsgUpdate::NoChange);
            }
        }

        Ok(mock_state)
    }
}

impl<Id> Default for MockItem<Id>
where
    Id: Clone + Debug + Default + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new(Self::ID_DEFAULT.clone())
    }
}

#[async_trait(?Send)]
impl<Id> Item for MockItem<Id>
where
    Id: Clone + Debug + Default + Send + Sync + 'static,
{
    type Data<'exec> = MockData<'exec, Id>;
    type Error = MockItemError;
    type Params<'exec> = MockSrc;
    type State = MockState;
    type StateDiff = MockDiff;

    fn id(&self) -> &ItemId {
        &self.id
    }

    async fn try_state_current(
        fn_ctx: FnCtx<'_>,
        params_partial: &<Self::Params<'_> as Params>::Partial,
        data: Self::Data<'_>,
    ) -> Result<Option<Self::State>, MockItemError> {
        if let Some(try_state_current) = data.mock_fns().try_state_current.as_ref() {
            try_state_current(fn_ctx, params_partial, data)
        } else {
            Self::state_current_internal(fn_ctx, data).await.map(Some)
        }
    }

    async fn state_current(
        fn_ctx: FnCtx<'_>,
        _params: &Self::Params<'_>,
        data: Self::Data<'_>,
    ) -> Result<Self::State, MockItemError> {
        Self::state_current_internal(fn_ctx, data).await
    }

    async fn try_state_goal(
        fn_ctx: FnCtx<'_>,
        params_partial: &<Self::Params<'_> as Params>::Partial,
        data: Self::Data<'_>,
    ) -> Result<Option<Self::State>, MockItemError> {
        if let Some(try_state_goal) = data.mock_fns().try_state_goal.as_ref() {
            try_state_goal(fn_ctx, params_partial, data)
        } else if let Some(mock_src) = params_partial.0.as_ref() {
            Self::state_goal_internal(fn_ctx, mock_src).await.map(Some)
        } else {
            Ok(None)
        }
    }

    async fn state_goal(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        _data: Self::Data<'_>,
    ) -> Result<Self::State, MockItemError> {
        Self::state_goal_internal(fn_ctx, &params.0).await
    }

    async fn state_diff(
        _params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: MockData<'_, Id>,
        state_current: &MockState,
        state_goal: &MockState,
    ) -> Result<Self::StateDiff, MockItemError> {
        Ok(i16::from(state_goal.0) - i16::from(state_current.0)).map(MockDiff)
    }

    async fn state_clean(
        _params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: Self::Data<'_>,
    ) -> Result<Self::State, MockItemError> {
        Ok(MockState::new())
    }

    async fn apply_check(
        _params: &Self::Params<'_>,
        _data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<ApplyCheck, Self::Error> {
        let apply_check = if diff.0 == 0 {
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
                let progress_limit = TryInto::<u64>::try_into(state_current.0 + state_target.0)
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
        // Would replace mock_dest's contents with mock_src's
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
        dest.0 = state_target.0;

        #[cfg(not(feature = "output_progress"))]
        let _fn_ctx = fn_ctx;
        #[cfg(feature = "output_progress")]
        if let Ok(n) = state_target.0.try_into() {
            fn_ctx.progress_sender().inc(n, ProgressMsgUpdate::NoChange);
        }

        Ok(state_target.clone())
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), MockItemError> {
        resources.insert(self.mock_fns.clone());

        let mock_dest = {
            let states_current_stored =
                <RMaybe<'_, StatesCurrentStored> as Data>::borrow(Self::ID_DEFAULT, resources);
            let mock_state_current_stored: Option<&'_ MockState> = states_current_stored
                .as_ref()
                .and_then(|states_current_stored| states_current_stored.get(self.id()));
            if let Some(mock_state) = mock_state_current_stored {
                MockDest(mock_state.0)
            } else {
                MockDest::default()
            }
        };
        resources.insert(mock_dest);
        Ok(())
    }
}

#[cfg(feature = "error_reporting")]
use peace::miette;

/// Error while executing a Mock.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum MockItemError {
    /// A synthetic error.
    #[error("Synthetic error: {}.", _0)]
    Synthetic(String),

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
pub struct MockData<'exec, Id>
where
    Id: Clone + Debug + Default + Send + Sync + 'static,
{
    /// Mock functions for this item.
    mock_fns: R<'exec, MockFns<Id>>,
    /// Destination `Vec` to write to.
    dest: W<'exec, MockDest>,
}

impl<'exec, Id> MockData<'exec, Id>
where
    Id: Clone + Debug + Default + Send + Sync + 'static,
{
    pub fn mock_fns(&self) -> &MockFns<Id> {
        &self.mock_fns
    }

    pub fn dest(&self) -> &MockDest {
        &self.dest
    }

    pub fn dest_mut(&mut self) -> &mut MockDest {
        &mut self.dest
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Params)]
pub struct MockSrc(pub u8);

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MockDest(pub u8);

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MockState(pub u8);

impl MockState {
    /// Returns an empty `MockState`.
    pub fn new() -> Self {
        Self(0)
    }
}

impl Deref for MockState {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MockState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for MockState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MockDiff(pub i16);

impl Deref for MockDiff {
    type Target = i16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MockDiff {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for MockDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
