#![allow(clippy::type_complexity)]

use std::{fmt::Debug, hash::Hash};

use peace_resources::{
    internal::{ProfileParamsFile, WorkspaceParamsFile},
    Resources,
};
use peace_rt_model::{
    cmd_context_params::{KeyMaybe, ParamsKeys, ParamsKeysImpl, ParamsTypeRegs},
    Error, Workspace, WorkspaceInitializer,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::ctx::{
    cmd_ctx_builder::{
        flow_id_selection::FlowIdSelected,
        flow_params_selection::{FlowParamsNone, FlowParamsSome},
        profile_params_selection::{ProfileParamsNone, ProfileParamsSome},
        workspace_params_selection::{WorkspaceParamsNone, WorkspaceParamsSome},
    },
    CmdCtxBuilder,
};

/// Data stored by `CmdCtxBuilder` while building a
/// `CmdCtx<SingleProfileSingleFlow>`.
#[peace_code_gen::cmd_ctx_builder_impl]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleProfileSingleFlowBuilder;

impl<'ctx, 'key, PKeys, ProfileSelection, ProfileParamsSelection, FlowParamsSelection>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileSelection,
            FlowIdSelected,
            WorkspaceParamsSome<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
            ProfileParamsSelection,
            FlowParamsSelection,
        >,
        PKeys,
    >
where
    PKeys: ParamsKeys + 'static,
{
    /// Merges workspace params provided by the caller with the workspace params
    /// on disk.
    async fn workspace_params_merge(
        &mut self,
        workspace_params_file: &WorkspaceParamsFile,
    ) -> Result<(), Error> {
        let storage = self.workspace.storage();
        let params_deserialized = WorkspaceInitializer::workspace_params_deserialize::<
            <PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key,
        >(
            storage,
            self.params_type_regs_builder.workspace_params_type_reg(),
            workspace_params_file,
        )
        .await?;
        match (
            self.scope_builder.workspace_params_selection.0.as_mut(),
            params_deserialized,
        ) {
            (Some(params), Some(params_deserialized)) => {
                // Merge `params` on top of `params_deserialized`.
                // or, copy `params_deserialized` to `params` where
                // there isn't a value.

                params_deserialized
                    .into_inner()
                    .into_inner()
                    .into_iter()
                    .for_each(|(key, param)| {
                        if !params.contains_key(&key) {
                            params.insert_raw(key, param);
                        }
                    });
            }
            (None, Some(params_deserialized)) => {
                self.scope_builder.workspace_params_selection.0 = Some(params_deserialized)
            }
            (Some(_), None) => {}
            (None, None) => {}
        }

        Ok(())
    }
}

impl<'ctx, 'key, PKeys, ProfileSelection, WorkspaceParamsSelection, FlowParamsSelection>
    CmdCtxBuilder<
        'ctx,
        SingleProfileSingleFlowBuilder<
            ProfileSelection,
            FlowIdSelected,
            WorkspaceParamsSelection,
            ProfileParamsSome<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
            FlowParamsSelection,
        >,
        PKeys,
    >
where
    PKeys: ParamsKeys + 'static,
{
    /// Merges profile params provided by the caller with the profile params
    /// on disk.
    async fn profile_params_merge(
        &mut self,
        profile_params_file: &ProfileParamsFile,
    ) -> Result<(), Error> {
        let storage = self.workspace.storage();
        let params_deserialized = WorkspaceInitializer::profile_params_deserialize::<
            <PKeys::ProfileParamsKMaybe as KeyMaybe>::Key,
        >(
            storage,
            self.params_type_regs_builder.profile_params_type_reg(),
            profile_params_file,
        )
        .await?;
        match (
            self.scope_builder.profile_params_selection.0.as_mut(),
            params_deserialized,
        ) {
            (Some(params), Some(params_deserialized)) => {
                // Merge `params` on top of `params_deserialized`.
                // or, copy `params_deserialized` to `params` where
                // there isn't a value.

                params_deserialized
                    .into_inner()
                    .into_inner()
                    .into_iter()
                    .for_each(|(key, param)| {
                        if !params.contains_key(&key) {
                            params.insert_raw(key, param);
                        }
                    });
            }
            (None, Some(params_deserialized)) => {
                self.scope_builder.profile_params_selection.0 = Some(params_deserialized)
            }
            (Some(_), None) => {}
            (None, None) => {}
        }

        Ok(())
    }
}
