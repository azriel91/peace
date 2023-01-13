use peace::cfg::{async_trait, StateDiffFnSpec};

use crate::{BlankError, BlankState, BlankStateDiff};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct BlankStateDiffFnSpec;

#[async_trait(?Send)]
impl StateDiffFnSpec for BlankStateDiffFnSpec {
    type Data<'op> = &'op ();
    type Error = BlankError;
    type State = BlankState;
    type StateDiff = BlankStateDiff;

    async fn exec(
        _: &(),
        blank_state_current: &BlankState,
        blank_state_desired: &BlankState,
    ) -> Result<Self::StateDiff, BlankError> {
        let diff = match (blank_state_current, blank_state_desired) {
            (BlankState(Some(current)), BlankState(Some(desired))) if current == desired => {
                BlankStateDiff::InSync { value: *current }
            }
            (BlankState(Some(current)), BlankState(Some(desired))) => BlankStateDiff::OutOfSync {
                diff: i64::from(desired - current),
            },
            (BlankState(None), BlankState(Some(desired))) => {
                BlankStateDiff::Added { value: *desired }
            }
            (BlankState(_), BlankState(None)) => unreachable!("desired state is always Some"),
        };

        Ok(diff)
    }
}
