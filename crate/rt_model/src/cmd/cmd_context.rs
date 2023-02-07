use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use peace_resources::{
    resources::ts::SetUp,
    type_reg::untagged::{BoxDt, TypeReg},
    Resources,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    cmd::{cmd_context_builder::KeyUnknown, ts::CmdContextCommon, CmdContextBuilder},
    ItemSpecGraph, StatesTypeRegs, Workspace,
};

/// Information needed to execute a command.
///
/// This includes:
///
/// * `ItemSpecGraph`: Logic to run.
/// * `Resources`: Data consumed / produced by the command.
///
/// Members of `Workspace` -- where the command should be run -- are
/// individually stored in `Resources`:
///
/// * [`Profile`]
/// * [`WorkspaceDir`]
/// * [`PeaceDir`]
/// * [`ProfileDir`]
/// * [`ProfileHistoryDir`]
///
/// # Type Parameters
///
/// * `E`: Consumer provided error type.
/// * `O`: [`OutputWrite`] to return values / errors to.
/// * `TS`: Type state of `Resources`.
///
/// [`Profile`]: peace_cfg::Profile
/// [`WorkspaceDir`]: peace_resources::paths::WorkspaceDir
/// [`PeaceDir`]: peace_resources::paths::PeaceDir
/// [`ProfileDir`]: peace_resources::paths::ProfileDir
/// [`ProfileHistoryDir`]: peace_resources::paths::ProfileHistoryDir
/// [`OutputWrite`]: peace_rt_model_core::OutputWrite
#[derive(Debug)]
pub struct CmdContext<'ctx, E, O, TS, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
where
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    /// Workspace that the `peace` tool runs in.
    pub workspace: &'ctx Workspace,
    /// Graph of item specs.
    pub item_spec_graph: &'ctx ItemSpecGraph<E>,
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    pub output: &'ctx mut O,
    /// `Resources` in this workspace.
    pub resources: Resources<TS>,
    /// Type registry for `WorkspaceParams` deserialization.
    pub workspace_params_type_reg: TypeReg<WorkspaceParamsK, BoxDt>,
    /// Type registry for `ProfileParams` deserialization.
    pub profile_params_type_reg: TypeReg<ProfileParamsK, BoxDt>,
    /// Type registry for `FlowParams` deserialization.
    pub flow_params_type_reg: TypeReg<FlowParamsK, BoxDt>,
    /// Type registries to deserialize `StatesSavedFile` and
    /// `StatesDesiredFile`.
    pub states_type_regs: StatesTypeRegs,
    /// Multi-progress to track progress of each operation execution.
    #[cfg(feature = "output_progress")]
    pub cmd_progress_tracker: crate::CmdProgressTracker,
    /// Prevents instantiation not through builder.
    pub(crate) marker: PhantomData<()>,
}

impl<'ctx, E, O> CmdContext<'ctx, E, O, SetUp, (), (), ()>
where
    E: std::error::Error + From<crate::Error>,
{
    /// Returns a builder for the command context.
    ///
    /// # Parameters
    ///
    /// * `workspace`: Defines how to discover the workspace.
    /// * `item_spec_graph`: Logic to run in the command.
    /// * `output`: [`OutputWrite`] to return values or errors.
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    pub fn builder(
        workspace: &'ctx Workspace,
        item_spec_graph: &'ctx ItemSpecGraph<E>,
        output: &'ctx mut O,
    ) -> CmdContextBuilder<'ctx, E, O, CmdContextCommon, KeyUnknown, KeyUnknown, KeyUnknown> {
        CmdContextBuilder::new(workspace, item_spec_graph, output)
    }
}

#[cfg(not(feature = "output_progress"))]
type CmdContextFields<'ctx, E, O, TS, WorkspaceParamsK, ProfileParamsK, FlowParamsK> = (
    &'ctx Workspace,
    &'ctx ItemSpecGraph<E>,
    &'ctx mut O,
    Resources<TS>,
    TypeReg<WorkspaceParamsK, BoxDt>,
    TypeReg<ProfileParamsK, BoxDt>,
    TypeReg<FlowParamsK, BoxDt>,
    StatesTypeRegs,
);

#[cfg(feature = "output_progress")]
type CmdContextFields<'ctx, E, O, TS, WorkspaceParamsK, ProfileParamsK, FlowParamsK> = (
    &'ctx Workspace,
    &'ctx ItemSpecGraph<E>,
    &'ctx mut O,
    Resources<TS>,
    TypeReg<WorkspaceParamsK, BoxDt>,
    TypeReg<ProfileParamsK, BoxDt>,
    TypeReg<FlowParamsK, BoxDt>,
    StatesTypeRegs,
    crate::CmdProgressTracker,
);

impl<'ctx, E, O, TS, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
    CmdContext<'ctx, E, O, TS, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
where
    E: std::error::Error,
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    /// Returns the underlying data.
    pub fn into_inner(
        self,
    ) -> CmdContextFields<'ctx, E, O, TS, WorkspaceParamsK, ProfileParamsK, FlowParamsK> {
        let Self {
            workspace,
            item_spec_graph,
            output,
            resources,
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            marker: _,
        } = self;

        (
            workspace,
            item_spec_graph,
            output,
            resources,
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
        )
    }

    /// Returns a reference to the workspace.
    pub fn workspace(&self) -> &Workspace {
        self.workspace
    }

    /// Returns a reference to the item spec graph.
    pub fn item_spec_graph(&self) -> &ItemSpecGraph<E> {
        self.item_spec_graph
    }

    /// Returns a reference to the output.
    pub fn output(&self) -> &O {
        &*self.output
    }

    /// Returns a mutable reference to the output.
    pub fn output_mut(&mut self) -> &mut O {
        self.output
    }

    /// Returns a reference to the resources.
    pub fn resources(&self) -> &Resources<TS> {
        &self.resources
    }

    /// Returns a mutable reference to the resources.
    pub fn resources_mut(&mut self) -> &mut Resources<TS> {
        &mut self.resources
    }

    /// Returns a reference to the workspace params type registry
    pub fn workspace_params_type_reg(&self) -> &TypeReg<WorkspaceParamsK, BoxDt> {
        &self.workspace_params_type_reg
    }

    /// Returns a reference to the profile params type registry
    pub fn profile_params_type_reg(&self) -> &TypeReg<ProfileParamsK, BoxDt> {
        &self.profile_params_type_reg
    }

    /// Returns a reference to the flow params type registry
    pub fn flow_params_type_reg(&self) -> &TypeReg<FlowParamsK, BoxDt> {
        &self.flow_params_type_reg
    }

    /// Returns a reference to the states type registries
    pub fn states_type_regs(&self) -> &StatesTypeRegs {
        &self.states_type_regs
    }
}

impl<'ctx, E, O, TS, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
    From<CmdContextFields<'ctx, E, O, TS, WorkspaceParamsK, ProfileParamsK, FlowParamsK>>
    for CmdContext<'ctx, E, O, TS, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
where
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    fn from(
        #[cfg(not(feature = "output_progress"))] (
            workspace,
            item_spec_graph,
            output,
            resources,
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
            states_type_regs,
        ): CmdContextFields<'ctx, E, O, TS, WorkspaceParamsK, ProfileParamsK, FlowParamsK>,
        #[cfg(feature = "output_progress")] (
            workspace,
            item_spec_graph,
            output,
            resources,
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
            states_type_regs,
            cmd_progress_tracker,
        ): CmdContextFields<'ctx, E, O, TS, WorkspaceParamsK, ProfileParamsK, FlowParamsK>,
    ) -> Self {
        Self {
            workspace,
            item_spec_graph,
            output,
            resources,
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            marker: PhantomData,
        }
    }
}

impl<'ctx, E, O, TS0, TS1, WorkspaceParamsK, ProfileParamsK, FlowParamsK, F>
    From<(
        CmdContext<'ctx, E, O, TS0, WorkspaceParamsK, ProfileParamsK, FlowParamsK>,
        F,
    )> for CmdContext<'ctx, E, O, TS1, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
where
    E: std::error::Error,
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    F: FnOnce(Resources<TS0>) -> Resources<TS1>,
{
    fn from(
        (cmd_context_ts0, f): (
            CmdContext<'ctx, E, O, TS0, WorkspaceParamsK, ProfileParamsK, FlowParamsK>,
            F,
        ),
    ) -> Self {
        #[cfg(not(feature = "output_progress"))]
        let (
            workspace,
            item_spec_graph,
            output,
            resources,
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
            states_type_regs,
        ) = cmd_context_ts0.into_inner();
        #[cfg(feature = "output_progress")]
        let (
            workspace,
            item_spec_graph,
            output,
            resources,
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
            states_type_regs,
            cmd_progress_tracker,
        ) = cmd_context_ts0.into_inner();
        let resources: Resources<TS1> = f(resources);

        Self {
            workspace,
            item_spec_graph,
            output,
            resources,
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            marker: PhantomData,
        }
    }
}
