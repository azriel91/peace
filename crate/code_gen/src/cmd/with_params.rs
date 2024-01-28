use syn::{parse_quote, Ident, Path};

use crate::cmd::{
    type_params_selection::{FlowParamsSelection, ProfileParamsSelection},
    CmdCtxBuilderTypeBuilder, ParamsScope, ScopeStruct,
};

pub(crate) fn cmd_ctx_builder_with_params_selected(
    scope_builder_name: &Ident,
    scope_struct: &ScopeStruct,
    params_scope: ParamsScope,
) -> Path {
    let builder_type_builder = CmdCtxBuilderTypeBuilder::new(scope_builder_name.clone());
    let profile_count = scope_struct.scope().profile_count();
    match params_scope {
        ParamsScope::Workspace => builder_type_builder
            .with_workspace_params_k_maybe(parse_quote!(
                peace_rt_model::params::KeyKnown<WorkspaceParamsK>
            ))
            .with_workspace_params_selection(parse_quote!(
                crate::scopes::type_params::WorkspaceParamsSome<WorkspaceParamsK>
            )),
        ParamsScope::Profile => {
            let profile_params_selection = ProfileParamsSelection::Some;
            builder_type_builder
                .with_profile_params_k_maybe(profile_params_selection.k_maybe_type_param())
                .with_profile_params_selection(profile_params_selection.type_param(profile_count))
        }
        ParamsScope::Flow => {
            let flow_params_selection = FlowParamsSelection::Some;
            builder_type_builder
                .with_flow_params_k_maybe(flow_params_selection.k_maybe_type_param())
                .with_flow_params_selection(flow_params_selection.type_param(profile_count))
        }
    }
    .build()
}
