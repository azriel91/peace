use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use interruptible::InterruptibilityState;
use peace_resources::paths::{PeaceAppDir, PeaceDir, WorkspaceDir};
use peace_rt_model::{
    params::{KeyKnown, KeyMaybe, ParamsKeys, ParamsKeysImpl, ParamsTypeRegs, WorkspaceParams},
    Workspace,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::ctx::CmdCtxTypeParams;

/// A command that only works with workspace parameters.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 ..                       # ❌ cannot read or write `Profile` information
/// ```
///
/// ## Capabilities
///
/// This kind of command can:
///
/// * Read or write workspace parameters.
///
/// This kind of command cannot:
///
/// * Read or write profile parameters -- see `SingleProfileNoFlow` or
///   `MultiProfileNoFlow`.
/// * Read or write flow parameters -- see `MultiProfileNoFlow`.
/// * Read or write flow state -- see `NoProfileSingleFlow` or
///   `MultiProfileSingleFlow`.
#[derive(Debug)]
pub struct NoProfileNoFlow<'ctx, CmdCtxTypeParamsT>
where
    CmdCtxTypeParamsT: CmdCtxTypeParams,
{
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    output: &'ctx mut CmdCtxTypeParamsT::Output,
    /// Whether the `CmdExecution` is interruptible.
    ///
    /// If it is, this holds the interrupt channel receiver.
    interruptibility_state: InterruptibilityState<'ctx, 'ctx>,
    /// Workspace that the `peace` tool runs in.
    workspace: &'ctx Workspace,
    /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
    /// [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: peace_rt_model::params::WorkspaceParams
    /// [`ProfileParams`]: peace_rt_model::params::ProfileParams
    /// [`FlowParams`]: peace_rt_model::params::FlowParams
    params_type_regs: ParamsTypeRegs<CmdCtxTypeParamsT::ParamsKeys>,
    /// Workspace params.
    workspace_params: WorkspaceParams<
        <<CmdCtxTypeParamsT::ParamsKeys as ParamsKeys>::WorkspaceParamsKMaybe as KeyMaybe>::Key,
    >,
    /// Marker.
    marker: PhantomData<E>,
}

impl<'ctx, CmdCtxTypeParamsT> NoProfileNoFlow<'ctx, CmdCtxTypeParamsT>
where
    CmdCtxTypeParamsT: CmdCtxTypeParams,
{
    pub(crate) fn new(
        output: &'ctx mut CmdCtxTypeParamsT::Output,
        interruptibility_state: InterruptibilityState<'ctx, 'ctx>,
        workspace: &'ctx Workspace,
        params_type_regs: ParamsTypeRegs<CmdCtxTypeParamsT::ParamsKeys>,
        workspace_params: WorkspaceParams<
            <<CmdCtxTypeParamsT::ParamsKeys as ParamsKeys>::WorkspaceParamsKMaybe as KeyMaybe>::Key,
        >,
    ) -> Self {
        Self {
            output,
            interruptibility_state,
            workspace,
            params_type_regs,
            workspace_params,
            marker: PhantomData,
        }
    }

    /// Returns a reference to the output.
    pub fn output(&self) -> &CmdCtxTypeParamsT::Output {
        self.output
    }

    /// Returns a mutable reference to the output.
    pub fn output_mut(&mut self) -> &mut CmdCtxTypeParamsT::Output {
        self.output
    }

    //// Returns the interruptibility capability.
    pub fn interruptibility_state(&mut self) -> InterruptibilityState<'_, '_> {
        self.interruptibility_state.reborrow()
    }

    /// Returns the workspace that the `peace` tool runs in.
    pub fn workspace(&self) -> &Workspace {
        self.workspace
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

    /// Returns the type registries for [`WorkspaceParams`], [`ProfileParams`],
    /// and [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: peace_rt_model::params::WorkspaceParams
    /// [`ProfileParams`]: peace_rt_model::params::ProfileParams
    /// [`FlowParams`]: peace_rt_model::params::FlowParams
    pub fn params_type_regs(&self) -> &ParamsTypeRegs<CmdCtxTypeParamsT::ParamsKeys> {
        &self.params_type_regs
    }
}

impl<'ctx, CmdCtxTypeParamsT, WorkspaceParamsK, ProfileParamsKMaybe, FlowParamsKMaybe>
    NoProfileNoFlow<'ctx, CmdCtxTypeParamsT>
where
    CmdCtxTypeParamsT: CmdCtxTypeParams<
        ParamsKeys = ParamsKeysImpl<
            KeyKnown<WorkspaceParamsK>,
            ProfileParamsKMaybe,
            FlowParamsKMaybe,
        >,
    >,
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + Unpin + 'static,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Returns the workspace params.
    pub fn workspace_params(&self) -> &WorkspaceParams<WorkspaceParamsK> {
        &self.workspace_params
    }
}
