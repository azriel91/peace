use quote::quote;
use syn::parse_macro_input;

use crate::cmd::scope_struct::ScopeStruct;

pub use self::{
    flow_count::FlowCount, impl_build::impl_build, impl_constructor::impl_constructor,
    impl_params_deserialize::impl_params_deserialize, impl_params_merge::impl_params_merge,
    impl_with_flow_id::impl_with_flow_id, impl_with_param::impl_with_param,
    impl_with_profile::impl_with_profile, params_scope::ParamsScope, profile_count::ProfileCount,
    scope::Scope, struct_definition::struct_definition,
};

mod flow_count;
mod impl_build;
mod impl_constructor;
mod impl_params_deserialize;
mod impl_params_merge;
mod impl_with_flow_id;
mod impl_with_param;
mod impl_with_profile;
mod param_key_impl;
mod params_scope;
mod profile_count;
mod scope;
mod scope_struct;
mod struct_definition;
mod type_parameters_impl;

pub(crate) mod type_params_selection;

/// Generates the command context builder implementation for the given scope.
pub fn cmd_ctx_builder_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut scope_struct = parse_macro_input!(input as ScopeStruct);

    let struct_definition = struct_definition(&mut scope_struct);
    let impl_constructor = impl_constructor(&scope_struct);
    let impl_with_param = impl_with_param(&scope_struct);

    let impl_with_profile = if scope_struct.scope().profile_count() == ProfileCount::One {
        impl_with_profile(&scope_struct)
    } else {
        // `with_profile` is not supported.
        proc_macro2::TokenStream::new()
    };

    let impl_with_flow_id = impl_with_flow_id(&scope_struct);

    let impl_build = impl_build(&scope_struct);

    let impl_params_deserialize = impl_params_deserialize(&scope_struct);
    let impl_params_merge = impl_params_merge(&scope_struct);

    let tokens = quote! {
        #struct_definition

        #impl_constructor

        #impl_with_param

        #impl_with_profile

        #impl_with_flow_id

        #impl_build

        #impl_params_deserialize

        #impl_params_merge
    };

    tokens.into()
}
