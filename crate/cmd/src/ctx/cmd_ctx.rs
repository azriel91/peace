use peace_core::{FlowId, Profile};
use peace_resources::paths::{FlowDir, PeaceAppDir, PeaceDir, ProfileDir, WorkspaceDir};
use peace_rt_model::{
    cmd_context_params::{KeyUnknown, ParamsKeys, ParamsKeysImpl, ParamsTypeRegs},
    Workspace,
};

use crate::{
    ctx::CmdCtxBuilder,
    scopes::{NoProfileNoFlow, SingleProfileSingleFlow},
};

use super::cmd_ctx_builder::{
    FlowIdNotSelected, ProfileNotSelected, SingleProfileSingleFlowBuilder, WorkspaceParamsNone,
};

/// Collects parameters and initializes values relevant to the built [`CmdCtx`].
#[derive(Debug)]
pub struct CmdCtx<'ctx, Scope, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Workspace that the `peace` tool runs in.
    pub workspace: &'ctx Workspace,
    /// Scope of the command.
    pub scope: Scope,
    /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
    /// [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: crate::cmd_context_params::WorkspaceParams
    /// [`ProfileParams`]: crate::cmd_context_params::ProfileParams
    /// [`FlowParams`]: crate::cmd_context_params::FlowParams
    pub params_type_regs: ParamsTypeRegs<PKeys>,
}

impl<'ctx, Scope, PKeys> CmdCtx<'ctx, Scope, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns the workspace that the `peace` tool runs in.
    pub fn workspace(&self) -> &Workspace {
        self.workspace
    }

    /// Returns the scope of the command.
    pub fn scope(&self) -> &Scope {
        &self.scope
    }
}

impl<'ctx> CmdCtx<'ctx, NoProfileNoFlow, ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>> {
    /// Returns a `CmdCtxBuilder` for a single profile and flow.
    pub fn builder_no_profile_no_flow(
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<NoProfileNoFlow, ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>> {
        CmdCtxBuilder::no_profile_no_flow(workspace)
    }
}

impl<'ctx>
    CmdCtx<'ctx, SingleProfileSingleFlow, ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>>
{
    /// Returns a `CmdCtxBuilder` for a single profile and flow.
    pub fn builder_single_profile_single_flow(
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        SingleProfileSingleFlowBuilder<ProfileNotSelected, FlowIdNotSelected, WorkspaceParamsNone>,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    > {
        CmdCtxBuilder::single_profile_single_flow(workspace)
    }
}

impl<'ctx, PKeys> CmdCtx<'ctx, SingleProfileSingleFlow, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns a reference to the workspace directory.
    pub fn workspace_dir(&self) -> &WorkspaceDir {
        self.workspace.dirs().workspace_dir()
    }

    /// Returns a reference to the `.peace` directory.
    pub fn peace_dir(&self) -> &PeaceDir {
        self.workspace.dirs().peace_dir()
    }

    /// Returns a reference to the `.peace/$app` directory.
    pub fn peace_app_dir(&self) -> &PeaceAppDir {
        self.workspace.dirs().peace_app_dir()
    }

    /// Returns the profile this command is for.
    pub fn profile(&self) -> &Profile {
        self.scope.profile()
    }

    /// Returns the profile directory.
    pub fn profile_dir(&self) -> &ProfileDir {
        self.scope.profile_dir()
    }

    /// Returns the flow ID.
    pub fn flow_id(&self) -> &FlowId {
        self.scope.flow_id()
    }

    /// Returns the flow directory.
    pub fn flow_dir(&self) -> &FlowDir {
        self.scope.flow_dir()
    }
}
