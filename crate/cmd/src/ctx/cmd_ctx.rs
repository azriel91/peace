#![allow(clippy::type_complexity)]

use std::ops::{Deref, DerefMut};

use peace_resources::Resources;
use peace_rt_model::Workspace;

use crate::{
    ctx::{
        cmd_ctx_builder::{
            MultiProfileNoFlowBuilder, MultiProfileSingleFlowBuilder, NoProfileNoFlowBuilder,
            SingleProfileNoFlowBuilder, SingleProfileSingleFlowBuilder,
        },
        CmdCtxBuilder, CmdCtxTypeParams,
    },
    scopes::SingleProfileSingleFlow,
};

use super::CmdCtxTypeParamsCollectorEmpty;

/// Information needed to execute a command.
///
/// Importantly, as commands have different purposes, different command scopes
/// exist to cater for each kind of command. This means the data available in a
/// command context differs per scope, to accurately reflect what is available.
#[derive(Debug)]
pub struct CmdCtx<Scope> {
    /// Scope of the command.
    pub(crate) scope: Scope,
}

impl<Scope> CmdCtx<Scope> {
    /// Returns the scope of the command.
    pub fn scope(&self) -> &Scope {
        &self.scope
    }

    /// Returns a mutable reference to the scope of the command.
    pub fn scope_mut(&mut self) -> &mut Scope {
        &mut self.scope
    }
}

impl CmdCtx<()> {
    /// Returns a `CmdCtxBuilder` for a single profile and no flow.
    pub fn builder_no_profile_no_flow<'ctx, AppError, Output>(
        output: &'ctx mut Output,
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        'ctx,
        CmdCtxTypeParamsCollectorEmpty<AppError, Output>,
        NoProfileNoFlowBuilder<CmdCtxTypeParamsCollectorEmpty<AppError, Output>>,
    > {
        CmdCtxBuilder::no_profile_no_flow(output, workspace)
    }

    /// Returns a `CmdCtxBuilder` for multiple profiles and no flow.
    pub fn builder_multi_profile_no_flow<'ctx, AppError, Output>(
        output: &'ctx mut Output,
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        'ctx,
        CmdCtxTypeParamsCollectorEmpty<AppError, Output>,
        MultiProfileNoFlowBuilder<CmdCtxTypeParamsCollectorEmpty<AppError, Output>>,
    > {
        CmdCtxBuilder::multi_profile_no_flow(output, workspace)
    }

    /// Returns a `CmdCtxBuilder` for multiple profiles and one flow.
    pub fn builder_multi_profile_single_flow<'ctx, AppError, Output>(
        output: &'ctx mut Output,
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        'ctx,
        CmdCtxTypeParamsCollectorEmpty<AppError, Output>,
        MultiProfileSingleFlowBuilder<CmdCtxTypeParamsCollectorEmpty<AppError, Output>>,
    > {
        CmdCtxBuilder::multi_profile_single_flow(output, workspace)
    }

    /// Returns a `CmdCtxBuilder` for a single profile and flow.
    pub fn builder_single_profile_no_flow<'ctx, AppError, Output>(
        output: &'ctx mut Output,
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        'ctx,
        CmdCtxTypeParamsCollectorEmpty<AppError, Output>,
        SingleProfileNoFlowBuilder<CmdCtxTypeParamsCollectorEmpty<AppError, Output>>,
    > {
        CmdCtxBuilder::single_profile_no_flow(output, workspace)
    }

    /// Returns a `CmdCtxBuilder` for a single profile and flow.
    pub fn builder_single_profile_single_flow<'ctx, AppError, Output>(
        output: &'ctx mut Output,
        workspace: &'ctx Workspace,
    ) -> CmdCtxBuilder<
        'ctx,
        CmdCtxTypeParamsCollectorEmpty<AppError, Output>,
        SingleProfileSingleFlowBuilder<CmdCtxTypeParamsCollectorEmpty<AppError, Output>>,
    > {
        CmdCtxBuilder::single_profile_single_flow(output, workspace)
    }
}

impl<'ctx, CmdCtxTypeParamsT, ResTs0>
    CmdCtx<SingleProfileSingleFlow<'ctx, CmdCtxTypeParamsT, ResTs0>>
where
    CmdCtxTypeParamsT: CmdCtxTypeParams,
{
    /// Updates `resources` to a different type state based on the given
    /// function.
    pub fn resources_update<ResTs1, F>(
        self,
        f: F,
    ) -> CmdCtx<SingleProfileSingleFlow<'ctx, CmdCtxTypeParamsT, ResTs1>>
    where
        F: FnOnce(Resources<ResTs0>) -> Resources<ResTs1>,
    {
        let CmdCtx { scope } = self;

        let scope = scope.resources_update(f);

        CmdCtx { scope }
    }
}

impl<Scope> Deref for CmdCtx<Scope> {
    type Target = Scope;

    fn deref(&self) -> &Self::Target {
        &self.scope
    }
}

impl<Scope> DerefMut for CmdCtx<Scope> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.scope
    }
}
