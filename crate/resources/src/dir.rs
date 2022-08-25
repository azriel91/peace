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
//!         |- StatesDir
//!         |   |- StatesCurrentDir  # `States` split per item_spec
//!         |   |   |- StatesCurrentMeta
//!         |   |   |- State0
//!         |   |   |- ..
//!         |   |   |- StateN
//!         |   |
//!         |   |- StatesDesiredDir  # `StatesDesired` split per item_spec
//!         |       |- StatesDesiredMeta
//!         |       |- StateLogical0
//!         |       |- ..
//!         |       |- StateLogicalN
//!         |
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
//!     |   |- states
//!     |   |   |- current
//!     |   |   |   |  # Also store the last fetched time so we can inform the user.
//!     |   |   |   |  # Should time be stored per item spec, or per invocation?
//!     |   |   |   |- .meta.yaml
//!     |   |   |   |- item_spec_a.yaml
//!     |   |   |   |- item_spec_b.yaml
//!     |   |   |   |- item_spec_c.yaml
//!     |   |   |
//!     |   |   |- desired
//!     |   |       |- .meta.yaml
//!     |   |       |- item_spec_a.yaml
//!     |   |       |- item_spec_b.yaml
//!     |   |       |- item_spec_c.yaml
//!     |   |
//!     |   |- init.yaml  # Parameters used to initialize this profile
//!     |                 # We write to this so that each time the user re-`init`s,
//!     |                 # if they version control it, they can rediscover the
//!     |                 # states from previous inits, and clean them up.
//!     |
//!     |- profile2
//!         |- .history
//!         |   |- 00000000_2022-08-21T20_48_02_init.yaml
//!         |   |- 00000001_2022-08-21T20_48_07_fetch.yaml
//!         |
//!         |- states
//!         |   |- current
//!         |   |   |- .meta.yaml
//!         |   |   |- item_spec_a.yaml
//!         |   |   |- item_spec_b.yaml
//!         |   |   |- item_spec_c.yaml
//!         |   |
//!         |   |- desired
//!         |       |- .meta.yaml
//!         |       |- item_spec_a.yaml
//!         |       |- item_spec_b.yaml
//!         |       |- item_spec_c.yaml
//!         |
//!         |- init.yaml
//! ```

pub use self::{peace_dir::PeaceDir, profile_dir::ProfileDir, workspace_dir::WorkspaceDir};

mod peace_dir;
mod profile_dir;
mod workspace_dir;
