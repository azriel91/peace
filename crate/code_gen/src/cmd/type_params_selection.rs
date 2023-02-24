use syn::{parse_quote, FieldValue, GenericArgument};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ProfileSelection {
    NotSelected,
    Selected,
    FromWorkspaceParam,
    /// Only applicable to MultiProfile scopes.
    FilterFunction,
}

impl ProfileSelection {
    pub(crate) fn iter() -> std::array::IntoIter<Self, 4> {
        [
            Self::NotSelected,
            Self::Selected,
            Self::FromWorkspaceParam,
            Self::FilterFunction,
        ]
        .into_iter()
    }

    pub(crate) fn type_param(self) -> GenericArgument {
        match self {
            Self::NotSelected => parse_quote!(crate::scopes::type_params::ProfileNotSelected),
            Self::Selected => parse_quote!(crate::scopes::type_params::ProfileSelected),
            Self::FromWorkspaceParam => parse_quote!(
                crate::scopes::type_params::ProfileFromWorkspaceParam<
                    'key,
                    <
                        PKeys::WorkspaceParamsKMaybe as
                        peace_rt_model::cmd_context_params::KeyMaybe
                    >::Key
                >
            ),
            Self::FilterFunction => {
                parse_quote!(crate::scopes::type_params::ProfileFilterFn)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FlowIdSelection {
    Selected,
}

impl FlowIdSelection {
    pub(crate) fn iter() -> std::array::IntoIter<Self, 1> {
        [Self::Selected].into_iter()
    }

    pub(crate) fn type_param(&self) -> GenericArgument {
        match self {
            Self::Selected => parse_quote!(crate::scopes::type_params::FlowIdSelected),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WorkspaceParamsSelection {
    None,
    Some,
}

impl WorkspaceParamsSelection {
    pub(crate) fn iter() -> std::array::IntoIter<Self, 2> {
        [Self::None, Self::Some].into_iter()
    }

    pub(crate) fn type_param(&self) -> GenericArgument {
        match self {
            Self::None => parse_quote!(crate::scopes::type_params::WorkspaceParamsNone),
            Self::Some => parse_quote! {
                crate::scopes::type_params::WorkspaceParamsSome<
                    <
                        PKeys::WorkspaceParamsKMaybe
                        as peace_rt_model::cmd_context_params::KeyMaybe
                    >::Key
                >
            },
        }
    }

    pub(crate) fn deconstruct(self) -> FieldValue {
        match self {
            Self::None => parse_quote!(
                workspace_params_selection: crate::scopes::type_params::WorkspaceParamsNone
            ),
            Self::Some => parse_quote! {
                workspace_params_selection:
                    crate::scopes::type_params::WorkspaceParamsSome(workspace_params)
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ProfileParamsSelection {
    None,
    Some,
}

impl ProfileParamsSelection {
    pub(crate) fn iter() -> std::array::IntoIter<Self, 2> {
        [Self::None, Self::Some].into_iter()
    }

    pub(crate) fn type_param(&self) -> GenericArgument {
        match self {
            Self::None => parse_quote!(crate::scopes::type_params::ProfileParamsNone),
            Self::Some => parse_quote! {
                crate::scopes::type_params::ProfileParamsSome<
                    <
                        PKeys::ProfileParamsKMaybe
                        as peace_rt_model::cmd_context_params::KeyMaybe
                    >::Key
                >
            },
        }
    }

    pub(crate) fn deconstruct(self) -> FieldValue {
        match self {
            Self::None => {
                parse_quote!(
                    profile_params_selection: crate::scopes::type_params::ProfileParamsNone
                )
            }
            Self::Some => parse_quote! {
                profile_params_selection:
                    crate::scopes::type_params::ProfileParamsSome(profile_params)
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FlowParamsSelection {
    None,
    Some,
}

impl FlowParamsSelection {
    pub(crate) fn iter() -> std::array::IntoIter<Self, 2> {
        [Self::None, Self::Some].into_iter()
    }

    pub(crate) fn type_param(&self) -> GenericArgument {
        match self {
            Self::None => {
                parse_quote!(crate::scopes::type_params::FlowParamsNone)
            }
            Self::Some => parse_quote! {
                crate::scopes::type_params::FlowParamsSome<
                    <
                        PKeys::FlowParamsKMaybe
                        as peace_rt_model::cmd_context_params::KeyMaybe
                    >::Key
                >
            },
        }
    }

    pub(crate) fn deconstruct(self) -> FieldValue {
        match self {
            Self::None => {
                parse_quote!(flow_params_selection: crate::scopes::type_params::FlowParamsNone)
            }
            Self::Some => parse_quote! {
                flow_params_selection:
                    crate::scopes::type_params::FlowParamsSome(flow_params)
            },
        }
    }
}
