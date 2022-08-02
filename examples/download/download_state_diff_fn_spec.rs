use std::path::PathBuf;

#[nougat::gat(Data)]
use peace::cfg::StateDiffFnSpec;
use peace::{
    cfg::{async_trait, nougat, State},
    diff::{Changeable, Tracked},
};

use crate::{DownloadError, FileState, FileStateDiff};

/// Download status diff function.
#[derive(Debug)]
pub struct DownloadStateDiffFnSpec;

#[async_trait]
#[nougat::gat]
impl StateDiffFnSpec for DownloadStateDiffFnSpec {
    type Data<'op> = &'op()
        where Self: 'op;
    type Error = DownloadError;
    type StateDiff = FileStateDiff;
    type StateLogical = Option<FileState>;
    type StatePhysical = PathBuf;

    async fn exec(
        _: &(),
        state_current: &State<Option<FileState>, PathBuf>,
        file_state_desired: &Option<FileState>,
    ) -> Result<Self::StateDiff, DownloadError> {
        let file_state_diff = {
            let file_state_current = &state_current.logical;
            match (file_state_current.as_ref(), file_state_desired.as_ref()) {
                (Some(_file_state_current), None) => FileStateDiff::Deleted,

                (file_state_current @ Some(_), file_state_desired @ Some(_))
                | (file_state_current @ None, file_state_desired @ Some(_)) => {
                    let (from_bytes, from_content) = file_state_current
                        .map(to_file_state_diff)
                        .unwrap_or((Tracked::None, Tracked::None));

                    let (to_bytes, to_content) = file_state_desired
                        .map(to_file_state_diff)
                        .unwrap_or((Tracked::None, Tracked::None));

                    FileStateDiff::Change {
                        byte_len: Changeable::new(from_bytes, to_bytes),
                        contents: Changeable::new(from_content, to_content),
                    }
                }
                (None, None) => FileStateDiff::NoChangeNonExistent,
            }
        };

        Ok(file_state_diff)
    }
}

fn to_file_state_diff(file_state: &FileState) -> (Tracked<usize>, Tracked<String>) {
    match file_state {
        FileState::StringContents(s) => (
            Tracked::Known(s.bytes().len()),
            Tracked::Known(s.to_owned()),
        ),
        FileState::Length(len) => (
            (*len)
                .try_into()
                .map(Tracked::Known)
                .unwrap_or(Tracked::Unknown),
            Tracked::Unknown,
        ),
        FileState::Unknown => (Tracked::Unknown, Tracked::Unknown),
    }
}
