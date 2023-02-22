use syn::{parse_quote, punctuated::Punctuated, Path, Token};

use crate::cmd::{FlowCount, ProfileCount, Scope};

use super::ParamsScope;

/// Appends profile / flow ID selection type parameters if applicable to the
/// given scope.
pub fn profile_and_flow_selection_push(
    type_params: &mut Punctuated<Path, Token![,]>,
    scope: Scope,
) {
    if scope.profile_count() == ProfileCount::One {
        type_params.push(parse_quote!(ProfileSelection));
    }
    if scope.flow_count() == FlowCount::One {
        type_params.push(parse_quote!(FlowIdSelection));
    }
}

/// Appends workspace / profile / flow params selection type parameters if
/// applicable to the given scope.
pub fn params_selection_push(type_params: &mut Punctuated<Path, Token![,]>, scope: Scope) {
    // Workspace params are supported by all scopes.
    type_params.push(parse_quote!(WorkspaceParamsSelection));

    if scope.profile_params_supported() {
        type_params.push(parse_quote!(ProfileParamsSelection));
    }

    if scope.flow_params_supported() {
        type_params.push(parse_quote!(FlowParamsSelection));
    }
}

/// Appends the type parameters for params selection for the `impl`.
///
/// For the `Workspace` params scope, the following are generated, if
/// applicable to the current command context builder scope:
///
/// * `ProfileParamsSelection`: To retain any existing profile params selection.
/// * `FlowParamsSelection`: To retain any existing flow params selection.
/// * `ProfileParamsKMaybe`: To retain the key for existing profile params
///   selection.
/// * `FlowParamsKMaybe`: To retain the key for existing flow params selection.
pub fn params_selection_maybe_push(
    type_params: &mut Punctuated<Path, Token![,]>,
    scope: Scope,
    params_scope: ParamsScope,
    params_key_known: bool,
) {
    match params_scope {
        ParamsScope::Workspace => {
            if params_key_known {
                type_params.push(parse_quote!(WorkspaceParamsK));
            }

            if scope.profile_params_supported() {
                type_params.push(parse_quote!(ProfileParamsSelection));
                type_params.push(parse_quote!(ProfileParamsKMaybe));
            }

            if scope.flow_params_supported() {
                type_params.push(parse_quote!(FlowParamsSelection));
                type_params.push(parse_quote!(FlowParamsKMaybe));
            }
        }
        ParamsScope::Profile => {
            // Workspace params are supported by all scopes.
            type_params.push(parse_quote!(WorkspaceParamsSelection));
            type_params.push(parse_quote!(WorkspaceParamsKMaybe));

            if scope.profile_params_supported() && params_key_known {
                type_params.push(parse_quote!(ProfileParamsK));
            }

            if scope.flow_params_supported() {
                type_params.push(parse_quote!(FlowParamsSelection));
                type_params.push(parse_quote!(FlowParamsKMaybe));
            }
        }
        ParamsScope::Flow => {
            // Workspace params are supported by all scopes.
            type_params.push(parse_quote!(WorkspaceParamsSelection));
            type_params.push(parse_quote!(WorkspaceParamsKMaybe));

            if scope.profile_params_supported() {
                type_params.push(parse_quote!(ProfileParamsSelection));
                type_params.push(parse_quote!(ProfileParamsKMaybe));
            }

            if scope.flow_params_supported() && params_key_known {
                type_params.push(parse_quote!(FlowParamsK));
            }
        }
    }
}

/// Appends the type parameters for params selection for the `ScopeBuilder` in
/// the `impl`.
///
/// For the `Workspace` params scope, the following are generated, if
/// applicable to the current command context builder scope:
///
/// * `WorkspaceParamsNone`: Indicates that the incoming that params selection
///   is none.
/// * `ProfileParamsSelection`: To retain any existing profile params selection.
/// * `FlowParamsSelection`: To retain any existing flow params selection.
pub fn params_selection_none_push(
    type_params: &mut Punctuated<Path, Token![,]>,
    scope: Scope,
    params_scope: ParamsScope,
) {
    match params_scope {
        ParamsScope::Workspace => {
            type_params.push(parse_quote!(
                crate::ctx::cmd_ctx_builder::WorkspaceParamsNone
            ));

            if scope.profile_params_supported() {
                type_params.push(parse_quote!(ProfileParamsSelection));
            }

            if scope.flow_params_supported() {
                type_params.push(parse_quote!(FlowParamsSelection));
            }
        }
        ParamsScope::Profile => {
            // Workspace params are supported by all scopes.
            type_params.push(parse_quote!(WorkspaceParamsSelection));

            if scope.profile_params_supported() {
                type_params.push(parse_quote!(crate::ctx::cmd_ctx_builder::ProfileParamsNone));
            }

            if scope.flow_params_supported() {
                type_params.push(parse_quote!(FlowParamsSelection));
            }
        }
        ParamsScope::Flow => {
            // Workspace params are supported by all scopes.
            type_params.push(parse_quote!(WorkspaceParamsSelection));

            if scope.profile_params_supported() {
                type_params.push(parse_quote!(ProfileParamsSelection));
            }

            if scope.flow_params_supported() {
                type_params.push(parse_quote!(crate::ctx::cmd_ctx_builder::FlowParamsNone));
            }
        }
    }
}

/// Appends the type parameters for params selection for the `ScopeBuilder` in
/// the `impl`.
///
/// For the `Workspace` params scope, the following are generated, if
/// applicable to the current command context builder scope:
///
/// * `WorkspaceParamsSome<WorkspaceParamsK>`: Indicates that the incoming
///   params selection is none.
/// * `ProfileParamsSelection`: To retain any existing profile params selection.
/// * `FlowParamsSelection`: To retain any existing flow params selection.
pub fn params_selection_some_push(
    type_params: &mut Punctuated<Path, Token![,]>,
    scope: Scope,
    params_scope: ParamsScope,
) {
    match params_scope {
        ParamsScope::Workspace => {
            type_params.push(parse_quote!(
                crate::ctx::cmd_ctx_builder::WorkspaceParamsSome<WorkspaceParamsK>
            ));

            if scope.profile_params_supported() {
                type_params.push(parse_quote!(ProfileParamsSelection));
            }

            if scope.flow_params_supported() {
                type_params.push(parse_quote!(FlowParamsSelection));
            }
        }
        ParamsScope::Profile => {
            // Workspace params are supported by all scopes.
            type_params.push(parse_quote!(WorkspaceParamsSelection));

            if scope.profile_params_supported() {
                type_params.push(parse_quote!(
                    crate::ctx::cmd_ctx_builder::ProfileParamsSome<ProfileParamsK>
                ));
            }

            if scope.flow_params_supported() {
                type_params.push(parse_quote!(FlowParamsSelection));
            }
        }
        ParamsScope::Flow => {
            // Workspace params are supported by all scopes.
            type_params.push(parse_quote!(WorkspaceParamsSelection));

            if scope.profile_params_supported() {
                type_params.push(parse_quote!(ProfileParamsSelection));
            }

            if scope.flow_params_supported() {
                type_params.push(parse_quote!(
                    crate::ctx::cmd_ctx_builder::FlowParamsSome<FlowParamsK>
                ));
            }
        }
    }
}

/// Appends the type parameters for params selection for the `ScopeBuilder` in
/// the `impl`.
///
/// For the `Workspace` params scope, the following are generated, if
/// applicable to the current command context builder scope:
///
/// * `KeyUnknown`: Indicates that the incoming params key is known.
/// * `ProfileParamsKMaybe`: To retain any existing profile params key.
/// * `FlowParamsKMaybe`: To retain any existing flow params key.
pub fn params_key_unknown_push(
    type_params: &mut Punctuated<Path, Token![,]>,
    scope: Scope,
    params_scope: ParamsScope,
) {
    match params_scope {
        ParamsScope::Workspace => {
            type_params.push(parse_quote!(peace_rt_model::cmd_context_params::KeyUnknown));

            if scope.profile_params_supported() {
                type_params.push(parse_quote!(ProfileParamsKMaybe));
            }

            if scope.flow_params_supported() {
                type_params.push(parse_quote!(FlowParamsKMaybe));
            }
        }
        ParamsScope::Profile => {
            // Workspace params are supported by all scopes.
            type_params.push(parse_quote!(WorkspaceParamsKMaybe));

            if scope.profile_params_supported() {
                type_params.push(parse_quote!(peace_rt_model::cmd_context_params::KeyUnknown));
            }

            if scope.flow_params_supported() {
                type_params.push(parse_quote!(FlowParamsKMaybe));
            }
        }
        ParamsScope::Flow => {
            // Workspace params are supported by all scopes.
            type_params.push(parse_quote!(WorkspaceParamsKMaybe));

            if scope.profile_params_supported() {
                type_params.push(parse_quote!(ProfileParamsKMaybe));
            }

            if scope.flow_params_supported() {
                type_params.push(parse_quote!(peace_rt_model::cmd_context_params::KeyUnknown));
            }
        }
    }
}

/// Appends the type parameters for params selection for the `ScopeBuilder` in
/// the `impl`.
///
/// For the `Workspace` params scope, the following are generated, if
/// applicable to the current command context builder scope:
///
/// * `KeyKnown<WorkspaceParamsK>`: Indicates that the outgoing params key is
///   known.
/// * `ProfileParamsKMaybe`: To retain any existing profile params key.
/// * `FlowParamsKMaybe`: To retain any existing flow params key.
pub fn params_key_known_push(
    type_params: &mut Punctuated<Path, Token![,]>,
    scope: Scope,
    params_scope: ParamsScope,
) {
    match params_scope {
        ParamsScope::Workspace => {
            type_params.push(parse_quote!(
                peace_rt_model::cmd_context_params::KeyKnown<WorkspaceParamsK>
            ));

            if scope.profile_params_supported() {
                type_params.push(parse_quote!(ProfileParamsKMaybe));
            }

            if scope.flow_params_supported() {
                type_params.push(parse_quote!(FlowParamsKMaybe));
            }
        }
        ParamsScope::Profile => {
            // Workspace params are supported by all scopes.
            type_params.push(parse_quote!(WorkspaceParamsKMaybe));

            if scope.profile_params_supported() {
                type_params.push(parse_quote!(
                    peace_rt_model::cmd_context_params::KeyKnown<ProfileParamsK>
                ));
            }

            if scope.flow_params_supported() {
                type_params.push(parse_quote!(FlowParamsKMaybe));
            }
        }
        ParamsScope::Flow => {
            // Workspace params are supported by all scopes.
            type_params.push(parse_quote!(WorkspaceParamsKMaybe));

            if scope.profile_params_supported() {
                type_params.push(parse_quote!(ProfileParamsKMaybe));
            }

            if scope.flow_params_supported() {
                type_params.push(parse_quote!(
                    peace_rt_model::cmd_context_params::KeyKnown<FlowParamsK>
                ));
            }
        }
    }
}
