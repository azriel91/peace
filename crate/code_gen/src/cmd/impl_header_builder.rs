use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, LifetimeParam, Path, TypePath,
    WherePredicate,
};

#[derive(Clone, Debug)]
pub(crate) struct ImplHeaderBuilder {
    /// Defaults to `'ctx`.
    lifetimes: Punctuated<LifetimeParam, Comma>,
    /// Defaults to `None`.
    trait_name: Option<Path>,
    /// Defaults to `Output`.
    output: Option<TypePath>,
    /// Defaults to `AppError`.
    app_error: Option<TypePath>,
    /// Defaults to `WorkspaceParamsKMaybe`.
    workspace_params_k_maybe: Option<TypePath>,
    /// Defaults to `ProfileParamsKMaybe`.
    profile_params_k_maybe: Option<TypePath>,
    /// Defaults to `FlowParamsKMaybe`.
    flow_params_k_maybe: Option<TypePath>,
    /// Defaults to `WorkspaceParamsSelection`.
    workspace_params_selection: Option<TypePath>,
    /// Defaults to `ProfileParamsSelection`.
    profile_params_selection: Option<TypePath>,
    /// Defaults to `FlowParamsSelection`.
    flow_params_selection: Option<TypePath>,
    /// Defaults to `ProfileSelection`.
    profile_selection: Option<TypePath>,
    /// The `CtxCtxBuilder` with type parameters all filled in.
    builder_type: Path,
    /// Defaults to `peace_rt_model::output::OutputWrite<AppError> + 'static`.
    constraint_output: Option<WherePredicate>,
    /// Defaults to `peace_value_traits::AppError +
    /// From<peace_rt_model::Error>`.
    constraint_app_error: Option<WherePredicate>,
    /// Defaults to `peace_rt_model::params::KeyMaybe`.
    constraint_workspace_params_k_maybe: Option<WherePredicate>,
    /// Defaults to `peace_rt_model::params::KeyMaybe`.
    constraint_profile_params_k_maybe: Option<WherePredicate>,
    /// Defaults to `peace_rt_model::params::KeyMaybe`.
    constraint_flow_params_k_maybe: Option<WherePredicate>,
}

impl ImplHeaderBuilder {
    pub fn new(builder_type: Path) -> Self {
        Self {
            lifetimes: parse_quote!('ctx),
            trait_name: None,
            output: Some(parse_quote!(Output)),
            app_error: Some(parse_quote!(AppError)),
            workspace_params_k_maybe: Some(parse_quote!(WorkspaceParamsKMaybe)),
            profile_params_k_maybe: Some(parse_quote!(ProfileParamsKMaybe)),
            flow_params_k_maybe: Some(parse_quote!(FlowParamsKMaybe)),
            workspace_params_selection: Some(parse_quote!(WorkspaceParamsSelection)),
            profile_params_selection: Some(parse_quote!(ProfileParamsSelection)),
            flow_params_selection: Some(parse_quote!(FlowParamsSelection)),
            profile_selection: Some(parse_quote!(ProfileSelection)),
            builder_type,
            constraint_output: Some(parse_quote!(
                peace_rt_model::output::OutputWrite<AppError> + 'static
            )),
            constraint_app_error: Some(parse_quote!(
                peace_value_traits::AppError + From<peace_rt_model::Error>
            )),
            constraint_workspace_params_k_maybe: Some(parse_quote!(
                peace_rt_model::params::KeyMaybe
            )),
            constraint_profile_params_k_maybe: Some(parse_quote!(peace_rt_model::params::KeyMaybe)),
            constraint_flow_params_k_maybe: Some(parse_quote!(peace_rt_model::params::KeyMaybe)),
        }
    }

    pub fn with_lifetimes(mut self, lifetimes: Punctuated<LifetimeParam, Comma>) -> Self {
        self.lifetimes = lifetimes;
        self
    }

    pub fn with_trait_name(mut self, trait_name: Option<Path>) -> Self {
        self.trait_name = trait_name;
        self
    }

    pub fn with_output(mut self, output: Option<TypePath>) -> Self {
        self.output = output;
        self
    }

    pub fn with_app_error(mut self, app_error: Option<TypePath>) -> Self {
        self.app_error = app_error;
        self
    }

    pub fn with_workspace_params_k_maybe(
        mut self,
        workspace_params_k_maybe: Option<TypePath>,
    ) -> Self {
        self.workspace_params_k_maybe = workspace_params_k_maybe;
        self
    }

    pub fn with_profile_params_k_maybe(mut self, profile_params_k_maybe: Option<TypePath>) -> Self {
        self.profile_params_k_maybe = profile_params_k_maybe;
        self
    }

    pub fn with_flow_params_k_maybe(mut self, flow_params_k_maybe: Option<TypePath>) -> Self {
        self.flow_params_k_maybe = flow_params_k_maybe;
        self
    }

    pub fn with_workspace_params_selection(
        mut self,
        workspace_params_selection: Option<TypePath>,
    ) -> Self {
        self.workspace_params_selection = workspace_params_selection;
        self
    }

    pub fn with_profile_params_selection(
        mut self,
        profile_params_selection: Option<TypePath>,
    ) -> Self {
        self.profile_params_selection = profile_params_selection;
        self
    }

    pub fn with_flow_params_selection(mut self, flow_params_selection: Option<TypePath>) -> Self {
        self.flow_params_selection = flow_params_selection;
        self
    }

    pub fn with_profile_selection(mut self, profile_selection: Option<TypePath>) -> Self {
        self.profile_selection = profile_selection;
        self
    }

    pub fn with_builder_type(mut self, builder_type: Path) -> Self {
        self.builder_type = builder_type;
        self
    }

    pub fn with_constraint_output(mut self, constraint_output: Option<WherePredicate>) -> Self {
        self.constraint_output = constraint_output;
        self
    }

    pub fn with_constraint_app_error(
        mut self,
        constraint_app_error: Option<WherePredicate>,
    ) -> Self {
        self.constraint_app_error = constraint_app_error;
        self
    }

    pub fn with_constraint_workspace_params_k_maybe(
        mut self,
        constraint_workspace_params_k_maybe: Option<WherePredicate>,
    ) -> Self {
        self.constraint_workspace_params_k_maybe = constraint_workspace_params_k_maybe;
        self
    }

    pub fn with_constraint_profile_params_k_maybe(
        mut self,
        constraint_profile_params_k_maybe: Option<WherePredicate>,
    ) -> Self {
        self.constraint_profile_params_k_maybe = constraint_profile_params_k_maybe;
        self
    }

    pub fn with_constraint_flow_params_k_maybe(
        mut self,
        constraint_flow_params_k_maybe: Option<WherePredicate>,
    ) -> Self {
        self.constraint_flow_params_k_maybe = constraint_flow_params_k_maybe;
        self
    }

    pub fn build(self) -> proc_macro2::TokenStream {
        let Self {
            lifetimes,
            trait_name,
            output,
            app_error,
            workspace_params_k_maybe,
            profile_params_k_maybe,
            flow_params_k_maybe,
            workspace_params_selection,
            profile_params_selection,
            flow_params_selection,
            profile_selection,
            builder_type,
            constraint_output,
            constraint_app_error,
            constraint_workspace_params_k_maybe,
            constraint_profile_params_k_maybe,
            constraint_flow_params_k_maybe,
        } = self;

        let trait_name_for = trait_name.map(|trait_name| quote!(#trait_name for));
        let output = output.map(|output| quote!(#output,));
        let app_error = app_error.map(|app_error| quote!(#app_error,));
        let workspace_params_k_maybe = workspace_params_k_maybe
            .map(|workspace_params_k_maybe| quote!(#workspace_params_k_maybe,));
        let profile_params_k_maybe =
            profile_params_k_maybe.map(|profile_params_k_maybe| quote!(#profile_params_k_maybe,));
        let flow_params_k_maybe =
            flow_params_k_maybe.map(|flow_params_k_maybe| quote!(#flow_params_k_maybe,));
        let workspace_params_selection = workspace_params_selection
            .map(|workspace_params_selection| quote!(#workspace_params_selection,));
        let profile_params_selection = profile_params_selection
            .map(|profile_params_selection| quote!(#profile_params_selection,));
        let flow_params_selection =
            flow_params_selection.map(|flow_params_selection| quote!(#flow_params_selection,));
        let profile_selection =
            profile_selection.map(|profile_selection| quote!(#profile_selection,));

        let where_ = if constraint_output.is_some()
            || constraint_app_error.is_some()
            || constraint_workspace_params_k_maybe.is_some()
            || constraint_profile_params_k_maybe.is_some()
            || constraint_flow_params_k_maybe.is_some()
        {
            quote!(where)
        } else {
            proc_macro2::TokenStream::new()
        };

        quote! {
            impl<
                #lifetimes,
                #output
                #app_error
                #workspace_params_k_maybe
                #profile_params_k_maybe
                #flow_params_k_maybe
                #workspace_params_selection
                #profile_params_selection
                #flow_params_selection
                #profile_selection
            >
            #trait_name_for
            #builder_type
            #where_
                #constraint_output,
                #constraint_app_error,
                #constraint_workspace_params_k_maybe,
                #constraint_profile_params_k_maybe,
                #constraint_flow_params_k_maybe,
        }
    }
}
