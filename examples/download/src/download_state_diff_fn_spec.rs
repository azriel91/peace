#[nougat::gat(Data)]
use peace::cfg::StateDiffFnSpec;
use peace::{
    cfg::{async_trait, nougat, state::Nothing, State},
    diff::{Changeable, Tracked},
};

use crate::{DownloadError, FileState, FileStateDiff};

/// Download status diff function.
#[derive(Debug)]
pub struct DownloadStateDiffFnSpec;

#[async_trait(?Send)]
#[nougat::gat]
impl StateDiffFnSpec for DownloadStateDiffFnSpec {
    type Data<'op> = &'op()
        where Self: 'op;
    type Error = DownloadError;
    type StateDiff = FileStateDiff;
    type StateLogical = FileState;
    type StatePhysical = Nothing;

    async fn exec(
        _: &(),
        state_current: &State<FileState, Nothing>,
        file_state_desired: &FileState,
    ) -> Result<Self::StateDiff, DownloadError> {
        let file_state_diff = {
            let file_state_current = &state_current.logical;
            match (file_state_current, file_state_desired) {
                (
                    FileState::StringContents { .. }
                    | FileState::Length { .. }
                    | FileState::Unknown { .. },
                    FileState::None,
                ) => FileStateDiff::Deleted,

                (
                    file_state_current @ (FileState::StringContents { .. }
                    | FileState::Length { .. }
                    | FileState::Unknown { .. }),
                    file_state_desired @ (FileState::StringContents { .. }
                    | FileState::Length { .. }
                    | FileState::Unknown { .. }),
                )
                | (
                    file_state_current @ FileState::None,
                    file_state_desired @ (FileState::StringContents { .. }
                    | FileState::Length { .. }
                    | FileState::Unknown { .. }),
                ) => {
                    let (from_bytes, from_content) = to_file_state_diff(file_state_current);
                    let (to_bytes, to_content) = to_file_state_diff(file_state_desired);

                    match (from_bytes == to_bytes, from_content == to_content) {
                        (false, false) | (false, true) | (true, false) => FileStateDiff::Change {
                            byte_len: Changeable::new(from_bytes, to_bytes),
                            contents: Changeable::new(from_content, to_content),
                        },
                        (true, true) => FileStateDiff::NoChangeSync,
                    }
                }
                (FileState::None, FileState::None) => FileStateDiff::NoChangeNonExistent,
            }
        };

        Ok(file_state_diff)
    }
}

fn to_file_state_diff(file_state: &FileState) -> (Tracked<usize>, Tracked<String>) {
    match file_state {
        FileState::None => (Tracked::None, Tracked::None),
        FileState::StringContents { path: _, contents } => (
            Tracked::Known(contents.bytes().len()),
            Tracked::Known(contents.to_owned()),
        ),
        FileState::Length {
            path: _,
            byte_count,
        } => (
            (*byte_count)
                .try_into()
                .map(Tracked::Known)
                .unwrap_or(Tracked::Unknown),
            Tracked::Unknown,
        ),
        FileState::Unknown { .. } => (Tracked::Unknown, Tracked::Unknown),
    }
}
