use syn::{parse_quote, punctuated::Punctuated, token::Comma, Token, WherePredicate};

use crate::cmd::{ParamsScope, Scope};

pub fn known_predicates(
    scope: Scope,
    params_scope: ParamsScope,
) -> Punctuated<WherePredicate, Comma> {
    let mut predicates = Punctuated::<WherePredicate, Token![,]>::new();
    match params_scope {
        ParamsScope::Workspace => {
            predicates.push(parse_quote! {
                WorkspaceParamsK:
                    Clone + std::fmt::Debug + Eq + std::hash::Hash + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + Unpin + 'static
            });
        }
        ParamsScope::Profile => {
            if scope.profile_params_supported() {
                predicates.push(parse_quote! {
                    ProfileParamsK:
                        Clone + std::fmt::Debug + Eq + std::hash::Hash + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + Unpin + 'static
                });
            }
        }
        ParamsScope::Flow => {
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
