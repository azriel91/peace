use proc_macro2::Span;
use quote::ToTokens;
use syn::{Data, DeriveInput, Fields, Ident, ImplGenerics, Path, TypeGenerics, WhereClause};

use crate::util::{
    field_or_wrapper_ty, field_spec_ty_deconstruct, field_spec_ty_path, fields_deconstruct,
    fields_vars_map, is_phantom_data, t_value_and_try_from_partial_bounds,
    tuple_ident_from_field_index, tuple_index_from_field_index, value_spec_ty, value_spec_ty_path,
    ImplMode,
};

/// `impl MyParamsFieldWiseBuilder`, so that Peace can resolve the params
/// type as well as its values from the spec.
pub fn impl_field_wise_builder(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    peace_params_path: &Path,
    value_field_wise_name: &Ident,
    value_field_wise_builder_name: &Ident,
    impl_mode: ImplMode,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics_split;
    // Needed for type parameterized type, only if the type parameter is an actual
    // value / not a marker.
    let where_clause = where_clause.cloned().map(|mut where_clause| {
        where_clause
            .predicates
            .extend(t_value_and_try_from_partial_bounds(ast, peace_params_path));

        where_clause
    });

    // enum builder should only expose builder methods for the variant it is in,
    // which means it needs a type state parameter for its variant.

    match &ast.data {
        Data::Struct(data_struct) => {
            // Note: struct builder has getters / setters generated in
            // `TypeGen::gen_from_value_type`.

            let fields = &data_struct.fields;

            let builder_field_methods = builder_field_methods(ast, fields, peace_params_path);
            let build_method_body = build_method_body(
                ast,
                ty_generics,
                peace_params_path,
                value_field_wise_name,
                fields,
                impl_mode,
            );

            let value_spec_ty = value_spec_ty(ast, ty_generics, peace_params_path, impl_mode);

            quote! {
                impl #impl_generics #value_field_wise_builder_name #ty_generics
                #where_clause
                {
                    #builder_field_methods

                    pub fn build(self) -> #value_spec_ty {
                        #build_method_body
                    }
                }
            }
        }
        Data::Enum(_data_enum) => quote!(),
        Data::Union(_data_union) => quote!(),
    }
}

/// Returns `with_field(mut self, value_spec: ValueSpec<FieldType>) -> Self` for
/// all non-`PhantomData` fields.
pub fn builder_field_methods(
    parent_ast: &DeriveInput,
    fields: &Fields,
    peace_params_path: &Path,
) -> proc_macro2::TokenStream {
    let fields_as_params = fields
        .iter()
        .enumerate()
        .filter(|(_field_index, field)| !is_phantom_data(&field.ty))
        .map(|(field_index, field)| {
            let field_ty = &field.ty;
            let (self_field_name, field_name) = if let Some(field_ident) = field.ident.as_ref() {
                let self_field_name = field_ident.to_token_stream();
                let field_name = field_ident.clone();
                (self_field_name, field_name)
            } else {
                let self_field_name = tuple_index_from_field_index(field_index).to_token_stream();
                let field_name = tuple_ident_from_field_index(field_index);
                (self_field_name, field_name)
            };
            let field_spec_ty_path = field_spec_ty_path(Some(parent_ast), peace_params_path, field);
            let with_field_name = Ident::new(&format!("with_{field_name}"), Span::call_site());
            let with_field_name_from =
                Ident::new(&format!("with_{field_name}_from"), Span::call_site());
            let with_field_name_from_map =
                Ident::new(&format!("with_{field_name}_from_map"), Span::call_site());

            let field_spec_ty_deconstruct =
                field_spec_ty_deconstruct(Some(parent_ast), peace_params_path, field, &field_name);

            let field_or_wrapper_ty = field_or_wrapper_ty(Some(parent_ast), field);

            // Specifies how to determine the value of this field.
            //
            // # Example
            //
            // ```rust,ignore
            // let params_spec = FileDownloadParams::field_wise()
            //     .with_src(Url::parse("https://../web_app.tar")) // direct value
            //     .with_dest_from_map(|workspace_dir: &WorkspaceDir| {
            //         workspace.dir.join("web_app.tar")
            //     })
            //     .build();
            //
            // let mut cmd_ctx = // ..
            //     .with_item_spec_params::<_>(item_spec_id, params_spec)
            //     .await?;
            // ```
            quote! {
                pub fn #with_field_name(mut self, #field_name: #field_ty) -> Self {
                    self.#self_field_name = Some(#field_spec_ty_deconstruct);
                    self
                }

                pub fn #with_field_name_from(mut self) -> Self {
                    self.#self_field_name = Some(#field_spec_ty_path::From);
                    self
                }

                pub fn #with_field_name_from_map<F, Args>(mut self, f: F) -> Self
                where
                    #peace_params_path::MappingFnImpl<#field_or_wrapper_ty, F, Args>:
                        From<(Option<String>, F)>
                        + #peace_params_path::MappingFn<Output = #field_or_wrapper_ty>
                {
                    self.#self_field_name = Some(#field_spec_ty_path::from_map(
                        Some(String::from(stringify!(#field_name))),
                        f,
                    ));
                    self
                }
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    quote! {
        #(#fields_as_params)*
    }
}

fn build_method_body(
    parent_ast: &DeriveInput,
    ty_generics: &TypeGenerics,
    peace_params_path: &Path,
    value_field_wise_name: &Ident,
    fields: &Fields,
    impl_mode: ImplMode,
) -> proc_macro2::TokenStream {
    // let field_spec_ty_path = field_spec_ty_path(Some(parent_ast),
    // peace_params_path, field);

    let value_spec_ty_path =
        value_spec_ty_path(parent_ast, ty_generics, peace_params_path, impl_mode);

    let fields_deconstruct = fields_deconstruct(fields);
    // let field_name = field_name.unwrap_or(ValueSpec::<FieldTy>::Stored);
    let fields_unwrap_to_value_spec_fieldless = fields_vars_map(fields, |field, field_name| {
        let field_spec_ty_path = field_spec_ty_path(Some(parent_ast), peace_params_path, field);

        quote! {
            #field_name.unwrap_or(#field_spec_ty_path::Stored)
        }
    });

    match fields {
        Fields::Named(_) => {
            quote! {
                let Self {
                    #(#fields_deconstruct),*
                } = self;

                #(#fields_unwrap_to_value_spec_fieldless)*

                let field_wise = #value_field_wise_name {
                    #(#fields_deconstruct),*
                };

                #value_spec_ty_path::FieldWise(field_wise)
            }
        }
        Fields::Unnamed(_) => {
            quote! {
                let Self(#(#fields_deconstruct),*) = self;

                #(#fields_unwrap_to_value_spec_fieldless)*

                let field_wise = #value_field_wise_name(#(#fields_deconstruct),*);

                #value_spec_ty_path::FieldWise(field_wise)
            }
        }
        Fields::Unit => quote!(#value_spec_ty_path::FieldWise(#value_field_wise_name)),
    }
}
