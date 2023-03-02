use quote::ToTokens;
use syn::{parse_quote, punctuated::Punctuated, Fields, FieldsNamed, GenericArgument, Token};

use crate::cmd::{scope_struct::ScopeStruct, type_parameters_impl};

/// Generates the struct definition for a scope struct builder.
///
/// This includes:
///
/// * Type parameters for profile and flow selection.
/// * Type parameters for workspace, profile, and flow params selection.
/// * Fields for each of the above.
///
/// For example, the `SingleProfileSingleFlowBuilder` will look like the
/// following:
///
/// ```rust,ignore
/// pub struct SingleProfileSingleFlowBuilder<
///     ProfileSelection,
///     FlowSelection,
///     WorkspaceParamsSelection,
///     ProfileParamsSelection,
///     FlowParamsSelection,
/// > {
///     /// The profile this command operates on.
///     pub(crate) profile_selection: ProfileSelection,
///     /// Identifier or name of the chosen process flow.
///     pub(crate) flow_selection: FlowSelection,
///     /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
///     /// [`FlowParams`] deserialization.
///     ///
///     /// [`WorkspaceParams`]: peace_rt_model::cmd_context_params::WorkspaceParams
///     /// [`ProfileParams`]: peace_rt_model::cmd_context_params::ProfileParams
///     /// [`FlowParams`]: peace_rt_model::cmd_context_params::FlowParams
///     pub(crate) params_type_regs_builder: ParamsTypeRegsBuilder<PKeys>,
///     /// Workspace parameters.
///     pub(crate) workspace_params_selection: WorkspaceParamsSelection,
///     /// Profile parameters.
///     pub(crate) profile_params_selection: ProfileParamsSelection,
///     /// Flow parameters.
///     pub(crate) flow_params_selection: FlowParamsSelection,
/// }
/// ```
pub fn struct_definition(scope_struct: &mut ScopeStruct) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();

    scope_struct.item_struct_mut().generics = {
        let mut type_params = Punctuated::<GenericArgument, Token![,]>::new();

        type_params.push(parse_quote!(E));
        type_parameters_impl::profile_and_flow_selection_push(&mut type_params, scope);
        type_parameters_impl::params_selection_push(&mut type_params, scope);

        // <
        //     E,
        //
        //     // SingleProfile / MultiProfile
        //     ProfileSelection,
        //     // SingleFlow
        //     FlowSelection,
        //
        //     PKeys,
        //     WorkspaceParamsSelection,
        //     // SingleProfile / MultiProfile
        //     ProfileParamsSelection,
        //     // SingleFlow
        //     FlowParamsSelection,
        // >
        parse_quote!(<#type_params>)
    };
    scope_struct.item_struct_mut().generics.where_clause = Some(parse_quote! {
        where
            PKeys: peace_rt_model::cmd_context_params::ParamsKeys + 'static,
    });

    scope_struct.item_struct_mut().fields = {
        let mut fields: FieldsNamed = parse_quote!({});

        fields::profile_and_flow_selection_push(&mut fields, scope);
        fields::params_selection_push(&mut fields, scope);
        fields::marker_push(&mut fields);

        Fields::from(fields)
    };

    scope_struct.item_struct().to_token_stream()
}

mod fields {
    use syn::{parse_quote, FieldsNamed};

    use crate::cmd::{FlowCount, ProfileCount, Scope};

    /// Appends profile / flow ID selection type parameters if applicable to the
    /// given scope.
    pub fn profile_and_flow_selection_push(fields_named: &mut FieldsNamed, scope: Scope) {
        match scope.profile_count() {
            ProfileCount::None => {}
            ProfileCount::One | ProfileCount::Multiple => {
                let fields: FieldsNamed = parse_quote!({
                    /// The profile this command operates on.
                    pub(crate) profile_selection: ProfileSelection
                });
                fields_named.named.extend(fields.named);
            }
        }
        if scope.flow_count() == FlowCount::One {
            let fields: FieldsNamed = parse_quote!({
                /// Identifier or name of the chosen process flow.
                pub(crate) flow_selection: FlowSelection
            });
            fields_named.named.extend(fields.named);
        }
    }

    /// Appends workspace / profile / flow params selection type parameters if
    /// applicable to the given scope.
    pub fn params_selection_push(fields_named: &mut FieldsNamed, scope: Scope) {
        // Workspace params are supported by all scopes.
        let fields: FieldsNamed = parse_quote!({
            /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
            /// [`FlowParams`] deserialization.
            ///
            /// [`WorkspaceParams`]: peace_rt_model::cmd_context_params::WorkspaceParams
            /// [`ProfileParams`]: peace_rt_model::cmd_context_params::ProfileParams
            /// [`FlowParams`]: peace_rt_model::cmd_context_params::FlowParams
            pub(crate) params_type_regs_builder:
                peace_rt_model::cmd_context_params::ParamsTypeRegsBuilder<PKeys>
        });
        fields_named.named.extend(fields.named);

        // Workspace params are supported by all scopes.
        let fields: FieldsNamed = parse_quote!({
            /// Workspace parameters.
            pub(crate) workspace_params_selection: WorkspaceParamsSelection
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

    /// Appends a `marker: PhantomData` field to the given fields..
    pub fn marker_push(fields_named: &mut FieldsNamed) {
        let fields_marker: FieldsNamed = parse_quote!({
            /// Marker.
            pub(crate) marker: std::marker::PhantomData<E>
        });
        fields_named.named.extend(fields_marker.named);
    }
}
