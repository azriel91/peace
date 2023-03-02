use std::fmt::Debug;

use crate::params::{
    CmdParamsBuilder, FlowParams, KeyMaybe, KeyUnknown, ParamsKeys, ParamsKeysImpl, ProfileParams,
    WorkspaceParams,
};

/// Maps of parameters stored for the workspace, profile, and flow.
#[derive(Debug)]
pub struct CmdParams<PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Workspace parameters type map.
    workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    /// Profile parameters type map.
    profile_params: ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
    /// Flow parameters type map.
    flow_params: FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>,
}

impl CmdParams<ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>> {
    /// Returns a new `CmdParamsBuilder`.
    pub fn builder() -> CmdParamsBuilder<ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>> {
        CmdParamsBuilder::new()
    }
}

impl<PKeys> CmdParams<PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns a new `CmdParams`.
    pub(crate) fn new(
        workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
        profile_params: ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
        flow_params: FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>,
    ) -> Self {
        Self {
            workspace_params,
            profile_params,
            flow_params,
        }
    }

    /// Returns a reference to the workspace params type registry.
    pub fn workspace_params(
        &self,
    ) -> &WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key> {
        &self.workspace_params
    }

    /// Returns a mutable reference to the workspace params type registry.
    pub fn workspace_params_mut(
        &mut self,
    ) -> &mut WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key> {
        &mut self.workspace_params
    }

    /// Returns a reference to the profile params type registry.
    pub fn profile_params(&self) -> &ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key> {
        &self.profile_params
    }

    /// Returns a mutable reference to the profile params type registry.
    pub fn profile_params_mut(
        &mut self,
    ) -> &mut ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key> {
        &mut self.profile_params
    }

    /// Returns a reference to the flow params type registry.
    pub fn flow_params(&self) -> &FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key> {
        &self.flow_params
    }

    /// Returns a mutable reference to the flow params type registry.
    pub fn flow_params_mut(
        &mut self,
    ) -> &mut FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key> {
        &mut self.flow_params
    }
}

impl<PKeys> Default for CmdParams<PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    fn default() -> Self {
        Self {
            workspace_params: WorkspaceParams::default(),
            profile_params: ProfileParams::default(),
            flow_params: FlowParams::default(),
        }
    }
}
