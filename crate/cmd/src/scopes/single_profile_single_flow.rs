use std::{fmt::Debug, hash::Hash};

use peace_core::Profile;
use peace_params::ParamsSpecs;
use peace_resources::{
    paths::{FlowDir, PeaceAppDir, PeaceDir, ProfileDir, ProfileHistoryDir, WorkspaceDir},
    resources::ts::SetUp,
    Resources,
};
use peace_rt_model::{
    params::{
        FlowParams, KeyKnown, KeyMaybe, ParamsKeys, ParamsKeysImpl, ParamsTypeRegs, ProfileParams,
        WorkspaceParams,
    },
    Flow, ItemSpecParamsTypeReg, ParamsSpecsDeTypeReg, StatesTypeReg, Workspace,
};
use serde::{de::DeserializeOwned, Serialize};

/// A command that works with one profile and one flow.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 internal_dev_a
/// |   |- 📝 profile_params.yaml  # ✅ can read or write `ProfileParams`
/// |   |
/// |   |- 🌊 deploy                   # ✅ can read `FlowId`
/// |   |   |- 📝 flow_params.yaml     # ✅ can read or write `FlowParams`
/// |   |   |- 📋 states_desired.yaml  # ✅ can read or write `StatesDesired`
/// |   |   |- 📋 states_saved.yaml    # ✅ can read or write `StatesSaved`
/// |   |
/// |   |- 🌊 ..                   # ❌ cannot read or write other `Flow` information
/// |
/// |- 🌏 ..                       # ❌ cannot read or write other `Profile` information
/// ```
///
/// ## Capabilities
///
/// This kind of command can:
///
/// * Read or write workspace parameters.
/// * Read or write a single profile's parameters. For multiple profiles, see
///   `MultiProfileNoFlow`.
///
/// This kind of command cannot:
///
/// * Read or write flow parameters -- see `MultiProfileNoFlow`.
/// * Read or write flow state -- see `SingleProfileSingleFlow` or
///   `MultiProfileSingleFlow`.
#[derive(Debug)]
pub struct SingleProfileSingleFlow<'ctx, E, O, PKeys, TS>
where
    PKeys: ParamsKeys + 'static,
{
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    output: &'ctx mut O,
    /// Workspace that the `peace` tool runs in.
    workspace: &'ctx Workspace,
    /// Tracks progress of each function execution.
    #[cfg(feature = "output_progress")]
    cmd_progress_tracker: peace_rt_model::CmdProgressTracker,
    /// The profile this command operates on.
    profile: Profile,
    /// Profile directory that stores params and flows.
    profile_dir: ProfileDir,
    /// Directory to store profile execution history.
    profile_history_dir: ProfileHistoryDir,
    /// The chosen process flow.
    flow: &'ctx Flow<E>,
    /// Flow directory that stores params and states.
    flow_dir: FlowDir,
    /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
    /// [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: peace_rt_model::params::WorkspaceParams
    /// [`ProfileParams`]: peace_rt_model::params::ProfileParams
    /// [`FlowParams`]: peace_rt_model::params::FlowParams
    params_type_regs: ParamsTypeRegs<PKeys>,
    /// Workspace params.
    workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    /// Profile params for the profile.
    profile_params: ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
    /// Flow params for the selected flow.
    flow_params: FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>,
    /// Type registry for each item spec's [`Params`].
    ///
    /// This is used to deserialize [`ItemSpecParamsFile`].
    ///
    /// [`Params`]: peace_cfg::ItemSpec::Params
    /// [`ItemSpecParamsFile`]: peace_resources::paths::ItemSpecParamsFile
    item_spec_params_type_reg: ItemSpecParamsTypeReg,
    /// Type registry for each item spec's [`Params`]`::SpecDe`.
    ///
    /// This is used to deserialize [`ParamsSpecsFile`].
    ///
    /// [`Params`]: peace_cfg::ItemSpec::Params
    /// [`ParamsSpecsFile`]: peace_resources::paths::ParamsSpecsFile
    params_specs_de_type_reg: ParamsSpecsDeTypeReg,
    /// Item spec params specs for the selected flow.
    params_specs: ParamsSpecs,
    /// Type registry for each item spec's `State`.
    ///
    /// This is used to deserialize [`StatesSavedFile`] and
    /// [`StatesDesiredFile`].
    ///
    /// [`StatesSavedFile`]: peace_resources::paths::StatesSavedFile
    /// [`StatesDesiredFile`]: peace_resources::paths::StatesDesiredFile
    states_type_reg: StatesTypeReg,
    /// `Resources` for flow execution.
    resources: Resources<TS>,
}

/// A command that works with one profile and one flow.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 internal_dev_a
/// |   |- 📝 profile_params.yaml  # ✅ can read or write `ProfileParams`
/// |   |
/// |   |- 🌊 deploy                   # ✅ can read `FlowId`
/// |   |   |- 📝 flow_params.yaml     # ✅ can read or write `FlowParams`
/// |   |   |- 📋 states_desired.yaml  # ✅ can read or write `StatesDesired`
/// |   |   |- 📋 states_saved.yaml    # ✅ can read or write `StatesSaved`
/// |   |
/// |   |- 🌊 ..                   # ❌ cannot read or write other `Flow` information
/// |
/// |- 🌏 ..                       # ❌ cannot read or write other `Profile` information
/// ```
///
/// ## Capabilities
///
/// This kind of command can:
///
/// * Read or write workspace parameters.
/// * Read or write a single profile's parameters. For multiple profiles, see
///   `MultiProfileNoFlow`.
///
/// This kind of command cannot:
///
/// * Read or write flow parameters -- see `MultiProfileNoFlow`.
/// * Read or write flow state -- see `SingleProfileSingleFlow` or
///   `MultiProfileSingleFlow`.
#[derive(Debug)]
pub struct SingleProfileSingleFlowView<'view, E, O, PKeys, TS>
where
    PKeys: ParamsKeys + 'static,
{
    /// Output endpoint to return values / errors, and write progress
    /// information to.
    ///
    /// See [`OutputWrite`].
    ///
    /// [`OutputWrite`]: peace_rt_model_core::OutputWrite
    pub output: &'view mut O,
    /// Workspace that the `peace` tool runs in.
    pub workspace: &'view Workspace,
    /// Tracks progress of each function execution.
    #[cfg(feature = "output_progress")]
    pub cmd_progress_tracker: &'view mut peace_rt_model::CmdProgressTracker,
    /// The profile this command operates on.
    pub profile: &'view Profile,
    /// Profile directory that stores params and flows.
    pub profile_dir: &'view ProfileDir,
    /// Directory to store profile execution history.
    pub profile_history_dir: &'view ProfileHistoryDir,
    /// The chosen process flow.
    pub flow: &'view Flow<E>,
    /// Flow directory that stores params and states.
    pub flow_dir: &'view FlowDir,
    /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
    /// [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: peace_rt_model::params::WorkspaceParams
    /// [`ProfileParams`]: peace_rt_model::params::ProfileParams
    /// [`FlowParams`]: peace_rt_model::params::FlowParams
    pub params_type_regs: &'view ParamsTypeRegs<PKeys>,
    /// Workspace params.
    pub workspace_params: &'view WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    /// Profile params for the profile.
    pub profile_params: &'view ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
    /// Flow params for the selected flow.
    pub flow_params: &'view FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>,
    /// Type registry for each item spec's [`Params`].
    ///
    /// This is used to deserialize [`ItemSpecParamsFile`].
    ///
    /// [`Params`]: peace_cfg::ItemSpec::Params
    /// [`ItemSpecParamsFile`]: peace_resources::paths::ItemSpecParamsFile
    pub item_spec_params_type_reg: &'view ItemSpecParamsTypeReg,
    /// Type registry for each item spec's [`Params`]`::SpecDe`.
    ///
    /// This is used to deserialize [`ParamsSpecsFile`].
    ///
    /// [`Params`]: peace_cfg::ItemSpec::Params
    /// [`ParamsSpecsFile`]: peace_resources::paths::ParamsSpecsFile
    pub params_specs_de_type_reg: &'view ParamsSpecsDeTypeReg,
    /// Item spec params specs for the selected flow.
    pub params_specs: &'view ParamsSpecs,
    /// Type registry for each item spec's `State`.
    ///
    /// This is used to deserialize [`StatesSavedFile`] and
    /// [`StatesDesiredFile`].
    ///
    /// [`StatesSavedFile`]: peace_resources::paths::StatesSavedFile
    /// [`StatesDesiredFile`]: peace_resources::paths::StatesDesiredFile
    pub states_type_reg: &'view StatesTypeReg,
    /// `Resources` for flow execution.
    pub resources: &'view mut Resources<TS>,
}

impl<'ctx, E, O, PKeys> SingleProfileSingleFlow<'ctx, E, O, PKeys, SetUp>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns a new `SingleProfileSingleFlow` scope.
    #[allow(clippy::too_many_arguments)] // Constructed by proc macro
    pub(crate) fn new(
        output: &'ctx mut O,
        workspace: &'ctx Workspace,
        #[cfg(feature = "output_progress")]
        cmd_progress_tracker: peace_rt_model::CmdProgressTracker,
        profile: Profile,
        profile_dir: ProfileDir,
        profile_history_dir: ProfileHistoryDir,
        flow: &'ctx Flow<E>,
        flow_dir: FlowDir,
        params_type_regs: ParamsTypeRegs<PKeys>,
        workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
        profile_params: ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
        flow_params: FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>,
        item_spec_params_type_reg: ItemSpecParamsTypeReg,
        params_specs_de_type_reg: ParamsSpecsDeTypeReg,
        params_specs: ParamsSpecs,
        states_type_reg: StatesTypeReg,
        resources: Resources<SetUp>,
    ) -> Self {
        Self {
            output,
            workspace,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            profile,
            profile_dir,
            profile_history_dir,
            flow,
            flow_dir,
            params_type_regs,
            workspace_params,
            profile_params,
            flow_params,
            item_spec_params_type_reg,
            params_specs_de_type_reg,
            params_specs,
            states_type_reg,
            resources,
        }
    }
}

impl<'ctx, E, O, PKeys, TS> SingleProfileSingleFlow<'ctx, E, O, PKeys, TS>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns a view struct of this scope.
    ///
    /// This allows the flow and resources to be borrowed concurrently.
    pub fn view(&mut self) -> SingleProfileSingleFlowView<'_, E, O, PKeys, TS> {
        let Self {
            output,
            workspace,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            profile,
            profile_dir,
            profile_history_dir,
            flow,
            flow_dir,
            params_type_regs,
            workspace_params,
            profile_params,
            flow_params,
            item_spec_params_type_reg,
            params_specs_de_type_reg,
            params_specs,
            states_type_reg,
            resources,
        } = self;

        SingleProfileSingleFlowView {
            output,
            workspace,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            profile,
            profile_dir,
            profile_history_dir,
            flow,
            flow_dir,
            params_type_regs,
            workspace_params,
            profile_params,
            flow_params,
            item_spec_params_type_reg,
            params_specs_de_type_reg,
            params_specs,
            states_type_reg,
            resources,
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

    /// Returns the progress tracker for all functions' executions.
    #[cfg(feature = "output_progress")]
    pub fn cmd_progress_tracker(&self) -> &peace_rt_model::CmdProgressTracker {
        &self.cmd_progress_tracker
    }

    /// Returns a mutable reference to the progress tracker for all functions'
    /// executions.
    #[cfg(feature = "output_progress")]
    pub fn cmd_progress_tracker_mut(&mut self) -> &mut peace_rt_model::CmdProgressTracker {
        &mut self.cmd_progress_tracker
    }

    /// Returns a reference to the profile.
    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    /// Returns a reference to the profile directory.
    pub fn profile_dir(&self) -> &ProfileDir {
        &self.profile_dir
    }

    /// Returns a reference to the profile history directory.
    pub fn profile_history_dir(&self) -> &ProfileHistoryDir {
        &self.profile_history_dir
    }

    /// Returns a reference to the flow.
    pub fn flow(&self) -> &Flow<E> {
        self.flow
    }

    /// Returns a reference to the flow directory.
    pub fn flow_dir(&self) -> &FlowDir {
        &self.flow_dir
    }

    /// Returns the type registries for [`WorkspaceParams`], [`ProfileParams`],
    /// and [`FlowParams`] deserialization.
    ///
    /// Not to be confused with [`item_spec_params_type_reg`], which is used to
    /// deserialize [`ItemSpecParams`]
    ///
    /// [`FlowParams`]: peace_rt_model::params::FlowParams
    /// [`ItemSpecParams`]: peace_rt_model::ItemSpecParams
    /// [`ProfileParams`]: peace_rt_model::params::ProfileParams
    /// [`WorkspaceParams`]: peace_rt_model::params::WorkspaceParams
    /// [`item_spec_params_type_reg`]: Self::item_spec_params_type_reg
    pub fn params_type_regs(&self) -> &ParamsTypeRegs<PKeys> {
        &self.params_type_regs
    }

    /// Returns the type registry for each item spec's [`Params`].
    ///
    /// This is used to deserialize [`ItemSpecParamsFile`].
    ///
    /// [`Params`]: peace_cfg::ItemSpec::Params
    /// [`ItemSpecParamsFile`]: peace_resources::paths::ItemSpecParamsFile
    pub fn item_spec_params_type_reg(&self) -> &ItemSpecParamsTypeReg {
        &self.item_spec_params_type_reg
    }

    /// Returns the type registry for each item spec's [`Params`]`::SpecDe`.
    ///
    /// This is used to deserialize [`ParamsSpecsFile`].
    ///
    /// [`Params`]: peace_cfg::ItemSpec::Params
    /// [`ParamsSpecsFile`]: peace_resources::paths::ParamsSpecsFile
    pub fn params_specs_de_type_reg(&self) -> &ParamsSpecsDeTypeReg {
        &self.params_specs_de_type_reg
    }

    /// Returns the item spec params specs for the selected flow.
    pub fn params_specs(&self) -> &ParamsSpecs {
        &self.params_specs
    }

    /// Returns the type registry for each item spec's `State`.
    ///
    /// This is used to deserialize [`StatesSavedFile`] and
    /// [`StatesDesiredFile`].
    ///
    /// [`StatesSavedFile`]: peace_resources::paths::StatesSavedFile
    /// [`StatesDesiredFile`]: peace_resources::paths::StatesDesiredFile
    pub fn states_type_reg(&self) -> &StatesTypeReg {
        &self.states_type_reg
    }

    /// Returns a reference to the `Resources` for flow execution.
    pub fn resources(&self) -> &Resources<TS> {
        &self.resources
    }

    /// Returns a reference to the `Resources` for flow execution.
    pub fn resources_mut(&mut self) -> &mut Resources<TS> {
        &mut self.resources
    }

    /// Updates `resources` to a different type state based on the given
    /// function.
    pub fn resources_update<ResTs1, F>(
        self,
        f: F,
    ) -> SingleProfileSingleFlow<'ctx, E, O, PKeys, ResTs1>
    where
        F: FnOnce(Resources<TS>) -> Resources<ResTs1>,
    {
        let SingleProfileSingleFlow {
            output,
            workspace,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            profile,
            profile_dir,
            profile_history_dir,
            flow,
            flow_dir,
            params_type_regs,
            workspace_params,
            profile_params,
            flow_params,
            item_spec_params_type_reg,
            params_specs_de_type_reg,
            params_specs,
            states_type_reg,
            resources,
        } = self;

        let resources = f(resources);

        SingleProfileSingleFlow {
            output,
            workspace,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            profile,
            profile_dir,
            profile_history_dir,
            flow,
            flow_dir,
            params_type_regs,
            workspace_params,
            profile_params,
            flow_params,
            item_spec_params_type_reg,
            params_specs_de_type_reg,
            params_specs,
            states_type_reg,
            resources,
        }
    }
}

impl<'ctx, E, O, WorkspaceParamsK, ProfileParamsKMaybe, FlowParamsKMaybe, TS>
    SingleProfileSingleFlow<
        'ctx,
        E,
        O,
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
        TS,
    >
where
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Returns the workspace params.
    pub fn workspace_params(&self) -> &WorkspaceParams<WorkspaceParamsK> {
        &self.workspace_params
    }
}

impl<'ctx, E, O, WorkspaceParamsKMaybe, ProfileParamsK, FlowParamsKMaybe, TS>
    SingleProfileSingleFlow<
        'ctx,
        E,
        O,
        ParamsKeysImpl<WorkspaceParamsKMaybe, KeyKnown<ProfileParamsK>, FlowParamsKMaybe>,
        TS,
    >
where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Returns the profile params.
    pub fn profile_params(&self) -> &ProfileParams<ProfileParamsK> {
        &self.profile_params
    }
}

impl<'ctx, E, O, WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsK, TS>
    SingleProfileSingleFlow<
        'ctx,
        E,
        O,
        ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, KeyKnown<FlowParamsK>>,
        TS,
    >
where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    /// Returns the flow params for the selected flow.
    pub fn flow_params(&self) -> &FlowParams<FlowParamsK> {
        &self.flow_params
    }
}
