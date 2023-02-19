use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, punctuated::Punctuated, Fields, FieldsNamed, Token};

use crate::cmd::scope_struct::ScopeStruct;

pub use self::{flow_count::FlowCount, profile_count::ProfileCount, scope::Scope};

mod flow_count;
mod profile_count;
mod scope;
mod scope_struct;

/// Generates the command context builder implementation for the given scope.
pub fn cmd_ctx_builder_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut scope_struct = parse_macro_input!(input as ScopeStruct);

    let struct_definition = struct_definition(&mut scope_struct);

    quote! {
        #struct_definition
    }
    .into()
}

pub fn struct_definition(scope_struct: &mut ScopeStruct) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();

    scope_struct.item_struct_mut().generics = {
        let mut type_params = Punctuated::<Ident, Token![,]>::new();

        type_parameters::profile_and_flow_selection_push(&mut type_params, scope);
        type_parameters::params_selection_push(&mut type_params, scope);

        parse_quote!(<#type_params>)
    };

    scope_struct.item_struct_mut().fields = {
        let mut fields: FieldsNamed = parse_quote!({});

        fields::profile_and_flow_selection_push(&mut fields, scope);
        fields::params_selection_push(&mut fields, scope);

        Fields::from(fields)
    };

    scope_struct.item_struct().to_token_stream()
}

mod type_parameters {
    use proc_macro2::{Ident, Span};
    use syn::{punctuated::Punctuated, Token};

    use crate::cmd::{FlowCount, ProfileCount, Scope};

    /// Appends profile / flow ID selection type parameters if applicable to the
    /// given scope.
    pub fn profile_and_flow_selection_push(
        type_params: &mut Punctuated<Ident, Token![,]>,
        scope: Scope,
    ) {
        if scope.profile_count() == ProfileCount::One {
            type_params.push(Ident::new("ProfileSelection", Span::call_site()));
        }
        if scope.flow_count() == FlowCount::One {
            type_params.push(Ident::new("FlowIdSelection", Span::call_site()));
        }
    }

    /// Appends workspace / profile / flow params selection type parameters if
    /// applicable to the given scope.
    pub fn params_selection_push(type_params: &mut Punctuated<Ident, Token![,]>, scope: Scope) {
        // Workflow params are supported by all scopes.
        type_params.push(Ident::new("WorkflowParamsSelection", Span::call_site()));

        if scope.profile_params_supported() {
            type_params.push(Ident::new("ProfileParamsSelection", Span::call_site()));
        }

        if scope.flow_params_supported() {
            type_params.push(Ident::new("FlowParamsSelection", Span::call_site()));
        }
    }
}

mod fields {
    use syn::{parse_quote, FieldsNamed};

    use crate::cmd::{FlowCount, ProfileCount, Scope};

    /// Appends profile / flow ID selection type parameters if applicable to the
    /// given scope.
    pub fn profile_and_flow_selection_push(fields_named: &mut FieldsNamed, scope: Scope) {
        if scope.profile_count() == ProfileCount::One {
            let fields: FieldsNamed = parse_quote!({
                /// The profile this command operates on.
                pub(crate) profile_selection: ProfileSelection
            });
            fields_named.named.extend(fields.named);
        }
        if scope.flow_count() == FlowCount::One {
            let fields: FieldsNamed = parse_quote!({
                /// Identifier or name of the chosen process flow.
                pub(crate) flow_id_selection: FlowIdSelection
            });
            fields_named.named.extend(fields.named);
        }
    }

    /// Appends workspace / profile / flow params selection type parameters if
    /// applicable to the given scope.
    pub fn params_selection_push(fields_named: &mut FieldsNamed, scope: Scope) {
        // Workflow params are supported by all scopes.
        let fields: FieldsNamed = parse_quote!({
            /// Workspace parameters.
            pub(crate) workspace_params_selection: WorkflowParamsSelection
        });
        fields_named.named.extend(fields.named);

        if scope.profile_params_supported() {
            let fields: FieldsNamed = parse_quote!({
                /// Profile parameters.
                pub(crate) profile_params_selection: ProfileParamsSelection
            });
            fields_named.named.extend(fields.named);
        }

        if scope.flow_params_supported() {
            let fields: FieldsNamed = parse_quote!({
                /// Flow parameters.
                pub(crate) flow_params_selection: FlowParamsSelection
            });
            fields_named.named.extend(fields.named);
        }
    }
}
