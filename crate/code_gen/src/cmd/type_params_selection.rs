use syn::{parse_quote, FieldValue, TypePath};

use crate::cmd::ProfileCount;

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

    pub(crate) fn type_param(self) -> TypePath {
        match self {
            Self::NotSelected => parse_quote!(crate::scopes::type_params::ProfileNotSelected),
            Self::Selected => parse_quote!(crate::scopes::type_params::ProfileSelected),
            Self::FromWorkspaceParam => parse_quote!(
                crate::scopes::type_params::ProfileFromWorkspaceParam<'key, WorkspaceParamsK>
            ),
            Self::FilterFunction => {
                parse_quote!(crate::scopes::type_params::ProfileFilterFn<'ctx>)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FlowSelection {
    Selected,
}

impl FlowSelection {
    pub(crate) fn iter() -> std::array::IntoIter<Self, 1> {
        [Self::Selected].into_iter()
    }

    pub(crate) fn type_param(&self) -> TypePath {
        match self {
            Self::Selected => {
                parse_quote!(crate::scopes::type_params::FlowSelected<'ctx, AppError>)
            }
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

    pub(crate) fn type_param(&self) -> TypePath {
        match self {
            Self::None => parse_quote!(crate::scopes::type_params::WorkspaceParamsNone),
            Self::Some => parse_quote! {
                crate::scopes::type_params::WorkspaceParamsSome<WorkspaceParamsK>
            },
        }
    }

    pub(crate) fn k_maybe_type_param(&self) -> TypePath {
        match self {
            Self::None => parse_quote!(peace_rt_model::params::KeyUnknown),
            Self::Some => parse_quote!(peace_rt_model::params::KeyKnown<WorkspaceParamsK>),
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

    pub(crate) fn type_param(&self, profile_count: ProfileCount) -> TypePath {
        match self {
            Self::None => {
                parse_quote!(crate::scopes::type_params::ProfileParamsNone)
            }
            Self::Some => match profile_count {
                ProfileCount::None => parse_quote!(crate::scopes::type_params::ProfileParamsNone),
                ProfileCount::One => parse_quote! {
                    crate::scopes::type_params::ProfileParamsSome<ProfileParamsK>
                },
                ProfileCount::Multiple => parse_quote! {
                    crate::scopes::type_params::ProfileParamsSomeMulti<ProfileParamsK>
                },
            },
        }
    }

    pub(crate) fn k_maybe_type_param(&self) -> TypePath {
        match self {
            Self::None => parse_quote!(peace_rt_model::params::KeyUnknown),
            Self::Some => parse_quote!(peace_rt_model::params::KeyKnown<ProfileParamsK>),
        }
    }

    pub(crate) fn deconstruct(self, profile_count: ProfileCount) -> FieldValue {
        match self {
            Self::None => {
                parse_quote!(
                    profile_params_selection: crate::scopes::type_params::ProfileParamsNone
                )
            }
            Self::Some => match profile_count {
                ProfileCount::None => parse_quote! {
                    profile_params_selection: crate::scopes::type_params::ProfileParamsNone
                },
                ProfileCount::One => parse_quote! {
                    profile_params_selection: crate::scopes::type_params::ProfileParamsSome(profile_params)
                },
                ProfileCount::Multiple => parse_quote! {
                    profile_params_selection: crate::scopes::type_params::ProfileParamsSomeMulti(profile_to_profile_params)
                },
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

    pub(crate) fn type_param(&self, profile_count: ProfileCount) -> TypePath {
        match self {
            Self::None => {
                parse_quote!(crate::scopes::type_params::FlowParamsNone)
            }
            Self::Some => match profile_count {
                ProfileCount::None => parse_quote!(crate::scopes::type_params::FlowParamsNone),
                ProfileCount::One => parse_quote! {
                    crate::scopes::type_params::FlowParamsSome<FlowParamsK>
                },
                ProfileCount::Multiple => parse_quote! {
                    crate::scopes::type_params::FlowParamsSomeMulti<FlowParamsK>
                },
            },
        }
    }

    pub(crate) fn k_maybe_type_param(&self) -> TypePath {
        match self {
            Self::None => parse_quote!(peace_rt_model::params::KeyUnknown),
            Self::Some => parse_quote!(peace_rt_model::params::KeyKnown<FlowParamsK>),
        }
    }

    pub(crate) fn deconstruct(self, profile_count: ProfileCount) -> FieldValue {
        match self {
            Self::None => {
                parse_quote!(flow_params_selection: crate::scopes::type_params::FlowParamsNone)
            }
            Self::Some => match profile_count {
                ProfileCount::None => parse_quote! {
                    flow_params_selection: crate::scopes::type_params::FlowParamsNone
                },
                ProfileCount::One => parse_quote! {
                    flow_params_selection: crate::scopes::type_params::FlowParamsSome(flow_params)
                },
                ProfileCount::Multiple => parse_quote! {
                    flow_params_selection: crate::scopes::type_params::FlowParamsSomeMulti(profile_to_flow_params)
                },
            },
        }
    }
}
