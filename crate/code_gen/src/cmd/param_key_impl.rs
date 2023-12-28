use syn::{parse_quote, punctuated::Punctuated, token::Comma, Path, Token, WherePredicate};

use crate::cmd::{ParamsScope, Scope};

pub fn unknown_predicates(
    scope: Scope,
    params_scope: ParamsScope,
) -> Punctuated<WherePredicate, Comma> {
    let mut predicates = Punctuated::<WherePredicate, Token![,]>::new();
    let params_module: Path = parse_quote!(peace_rt_model::params);

    match params_scope {
        ParamsScope::Workspace => {
            if scope.profile_params_supported() {
                predicates.push(parse_quote! {
                    ProfileParamsKMaybe: #params_module::KeyMaybe
                });
            }

            if scope.flow_params_supported() {
                predicates.push(parse_quote! {
                    FlowParamsKMaybe: #params_module::KeyMaybe
                });
            }
        }
        ParamsScope::Profile => {
            // Workspace params are supported by all scopes.
            predicates.push(parse_quote! {
                WorkspaceParamsKMaybe: #params_module::KeyMaybe
            });

            if scope.flow_params_supported() {
                predicates.push(parse_quote! {
                    FlowParamsKMaybe: #params_module::KeyMaybe
                });
            }
        }
        ParamsScope::Flow => {
            // Workspace params are supported by all scopes.
            predicates.push(parse_quote! {
                WorkspaceParamsKMaybe: #params_module::KeyMaybe
            });

            if scope.profile_params_supported() {
                predicates.push(parse_quote! {
                    ProfileParamsKMaybe: #params_module::KeyMaybe
                });
            }
        }
    }

    predicates
}

pub fn known_predicates(
    scope: Scope,
    params_scope: ParamsScope,
) -> Punctuated<WherePredicate, Comma> {
    let mut predicates = Punctuated::<WherePredicate, Token![,]>::new();
    let params_module: Path = parse_quote!(peace_rt_model::params);

    match params_scope {
        ParamsScope::Workspace => {
            predicates.push(parse_quote! {
                WorkspaceParamsK:
                    Clone + std::fmt::Debug + Eq + std::hash::Hash + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + Unpin + 'static
            });

            if scope.profile_params_supported() {
                predicates.push(parse_quote! {
                    ProfileParamsKMaybe: #params_module::KeyMaybe
                });
            }

            if scope.flow_params_supported() {
                predicates.push(parse_quote! {
                    FlowParamsKMaybe: #params_module::KeyMaybe
                });
            }
        }
        ParamsScope::Profile => {
            // Workspace params are supported by all scopes.
            predicates.push(parse_quote! {
                WorkspaceParamsKMaybe: #params_module::KeyMaybe
            });

            if scope.profile_params_supported() {
                predicates.push(parse_quote! {
                    ProfileParamsK:
                        Clone + std::fmt::Debug + Eq + std::hash::Hash + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + Unpin + 'static
                });
            }

            if scope.flow_params_supported() {
                predicates.push(parse_quote! {
                    FlowParamsKMaybe: #params_module::KeyMaybe
                });
            }
        }
        ParamsScope::Flow => {
            // Workspace params are supported by all scopes.
            predicates.push(parse_quote! {
                WorkspaceParamsKMaybe: #params_module::KeyMaybe
            });

            if scope.profile_params_supported() {
                predicates.push(parse_quote! {
                    ProfileParamsKMaybe: #params_module::KeyMaybe
                });
            }

            if scope.flow_params_supported() {
                predicates.push(parse_quote! {
                    FlowParamsK:
                        Clone + std::fmt::Debug + Eq + std::hash::Hash + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + Unpin + 'static
                });
            }
        }
    }

    predicates
}
