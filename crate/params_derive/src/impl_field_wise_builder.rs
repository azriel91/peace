use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    Data, DataEnum, DeriveInput, Fields, Ident, ImplGenerics, Path, Type, TypeGenerics, WhereClause,
};

use crate::{
    field_wise_enum_builder_ctx::FieldWiseEnumBuilderCtx,
    fields_map::{field_to_optional_value_spec, fields_map, fields_to_optional_value_spec},
    impl_default,
    type_gen::TypeGen,
    util::{
        field_spec_ty_deconstruct, field_spec_ty_path, fields_deconstruct, fields_vars_map,
        is_phantom_data, tuple_ident_from_field_index, tuple_index_from_field_index, value_spec_ty,
        value_spec_ty_path, variant_generics_intersect, variant_generics_where_clause, ImplMode,
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
                        Builder for specification of how to look up the values for an item's \n\
                        parameters.\
                    "]
                }],
                false,
            );
            // Note: struct builder has getters / setters generated in
            // `TypeGen::gen_from_value_type`.

            let impl_default = impl_default(ast, generics_split, value_field_wise_builder_name);

            let fields = &data_struct.fields;

            let builder_field_methods = builder_field_methods(fields, peace_params_path, None);
            let build_method_body = build_method_body(
                ast,
                ty_generics,
                peace_params_path,
                BuildMode::Struct {
                    value_field_wise_name,
                },
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
            impl_mode,
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
#[allow(clippy::too_many_arguments)]
fn impl_enum_builder(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    peace_params_path: &Path,
    value_field_wise_name: &Ident,
    value_field_wise_builder_name: &Ident,
    field_wise_enum_builder_ctx: &FieldWiseEnumBuilderCtx,
    impl_mode: ImplMode,
    data_enum: &DataEnum,
) -> proc_macro2::TokenStream {
    let variant_selection_ident = &Ident::new("variant_selection", Span::call_site());
    let enum_builder_generics = &field_wise_enum_builder_ctx.generics;
    let enum_params_variant_none = &field_wise_enum_builder_ctx.variant_none;
    let ty_generics_idents = &field_wise_enum_builder_ctx.ty_generics_idents;
    let type_params_with_variant_none = &field_wise_enum_builder_ctx.type_params_with_variant_none;
    let (impl_generics, ty_generics, _where_clause) = generics_split;
    let (builder_impl_generics, builder_ty_generics, builder_where_clause) =
        enum_builder_generics.split_for_impl();
    let value_spec_ty = value_spec_ty(ast, ty_generics, peace_params_path, impl_mode);

    let variant_selection_struct_tokenses = data_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let fields = &variant.fields;

            let variant_name_snake_case = Ident::new(
                &format!("{}", heck::AsSnakeCase(format!("{variant_name}"))),
                Span::call_site(),
            );

            let variant_selection_name =
                &format_ident!("{}Variant{}", value_field_wise_builder_name, variant_name);
            let variant_selection_ty_params = variant_generics_intersect(&ast.generics, variant);
            let variant_selection_ty_params_angle_bracketed =
                if variant_selection_ty_params.is_empty() {
                    quote!()
                } else {
                    quote!(<#(#variant_selection_ty_params,)*>)
                };
            let variant_generics_where_clause = variant_generics_where_clause(
                &ast.generics,
                &variant_selection_ty_params
            );

            let variant_selection_struct: DeriveInput = {
                let variant_selection_struct_fields = fields_map(fields, |field| {
                    field_to_optional_value_spec(field, peace_params_path).to_token_stream()
                });
                if matches!(&variant_selection_struct_fields, Fields::Named(_)) {
                    parse_quote! {
                        #[derive(Debug)]
                        pub struct #variant_selection_name #variant_selection_ty_params_angle_bracketed
                        #variant_generics_where_clause
                        #variant_selection_struct_fields
                    }
                } else {
                    parse_quote! {
                        #[derive(Debug)]
                        pub struct #variant_selection_name #variant_selection_ty_params_angle_bracketed
                        #variant_selection_struct_fields #variant_generics_where_clause;
                    }
                }
            };
            let variant_selection_generics_split =
                variant_selection_struct.generics.split_for_impl();
            let impl_default = impl_default(
                &variant_selection_struct,
                &variant_selection_generics_split,
                variant_selection_name,
            );

            let impl_builder_fns = {
                let builder_field_methods =
                    builder_field_methods(fields, peace_params_path, None);
                let (
                    variant_selection_impl_generics,
                    variant_selection_ty_generics,
                    variant_selection_where_clause,
                ) = &variant_selection_generics_split;

                quote! {
                    impl #variant_selection_impl_generics
                    #variant_selection_name #variant_selection_ty_generics
                    #variant_selection_where_clause
                    {
                        #builder_field_methods
                    }
                }
            };

            // The enum builder, with this variant selection selected as its first type param.
            let value_field_wise_builder_with_variant_selected = quote! {
                #value_field_wise_builder_name<
                    #variant_selection_name #variant_selection_ty_params_angle_bracketed,
                    #ty_generics_idents
                >
            };

            let variant_builder_fn = quote! {
                pub fn #variant_name_snake_case(self)
                -> #value_field_wise_builder_with_variant_selected
                {
                    #value_field_wise_builder_name {
                        #variant_selection_ident: #variant_selection_name::default(),
                        marker: ::std::marker::PhantomData,
                    }
                }
            };

            let proxy_impl_builder_fns = {
                let builder_field_methods = builder_field_methods(
                    fields,
                    peace_params_path,
                    Some(variant_selection_ident),
                );
                let build_method_body = build_method_body(
                    ast,
                    ty_generics,
                    peace_params_path,
                    BuildMode::Enum {
                        value_field_wise_name,
                        variant_name,
                        variant_selection_name,
                        variant_selection_ident,
                    },
                    fields,
                    impl_mode,
                );

                // Note: We use `impl_generics` instead of `builder_impl_generics`
                // because we don't need the `VariantSelection` type parameter.
                quote! {
                    impl #impl_generics
                    #value_field_wise_builder_with_variant_selected
                    #builder_where_clause
                    {
                        #builder_field_methods

                        pub fn build(self) -> #value_spec_ty {
                            #build_method_body
                        }
                    }
                }
            };

            VariantSelectionStructTokens {
                struct_declaration: variant_selection_struct,
                impl_default,
                impl_builder_fns,
                variant_builder_fn,
                proxy_impl_builder_fns,
            }
        })
        .collect::<Vec<VariantSelectionStructTokens>>();
    let variant_selection_structs_and_impls =
        variant_selection_struct_tokenses
            .iter()
            .map(|variant_selection_struct_tokens| {
                let VariantSelectionStructTokens {
                    struct_declaration,
                    impl_default,
                    impl_builder_fns,
                    variant_builder_fn: _,
                    proxy_impl_builder_fns,
                } = variant_selection_struct_tokens;

                quote! {
                    #struct_declaration

                    #impl_default

                    #impl_builder_fns

                    #proxy_impl_builder_fns
                }
            });
    let variant_builder_fns = variant_selection_struct_tokenses
        .iter()
        .map(|variant_selection_struct_tokens| &variant_selection_struct_tokens.variant_builder_fn);

    let marker_type: Type = if ast.generics.params.is_empty() {
        parse_quote!(::std::marker::PhantomData<()>)
    } else {
        parse_quote!(::std::marker::PhantomData<(#ty_generics_idents)>)
    };

    quote! {
        pub struct #value_field_wise_builder_name #builder_ty_generics {
            #variant_selection_ident: VariantSelection,
            marker: #marker_type,
        }

        impl #impl_generics ::std::default::Default
        for #value_field_wise_builder_name #type_params_with_variant_none
        #builder_where_clause
        {
            fn default() -> Self {
                #value_field_wise_builder_name {
                    #variant_selection_ident: #enum_params_variant_none,
                    marker: ::std::marker::PhantomData,
                }
            }
        }

        impl #builder_impl_generics #value_field_wise_builder_name #builder_ty_generics
        #builder_where_clause
        {
            #(#variant_builder_fns)*
        }

        // VariantSelections

        #[derive(Clone, Copy, Debug)]
        pub struct #enum_params_variant_none;

        #(#variant_selection_structs_and_impls)*
    }
}

/// Returns `with_field(mut self, value_spec: ValueSpec<FieldType>) -> Self` for
/// all non-`PhantomData` fields.
fn builder_field_methods(
    fields: &Fields,
    peace_params_path: &Path,
    proxy_field: Option<&Ident>,
) -> proc_macro2::TokenStream {
    let proxy_call = proxy_field.map(|proxy_field| quote!(.#proxy_field));
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
            let with_field_name_in_memory = Ident::new(
                &format!("with_{self_field_name}_in_memory"),
                Span::call_site(),
            );
            let with_field_name_from_mapping_fn = Ident::new(
                &format!("with_{self_field_name}_from_mapping_fn"),
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
            //     .with_dest_from_mapping_fn(|workspace_dir: &WorkspaceDir| {
            //         workspace.dir.join("web_app.tar")
            //     })
            //     .build();
            //
            // let mut cmd_ctx = // ..
            //     .with_item_params::<_>(item_id, params_spec)
            //     .await?;
            // ```
            quote! {
                pub fn #with_field_name(mut self, #field_name: #field_ty) -> Self {
                    self #proxy_call.#self_field_name = Some(#field_spec_ty_deconstruct);
                    self
                }

                pub fn #with_field_name_in_memory(mut self) -> Self {
                    self #proxy_call.#self_field_name = Some(#field_spec_ty_path::InMemory);
                    self
                }

                pub fn #with_field_name_from_mapping_fn<MFns>(mut self, mapping_fn: MFns) -> Self
                where
                    MFns: #peace_params_path::MappingFns,
                {
                    self #proxy_call.#self_field_name = Some(#field_spec_ty_path::MappingFn {
                        field_name: Some(String::from(stringify!(#field_name))),
                        mapping_fn_id: mapping_fn.id(),
                    });
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
    build_mode: BuildMode<'_>,
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

    let (deconstructed_type, deconstructed_object) = match build_mode {
        BuildMode::Struct { .. } => (quote!(Self), quote!(self)),
        BuildMode::Enum {
            variant_selection_name,
            variant_selection_ident,
            ..
        } => (
            quote!(#variant_selection_name),
            quote!(self.#variant_selection_ident),
        ),
    };

    let value_field_wise_type_or_variant = match build_mode {
        BuildMode::Struct {
            value_field_wise_name,
        } => quote!(#value_field_wise_name),
        BuildMode::Enum {
            value_field_wise_name,
            variant_name,
            variant_selection_name: _,
            variant_selection_ident: _,
        } => quote!(#value_field_wise_name::#variant_name),
    };

    match fields {
        Fields::Named(_) => {
            quote! {
                let #deconstructed_type {
                    #(#fields_deconstruct),*
                } = #deconstructed_object;

                #(#fields_unwrap_to_value_spec_fieldless)*

                let field_wise_spec = #value_field_wise_type_or_variant {
                    #(#fields_deconstruct),*
                };

                #value_spec_ty_path::FieldWise { field_wise_spec }
            }
        }
        Fields::Unnamed(_) => {
            quote! {
                let #deconstructed_type(#(#fields_deconstruct),*) = #deconstructed_object;

                #(#fields_unwrap_to_value_spec_fieldless)*

                let field_wise_spec = #value_field_wise_type_or_variant(#(#fields_deconstruct),*);

                #value_spec_ty_path::FieldWise { field_wise_spec }
            }
        }
        Fields::Unit => quote!(#value_spec_ty_path::FieldWise {
            field_wise_spec: #value_field_wise_type_or_variant,
        }),
    }
}

struct VariantSelectionStructTokens {
    /// AST of the variant selection struct.
    struct_declaration: DeriveInput,
    /// impl Default for `VariantSelectionStruct`
    impl_default: Option<proc_macro2::TokenStream>,
    /// Functions on the variant builder itself
    impl_builder_fns: proc_macro2::TokenStream,
    /// Function to return the variant builder, from the enum builder.
    variant_builder_fn: proc_macro2::TokenStream,
    /// Builder functions for the enum to proxy to the variant selection
    /// builder.
    proxy_impl_builder_fns: proc_macro2::TokenStream,
}

enum BuildMode<'name> {
    Struct {
        /// Name of the FieldWise type to build.
        value_field_wise_name: &'name Ident,
    },
    Enum {
        /// Name of the FieldWise type to build.
        value_field_wise_name: &'name Ident,
        /// Variant within the type to build.
        variant_name: &'name Ident,
        /// Name of the variant selection type within the enum builder.
        variant_selection_name: &'name Ident,
        /// Name of the variant selection field within the enum builder.
        variant_selection_ident: &'name Ident,
    },
}
