//! Types used for parameters type state for scopes.

pub use self::{
    flow_params_selection::{FlowParamsNone, FlowParamsSome},
    flow_selection::{FlowNotSelected, FlowSelected},
    profile_params_selection::{ProfileParamsNone, ProfileParamsSome, ProfileParamsSomeMulti},
    profile_selection::{
        ProfileFilterFn, ProfileFromWorkspaceParam, ProfileNotSelected, ProfileSelected,
    },
    workspace_params_selection::{WorkspaceParamsNone, WorkspaceParamsSome},
};

mod flow_params_selection;
mod flow_selection;
mod profile_params_selection;
mod profile_selection;
mod workspace_params_selection;
