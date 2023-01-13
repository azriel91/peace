use peace::{
    cfg::{async_trait, StateDiffFnSpec},
    diff::{Changeable, Tracked},
};

use crate::{FileDownloadError, FileDownloadState, FileDownloadStateDiff};

/// Download status diff function.
#[derive(Debug)]
pub struct FileDownloadStateDiffFnSpec;

#[async_trait(?Send)]
impl StateDiffFnSpec for FileDownloadStateDiffFnSpec {
    type Data<'op> = &'op ();
    type Error = FileDownloadError;
    type State = FileDownloadState;
    type StateDiff = FileDownloadStateDiff;

    async fn exec(
        _: &(),
        file_state_current: &FileDownloadState,
        file_state_desired: &FileDownloadState,
    ) -> Result<Self::StateDiff, FileDownloadError> {
        let file_state_diff = {
            match (file_state_current, file_state_desired) {
                (
                    FileDownloadState::StringContents { path, .. }
                    | FileDownloadState::Length { path, .. }
                    | FileDownloadState::Unknown { path, .. },
                    FileDownloadState::None { .. },
                ) => FileDownloadStateDiff::Deleted {
                    path: path.to_path_buf(),
                },

                (
                    file_state_current @ (FileDownloadState::StringContents { .. }
                    | FileDownloadState::Length { .. }
                    | FileDownloadState::Unknown { .. }),
                    file_state_desired @ (FileDownloadState::StringContents { path, .. }
                    | FileDownloadState::Length { path, .. }
                    | FileDownloadState::Unknown { path, .. }),
                )
                | (
                    file_state_current @ FileDownloadState::None { .. },
                    file_state_desired @ (FileDownloadState::StringContents { path, .. }
                    | FileDownloadState::Length { path, .. }
                    | FileDownloadState::Unknown { path, .. }),
                ) => {
                    let path = path.to_path_buf();
                    let (from_bytes, from_content) = to_file_state_diff(file_state_current);
                    let (to_bytes, to_content) = to_file_state_diff(file_state_desired);

                    match (from_bytes == to_bytes, from_content == to_content) {
                        (false, false) | (false, true) | (true, false) => {
                            FileDownloadStateDiff::Change {
                                path,
                                byte_len: Changeable::new(from_bytes, to_bytes),
                                contents: Changeable::new(from_content, to_content),
                            }
                        }
                        (true, true) => FileDownloadStateDiff::NoChangeSync { path },
                    }
                }
                (FileDownloadState::None { .. }, FileDownloadState::None { path }) => {
                    FileDownloadStateDiff::NoChangeNotExists {
                        path: path.to_path_buf(),
                    }
                }
            }
        };

        Ok(file_state_diff)
    }
}

fn to_file_state_diff(file_state: &FileDownloadState) -> (Tracked<usize>, Tracked<String>) {
    match file_state {
        FileDownloadState::None { .. } => (Tracked::None, Tracked::None),
        FileDownloadState::StringContents { path: _, contents } => (
            Tracked::Known(contents.bytes().len()),
            Tracked::Known(contents.to_owned()),
        ),
        FileDownloadState::Length {
            path: _,
            byte_count,
        } => (
            (*byte_count)
                .try_into()
                .map(Tracked::Known)
                .unwrap_or(Tracked::Unknown),
            Tracked::Unknown,
        ),
        FileDownloadState::Unknown { .. } => (Tracked::Unknown, Tracked::Unknown),
    }
}
