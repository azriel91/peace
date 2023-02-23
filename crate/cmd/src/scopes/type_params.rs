//! Types used for parameters type state for scopes.

pub use self::{
    flow_id_selection::{FlowIdNotSelected, FlowIdSelected},
    flow_params_selection::{FlowParamsNone, FlowParamsSome},
    profile_params_selection::{ProfileParamsNone, ProfileParamsSome},
    profile_selection::{
        ProfileFromWorkspaceParam, ProfileNotSelected, ProfileSelected, ProfilesFilterFunction,
    },
    workspace_params_selection::{WorkspaceParamsNone, WorkspaceParamsSome},
};

mod flow_id_selection;
mod flow_params_selection;
mod profile_params_selection;
mod profile_selection;
mod workspace_params_selection;
