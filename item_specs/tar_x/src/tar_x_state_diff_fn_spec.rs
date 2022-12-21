use peace::cfg::{async_trait, state::Nothing, State, StateDiffFnSpec};

use crate::{FileMetadatas, TarXError, TarXStateDiff};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct TarXStateDiffFnSpec;

#[async_trait(?Send)]
impl StateDiffFnSpec for TarXStateDiffFnSpec {
    type Data<'op> = &'op ();
    type Error = TarXError;
    type StateDiff = TarXStateDiff;
    type StateLogical = FileMetadatas;
    type StatePhysical = Nothing;

    async fn exec(
        _: &(),
        _state_current: &State<FileMetadatas, Nothing>,
        _state_desired: &FileMetadatas,
    ) -> Result<Self::StateDiff, TarXError> {
        todo!()
    }
}
