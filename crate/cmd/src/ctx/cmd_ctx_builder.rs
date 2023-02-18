#![allow(clippy::type_complexity)]

use std::fmt::Debug;

use peace_resources::{internal::WorkspaceParamsFile, resources::ts::Empty, Resources};
use peace_rt_model::{
    cmd_context_params::{
        KeyMaybe, KeyUnknown, ParamsKeys, ParamsKeysImpl, ParamsTypeRegs, ParamsTypeRegsBuilder,
        WorkspaceParams,
    },
    fn_graph::resman::Resource,
    Error, Storage, Workspace, WorkspaceInitializer,
};

use crate::{ctx::CmdCtx, scopes::NoProfileNoFlow};

pub use self::{
    flow_id_selection::{FlowIdNotSelected, FlowIdSelected},
    profile_selection::{ProfileFromWorkspaceParam, ProfileNotSelected, ProfileSelected},
    single_profile_single_flow_builder::SingleProfileSingleFlowBuilder,
    workspace_params_selection::{WorkspaceParamsNone, WorkspaceParamsSome},
};

mod flow_id_selection;
mod profile_selection;
mod single_profile_single_flow_builder;
mod workspace_params_selection;

/// Collects parameters and initializes values relevant to the built [`CmdCtx`].
#[derive(Debug)]
pub struct CmdCtxBuilder<'ctx, ScopeBuilder, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Workspace that the `peace` tool runs in.
    workspace: &'ctx Workspace,
    /// Data held while building `CmdCtx`.
    scope_builder: ScopeBuilder,
    /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
    /// [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: crate::cmd_context_params::WorkspaceParams
    /// [`ProfileParams`]: crate::cmd_context_params::ProfileParams
    /// [`FlowParams`]: crate::cmd_context_params::FlowParams
    params_type_regs_builder: ParamsTypeRegsBuilder<PKeys>,
}

impl<'ctx, ScopeBuilder, PKeys> CmdCtxBuilder<'ctx, ScopeBuilder, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Serializes workspace params to storage.
    async fn workspace_params_serialize(
        workspace_params: Option<&WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>>,
        storage: &Storage,
        workspace_params_file: &WorkspaceParamsFile,
    ) -> Result<(), Error> {
        if let Some(workspace_params) = workspace_params {
            WorkspaceInitializer::workspace_params_serialize(
                storage,
                workspace_params,
                workspace_params_file,
            )
            .await?;
        }

        Ok(())
    }

    /// Inserts workspace params into the `Resources` map.
    fn workspace_params_insert(
        mut workspace_params: Option<
            WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
        >,
        resources: &mut Resources<Empty>,
    ) {
        if let Some(workspace_params) = workspace_params.as_mut() {
            workspace_params
                .drain(..)
                .for_each(|(_key, workspace_param)| {
                    let workspace_param = workspace_param.into_inner().upcast();
                    let type_id = Resource::type_id(&*workspace_param);
                    resources.insert_raw(type_id, workspace_param);
                });
        }
    }
}

impl<'ctx>
    CmdCtxBuilder<'ctx, NoProfileNoFlow, ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>>
{
    /// Returns a `CmdCtxBuilder` for no profile.
    pub fn no_profile_no_flow(workspace: &'ctx Workspace) -> Self {
        Self {
            workspace,
            scope_builder: NoProfileNoFlow,
            params_type_regs_builder: ParamsTypeRegs::builder(),
        }
    }
}

impl<'ctx, PKeys> CmdCtxBuilder<'ctx, NoProfileNoFlow, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Builds the command context.
    ///
    /// This includes creating directories and deriving values based on the
    /// given parameters.
    pub fn build(
        self,
    ) -> CmdCtx<
        'ctx,
        NoProfileNoFlow,
        ParamsKeysImpl<
            PKeys::WorkspaceParamsKMaybe,
            PKeys::ProfileParamsKMaybe,
            PKeys::FlowParamsKMaybe,
        >,
    > {
        let CmdCtxBuilder {
            workspace,
            scope_builder: scope,
            params_type_regs_builder,
        } = self;

        let params_type_regs = params_type_regs_builder.build();

        CmdCtx {
            workspace,
            scope,
            params_type_regs,
        }
    }
}
