use syn::{parse_quote, FieldValue, Path};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ProfileSelection {
    Selected,
    FromWorkspaceParam,
}

impl ProfileSelection {
    pub(crate) fn iter() -> std::array::IntoIter<Self, 2> {
        [Self::Selected, Self::FromWorkspaceParam].into_iter()
    }

    pub(crate) fn type_param(self) -> Path {
        match self {
            Self::Selected => parse_quote!(crate::ctx::cmd_ctx_builder::ProfileSelected),
            Self::FromWorkspaceParam => parse_quote!(
                crate::ctx::cmd_ctx_builder::ProfileFromWorkspaceParam<
                    'key,
                    <
                        PKeys::WorkspaceParamsKMaybe as
                        peace_rt_model::cmd_context_params::KeyMaybe
                    >::Key
                >
            ),
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

    pub(crate) fn type_param(&self) -> Path {
        match self {
            Self::Selected => parse_quote!(crate::ctx::cmd_ctx_builder::FlowIdSelected),
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

    pub(crate) fn type_param(&self) -> Path {
        match self {
            Self::None => parse_quote!(crate::ctx::cmd_ctx_builder::WorkspaceParamsNone),
            Self::Some => parse_quote! {
                crate::ctx::cmd_ctx_builder::WorkspaceParamsSome<
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
                workspace_params_selection: crate::ctx::cmd_ctx_builder::WorkspaceParamsNone
            ),
            Self::Some => parse_quote! {
                workspace_params_selection:
                    crate::ctx::cmd_ctx_builder::WorkspaceParamsSome(workspace_params)
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

    pub(crate) fn type_param(&self) -> Path {
        match self {
            Self::None => parse_quote!(crate::ctx::cmd_ctx_builder::ProfileParamsNone),
            Self::Some => parse_quote! {
                crate::ctx::cmd_ctx_builder::ProfileParamsSome<
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
            Self::None => parse_quote!(
                profile_params_selection: crate::ctx::cmd_ctx_builder::ProfileParamsNone
            ),
            Self::Some => parse_quote! {
                profile_params_selection:
                    crate::ctx::cmd_ctx_builder::ProfileParamsSome(profile_params)
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

    pub(crate) fn type_param(&self) -> Path {
        match self {
            Self::None => {
                parse_quote!(crate::ctx::cmd_ctx_builder::FlowParamsNone)
            }
            Self::Some => parse_quote! {
                crate::ctx::cmd_ctx_builder::FlowParamsSome<
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
                parse_quote!(flow_params_selection: crate::ctx::cmd_ctx_builder::FlowParamsNone)
            }
            Self::Some => parse_quote! {
                flow_params_selection:
                    crate::ctx::cmd_ctx_builder::FlowParamsSome(flow_params)
            },
        }
    }
}
