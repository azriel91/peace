use quote::quote;
use syn::parse_macro_input;

use crate::cmd::scope_struct::ScopeStruct;

pub use self::{
    flow_count::FlowCount, profile_count::ProfileCount, scope::Scope,
    struct_definition::struct_definition,
};

mod flow_count;
mod profile_count;
mod scope;
mod scope_struct;
mod struct_definition;

/// Generates the command context builder implementation for the given scope.
pub fn cmd_ctx_builder_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut scope_struct = parse_macro_input!(input as ScopeStruct);

    let struct_definition = struct_definition(&mut scope_struct);

    quote! {
        #struct_definition
    }
    .into()
}
