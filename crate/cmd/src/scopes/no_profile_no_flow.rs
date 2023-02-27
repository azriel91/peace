use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use peace_rt_model::cmd_context_params::{
    KeyKnown, KeyMaybe, ParamsKeys, ParamsKeysImpl, WorkspaceParams,
};
use serde::{de::DeserializeOwned, Serialize};

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
/// * Read or write flow state -- see `SingleProfileSingleFlow` or
///   `MultiProfileSingleFlow`.
#[derive(Debug)]
pub struct NoProfileNoFlow<E, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Workspace params.
    workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    /// Marker.
    marker: PhantomData<E>,
}

impl<E, PKeys> NoProfileNoFlow<E, PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    pub(crate) fn new(
        workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    ) -> Self {
        Self {
            workspace_params,
            marker: PhantomData,
        }
    }
}

impl<E, WorkspaceParamsK, ProfileParamsKMaybe, FlowParamsKMaybe>
    NoProfileNoFlow<
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
