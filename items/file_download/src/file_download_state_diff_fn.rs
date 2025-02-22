use peace::{
    cfg::{state::FetchedOpt, State},
    diff::{Changeable, Tracked},
};

use crate::{
    FileDownloadError, FileDownloadState, FileDownloadStateDiff, FileDownloadStateLogical,
};

/// Download status diff function.
#[derive(Debug)]
pub struct FileDownloadStateDiffFn;

impl FileDownloadStateDiffFn {
    pub async fn state_diff(
        state_current: &FileDownloadState,
        state_goal: &FileDownloadState,
    ) -> Result<FileDownloadStateDiff, FileDownloadError> {
        let FileDownloadState(State {
            logical: file_state_current,
            physical: e_tag_current,
        }) = state_current;
        let FileDownloadState(State {
            logical: file_state_goal,
            physical: e_tag_goal,
        }) = state_goal;

        let file_state_diff = {
            match (file_state_current, file_state_goal) {
                (
                    FileDownloadStateLogical::StringContents { path, .. }
                    | FileDownloadStateLogical::Length { path, .. }
                    | FileDownloadStateLogical::Unknown { path, .. },
                    FileDownloadStateLogical::None { .. },
                ) => FileDownloadStateDiff::Deleted {
                    path: path.to_path_buf(),
                },

                (
                    file_state_current @ (FileDownloadStateLogical::StringContents { .. }
                    | FileDownloadStateLogical::Length { .. }
                    | FileDownloadStateLogical::Unknown { .. }),
                    file_state_goal @ (FileDownloadStateLogical::StringContents { path, .. }
                    | FileDownloadStateLogical::Length { path, .. }
                    | FileDownloadStateLogical::Unknown { path, .. }),
                )
                | (
                    file_state_current @ FileDownloadStateLogical::None { .. },
                    file_state_goal @ (FileDownloadStateLogical::StringContents { path, .. }
                    | FileDownloadStateLogical::Length { path, .. }
                    | FileDownloadStateLogical::Unknown { path, .. }),
                ) => {
                    let path = path.to_path_buf();
                    let (from_bytes, from_content) = to_file_state_diff(file_state_current);
                    let (to_bytes, to_content) = to_file_state_diff(file_state_goal);

                    match (from_bytes == to_bytes, from_content == to_content) {
                        (_, false) => {
                            // File contents are either changed, or unknown
                            match (e_tag_current, e_tag_goal) {
                                (
                                    FetchedOpt::Value(e_tag_current),
                                    FetchedOpt::Value(e_tag_goal),
                                ) if e_tag_current == e_tag_goal => {
                                    FileDownloadStateDiff::NoChangeSync { path }
                                }
                                _ => FileDownloadStateDiff::Change {
                                    path,
                                    byte_len: Changeable::new(from_bytes, to_bytes),
                                    contents: Changeable::new(from_content, to_content),
                                },
                            }
                        }
                        (false, true) => {
                            // File contents are the same, length is unknown
                            FileDownloadStateDiff::NoChangeSync { path }
                        }
                        (true, true) => FileDownloadStateDiff::NoChangeSync { path },
                    }
                }
                (
                    FileDownloadStateLogical::None { .. },
                    FileDownloadStateLogical::None { path },
                ) => FileDownloadStateDiff::NoChangeNotExists { path: path.clone() },
            }
        };

        Ok(file_state_diff)
    }
}

fn to_file_state_diff(file_state: &FileDownloadStateLogical) -> (Tracked<usize>, Tracked<String>) {
    match file_state {
        FileDownloadStateLogical::None { .. } => (Tracked::None, Tracked::None),
        FileDownloadStateLogical::StringContents { path: _, contents } => (
            Tracked::Known(contents.len()),
            Tracked::Known(contents.to_owned()),
        ),
        FileDownloadStateLogical::Length {
            path: _,
            byte_count,
        } => (
            (*byte_count)
                .try_into()
                .map(Tracked::Known)
                .unwrap_or(Tracked::Unknown),
            Tracked::Unknown,
        ),
        FileDownloadStateLogical::Unknown { .. } => (Tracked::Unknown, Tracked::Unknown),
    }
}
