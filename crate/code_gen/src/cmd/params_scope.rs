use proc_macro2::{Ident, Span};

/// Workspace, profile, or flow params.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParamsScope {
    Workspace,
    Profile,
    Flow,
}

impl ParamsScope {
    pub fn iter() -> std::array::IntoIter<ParamsScope, 3> {
        [Self::Workspace, Self::Profile, Self::Flow].into_iter()
    }

    /// Returns the name to use for the `*Param` type parameter.
    pub fn param_type_param(self) -> Ident {
        match self {
            Self::Workspace => Ident::new("WorkspaceParam", Span::call_site()),
            Self::Profile => Ident::new("ProfileParam", Span::call_site()),
            Self::Flow => Ident::new("FlowParam", Span::call_site()),
        }
    }

    /// Returns the `*Params` type map name.
    pub fn params_map_type(self) -> Ident {
        match self {
            Self::Workspace => Ident::new("WorkspaceParams", Span::call_site()),
            Self::Profile => Ident::new("ProfileParams", Span::call_site()),
            Self::Flow => Ident::new("FlowParams", Span::call_site()),
        }
    }

    /// Returns the name to use for the `*ParamsK` type parameter.
    pub fn params_k_type_param(self) -> Ident {
        match self {
            Self::Workspace => Ident::new("WorkspaceParamsK", Span::call_site()),
            Self::Profile => Ident::new("ProfileParamsK", Span::call_site()),
            Self::Flow => Ident::new("FlowParamsK", Span::call_site()),
        }
    }

    /// Returns the name of the `with_*_params_k` method to register the key
    /// type.
    pub fn params_k_method_name(self) -> Ident {
        match self {
            Self::Workspace => Ident::new("with_workspace_params_k", Span::call_site()),
            Self::Profile => Ident::new("with_profile_params_k", Span::call_site()),
            Self::Flow => Ident::new("with_flow_params_k", Span::call_site()),
        }
    }

    /// Returns the name to use for the `with_*_param` method.
    pub fn param_method_name(self) -> Ident {
        match self {
            Self::Workspace => Ident::new("with_workspace_param", Span::call_site()),
            Self::Profile => Ident::new("with_profile_param", Span::call_site()),
            Self::Flow => Ident::new("with_flow_param", Span::call_site()),
        }
    }

    /// Returns the name to use for the `*_param` variable.
    pub fn param_name(self) -> Ident {
        match self {
            Self::Workspace => Ident::new("workspace_param", Span::call_site()),
            Self::Profile => Ident::new("profile_param", Span::call_site()),
            Self::Flow => Ident::new("flow_param", Span::call_site()),
        }
    }

    /// Returns the name to use for the `*_param` variable.
    pub fn params_selection_name(self) -> Ident {
        match self {
            Self::Workspace => Ident::new("workspace_params_selection", Span::call_site()),
            Self::Profile => Ident::new("profile_params_selection", Span::call_site()),
            Self::Flow => Ident::new("flow_params_selection", Span::call_site()),
        }
    }

    /// Returns the lowercase str: "workspace", "profile", "flow".
    pub fn to_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Profile => "profile",
            Self::Flow => "flow",
        }
    }
}
