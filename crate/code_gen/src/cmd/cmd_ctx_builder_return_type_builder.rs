use quote::quote;
use syn::{parse_quote, Ident, TypePath};

/// Collects the tokens to build the `CmdCtxBuilder` return type.
///
/// By default, this references the associated types from the passed in
/// `CmdCtxBuilderTypeParamsT`.
#[derive(Debug)]
pub struct CmdCtxBuilderReturnTypeBuilder {
    /// Name of the scope builder struct.
    scope_builder_name: Ident,
    /// Path of the output type.
    ///
    /// Defaults to `CmdCtxBuilderTypeParamsT::Output`.
    ///
    /// You may want to set this to be one of:
    ///
    /// * `O`
    output: TypePath,
    /// Path of the app error type.
    ///
    /// Defaults to `CmdCtxBuilderTypeParamsT::AppError`.
    ///
    /// You may want to set this to be one of:
    ///
    /// * `AppError`
    app_error: TypePath,
    /// Path of the workspace params key type.
    ///
    /// Defaults to `<CmdCtxBuilderTypeParamsT::ParamsKeys as
    /// ParamsKeys>::WorkspaceParamsKMaybe`.
    ///
    /// You may want to set this to be one of:
    ///
    /// * `peace_rt_model::params::KeyKnown<WorkspaceParamsK>`
    /// * `peace_rt_model::params::KeyUnknown`
    workspace_params_k: TypePath,
    /// Path of the profile params key type.
    ///
    /// Defaults to `<CmdCtxBuilderTypeParamsT::ParamsKeys as
    /// ParamsKeys>::ProfileParamsKMaybe`.
    ///
    /// You may want to set this to be one of:
    ///
    /// * `peace_rt_model::params::KeyKnown<ProfileParamsK>`
    /// * `peace_rt_model::params::KeyUnknown`
    profile_params_k: TypePath,
    /// Path of the flow params key type.
    ///
    /// Defaults to `<CmdCtxBuilderTypeParamsT::ParamsKeys as
    /// ParamsKeys>::FlowParamsKMaybe`
    ///
    /// You may want to set this to be one of:
    ///
    /// * `peace_rt_model::params::KeyKnown<FlowParamsK>`
    /// * `peace_rt_model::params::KeyUnknown`
    flow_params_k: TypePath,

    /// Type state to track whether workspace params has been selected.
    ///
    /// Defaults to `CmdCtxBuilderTypeParamsT::WorkspaceParamsSelection`
    ///
    /// You may want to set this to be one of:
    ///
    /// * `crate::scopes::type_params::WorkspaceParamsNone`
    /// * `crate::scopes::type_params::WorkspaceParamsSome<WorkspaceParamsK>`
    workspace_params_selection: TypePath,
    /// Type state to track whether profile params has been selected.
    ///
    /// Defaults to `CmdCtxBuilderTypeParamsT::ProfileParamsSelection`
    ///
    /// You may want to set this to be one of:
    ///
    /// * `crate::scopes::type_params::ProfileParamsNone`
    /// * `crate::scopes::type_params::ProfileParamsSome<ProfileParamsK>`
    /// * `crate::scopes::type_params::ProfileParamsSomeMulti<ProfileParamsK>`
    profile_params_selection: TypePath,
    /// Type state to track whether flow params has been selected.
    ///
    /// Defaults to `CmdCtxBuilderTypeParamsT::FlowParamsSelection`
    ///
    /// You may want to set this to be one of:
    ///
    /// * `crate::scopes::type_params::FlowParamsNone`
    /// * `crate::scopes::type_params::FlowParamsSome<FlowParamsK>`
    /// * `crate::scopes::type_params::FlowParamsSomeMulti<FlowParamsK>`
    flow_params_selection: TypePath,

    /// Type state to track whether profile params has been selected.
    ///
    /// Defaults to `CmdCtxBuilderTypeParamsT::ProfileSelection`
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
    /// Defaults to `CmdCtxBuilderTypeParamsT::FlowSelection`
    ///
    /// You may want to set this to be one of:
    ///
    /// * `crate::scopes::type_params::FlowParamsNone`
    /// * `crate::scopes::type_params::FlowParamsSome<FlowParamsK>`
    /// * `crate::scopes::type_params::FlowParamsSomeMulti<FlowParamsK>`
    flow_selection: TypePath,
}

impl CmdCtxBuilderReturnTypeBuilder {
    pub fn new(scope_builder_name: Ident) -> Self {
        Self {
            scope_builder_name,
            output: parse_quote!(CmdCtxBuilderTypeParamsT::Output),
            app_error: parse_quote!(CmdCtxBuilderTypeParamsT::AppError),
            workspace_params_k: parse_quote!(
                <CmdCtxBuilderTypeParamsT::ParamsKeys
                    as peace_rt_model::params::ParamsKeys
                >::WorkspaceParamsKMaybe
            ),
            profile_params_k: parse_quote!(
                <CmdCtxBuilderTypeParamsT::ParamsKeys
                    as peace_rt_model::params::ParamsKeys
                >::ProfileParamsKMaybe
            ),
            flow_params_k: parse_quote!(
                <CmdCtxBuilderTypeParamsT::ParamsKeys
                    as peace_rt_model::params::ParamsKeys
                >::FlowParamsKMaybe
            ),
            workspace_params_selection: parse_quote!(
                CmdCtxBuilderTypeParamsT::WorkspaceParamsSelection
            ),
            profile_params_selection: parse_quote!(
                CmdCtxBuilderTypeParamsT::ProfileParamsSelection
            ),
            flow_params_selection: parse_quote!(CmdCtxBuilderTypeParamsT::FlowParamsSelection),
            profile_selection: parse_quote!(CmdCtxBuilderTypeParamsT::ProfileSelection),
            flow_selection: parse_quote!(CmdCtxBuilderTypeParamsT::FlowSelection),
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

    pub fn with_workspace_params_k(mut self, workspace_params_k: TypePath) -> Self {
        self.workspace_params_k = workspace_params_k;
        self
    }

    pub fn with_profile_params_k(mut self, profile_params_k: TypePath) -> Self {
        self.profile_params_k = profile_params_k;
        self
    }

    pub fn with_flow_params_k(mut self, flow_params_k: TypePath) -> Self {
        self.flow_params_k = flow_params_k;
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

    pub fn build(self) -> proc_macro2::TokenStream {
        let CmdCtxBuilderReturnTypeBuilder {
            output,
            app_error,
            scope_builder_name,
            workspace_params_k,
            profile_params_k,
            flow_params_k,
            workspace_params_selection,
            profile_params_selection,
            flow_params_selection,
            profile_selection,
            flow_selection,
        } = self;

        let cmd_ctx_builder_type_params_collector = quote! {
            crate::ctx::CmdCtxBuilderTypeParamsCollector<
                #output,
                #app_error,
                peace_rt_model::params::ParamsKeysImpl<
                    #workspace_params_k,
                    #profile_params_k,
                    #flow_params_k,
                >,
                #workspace_params_selection,
                #profile_params_selection,
                #flow_params_selection,
                #profile_selection,
                #flow_selection,
            >
        };
        let cmd_ctx_builder_type_params_collector = &cmd_ctx_builder_type_params_collector;
        quote! {
            crate::ctx::CmdCtxBuilder<
                'ctx,
                #cmd_ctx_builder_type_params_collector,
                #scope_builder_name<#cmd_ctx_builder_type_params_collector>,
            >
        }
    }
}
