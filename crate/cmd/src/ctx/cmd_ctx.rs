#![allow(clippy::type_complexity)]

use std::ops::Deref;

use peace_resources::paths::{PeaceAppDir, PeaceDir, WorkspaceDir};
use peace_rt_model::{
    cmd_context_params::{KeyUnknown, ParamsKeys, ParamsKeysImpl, ParamsTypeRegs},
    Workspace,
};

use crate::{
    ctx::{
        cmd_ctx_builder::{
            MultiProfileNoFlowBuilder, MultiProfileSingleFlowBuilder, NoProfileNoFlowBuilder,
            SingleProfileNoFlowBuilder, SingleProfileSingleFlowBuilder,
        },
        CmdCtxBuilder,
    },
    scopes::{
        type_params::{
            FlowNotSelected, FlowParamsNone, ProfileNotSelected, ProfileParamsNone,
            WorkspaceParamsNone,
        },
        MultiProfileSingleFlow, NoProfileNoFlow, SingleProfileSingleFlow,
    },
};

/// Collects parameters and initializes values relevant to the built [`CmdCtx`].
#[derive(Debug)]
pub struct CmdCtx<'ctx, Scope, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Workspace that the `peace` tool runs in.
    pub(crate) workspace: &'ctx Workspace,
    /// Scope of the command.
    pub(crate) scope: Scope,
    /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
    /// [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: crate::cmd_context_params::WorkspaceParams
    /// [`ProfileParams`]: crate::cmd_context_params::ProfileParams
    /// [`FlowParams`]: crate::cmd_context_params::FlowParams
    pub(crate) params_type_regs: ParamsTypeRegs<PKeys>,
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

    /// Returns the type registries for [`WorkspaceParams`], [`ProfileParams`],
    /// and [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: crate::cmd_context_params::WorkspaceParams
    /// [`ProfileParams`]: crate::cmd_context_params::ProfileParams
    /// [`FlowParams`]: crate::cmd_context_params::FlowParams
    pub fn params_type_regs(&self) -> &ParamsTypeRegs<PKeys> {
        &self.params_type_regs
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
}

impl<'ctx>
    CmdCtx<
        'ctx,
        NoProfileNoFlow<ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>>,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    >
{
    /// Returns a `CmdCtxBuilder` for a single profile and no flow.
    pub fn builder_no_profile_no_flow(
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        NoProfileNoFlowBuilder<WorkspaceParamsNone>,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    > {
        CmdCtxBuilder::no_profile_no_flow(workspace)
    }
}

impl<'ctx>
    CmdCtx<
        'ctx,
        MultiProfileSingleFlow<ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>>,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    >
{
    /// Returns a `CmdCtxBuilder` for multiple profiles and no flow.
    pub fn builder_multi_profile_no_flow(
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        MultiProfileNoFlowBuilder<ProfileNotSelected, WorkspaceParamsNone, ProfileParamsNone>,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    > {
        CmdCtxBuilder::multi_profile_no_flow(workspace)
    }
}

impl<'ctx>
    CmdCtx<
        'ctx,
        MultiProfileSingleFlow<ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>>,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    >
{
    /// Returns a `CmdCtxBuilder` for multiple profiles and one flow.
    pub fn builder_multi_profile_single_flow(
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        MultiProfileSingleFlowBuilder<
            ProfileNotSelected,
            FlowNotSelected,
            WorkspaceParamsNone,
            ProfileParamsNone,
            FlowParamsNone,
        >,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    > {
        CmdCtxBuilder::multi_profile_single_flow(workspace)
    }
}

impl<'ctx>
    CmdCtx<
        'ctx,
        SingleProfileSingleFlow<ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>>,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    >
{
    /// Returns a `CmdCtxBuilder` for a single profile and flow.
    pub fn builder_single_profile_no_flow(
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        SingleProfileNoFlowBuilder<ProfileNotSelected, WorkspaceParamsNone, ProfileParamsNone>,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    > {
        CmdCtxBuilder::single_profile_no_flow(workspace)
    }
}

impl<'ctx>
    CmdCtx<
        'ctx,
        SingleProfileSingleFlow<ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>>,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    >
{
    /// Returns a `CmdCtxBuilder` for a single profile and flow.
    pub fn builder_single_profile_single_flow(
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        SingleProfileSingleFlowBuilder<
            ProfileNotSelected,
            FlowNotSelected,
            WorkspaceParamsNone,
            ProfileParamsNone,
            FlowParamsNone,
        >,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    > {
        CmdCtxBuilder::single_profile_single_flow(workspace)
    }
}

impl<'ctx, Scope, PKeys> Deref for CmdCtx<'ctx, Scope, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    type Target = Scope;

    fn deref(&self) -> &Self::Target {
        &self.scope
    }
}
