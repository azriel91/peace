use peace::cfg::{async_trait, state::Nothing, State, StateDiffFnSpec};

use crate::{TarXError, TarXState, TarXStateDiff};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct TarXStateDiffFnSpec;

#[async_trait(?Send)]
impl StateDiffFnSpec for TarXStateDiffFnSpec {
    type Data<'op> = &'op()
        where Self: 'op;
    type Error = TarXError;
    type StateDiff = TarXStateDiff;
    type StateLogical = TarXState;
    type StatePhysical = Nothing;

    async fn exec(
        _: &(),
        _state_current: &State<TarXState, Nothing>,
        _state_desired: &TarXState,
    ) -> Result<Self::StateDiff, TarXError> {
        todo!()
    }
}
