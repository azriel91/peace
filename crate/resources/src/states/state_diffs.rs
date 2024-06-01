use std::ops::Deref;

use peace_core::StepId;
use peace_fmt::{Presentable, Presenter};
use serde::Serialize;
use type_reg::untagged::{BoxDtDisplay, TypeMap};

use crate::internal::StateDiffsMut;

/// Diffs of `State`s for each `Step`s. `TypeMap<StepId, BoxDtDisplay>`
/// newtype.
///
/// [`External`] fields are not necessarily used in `StateDiff` computations.
///
/// # Implementors
///
/// [`StateDiffs`] is a read-only resource, stored in [`Resources`] after
/// `DiffCmd` has been executed.
///
/// [`External`]: peace_cfg::state::External
/// [`Resources`]: crate::Resources
#[derive(Debug, Default, Serialize)]
pub struct StateDiffs(TypeMap<StepId, BoxDtDisplay>);

impl StateDiffs {
    /// Returns a new `StateDiffs` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `StateDiffs` map with the specified capacity.
    ///
    /// The `StateDiffs` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity_typed(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<StepId, BoxDtDisplay> {
        self.0
    }
}

impl Deref for StateDiffs {
    type Target = TypeMap<StepId, BoxDtDisplay>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TypeMap<StepId, BoxDtDisplay>> for StateDiffs {
    fn from(type_map: TypeMap<StepId, BoxDtDisplay>) -> Self {
        Self(type_map)
    }
}

impl From<StateDiffsMut> for StateDiffs {
    fn from(state_diffs_mut: StateDiffsMut) -> Self {
        Self(state_diffs_mut.into_inner())
    }
}

#[peace_fmt::async_trait(?Send)]
impl Presentable for StateDiffs {
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        presenter
            .list_numbered_with(self.iter(), |(step_id, state_diff)| {
                (step_id, format!(": {state_diff}"))
            })
            .await
    }
}
