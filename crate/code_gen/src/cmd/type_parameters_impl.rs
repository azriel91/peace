use syn::{parse_quote, punctuated::Punctuated, GenericArgument, Token};

use crate::cmd::{FlowCount, ProfileCount, Scope};

/// Appends profile / flow ID selection type parameters if applicable to the
/// given scope.
pub fn profile_and_flow_selection_push(
    type_params: &mut Punctuated<GenericArgument, Token![,]>,
    scope: Scope,
) {
    match scope.profile_count() {
        ProfileCount::None => {}
        ProfileCount::One | ProfileCount::Multiple => {
            type_params.push(parse_quote!(ProfileSelection));
        }
    }
    if scope.flow_count() == FlowCount::One {
        type_params.push(parse_quote!(FlowSelection));
    }
}
