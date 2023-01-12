use peace::cfg::{async_trait, state::Nothing, State, StateDiffFnSpec};

use crate::{BlankError, BlankState, BlankStateDiff};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct BlankStateDiffFnSpec;

#[async_trait(?Send)]
impl StateDiffFnSpec for BlankStateDiffFnSpec {
    type Data<'op> = &'op ();
    type Error = BlankError;
    type StateDiff = BlankStateDiff;
    type StateLogical = BlankState;
    type StatePhysical = Nothing;

    async fn exec(
        _: &(),
        state_current: &State<BlankState, Nothing>,
        state_desired: &State<BlankState, Nothing>,
    ) -> Result<Self::StateDiff, BlankError> {
        let diff = match (state_current.logical, state_desired.logical) {
            (BlankState(Some(current)), BlankState(Some(desired))) if current == desired => {
                BlankStateDiff::InSync { value: current }
            }
            (BlankState(Some(current)), BlankState(Some(desired))) => BlankStateDiff::OutOfSync {
                diff: i64::from(desired - current),
            },
            (BlankState(None), BlankState(Some(desired))) => {
                BlankStateDiff::Added { value: desired }
            }
            (BlankState(_), BlankState(None)) => unreachable!("desired state is always Some"),
        };

        Ok(diff)
    }
}
