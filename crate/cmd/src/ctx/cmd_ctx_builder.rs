use peace_core::{FlowId, Profile};
use peace_resources::paths::{FlowDir, ProfileDir, ProfileHistoryDir};
use peace_rt_model::Workspace;

use crate::{
    ctx::CmdCtx,
    scopes::{NoProfileNoFlow, SingleProfileSingleFlow},
};

pub(crate) use self::{
    flow_id_selection::{FlowIdNotSelected, FlowIdSelected},
    profile_selection::{ProfileFromWorkspaceParam, ProfileNotSelected, ProfileSelected},
    single_profile_single_flow_builder::SingleProfileSingleFlowBuilder,
};

mod flow_id_selection;
mod profile_selection;
mod single_profile_single_flow_builder;

/// Collects parameters and initializes values relevant to the built [`CmdCtx`].
#[derive(Debug)]
pub struct CmdCtxBuilder<'ctx, ScopeBuilder> {
    /// Workspace that the `peace` tool runs in.
    workspace: &'ctx Workspace,
    /// Data held while building `CmdCtx`.
    scope_builder: ScopeBuilder,
}

impl<'ctx> CmdCtxBuilder<'ctx, NoProfileNoFlow> {
    /// Returns a `CmdCtxBuilder` for no profile.
    pub fn new(workspace: &'ctx Workspace) -> Self {
        Self {
            workspace,
            scope_builder: NoProfileNoFlow,
        }
    }

    /// Builds the command context.
    ///
    /// This includes creating directories and deriving values based on the
    /// given parameters.
    pub fn build(self) -> CmdCtx<'ctx, NoProfileNoFlow> {
        let CmdCtxBuilder {
            workspace,
            scope_builder: scope,
        } = self;

        CmdCtx { workspace, scope }
    }
}

impl<'ctx>
    CmdCtxBuilder<'ctx, SingleProfileSingleFlowBuilder<ProfileNotSelected, FlowIdNotSelected>>
{
    /// Returns a `CmdCtxBuilder` for a single profile and flow.
    pub fn single_profile_single_flow(workspace: &'ctx Workspace) -> Self {
        let scope_builder = SingleProfileSingleFlowBuilder {
            profile_selection: ProfileNotSelected,
            flow_id_selection: FlowIdNotSelected,
        };

        Self {
            workspace,
            scope_builder,
        }
    }
}

impl<'ctx, FlowIdSelection>
    CmdCtxBuilder<'ctx, SingleProfileSingleFlowBuilder<ProfileNotSelected, FlowIdSelection>>
{
    pub fn with_profile(
        self,
        profile: Profile,
    ) -> CmdCtxBuilder<'ctx, SingleProfileSingleFlowBuilder<ProfileSelected, FlowIdSelection>> {
        let Self {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection: _,
                    flow_id_selection,
                },
        } = self;

        let scope_builder = SingleProfileSingleFlowBuilder {
            profile_selection: ProfileSelected(profile),
            flow_id_selection,
        };

        CmdCtxBuilder {
            workspace,
            scope_builder,
        }
    }

    pub fn with_profile_from_workspace_param<'key, WorkspaceParamsK>(
        self,
        workspace_param_k: &'key WorkspaceParamsK,
    ) -> CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileFromWorkspaceParam<'key, WorkspaceParamsK>,
            FlowIdSelection,
        >,
    > {
        let Self {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection: _,
                    flow_id_selection,
                },
        } = self;

        let scope_builder = SingleProfileSingleFlowBuilder {
            profile_selection: ProfileFromWorkspaceParam(workspace_param_k),
            flow_id_selection,
        };

        CmdCtxBuilder {
            workspace,
            scope_builder,
        }
    }
}

impl<'ctx, ProfileSelection>
    CmdCtxBuilder<'ctx, SingleProfileSingleFlowBuilder<ProfileSelection, FlowIdNotSelected>>
{
    pub fn with_flow_id(
        self,
        flow_id: FlowId,
    ) -> CmdCtxBuilder<'ctx, SingleProfileSingleFlowBuilder<ProfileSelection, FlowIdSelected>> {
        let Self {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection,
                    flow_id_selection: _,
                },
        } = self;

        let scope_builder = SingleProfileSingleFlowBuilder {
            profile_selection,
            flow_id_selection: FlowIdSelected(flow_id),
        };

        CmdCtxBuilder {
            workspace,
            scope_builder,
        }
    }
}

impl<'ctx> CmdCtxBuilder<'ctx, SingleProfileSingleFlowBuilder<ProfileSelected, FlowIdSelected>> {
    /// Builds the command context.
    ///
    /// This includes creating directories and deriving values based on the
    /// given parameters
    pub fn build(self) -> CmdCtx<'ctx, SingleProfileSingleFlow> {
        let CmdCtxBuilder {
            workspace,
            scope_builder:
                SingleProfileSingleFlowBuilder {
                    profile_selection: ProfileSelected(profile),
                    flow_id_selection: FlowIdSelected(flow_id),
                },
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
