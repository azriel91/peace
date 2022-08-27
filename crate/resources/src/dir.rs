//! Types for well-known directories.
//!
//! # Design
//!
//! The designed structure of directories and their contents is as follows:
//!
//! ```bash
//! WorkspaceDir
//! |- PeaceDir
//!     |- ProfileDir # ("profile_name")
//!         |- HistoryDir
//!         |   |- CmdExecution0
//!         |   |- ..
//!         |   |- CmdExecutionN
//!         |
//!         |- StatesMeta
//!         |- StatesCurrent
//!         |- StatesDesired
//!         |- ProfileInit
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
//!     |   |   |- 00000001_2022-08-21T20_48_07_fetch.yaml
//!     |   |   |- 00000002_2022-08-21T20_50_32_ensure_dry.yaml
//!     |   |   |- 00000003_2022-08-21T20_50_43_ensure.yaml
//!     |   |   |- 00000004_2022-08-22T08_16_09_clean_dry.yaml
//!     |   |   |- 00000005_2022-08-22T08_16_29_clean.yaml
//!     |   |
//!     |   |- .meta.yaml  # Store the last fetched time so we can inform the user.
//!     |   |              # Should time be stored per item spec, or per invocation?
//!     |   |
//!     |   |- init.yaml  # Parameters used to initialize this profile
//!     |   |             # We write to this so that each time the user re-`init`s,
//!     |   |             # if they version control it, they can rediscover the
//!     |   |             # states from previous inits, and clean them up.
//!     |   |
//!     |   |- states.yaml
//!     |   |- states_desired.yaml
//!     |
//!     |- profile2
//!         |- .history
//!         |   |- 00000000_2022-08-21T20_48_02_init.yaml
//!         |   |- 00000001_2022-08-21T20_48_07_fetch.yaml
//!         |
//!         |- .meta.yaml
//!         |- init.yaml
//!         |- states.yaml
//!         |- states_desired.yaml
//! ```

pub use self::{peace_dir::PeaceDir, profile_dir::ProfileDir, workspace_dir::WorkspaceDir};

mod peace_dir;
mod profile_dir;
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
