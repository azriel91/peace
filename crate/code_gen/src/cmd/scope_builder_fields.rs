use syn::{parse_quote, punctuated::Punctuated, token::Comma, FieldValue, Pat, Token};

use crate::cmd::{FlowCount, ParamsScope, ProfileCount, Scope};

pub(crate) fn passthrough(
    scope: Scope,
    params_scope: ParamsScope,
) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    match scope.profile_count() {
        ProfileCount::None => {}
        ProfileCount::One | ProfileCount::Multiple => {
            field_values.push(parse_quote!(profile_selection));
        }
    }
    if scope.flow_count() == FlowCount::One {
        field_values.push(parse_quote!(flow_selection));
    }

    match params_scope {
        ParamsScope::Workspace => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Profile => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Flow => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
    }

    field_values
}

pub(crate) fn params_none(
    scope: Scope,
    params_scope: ParamsScope,
) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    match scope.profile_count() {
        ProfileCount::None => {}
        ProfileCount::One | ProfileCount::Multiple => {
            field_values.push(parse_quote!(profile_selection));
        }
    }
    if scope.flow_count() == FlowCount::One {
        field_values.push(parse_quote!(flow_selection));
    }

    match params_scope {
        ParamsScope::Workspace => {
            field_values.push(parse_quote!(
                workspace_params_selection: crate::scopes::type_params::WorkspaceParamsNone
            ));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Profile => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(
                    profile_params_selection: crate::scopes::type_params::ProfileParamsNone
                ));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Flow => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(
                    flow_params_selection: crate::scopes::type_params::FlowParamsNone
                ));
            }
        }
    }

    field_values
}

pub(crate) fn params_some(scope: Scope, params_scope: ParamsScope) -> Punctuated<Pat, Comma> {
    let mut field_values = Punctuated::<Pat, Token![,]>::new();
    match scope.profile_count() {
        ProfileCount::None => {}
        ProfileCount::One | ProfileCount::Multiple => {
            field_values.push(parse_quote!(profile_selection));
        }
    }
    if scope.flow_count() == FlowCount::One {
        field_values.push(parse_quote!(flow_selection));
    }

    match params_scope {
        ParamsScope::Workspace => {
            field_values.push(parse_quote!(mut workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Profile => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(mut profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Flow => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(mut flow_params_selection));
            }
        }
    }

    field_values
}

pub(crate) fn params_some_new(
    scope: Scope,
    params_scope: ParamsScope,
) -> Punctuated<FieldValue, Comma> {
    let mut field_values = Punctuated::<FieldValue, Token![,]>::new();
    match scope.profile_count() {
        ProfileCount::None => {}
        ProfileCount::One | ProfileCount::Multiple => {
            field_values.push(parse_quote!(profile_selection));
        }
    }
    if scope.flow_count() == FlowCount::One {
        field_values.push(parse_quote!(flow_selection));
    }

    match params_scope {
        ParamsScope::Workspace => {
            field_values.push(parse_quote! {
                workspace_params_selection:
                    crate::scopes::type_params::WorkspaceParamsSome(params_map)
            });
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Profile => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote! {
                    profile_params_selection:
                        crate::scopes::type_params::ProfileParamsSome(params_map)
                });
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote!(flow_params_selection));
            }
        }
        ParamsScope::Flow => {
            field_values.push(parse_quote!(workspace_params_selection));
            if scope.profile_params_supported() {
                field_values.push(parse_quote!(profile_params_selection));
            }
            if scope.flow_params_supported() {
                field_values.push(parse_quote! {
                    flow_params_selection:
                        crate::scopes::type_params::FlowParamsSome(params_map)
                });
            }
        }
    }

    field_values
}
