use quote::ToTokens;
use syn::{parse_quote, Fields, FieldsNamed};

use crate::cmd::scope_struct::ScopeStruct;

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
/// pub struct SingleProfileSingleFlowBuilder<CmdCtxBuilderTypeParamsT>
/// where
///     CmdCtxBuilderTypeParamsT: CmdCtxBuilderTypeParams
/// {
///     /// The profile this command operates on.
///     pub(crate) profile_selection: CmdCtxBuilderTypeParamsT::ProfileSelection,
///     /// Identifier or name of the chosen process flow.
///     pub(crate) flow_selection: CmdCtxBuilderTypeParamsT::FlowSelection,
///     /// Type registries for [`WorkspaceParams`], [`ProfileParams`], and
///     /// [`FlowParams`] deserialization.
///     ///
///     /// [`WorkspaceParams`]: peace_rt_model::params::WorkspaceParams
///     /// [`ProfileParams`]: peace_rt_model::params::ProfileParams
///     /// [`FlowParams`]: peace_rt_model::params::FlowParams
///     pub(crate) params_type_regs_builder:
///         peace_rt_model::params::ParamsTypeRegsBuilder<CmdCtxBuilderTypeParamsT::PKeys>,
///     /// Workspace parameters.
///     pub(crate) workspace_params_selection: CmdCtxBuilderTypeParamsT::WorkspaceParamsSelection,
///     /// Profile parameters.
///     pub(crate) profile_params_selection: CmdCtxBuilderTypeParamsT::ProfileParamsSelection,
///     /// Flow parameters.
///     pub(crate) flow_params_selection: CmdCtxBuilderTypeParamsT::FlowParamsSelection,
///     /// Map of item ID to its parameters. `TypeMap<ItemId, AnySpecRtBoxed>` newtype.
///     pub(crate) params_specs_provided: peace_params::ParamsSpecs,
/// }
/// ```
pub fn struct_definition(scope_struct: &mut ScopeStruct) -> proc_macro2::TokenStream {
    let scope = scope_struct.scope();

    scope_struct.item_struct_mut().fields = {
        let mut fields: FieldsNamed = parse_quote!({});

        fields::profile_and_flow_selection_push(&mut fields, scope);
        fields::params_selection_push(&mut fields, scope);
        fields::params_specs_push(&mut fields, scope);

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
                    pub(crate) profile_selection: CmdCtxBuilderTypeParamsT::ProfileSelection
                });
                fields_named.named.extend(fields.named);
            }
        }
        if scope.flow_count() == FlowCount::One {
            let fields: FieldsNamed = parse_quote!({
                /// Identifier or name of the chosen process flow.
                pub(crate) flow_selection: CmdCtxBuilderTypeParamsT::FlowSelection
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
            /// [`WorkspaceParams`]: peace_rt_model::params::WorkspaceParams
            /// [`ProfileParams`]: peace_rt_model::params::ProfileParams
            /// [`FlowParams`]: peace_rt_model::params::FlowParams
            pub(crate) params_type_regs_builder:
                peace_rt_model::params::ParamsTypeRegsBuilder<CmdCtxBuilderTypeParamsT::ParamsKeys>
        });
        fields_named.named.extend(fields.named);

        // Workspace params are supported by all scopes.
        let fields: FieldsNamed = parse_quote!({
            /// Workspace parameters.
            pub(crate) workspace_params_selection: CmdCtxBuilderTypeParamsT::WorkspaceParamsSelection
        });
        fields_named.named.extend(fields.named);

        if scope.profile_params_supported() {
            let fields: FieldsNamed = parse_quote!({
                /// Profile parameters.
                pub(crate) profile_params_selection: CmdCtxBuilderTypeParamsT::ProfileParamsSelection
            });
            fields_named.named.extend(fields.named);
        }

        if scope.flow_params_supported() {
            let fields: FieldsNamed = parse_quote!({
                /// Flow parameters.
                pub(crate) flow_params_selection: CmdCtxBuilderTypeParamsT::FlowParamsSelection
            });
            fields_named.named.extend(fields.named);
        }
    }

    /// Appends a `params_specs_provided: ParamsSpecs` field to the given
    /// fields.
    pub fn params_specs_push(fields_named: &mut FieldsNamed, scope: Scope) {
        if scope.flow_count() == FlowCount::One {
            let fields_params_specs: FieldsNamed = parse_quote!({
                /// Map of item ID to its parameters. `TypeMap<ItemId, AnySpecRtBoxed>` newtype.
                pub(crate) params_specs_provided: peace_params::ParamsSpecs
            });
            fields_named.named.extend(fields_params_specs.named);
        }
    }
}
