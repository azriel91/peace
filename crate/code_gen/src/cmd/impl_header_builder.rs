use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, Ident, LifetimeParam, Path, TypePath,
    WherePredicate,
};

#[derive(Clone, Debug)]
pub(crate) struct ImplHeaderBuilder {
    /// Defaults to `'ctx`.
    lifetimes: Punctuated<LifetimeParam, Comma>,
    /// Defaults to `None`.
    trait_name: Option<Path>,
    /// Defaults to `AppError`.
    app_error: Option<TypePath>,
    /// Whether the `AppError` type param should be constrained.
    app_error_constraint_enabled: bool,
    /// Defaults to `Output`.
    output: Option<TypePath>,
    /// Whether the `OutputWrite` type param should be constrained.
    output_constraint_enabled: bool,
    /// Defaults to `WorkspaceParamsKMaybe`.
    workspace_params_k_maybe: Option<TypePath>,
    /// Defaults to `None`, you may wish to set this to `WorkspaceParamsK`.
    workspace_params_k: Option<Ident>,
    /// Defaults to `ProfileParamsKMaybe`.
    profile_params_k_maybe: Option<TypePath>,
    /// Defaults to `None`, you may wish to set this to `ProfileParamsK`.
    profile_params_k: Option<Ident>,
    /// Defaults to `FlowParamsKMaybe`.
    flow_params_k_maybe: Option<TypePath>,
    /// Defaults to `None`, you may wish to set this to `FlowParamsK`.
    flow_params_k: Option<Ident>,
    /// Defaults to `WorkspaceParamsSelection`.
    workspace_params_selection: Option<TypePath>,
    /// Defaults to `ProfileParamsSelection`.
    profile_params_selection: Option<TypePath>,
    /// Defaults to `FlowParamsSelection`.
    flow_params_selection: Option<TypePath>,
    /// Defaults to `ProfileSelection`.
    profile_selection: Option<TypePath>,
    /// Defaults to `FlowSelection`.
    flow_selection: Option<TypePath>,
    /// The `CtxCtxBuilder` with type parameters all filled in.
    builder_type: Path,
}

impl ImplHeaderBuilder {
    pub fn new(builder_type: Path) -> Self {
        Self {
            lifetimes: parse_quote!('ctx),
            trait_name: None,
            app_error: Some(parse_quote!(AppError)),
            app_error_constraint_enabled: true,
            output: Some(parse_quote!(Output)),
            output_constraint_enabled: true,
            workspace_params_k_maybe: Some(parse_quote!(WorkspaceParamsKMaybe)),
            workspace_params_k: None,
            profile_params_k_maybe: Some(parse_quote!(ProfileParamsKMaybe)),
            profile_params_k: None,
            flow_params_k_maybe: Some(parse_quote!(FlowParamsKMaybe)),
            flow_params_k: None,
            workspace_params_selection: Some(parse_quote!(WorkspaceParamsSelection)),
            profile_params_selection: Some(parse_quote!(ProfileParamsSelection)),
            flow_params_selection: Some(parse_quote!(FlowParamsSelection)),
            profile_selection: Some(parse_quote!(ProfileSelection)),
            flow_selection: Some(parse_quote!(FlowSelection)),
            builder_type,
        }
    }

    pub fn with_output_constraint_enabled(mut self, output_constraint_enabled: bool) -> Self {
        self.output_constraint_enabled = output_constraint_enabled;
        self
    }

    pub fn with_app_error_constraint_enabled(mut self, app_error_constraint_enabled: bool) -> Self {
        self.app_error_constraint_enabled = app_error_constraint_enabled;
        self
    }

    pub fn with_lifetimes(mut self, lifetimes: Punctuated<LifetimeParam, Comma>) -> Self {
        self.lifetimes = lifetimes;
        self
    }

    pub fn with_trait_name(mut self, trait_name: Option<Path>) -> Self {
        self.trait_name = trait_name;
        self
    }

    pub fn with_workspace_params_k_maybe(
        mut self,
        workspace_params_k_maybe: Option<TypePath>,
    ) -> Self {
        self.workspace_params_k_maybe = workspace_params_k_maybe;
        self
    }

    pub fn with_workspace_params_k(mut self, workspace_params_k: Option<Ident>) -> Self {
        self.workspace_params_k = workspace_params_k;
        self
    }

    pub fn with_profile_params_k_maybe(mut self, profile_params_k_maybe: Option<TypePath>) -> Self {
        self.profile_params_k_maybe = profile_params_k_maybe;
        self
    }

    pub fn with_profile_params_k(mut self, profile_params_k: Option<Ident>) -> Self {
        self.profile_params_k = profile_params_k;
        self
    }

    pub fn with_flow_params_k_maybe(mut self, flow_params_k_maybe: Option<TypePath>) -> Self {
        self.flow_params_k_maybe = flow_params_k_maybe;
        self
    }

    pub fn with_flow_params_k(mut self, flow_params_k: Option<Ident>) -> Self {
        self.flow_params_k = flow_params_k;
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

    pub fn with_flow_selection(mut self, flow_selection: Option<TypePath>) -> Self {
        self.flow_selection = flow_selection;
        self
    }

    pub fn build(self) -> proc_macro2::TokenStream {
        let Self {
            lifetimes,
            trait_name,
            app_error,
            app_error_constraint_enabled,
            output,
            output_constraint_enabled,
            workspace_params_k_maybe,
            workspace_params_k,
            profile_params_k_maybe,
            profile_params_k,
            flow_params_k_maybe,
            flow_params_k,
            workspace_params_selection,
            profile_params_selection,
            flow_params_selection,
            profile_selection,
            flow_selection,
            builder_type,
        } = self;

        let mut where_predicates = Punctuated::<WherePredicate, Comma>::new();
        if let Some(app_error) = app_error.as_ref() {
            if app_error_constraint_enabled {
                where_predicates.push(parse_quote!(
                    #app_error: peace_value_traits::AppError + From<peace_rt_model::Error>
                ));
            }
        };
        if let Some(output) = output.as_ref() {
            if output_constraint_enabled {
                where_predicates.push(parse_quote!(
                    #output: peace_rt_model::output::OutputWrite
                ));
            }
        };
        if let Some(workspace_params_k_maybe) = workspace_params_k_maybe.as_ref() {
            where_predicates.push(parse_quote!(
                #workspace_params_k_maybe: peace_rt_model::params::KeyMaybe
            ));
        };
        if let Some(workspace_params_k) = workspace_params_k.as_ref() {
            where_predicates.push(parse_quote!(
                #workspace_params_k: Clone + std::fmt::Debug + Eq + std::hash::Hash + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + Unpin + 'static
            ));
        }
        if let Some(profile_params_k_maybe) = profile_params_k_maybe.as_ref() {
            where_predicates.push(parse_quote!(
                #profile_params_k_maybe: peace_rt_model::params::KeyMaybe
            ));
        };
        if let Some(profile_params_k) = profile_params_k.as_ref() {
            where_predicates.push(parse_quote!(
                #profile_params_k: Clone + std::fmt::Debug + Eq + std::hash::Hash + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + Unpin + 'static
            ));
        }
        if let Some(flow_params_k_maybe) = flow_params_k_maybe.as_ref() {
            where_predicates.push(parse_quote!(
                #flow_params_k_maybe: peace_rt_model::params::KeyMaybe
            ));
        };
        if let Some(flow_params_k) = flow_params_k.as_ref() {
            where_predicates.push(parse_quote!(
                #flow_params_k: Clone + std::fmt::Debug + Eq + std::hash::Hash + serde::de::DeserializeOwned + serde::Serialize + Send + Sync + Unpin + 'static
            ));
        }

        let trait_name_for = trait_name.map(|trait_name| quote!(#trait_name for));
        let app_error = app_error.map(|app_error| quote!(#app_error,));
        let output = output.map(|output| quote!(#output,));
        let workspace_params_k_maybe = workspace_params_k_maybe
            .map(|workspace_params_k_maybe| quote!(#workspace_params_k_maybe,));
        let workspace_params_k =
            workspace_params_k.map(|workspace_params_k| quote!(#workspace_params_k,));
        let profile_params_k_maybe =
            profile_params_k_maybe.map(|profile_params_k_maybe| quote!(#profile_params_k_maybe,));
        let profile_params_k = profile_params_k.map(|profile_params_k| quote!(#profile_params_k,));
        let flow_params_k_maybe =
            flow_params_k_maybe.map(|flow_params_k_maybe| quote!(#flow_params_k_maybe,));
        let flow_params_k = flow_params_k.map(|flow_params_k| quote!(#flow_params_k,));
        let workspace_params_selection = workspace_params_selection
            .map(|workspace_params_selection| quote!(#workspace_params_selection,));
        let profile_params_selection = profile_params_selection
            .map(|profile_params_selection| quote!(#profile_params_selection,));
        let flow_params_selection =
            flow_params_selection.map(|flow_params_selection| quote!(#flow_params_selection,));
        let profile_selection =
            profile_selection.map(|profile_selection| quote!(#profile_selection,));
        let flow_selection = flow_selection.map(|flow_selection| quote!(#flow_selection,));

        let where_ = if where_predicates.is_empty() {
            None
        } else {
            Some(quote!(where))
        };

        quote! {
            impl<
                #lifetimes,
                #app_error
                #output
                #workspace_params_k_maybe
                #workspace_params_k
                #profile_params_k_maybe
                #profile_params_k
                #flow_params_k_maybe
                #flow_params_k
                #workspace_params_selection
                #profile_params_selection
                #flow_params_selection
                #profile_selection
                #flow_selection
            >
            #trait_name_for
            #builder_type
            #where_
                #where_predicates
        }
    }
}
