use syn::{parse_quote, Ident, Path};

use crate::cmd::ParamsScope;

use super::CmdCtxBuilderTypeBuilder;

pub(crate) fn params_selection_types_none(params_scope: ParamsScope) -> (Ident, Ident, Ident) {
    let (params_keys_assoc_type, params_selection_assoc_type, params_selection_struct): (
        Ident,
        Ident,
        Ident,
    ) = match params_scope {
        ParamsScope::Workspace => (
            parse_quote!(WorkspaceParamsKMaybe),
            parse_quote!(WorkspaceParamsSelection),
            parse_quote!(WorkspaceParamsNone),
        ),
        ParamsScope::Profile => (
            parse_quote!(ProfileParamsKMaybe),
            parse_quote!(ProfileParamsSelection),
            parse_quote!(ProfileParamsNone),
        ),
        ParamsScope::Flow => (
            parse_quote!(FlowParamsKMaybe),
            parse_quote!(FlowParamsSelection),
            parse_quote!(FlowParamsNone),
        ),
    };
    (
        params_keys_assoc_type,
        params_selection_assoc_type,
        params_selection_struct,
    )
}

pub(crate) fn params_selection_types_some(params_scope: ParamsScope) -> (Ident, Ident, Ident) {
    let (params_keys_assoc_type, params_selection_assoc_type, params_selection_struct): (
        Ident,
        Ident,
        Ident,
    ) = match params_scope {
        ParamsScope::Workspace => (
            parse_quote!(WorkspaceParamsKMaybe),
            parse_quote!(WorkspaceParamsSelection),
            parse_quote!(WorkspaceParamsSome),
        ),
        ParamsScope::Profile => (
            parse_quote!(ProfileParamsKMaybe),
            parse_quote!(ProfileParamsSelection),
            parse_quote!(ProfileParamsSome),
        ),
        ParamsScope::Flow => (
            parse_quote!(FlowParamsKMaybe),
            parse_quote!(FlowParamsSelection),
            parse_quote!(FlowParamsSome),
        ),
    };
    (
        params_keys_assoc_type,
        params_selection_assoc_type,
        params_selection_struct,
    )
}

pub(crate) fn cmd_ctx_builder_return_type_with_params_key_some(
    scope_builder_name: Ident,
    params_scope: ParamsScope,
) -> Path {
    let cmd_ctx_builder_type_builder = CmdCtxBuilderTypeBuilder::new(scope_builder_name);
    match params_scope {
        ParamsScope::Workspace => cmd_ctx_builder_type_builder
            .with_workspace_params_k_maybe(parse_quote!(
                peace_rt_model::params::KeyKnown<WorkspaceParamsK>
            ))
            .with_workspace_params_selection(parse_quote!(
                crate::scopes::type_params::WorkspaceParamsSome<WorkspaceParamsK>
            )),
        ParamsScope::Profile => cmd_ctx_builder_type_builder
            .with_profile_params_k_maybe(parse_quote!(
                peace_rt_model::params::KeyKnown<ProfileParamsK>
            ))
            .with_profile_params_selection(parse_quote!(
                crate::scopes::type_params::ProfileParamsSome<ProfileParamsK>
            )),
        ParamsScope::Flow => cmd_ctx_builder_type_builder
            .with_flow_params_k_maybe(parse_quote!(peace_rt_model::params::KeyKnown<FlowParamsK>))
            .with_flow_params_selection(parse_quote!(
                crate::scopes::type_params::FlowParamsSome<FlowParamsK>
            )),
    }
    .build()
}
