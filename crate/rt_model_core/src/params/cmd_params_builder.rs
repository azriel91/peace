use crate::params::{FlowParams, ProfileParams, WorkspaceParams};
use std::{fmt::Debug, hash::Hash};

use serde::{de::DeserializeOwned, Serialize};

use crate::params::{CmdParams, KeyKnown, KeyMaybe, KeyUnknown, ParamsKeys, ParamsKeysImpl};

/// Maps of parameters stored for the workspace, profile, and flow.
#[derive(Debug)]
pub struct CmdParamsBuilder<PKeys>
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

impl CmdParamsBuilder<ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>> {
    /// Returns a new `CmdParamsBuilder`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<PKeys> CmdParamsBuilder<PKeys>
where
    PKeys: ParamsKeys + 'static,
{
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

    /// Returns a `CmdParams` with the registered keys.
    pub fn build(
        self,
    ) -> CmdParams<
        ParamsKeysImpl<
            PKeys::WorkspaceParamsKMaybe,
            PKeys::ProfileParamsKMaybe,
            PKeys::FlowParamsKMaybe,
        >,
    > {
        let Self {
            workspace_params,
            profile_params,
            flow_params,
        } = self;

        CmdParams::new(workspace_params, profile_params, flow_params)
    }
}

impl<ProfileParamsKMaybe, FlowParamsKMaybe>
    CmdParamsBuilder<ParamsKeysImpl<KeyUnknown, ProfileParamsKMaybe, FlowParamsKMaybe>>
where
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    pub fn with_workspace_params<WorkspaceParamsK>(
        self,
        workspace_params: WorkspaceParams<WorkspaceParamsK>,
    ) -> CmdParamsBuilder<
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
    >
    where
        WorkspaceParamsK:
            Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        let CmdParamsBuilder {
            workspace_params: _,
            profile_params,
            flow_params,
        } = self;

        CmdParamsBuilder {
            workspace_params,
            profile_params,
            flow_params,
        }
    }
}

impl<WorkspaceParamsKMaybe, FlowParamsKMaybe>
    CmdParamsBuilder<ParamsKeysImpl<WorkspaceParamsKMaybe, KeyUnknown, FlowParamsKMaybe>>
where
    WorkspaceParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    pub fn with_profile_params<ProfileParamsK>(
        self,
        profile_params: ProfileParams<ProfileParamsK>,
    ) -> CmdParamsBuilder<
        ParamsKeysImpl<WorkspaceParamsKMaybe, KeyKnown<ProfileParamsK>, FlowParamsKMaybe>,
    >
    where
        ProfileParamsK:
            Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        let CmdParamsBuilder {
            workspace_params,
            profile_params: _,
            flow_params,
        } = self;

        CmdParamsBuilder {
            workspace_params,
            profile_params,
            flow_params,
        }
    }
}

impl<WorkspaceParamsKMaybe, ProfileParamsKMaybe>
    CmdParamsBuilder<ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, KeyUnknown>>
where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
{
    pub fn with_flow_params<FlowParamsK>(
        self,
        flow_params: FlowParams<FlowParamsK>,
    ) -> CmdParamsBuilder<
        ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, KeyKnown<FlowParamsK>>,
    >
    where
        FlowParamsK:
            Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        let CmdParamsBuilder {
            workspace_params,
            profile_params,
            flow_params: _,
        } = self;

        CmdParamsBuilder {
            workspace_params,
            profile_params,
            flow_params,
        }
    }
}

impl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsKMaybe> Default
    for CmdParamsBuilder<
        ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsKMaybe>,
    >
where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    fn default() -> Self {
        let workspace_params = WorkspaceParams::<WorkspaceParamsKMaybe::Key>::new();
        let profile_params = ProfileParams::<ProfileParamsKMaybe::Key>::new();
        let flow_params = FlowParams::<FlowParamsKMaybe::Key>::new();

        CmdParamsBuilder {
            workspace_params,
            profile_params,
            flow_params,
        }
    }
}
