#![allow(clippy::type_complexity)]

use std::ops::Deref;

use peace_resources::{
    paths::{PeaceAppDir, PeaceDir, WorkspaceDir},
    resources::ts::SetUp,
};
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
        SingleProfileSingleFlow,
    },
};

/// Information needed to execute a command.
///
/// Importantly, as commands have different purposes, different command scopes
/// exist to cater for each kind of command. This means the data available in a
/// command context differs per scope, to accurately reflect what is available.
#[derive(Debug)]
pub struct CmdCtx<'ctx, O, Scope, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    pub(crate) output: &'ctx mut O,
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

/// Information needed to execute a command.
///
/// Importantly, as commands have different purposes, different command scopes
/// exist to cater for each kind of command. This means the data available in a
/// command context differs per scope, to accurately reflect what is available.
#[derive(Debug)]
pub struct CmdCtxView<'view: 'ctx, 'ctx, O, Scope, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    pub output: &'ctx mut O,
    /// Workspace that the `peace` tool runs in.
    pub workspace: &'ctx Workspace,
    /// Scope of the command.
    pub scope: &'view Scope,
    /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
    /// [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: crate::cmd_context_params::WorkspaceParams
    /// [`ProfileParams`]: crate::cmd_context_params::ProfileParams
    /// [`FlowParams`]: crate::cmd_context_params::FlowParams
    pub params_type_regs: &'view ParamsTypeRegs<PKeys>,
}

impl<'ctx, O, Scope, PKeys> CmdCtx<'ctx, O, Scope, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns a view struct of this command context.
    ///
    /// This allows the output and scope data to be borrowed concurrently.
    pub fn view<'view>(&'view mut self) -> CmdCtxView<'view, 'ctx, O, Scope, PKeys> {
        let Self {
            output,
            workspace,
            scope,
            params_type_regs,
        } = self;

        CmdCtxView {
            output,
            workspace,
            scope,
            params_type_regs,
        }
    }

    /// Returns a reference to the output.
    pub fn output(&self) -> &O {
        self.output
    }

    /// Returns a mutable reference to the output.
    pub fn output_mut(&mut self) -> &mut O {
        self.output
    }

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

impl<'ctx, O> CmdCtx<'ctx, O, (), ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>> {
    /// Returns a `CmdCtxBuilder` for a single profile and no flow.
    pub fn builder_no_profile_no_flow<E>(
        output: &'ctx mut O,
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        'ctx,
        O,
        NoProfileNoFlowBuilder<E, WorkspaceParamsNone>,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    > {
        CmdCtxBuilder::no_profile_no_flow(output, workspace)
    }

    /// Returns a `CmdCtxBuilder` for multiple profiles and no flow.
    pub fn builder_multi_profile_no_flow<E>(
        output: &'ctx mut O,
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        'ctx,
        O,
        MultiProfileNoFlowBuilder<E, ProfileNotSelected, WorkspaceParamsNone, ProfileParamsNone>,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    > {
        CmdCtxBuilder::multi_profile_no_flow(output, workspace)
    }

    /// Returns a `CmdCtxBuilder` for multiple profiles and one flow.
    pub fn builder_multi_profile_single_flow<E>(
        output: &'ctx mut O,
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        'ctx,
        O,
        MultiProfileSingleFlowBuilder<
            E,
            ProfileNotSelected,
            FlowNotSelected,
            WorkspaceParamsNone,
            ProfileParamsNone,
            FlowParamsNone,
        >,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    > {
        CmdCtxBuilder::multi_profile_single_flow(output, workspace)
    }

    /// Returns a `CmdCtxBuilder` for a single profile and flow.
    pub fn builder_single_profile_no_flow<E>(
        output: &'ctx mut O,
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        'ctx,
        O,
        SingleProfileNoFlowBuilder<E, ProfileNotSelected, WorkspaceParamsNone, ProfileParamsNone>,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    > {
        CmdCtxBuilder::single_profile_no_flow(output, workspace)
    }
}

impl<'ctx, E, O>
    CmdCtx<
        'ctx,
        O,
        SingleProfileSingleFlow<E, ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>, SetUp>,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    >
{
    /// Returns a `CmdCtxBuilder` for a single profile and flow.
    pub fn builder_single_profile_single_flow(
        output: &'ctx mut O,
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        'ctx,
        O,
        SingleProfileSingleFlowBuilder<
            E,
            ProfileNotSelected,
            FlowNotSelected,
            WorkspaceParamsNone,
            ProfileParamsNone,
            FlowParamsNone,
        >,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    > {
        CmdCtxBuilder::single_profile_single_flow(output, workspace)
    }
}

impl<'ctx, O, Scope, PKeys> Deref for CmdCtx<'ctx, O, Scope, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    type Target = Scope;

    fn deref(&self) -> &Self::Target {
        &self.scope
    }
}
