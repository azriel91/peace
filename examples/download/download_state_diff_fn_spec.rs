use std::path::PathBuf;

use diff::{Diff, OptionDiff};
#[nougat::gat(Data)]
use peace::cfg::StateDiffFnSpec;
use peace::cfg::{async_trait, nougat, State};

use crate::{DownloadError, FileState};

/// Download status diff function.
#[derive(Debug)]
pub struct DownloadStateDiffFnSpec;

#[async_trait]
#[nougat::gat]
impl StateDiffFnSpec for DownloadStateDiffFnSpec {
    type Data<'op> = &'op()
        where Self: 'op;
    type Error = DownloadError;
    type StateDiff = OptionDiff<FileState>;
    type StateLogical = Option<FileState>;
    type StatePhysical = PathBuf;

    async fn exec(
        _: &(),
        state_current: &State<Option<FileState>, PathBuf>,
        state_desired: &Option<FileState>,
    ) -> Result<Self::StateDiff, DownloadError> {
        Ok(state_current.logical.diff(&state_desired))
    }
}
