use std::{fmt::Debug, hash::Hash};

use peace_core::Profile;
use peace_resources::{
    paths::{FlowDir, ProfileDir, ProfileHistoryDir},
    resources::ts::SetUp,
    Resources,
};
use peace_rt_model::{
    cmd_context_params::{
        FlowParams, KeyKnown, KeyMaybe, ParamsKeys, ParamsKeysImpl, ProfileParams, WorkspaceParams,
    },
    Flow, StatesTypeRegs,
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
pub struct SingleProfileSingleFlow<E, PKeys, TS>
where
    PKeys: ParamsKeys + 'static,
{
    /// Tracks progress of each operation execution.
    #[cfg(feature = "output_progress")]
    cmd_progress_tracker: peace_rt_model::CmdProgressTracker,
    /// The profile this command operates on.
    profile: Profile,
    /// Profile directory that stores params and flows.
    profile_dir: ProfileDir,
    /// Directory to store profile execution history.
    profile_history_dir: ProfileHistoryDir,
    /// The chosen process flow.
    flow: Flow<E>,
    /// Flow directory that stores params and states.
    flow_dir: FlowDir,
    /// Workspace params.
    workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    /// Profile params for the profile.
    profile_params: ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
    /// Flow params for the selected flow.
    flow_params: FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>,
    /// Type registries to deserialize [`StatesSavedFile`] and
    /// [`StatesDesiredFile`].
    ///
    /// [`StatesSavedFile`]: peace_resources::paths::StatesSavedFile
    /// [`StatesDesiredFile`]: peace_resources::paths::StatesDesiredFile
    states_type_regs: StatesTypeRegs,
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
pub struct SingleProfileSingleFlowView<'view, E, PKeys, TS>
where
    PKeys: ParamsKeys + 'static,
{
    /// Tracks progress of each operation execution.
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
    /// Workspace params.
    pub workspace_params: &'view WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    /// Profile params for the profile.
    pub profile_params: &'view ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
    /// Flow params for the selected flow.
    pub flow_params: &'view FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>,
    /// Type registries to deserialize [`StatesSavedFile`] and
    /// [`StatesDesiredFile`].
    ///
    /// [`StatesSavedFile`]: peace_resources::paths::StatesSavedFile
    /// [`StatesDesiredFile`]: peace_resources::paths::StatesDesiredFile
    pub states_type_regs: &'view StatesTypeRegs,
    /// `Resources` for flow execution.
    pub resources: &'view mut Resources<TS>,
}

impl<E, PKeys> SingleProfileSingleFlow<E, PKeys, SetUp>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns a new `SingleProfileSingleFlow` scope.
    #[allow(clippy::too_many_arguments)] // Constructed by proc macro
    pub(crate) fn new(
        #[cfg(feature = "output_progress")]
        cmd_progress_tracker: peace_rt_model::CmdProgressTracker,
        profile: Profile,
        profile_dir: ProfileDir,
        profile_history_dir: ProfileHistoryDir,
        flow: Flow<E>,
        flow_dir: FlowDir,
        workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
        profile_params: ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
        flow_params: FlowParams<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key>,
        states_type_regs: StatesTypeRegs,
        resources: Resources<SetUp>,
    ) -> Self {
        Self {
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            profile,
            profile_dir,
            profile_history_dir,
            flow,
            flow_dir,
            workspace_params,
            profile_params,
            flow_params,
            states_type_regs,
            resources,
        }
    }
}

impl<E, PKeys, TS> SingleProfileSingleFlow<E, PKeys, TS>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns a view struct of this scope.
    ///
    /// This allows the flow and resources to be borrowed concurrently.
    pub fn view(&mut self) -> SingleProfileSingleFlowView<'_, E, PKeys, TS> {
        let Self {
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            profile,
            profile_dir,
            profile_history_dir,
            flow,
            flow_dir,
            workspace_params,
            profile_params,
            flow_params,
            states_type_regs,
            resources,
        } = self;

        SingleProfileSingleFlowView {
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            profile,
            profile_dir,
            profile_history_dir,
            flow,
            flow_dir,
            workspace_params,
            profile_params,
            flow_params,
            states_type_regs,
            resources,
        }
    }

    /// Returns the progress tracker for all operations' executions.
    #[cfg(feature = "output_progress")]
    pub fn cmd_progress_tracker(&self) -> &peace_rt_model::CmdProgressTracker {
        &self.cmd_progress_tracker
    }

    /// Returns a mutable reference to the progress tracker for all operations'
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
        &self.flow
    }

    /// Returns a reference to the flow directory.
    pub fn flow_dir(&self) -> &FlowDir {
        &self.flow_dir
    }

    /// Returns the type registries to deserialize [`StatesSavedFile`] and
    /// [`StatesDesiredFile`].
    ///
    /// [`StatesSavedFile`]: peace_resources::paths::StatesSavedFile
    /// [`StatesDesiredFile`]: peace_resources::paths::StatesDesiredFile
    pub fn states_type_regs(&self) -> &StatesTypeRegs {
        &self.states_type_regs
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
    pub fn resources_update<ResTs1, F>(self, f: F) -> SingleProfileSingleFlow<E, PKeys, ResTs1>
    where
        F: FnOnce(Resources<TS>) -> Resources<ResTs1>,
    {
        let SingleProfileSingleFlow {
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            profile,
            profile_dir,
            profile_history_dir,
            flow,
            flow_dir,
            workspace_params,
            profile_params,
            flow_params,
            states_type_regs,
            resources,
        } = self;

        let resources = f(resources);

        SingleProfileSingleFlow {
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            profile,
            profile_dir,
            profile_history_dir,
            flow,
            flow_dir,
            workspace_params,
            profile_params,
            flow_params,
            states_type_regs,
            resources,
        }
    }
}

impl<E, WorkspaceParamsK, ProfileParamsKMaybe, FlowParamsKMaybe, TS>
    SingleProfileSingleFlow<
        E,
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

impl<E, WorkspaceParamsKMaybe, ProfileParamsK, FlowParamsKMaybe, TS>
    SingleProfileSingleFlow<
        E,
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

impl<E, WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsK, TS>
    SingleProfileSingleFlow<
        E,
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
