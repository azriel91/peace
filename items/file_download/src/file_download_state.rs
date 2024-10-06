use std::fmt;

use peace::cfg::{state::FetchedOpt, State};
use serde::{Deserialize, Serialize};

use crate::{ETag, FileDownloadStatePhysical};

#[cfg(feature = "output_progress")]
use peace::item_model::ItemLocationState;

/// Newtype wrapper for `State<FileDownloadStatePhysical, FetchedOpt<ETag>>`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileDownloadState(pub State<FileDownloadStatePhysical, FetchedOpt<ETag>>);

impl FileDownloadState {
    /// Returns a new `FileDownloadState`.
    pub fn new(
        file_download_state_physical: FileDownloadStatePhysical,
        etag: FetchedOpt<ETag>,
    ) -> Self {
        Self(State::new(file_download_state_physical, etag))
    }
}

impl From<State<FileDownloadStatePhysical, FetchedOpt<ETag>>> for FileDownloadState {
    fn from(state: State<FileDownloadStatePhysical, FetchedOpt<ETag>>) -> Self {
        Self(state)
    }
}

impl fmt::Display for FileDownloadState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "output_progress")]
impl<'state> From<&'state FileDownloadState> for ItemLocationState {
    fn from(state: &'state FileDownloadState) -> ItemLocationState {
        match &state.0.logical {
            FileDownloadStatePhysical::None { .. } => ItemLocationState::NotExists,
            FileDownloadStatePhysical::StringContents { .. }
            | FileDownloadStatePhysical::Length { .. }
            | FileDownloadStatePhysical::Unknown { .. } => ItemLocationState::Exists,
        }
    }
}
