//! Types for well-known directories.
//!
//! # Design
//!
//! The designed structure of directories and their contents is as follows:
//!
//! ```bash
//! WorkspaceDir
//! |- PeaceDir
//!     |- ProfileDir  # "profile_name", multiple
//!         |- HistoryDir
//!         |   |- CmdExecution0
//!         |   |- ..
//!         |   |- CmdExecutionN
//!         |
//!         |- ProfileInit
//!         |
//!         |- FlowDir  # "flow_name", multiple
//!             |- StatesMeta
//!             |- StatesCurrent
//!             |- StatesDesired
//! ```
//!
//! Concrete folder structure example:
//!
//! ```bash
//! workspace
//! |- .peace
//!     |- profile1 / main / default
//!     |   |- .history
//!     |   |   |- 00000000_2022-08-21T20_48_02_init.yaml
//!     |   |   |- 00000001_2022-08-21T20_48_07_dev_env_discover.yaml
//!     |   |   |- 00000002_2022-08-21T20_50_32_dev_env_deploy.yaml  # dry
//!     |   |   |- 00000003_2022-08-21T20_50_43_dev_env_deploy.yaml
//!     |   |   |- 00000004_2022-08-22T08_16_09_dev_env_clean.yaml   # dry
//!     |   |   |- 00000005_2022-08-22T08_16_29_dev_env_clean.yaml
//!     |   |   |- 00000006_2022-08-23T13_02_14_artifact_discover.yaml
//!     |   |   |- 00000007_2022-08-23T13_07_31_artifact_publish.yaml
//!     |   |
//!     |   |- .meta.yaml  # Store the last discovered time so we can inform the user.
//!     |   |              # Should time be stored per item spec, or per invocation?
//!     |   |
//!     |   |- init.yaml  # Parameters used to initialize this profile
//!     |   |             # We write to this so that each time the user re-`init`s,
//!     |   |             # if they version control it, they can rediscover the
//!     |   |             # states from previous inits, and clean them up.
//!     |   |
//!     |   |- dev_env  # flow name
//!     |   |   |- states.yaml
//!     |   |   |- states_desired.yaml
//!     |   |
//!     |   |- artifact
//!     |       |- states.yaml
//!     |       |- states_desired.yaml
//!     |
//!     |- profile2
//!         |- .history
//!         |   |- 00000000_2022-08-21T20_48_02_init.yaml
//!         |   |- 00000001_2022-08-21T20_48_07_discover.yaml
//!         |
//!         |- .meta.yaml
//!         |- init.yaml
//!         |- states.yaml
//!         |- states_desired.yaml
//! ```

pub use self::{
    flow_dir::FlowDir, peace_dir::PeaceDir, profile_dir::ProfileDir,
    profile_history_dir::ProfileHistoryDir, states_current_file::StatesCurrentFile,
    workspace_dir::WorkspaceDir,
};

mod flow_dir;
mod peace_dir;
mod profile_dir;
mod profile_history_dir;
mod states_current_file;
mod workspace_dir;

/// Common impl logic for `PathBuf` newtypes.
///
/// This does not include declaring the type, as it may prevent IDEs from
/// discovering the type declaration, making those types harder to discover.
macro_rules! pathbuf_newtype {
    ($ty_name:ident) => {
        impl $ty_name {
            #[doc = concat!("Returns a new [`", stringify!($ty_name), "`].")]
            pub fn new(path: std::path::PathBuf) -> Self {
                Self(path)
            }

            /// Returns the inner [`PathBuf`].
            ///
            /// [`PathBuf`]: std::path::PathBuf
            pub fn into_inner(self) -> std::path::PathBuf {
                self.0
            }
        }

        impl From<std::path::PathBuf> for $ty_name {
            fn from(path_buf: std::path::PathBuf) -> Self {
                Self(path_buf)
            }
        }

        impl AsRef<std::ffi::OsStr> for $ty_name {
            fn as_ref(&self) -> &std::ffi::OsStr {
                self.0.as_ref()
            }
        }

        impl AsRef<std::path::Path> for $ty_name {
            fn as_ref(&self) -> &std::path::Path {
                &self.0
            }
        }

        impl std::ops::Deref for $ty_name {
            type Target = std::path::Path;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

pub(crate) use pathbuf_newtype;
