use quote::quote;
use syn::{parse_quote, Ident, Path, TypePath};

/// Collects the tokens to build the `CmdCtxBuilder` return type.
///
/// By default, this references the associated types from the passed in
/// `CmdCtxBuilderTypesT`.
#[derive(Clone, Debug)]
pub struct CmdCtxBuilderTypeBuilder {
    /// Name of the scope builder struct.
    scope_builder_name: Ident,
    /// Path of the app error type.
    ///
    /// Defaults to `AppError`.
    ///
    /// You may want to set this to be one of:
    ///
    /// * `AppError`
    app_error: TypePath,
    /// Path of the output type.
    ///
    /// Defaults to `Output`.
    ///
    /// You may want to set this to be one of:
    ///
    /// * `O`
    output: TypePath,
    /// Path of the params keys type.
    ///
    /// By default this is not set, and the `ParamsKeysImpl` type is used as the
    /// `ParamsKeys` type parameter, delegating the `*KeyMaybe` to the
    /// `*_params_k` fields.
    ///
    /// You may want to set this to `ParamsKeysT`, so that `ParamsKeysT` can be
    /// referenced by another trait bound in the `quote!` macro.
    params_keys: Option<TypePath>,
    /// Path of the workspace params key type.
    ///
    /// Defaults to `WorkspaceParamsKMaybe`.
    ///
    /// You may want to set this to be one of:
    ///
    /// * `peace_rt_model::params::KeyKnown<WorkspaceParamsK>`
    /// * `peace_rt_model::params::KeyUnknown`
    workspace_params_k_maybe: TypePath,
    /// Path of the profile params key type.
    ///
    /// Defaults to `ProfileParamsKMaybe`.
    ///
    /// You may want to set this to be one of:
    ///
    /// * `peace_rt_model::params::KeyKnown<ProfileParamsK>`
    /// * `peace_rt_model::params::KeyUnknown`
    profile_params_k_maybe: TypePath,
    /// Path of the flow params key type.
    ///
    /// Defaults to `FlowParamsKMaybe`
    ///
    /// You may want to set this to be one of:
    ///
    /// * `peace_rt_model::params::KeyKnown<FlowParamsK>`
    /// * `peace_rt_model::params::KeyUnknown`
    flow_params_k_maybe: TypePath,

    /// Type state to track whether workspace params has been selected.
    ///
    /// Defaults to `WorkspaceParamsSelection`
    ///
    /// You may want to set this to be one of:
    ///
    /// * `crate::scopes::type_params::WorkspaceParamsNone`
    /// * `crate::scopes::type_params::WorkspaceParamsSome<WorkspaceParamsK>`
    workspace_params_selection: TypePath,
    /// Type state to track whether profile params has been selected.
    ///
    /// Defaults to `ProfileParamsSelection`
    ///
    /// You may want to set this to be one of:
    ///
    /// * `crate::scopes::type_params::ProfileParamsNone`
    /// * `crate::scopes::type_params::ProfileParamsSome<ProfileParamsK>`
    /// * `crate::scopes::type_params::ProfileParamsSomeMulti<ProfileParamsK>`
    profile_params_selection: TypePath,
    /// Type state to track whether flow params has been selected.
    ///
    /// Defaults to `FlowParamsSelection`
    ///
    /// You may want to set this to be one of:
    ///
    /// * `crate::scopes::type_params::FlowParamsNone`
    /// * `crate::scopes::type_params::FlowParamsSome<FlowParamsK>`
    /// * `crate::scopes::type_params::FlowParamsSomeMulti<FlowParamsK>`
    flow_params_selection: TypePath,

    /// Type state to track whether profile params has been selected.
    ///
    /// Defaults to `ProfileSelection`
    ///
    /// You may want to set this to be one of:
    ///
    /// * `crate::scopes::type_params::ProfileNotSelected`
    /// * `crate::scopes::type_params::ProfileSelected`
    /// * `crate::scopes::type_params::ProfileFromWorkspaceParam<'_,
    ///   WorkspaceParamsK>`
    /// * `crate::scopes::type_params::ProfileFilterFn<'_>`
    profile_selection: TypePath,
    /// Type state to track whether flow params has been selected.
    ///
    /// Defaults to `FlowSelection`
    ///
    /// You may want to set this to be one of:
    ///
    /// * `crate::scopes::type_params::FlowParamsNone`
    /// * `crate::scopes::type_params::FlowParamsSome<FlowParamsK>`
    /// * `crate::scopes::type_params::FlowParamsSomeMulti<FlowParamsK>`
    flow_selection: TypePath,
}

impl CmdCtxBuilderTypeBuilder {
    pub fn new(scope_builder_name: Ident) -> Self {
        Self {
            scope_builder_name,
            output: parse_quote!(Output),
            app_error: parse_quote!(AppError),
            params_keys: None,
            workspace_params_k_maybe: parse_quote!(WorkspaceParamsKMaybe),
            profile_params_k_maybe: parse_quote!(ProfileParamsKMaybe),
            flow_params_k_maybe: parse_quote!(FlowParamsKMaybe),
            workspace_params_selection: parse_quote!(WorkspaceParamsSelection),
            profile_params_selection: parse_quote!(ProfileParamsSelection),
            flow_params_selection: parse_quote!(FlowParamsSelection),
            profile_selection: parse_quote!(ProfileSelection),
            flow_selection: parse_quote!(FlowSelection),
        }
    }

    pub fn with_output(mut self, output: TypePath) -> Self {
        self.output = output;
        self
    }

    pub fn with_app_error(mut self, app_error: TypePath) -> Self {
        self.app_error = app_error;
        self
    }

    pub fn with_workspace_params_k_maybe(mut self, workspace_params_k_maybe: TypePath) -> Self {
        self.workspace_params_k_maybe = workspace_params_k_maybe;
        self
    }

    pub fn with_profile_params_k_maybe(mut self, profile_params_k_maybe: TypePath) -> Self {
        self.profile_params_k_maybe = profile_params_k_maybe;
        self
    }

    pub fn with_flow_params_k_maybe(mut self, flow_params_k_maybe: TypePath) -> Self {
        self.flow_params_k_maybe = flow_params_k_maybe;
        self
    }

    pub fn with_workspace_params_selection(mut self, workspace_params_selection: TypePath) -> Self {
        self.workspace_params_selection = workspace_params_selection;
        self
    }

    pub fn with_profile_params_selection(mut self, profile_params_selection: TypePath) -> Self {
        self.profile_params_selection = profile_params_selection;
        self
    }

    pub fn with_flow_params_selection(mut self, flow_params_selection: TypePath) -> Self {
        self.flow_params_selection = flow_params_selection;
        self
    }

    pub fn with_profile_selection(mut self, profile_selection: TypePath) -> Self {
        self.profile_selection = profile_selection;
        self
    }

    pub fn with_flow_selection(mut self, flow_selection: TypePath) -> Self {
        self.flow_selection = flow_selection;
        self
    }

    pub fn build(self) -> Path {
        let CmdCtxBuilderTypeBuilder {
            app_error,
            output,
            params_keys,
            scope_builder_name,
            workspace_params_k_maybe,
            profile_params_k_maybe,
            flow_params_k_maybe,
            workspace_params_selection,
            profile_params_selection,
            flow_params_selection,
            profile_selection,
            flow_selection,
        } = self;

        let params_keys = params_keys.unwrap_or_else(|| {
            parse_quote! {
                peace_rt_model::params::ParamsKeysImpl<
                    #workspace_params_k_maybe,
                    #profile_params_k_maybe,
                    #flow_params_k_maybe,
                >
            }
        });

        let cmd_ctx_builder_types_collector = quote! {
            crate::ctx::CmdCtxBuilderTypesCollector<
                #app_error,
                #output,
                #params_keys,
                #workspace_params_selection,
                #profile_params_selection,
                #flow_params_selection,
                #profile_selection,
                #flow_selection,
            >
        };
        let cmd_ctx_builder_types_collector = &cmd_ctx_builder_types_collector;
        parse_quote! {
            crate::ctx::CmdCtxBuilder<
                'ctx,
                #cmd_ctx_builder_types_collector,
                #scope_builder_name<#cmd_ctx_builder_types_collector>,
            >
        }
    }
}
