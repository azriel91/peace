use peace_core::{FlowId, Profile};
use peace_resources::paths::{FlowDir, PeaceAppDir, PeaceDir, ProfileDir, WorkspaceDir};
use peace_rt_model::Workspace;

use crate::{
    ctx::CmdCtxBuilder,
    scopes::{NoProfileNoFlow, SingleProfileSingleFlow},
};

/// Collects parameters and initializes values relevant to the built [`CmdCtx`].
#[derive(Debug)]
pub struct CmdCtx<'ctx, Scope> {
    /// Workspace that the `peace` tool runs in.
    pub(crate) workspace: &'ctx Workspace,
    /// Scope of the command.
    pub(crate) scope: Scope,
}

impl<'ctx, Scope> CmdCtx<'ctx, Scope> {
    /// Returns the workspace that the `peace` tool runs in.
    pub fn workspace(&self) -> &Workspace {
        self.workspace
    }

    /// Returns the scope of the command.
    pub fn scope(&self) -> &Scope {
        &self.scope
    }
}

impl<'ctx> CmdCtx<'ctx, NoProfileNoFlow> {
    /// Returns a `CmdCtxBuilder` for a single profile and flow.
    pub fn builder(workspace: &'ctx Workspace) -> CmdCtxBuilder<NoProfileNoFlow> {
        CmdCtxBuilder::<NoProfileNoFlow>::new(workspace)
    }
}

impl<'ctx> CmdCtx<'ctx, SingleProfileSingleFlow> {
    /// Returns a `CmdCtxBuilder` for a single profile and flow.
    pub fn builder(workspace: &'ctx Workspace) -> CmdCtxBuilder<SingleProfileSingleFlow> {
        CmdCtxBuilder::<SingleProfileSingleFlow>::new(workspace)
    }

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
