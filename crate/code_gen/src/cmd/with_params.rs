use syn::{parse_quote, Ident};

use crate::cmd::ParamsScope;

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
