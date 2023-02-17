use std::marker::PhantomData;

use peace_core::{FlowId, Profile};
use peace_resources::paths::{FlowDir, ProfileDir, ProfileHistoryDir};
use peace_rt_model::Workspace;

use crate::{
    ctx::CmdCtx,
    scopes::{NoProfileNoFlow, SingleProfileSingleFlow},
};

/// Collects parameters and initializes values relevant to the built [`CmdCtx`].
#[derive(Debug)]
pub struct CmdCtxBuilder<'ctx, Scope> {
    /// Workspace that the `peace` tool runs in.
    workspace: &'ctx Workspace,
    /// Marker.
    marker: PhantomData<Scope>,
}

impl<'ctx> CmdCtxBuilder<'ctx, NoProfileNoFlow> {
    /// Returns a `CmdCtxBuilder` for no profile.
    pub fn new(workspace: &'ctx Workspace) -> Self {
        Self {
            workspace,
            marker: PhantomData,
        }
    }

    /// Builds the command context.
    ///
    /// This includes creating directories and deriving values based on the
    /// given parameters.
    pub fn build(self) -> CmdCtx<'ctx, NoProfileNoFlow> {
        let CmdCtxBuilder {
            workspace,
            marker: _,
        } = self;

        let scope = NoProfileNoFlow;

        CmdCtx { workspace, scope }
    }
}

impl<'ctx> CmdCtxBuilder<'ctx, SingleProfileSingleFlow> {
    /// Returns a `CmdCtxBuilder` for a single profile and flow.
    pub fn new(workspace: &'ctx Workspace) -> Self {
        Self {
            workspace,
            marker: PhantomData,
        }
    }

    /// Builds the command context.
    ///
    /// This includes creating directories and deriving values based on the
    /// given parameters
    pub fn build(self, profile: Profile, flow_id: FlowId) -> CmdCtx<'ctx, SingleProfileSingleFlow> {
        let CmdCtxBuilder {
            workspace,
            marker: _,
        } = self;
        let peace_app_dir = workspace.dirs().peace_app_dir();

        let profile_dir = ProfileDir::from((peace_app_dir, &profile));
        let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

        let flow_dir = FlowDir::from((&profile_dir, &flow_id));

        let scope = SingleProfileSingleFlow::new(
            profile,
            profile_dir,
            profile_history_dir,
            flow_id,
            flow_dir,
        );

        CmdCtx { workspace, scope }
    }
}
