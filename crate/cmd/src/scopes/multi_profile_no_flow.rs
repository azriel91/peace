use std::{collections::BTreeMap, fmt::Debug, hash::Hash, marker::PhantomData};

use peace_core::Profile;
use peace_resources::paths::{ProfileDir, ProfileHistoryDir};
use peace_rt_model::cmd_context_params::{
    KeyKnown, KeyMaybe, ParamsKeys, ParamsKeysImpl, ParamsTypeRegs, ProfileParams, WorkspaceParams,
};
use serde::{de::DeserializeOwned, Serialize};

/// A command that works with multiple profiles, without any item specs.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 internal_dev_a           # ✅ can list multiple `Profile`s
/// |   |- 📝 profile_params.yaml  # ✅ can read multiple `ProfileParams`
/// |   |
/// |   |- ..                      # ❌ cannot read or write `Flow` information
/// |
/// |- 🌏 customer_a_dev           # ✅
/// |   |- 📝 profile_params.yaml  # ✅
/// |
/// |- 🌏 customer_a_prod          # ✅
/// |   |- 📝 profile_params.yaml  # ✅
/// |
/// |- 🌏 workspace_init           # ✅ can list multiple `Profile`s
///     |- 📝 profile_params.yaml  # ❌ cannot read profile params of different underlying type
/// ```
///
/// ## Capabilities
///
/// This kind of command can:
///
/// * Read or write workspace parameters.
/// * Read or write multiple profiles' parameters &ndash; as long as they are of
///   the same type (same `struct`).
///
/// This kind of command cannot:
///
/// * Read or write flow parameters -- see `SingleProfileSingleFlow` or
///   `MultiProfileSingleFlow`.
/// * Read or write flow state -- see `SingleProfileSingleFlow` or
///   `MultiProfileSingleFlow`.
#[derive(Debug)]
pub struct MultiProfileNoFlow<E, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// The profiles that are accessible by this command.
    profiles: Vec<Profile>,
    /// Profile directories that store params and flows.
    profile_dirs: BTreeMap<Profile, ProfileDir>,
    /// Directories of each profile's execution history.
    profile_history_dirs: BTreeMap<Profile, ProfileHistoryDir>,
    /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
    /// [`FlowParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: crate::cmd_context_params::WorkspaceParams
    /// [`ProfileParams`]: crate::cmd_context_params::ProfileParams
    /// [`FlowParams`]: crate::cmd_context_params::FlowParams
    params_type_regs: ParamsTypeRegs<PKeys>,
    /// Workspace params.
    workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    /// Profile params for the profile.
    profile_to_profile_params:
        BTreeMap<Profile, ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>>,
    /// Marker.
    marker: PhantomData<E>,
}

impl<E, PKeys> MultiProfileNoFlow<E, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns a new `MultiProfileNoFlow` scope.
    pub(crate) fn new(
        profiles: Vec<Profile>,
        profile_dirs: BTreeMap<Profile, ProfileDir>,
        profile_history_dirs: BTreeMap<Profile, ProfileHistoryDir>,
        params_type_regs: ParamsTypeRegs<PKeys>,
        workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
        profile_to_profile_params: BTreeMap<
            Profile,
            ProfileParams<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key>,
        >,
    ) -> Self {
        Self {
            profiles,
            profile_dirs,
            profile_history_dirs,
            params_type_regs,
            workspace_params,
            profile_to_profile_params,
            marker: PhantomData,
        }
    }

    /// Returns the accessible profiles.
    ///
    /// These are the profiles that are filtered by the filter function, if
    /// provided.
    pub fn profiles(&self) -> &[Profile] {
        self.profiles.as_ref()
    }

    /// Returns the profile directories keyed by each profile.
    pub fn profile_dirs(&self) -> &BTreeMap<Profile, ProfileDir> {
        &self.profile_dirs
    }

    /// Returns the profile history directories keyed by each profile.
    pub fn profile_history_dirs(&self) -> &BTreeMap<Profile, ProfileHistoryDir> {
        &self.profile_history_dirs
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
}

impl<E, WorkspaceParamsK, ProfileParamsKMaybe, FlowParamsKMaybe>
    MultiProfileNoFlow<
        E,
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
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

impl<E, WorkspaceParamsKMaybe, ProfileParamsK, FlowParamsKMaybe>
    MultiProfileNoFlow<
        E,
        ParamsKeysImpl<WorkspaceParamsKMaybe, KeyKnown<ProfileParamsK>, FlowParamsKMaybe>,
    >
where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Returns the profile params for each profile.
    pub fn profile_to_profile_params(&self) -> &BTreeMap<Profile, ProfileParams<ProfileParamsK>> {
        &self.profile_to_profile_params
    }
}
