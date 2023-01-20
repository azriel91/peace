use peace_cfg::{FlowId, Profile};
use peace_resources::{
    internal::CmdDirs,
    paths::{FlowDir, PeaceAppDir, ProfileDir, ProfileHistoryDir},
};

/// Computes paths of well-known directories for a command.
#[derive(Debug)]
pub struct CmdDirsBuilder;

impl CmdDirsBuilder {
    /// Computes [`CmdDirs`] paths.
    pub fn build(peace_app_dir: &PeaceAppDir, profile: &Profile, flow_id: &FlowId) -> CmdDirs {
        let profile_dir = ProfileDir::from((peace_app_dir, profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);
        let flow_dir = FlowDir::from((&profile_dir, flow_id));

        CmdDirs::new(profile_dir, profile_history_dir, flow_dir)
    }
}
