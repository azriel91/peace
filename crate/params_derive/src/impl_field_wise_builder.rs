use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    Data, DataEnum, DeriveInput, Fields, Ident, ImplGenerics, Path, Type, TypeGenerics, WhereClause,
};

use crate::{
    field_wise_enum_builder_ctx::FieldWiseEnumBuilderCtx,
    fields_map::fields_to_optional_value_spec,
    impl_default,
    type_gen::TypeGen,
    util::{
        field_spec_ty_deconstruct, field_spec_ty_path, fields_deconstruct, fields_vars_map,
        is_phantom_data, tuple_ident_from_field_index, tuple_index_from_field_index, value_spec_ty,
        value_spec_ty_path, ImplMode,
    },
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
    field_wise_enum_builder_ctx: &FieldWiseEnumBuilderCtx,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics_split;

    // enum builder should only expose builder methods for the variant it is in,
    // which means it needs a type state parameter for its variant.

    match &ast.data {
        Data::Struct(data_struct) => {
            let field_wise_builder = TypeGen::gen_from_value_type(
                ast,
                generics_split,
                value_field_wise_builder_name,
                |fields| fields_to_optional_value_spec(fields, peace_params_path),
                &[parse_quote! {
                    #[doc="\
                        Builder for specification of how to look up the values for an item spec's \n\
                        parameters.\
                    "]
                }],
                false,
            );
            // Note: struct builder has getters / setters generated in
            // `TypeGen::gen_from_value_type`.

            let impl_default = impl_default(ast, generics_split, value_field_wise_builder_name);

            let fields = &data_struct.fields;

            let builder_field_methods = builder_field_methods(fields, peace_params_path);
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
                #field_wise_builder

                #impl_default

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
        Data::Enum(data_enum) => impl_enum_builder(
            ast,
            generics_split,
            peace_params_path,
            value_field_wise_name,
            value_field_wise_builder_name,
            field_wise_enum_builder_ctx,
            data_enum,
        ),
        Data::Union(_data_union) => quote!(),
    }
}

/// From a given enum:
///
/// ```rust,ignore
/// pub enum EnumParams<T1, T2> {
///     Variant1 {
///         field_1: T1,
///     },
///     Variant2(T2),
///     Variant3
/// }
/// ```
///
/// Generates a builder like the following:
///
/// ```rust,ignore
/// pub struct EnumParamsBuilder<VariantSelection, T1, T2> {
///     variant_selection: VariantSelection,
///     marker: PhantomData<(T1, T2)>,
/// }
///
/// impl Default for EnumParamsBuilder<EnumParamsBuilderVariantNone> {
///     fn default() -> Self {
///         Self { variant_selection: EnumParamsBuilderVariantNone, }
///     }
/// }
///
/// pub struct EnumParamsBuilderVariantNone;
///
/// pub struct EnumParamsVariant1Builder<T1> {
///     field_1: Option<T1>,
/// }
///
/// pub struct EnumParamsVariant2Builder<T2>(Option<T2>);
/// ```
fn impl_enum_builder(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    peace_params_path: &Path,
    value_field_wise_name: &Ident,
    value_field_wise_builder_name: &Ident,
    field_wise_enum_builder_ctx: &FieldWiseEnumBuilderCtx,
    data_enum: &DataEnum,
) -> proc_macro2::TokenStream {
    let enum_builder_generics = &field_wise_enum_builder_ctx.generics;
    let enum_params_variant_none = &field_wise_enum_builder_ctx.variant_none;
    let ty_generics_idents = &field_wise_enum_builder_ctx.ty_generics_idents;
    let type_params_with_variant_none = &field_wise_enum_builder_ctx.type_params_with_variant_none;

    let (impl_generics, _ty_generics, _where_clause) = generics_split;
    let (builder_impl_generics, builder_ty_generics, builder_where_clause) =
        enum_builder_generics.split_for_impl();

    let marker_type: Type = if ast.generics.params.is_empty() {
        parse_quote!(::std::marker::PhantomData<()>)
    } else {
        parse_quote!(::std::marker::PhantomData<(#ty_generics_idents)>)
    };

    quote! {
        pub struct #value_field_wise_builder_name #builder_ty_generics {
            variant_selection: VariantSelection,
            marker: #marker_type,
        }

        impl #impl_generics ::std::default::Default
        for #value_field_wise_builder_name #type_params_with_variant_none
        #builder_where_clause
        {
            fn default() -> Self {
                #value_field_wise_builder_name {
                    variant_selection: #enum_params_variant_none,
                    marker: ::std::marker::PhantomData,
                }
            }
        }

        impl #builder_impl_generics #value_field_wise_builder_name #builder_ty_generics
        #builder_where_clause
        {
            // #(#variant_builder_fns)*
        }

        // VariantSelections

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct #enum_params_variant_none;
    }
}

/// Returns `with_field(mut self, value_spec: ValueSpec<FieldType>) -> Self` for
/// all non-`PhantomData` fields.
fn builder_field_methods(fields: &Fields, peace_params_path: &Path) -> proc_macro2::TokenStream {
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
            let field_spec_ty_path = field_spec_ty_path(peace_params_path, field_ty);
            let with_field_name = Ident::new(&format!("with_{self_field_name}"), Span::call_site());
            let with_field_name_from =
                Ident::new(&format!("with_{self_field_name}_from"), Span::call_site());
            let with_field_name_from_map = Ident::new(
                &format!("with_{self_field_name}_from_map"),
                Span::call_site(),
            );

            let field_spec_ty_deconstruct =
                field_spec_ty_deconstruct(peace_params_path, &field_name);

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
                    #peace_params_path::MappingFnImpl<#field_ty, F, Args>:
                        From<(Option<String>, F)>
                        + #peace_params_path::MappingFn<Output = #field_ty>
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
    let value_spec_ty_path =
        value_spec_ty_path(parent_ast, ty_generics, peace_params_path, impl_mode);

    let fields_deconstruct = fields_deconstruct(fields);
    // let field_name = field_name.unwrap_or(ValueSpec::<FieldTy>::Stored);
    let fields_unwrap_to_value_spec_fieldless = fields_vars_map(fields, |field, field_name| {
        let field_ty = &field.ty;
        let field_spec_ty_path = field_spec_ty_path(peace_params_path, field_ty);

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
