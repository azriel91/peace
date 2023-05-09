use syn::{
    punctuated::Punctuated, DeriveInput, Fields, Ident, ImplGenerics, Path, TypeGenerics, Variant,
    WhereClause,
};

use crate::{
    external_type::ExternalType,
    util::{
        fields_deconstruct, is_external, is_phantom_data, tuple_ident_from_field_index,
        type_path_simple_name, variant_match_arm,
    },
};

/// `impl From<Params> for ParamsFieldWise`, so that users can provide
/// `params.into()` when building a cmd_ctx, instead of constructing a
/// `ParamsFieldWise`.
pub fn impl_from_params_for_params_field_wise(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    peace_params_path: &Path,
    params_name: &Ident,
    params_field_wise_name: &Ident,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics_split;

    let from_body = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_map_to_value(
                params_name,
                params_field_wise_name,
                fields,
                peace_params_path,
            )
        }
        syn::Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_map_to_value(
                params_name,
                params_field_wise_name,
                variants,
                peace_params_path,
            )
        }
        syn::Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_map_to_value(
                params_name,
                params_field_wise_name,
                &fields,
                peace_params_path,
            )
        }
    };

    quote! {
        impl #impl_generics From<#params_name #ty_generics>
        for #params_field_wise_name #ty_generics
        #where_clause
        {
            fn from(params: #params_name #ty_generics) -> Self {
                #from_body
            }
        }
    }
}

fn struct_fields_map_to_value(
    params_name: &Ident,
    params_field_wise_name: &Ident,
    fields: &Fields,
    peace_params_path: &Path,
) -> proc_macro2::TokenStream {
    let fields_deconstructed = fields_deconstruct(fields);
    let fields_map_to_value = fields_map_to_value(fields, peace_params_path);

    match fields {
        Fields::Named(_fields_named) => {
            // Generates:
            //
            // ```rust
            // let #params_name {
            //     field_1,
            //     field_2,
            //     marker: PhantomData,
            // } = params;
            //
            // #params_field_wise_name {
            //     #fields_map_to_value
            // }
            // ```
            quote! {
                let #params_name {
                    #(#fields_deconstructed),*
                } = params;

                #params_field_wise_name {
                    #fields_map_to_value
                }
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            // Generates:
            //
            // ```rust
            // let #params_name(#(#fields_deconstructed),*) = params;
            //
            // #params_field_wise_name(#fields_map_to_value)
            // ```
            quote! {
                let #params_name(#(#fields_deconstructed),*) = params;

                #params_field_wise_name(#fields_map_to_value)
            }
        }
        Fields::Unit => quote!(#params_field_wise_name),
    }
}

fn variants_map_to_value(
    params_name: &Ident,
    params_field_wise_name: &Ident,
    variants: &Punctuated<Variant, Token![,]>,
    peace_params_path: &Path,
) -> proc_macro2::TokenStream {
    // Generates:
    //
    // match params {
    //     Params::Variant1 => Params::Variant1,
    //     Params::Variant2(_0, _1, PhantomData) => {
    //         Params::Variant2(
    //             #peace_params_path::ValueSpec::Value(_0),
    //             #peace_params_path::ValueSpec::Value(_1),
    //             PhantomData,
    //
    //             // or
    //             // #peace_params_path::ValueSpec::Value(Wrapper(_0)),
    //             // #peace_params_path::ValueSpec::Value(Wrapper(_1)),
    //         )
    //     }
    //     Params::Variant3 { field_1, field_2, marker: PhantomData } => {
    //         Params::Variant3 {
    //             field_1: #peace_params_path::ValueSpec::Value(field_1),
    //             field_2: #peace_params_path::ValueSpec::Value(field_2),
    //             marker: PhantomData,
    //
    //             // or
    //             // field_1: #peace_params_path::ValueSpec::Value(Wrapper(_0)),
    //             // field_2: #peace_params_path::ValueSpec::Value(Wrapper(_1)),
    //         }
    //     }
    // }

    let variant_map_to_value_arms =
        variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                let variant_fields = fields_deconstruct(&variant.fields);
                let variant_fields_map_to_value = variant_fields_map_to_value(
                    params_field_wise_name,
                    &variant.ident,
                    &variant.fields,
                    peace_params_path,
                );
                tokens.extend(variant_match_arm(
                    params_name,
                    variant,
                    &variant_fields,
                    variant_fields_map_to_value,
                ));

                tokens
            });

    quote! {
        match params {
            #variant_map_to_value_arms
        }
    }
}

fn variant_fields_map_to_value(
    params_field_wise_name: &Ident,
    variant_name: &Ident,
    fields: &Fields,
    peace_params_path: &Path,
) -> proc_macro2::TokenStream {
    let fields_map_to_value = fields_map_to_value(fields, peace_params_path);
    match fields {
        Fields::Named(_fields_named) => {
            // Generates:
            //
            // ```rust
            // ParamsFieldWiseName {
            //     #fields_map_to_value
            // }
            // ```
            quote! {
                #params_field_wise_name::#variant_name {
                    #fields_map_to_value
                }
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            // Generates:
            //
            // ```rust
            // ParamsFieldWiseName(#fields_map_to_value)
            // ```

            quote! {
                #params_field_wise_name::#variant_name(#fields_map_to_value)
            }
        }
        Fields::Unit => quote!(Self::#variant_name),
    }
}

fn fields_map_to_value(fields: &Fields, peace_params_path: &Path) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => {
            // Generates:
            //
            // ```rust
            // field_1: #peace_params_path::ValueSpec::Value(field_1),
            // field_2: #peace_params_path::ValueSpec::Value(field_2),
            // marker: PhantomData,
            //
            // // or
            // // field_1: #peace_params_path::ValueSpec::Value(Wrapper(field_1)),
            // // field_2: #peace_params_path::ValueSpec::Value(Wrapper(field_2)),
            // ```

            fields_named
                .named
                .iter()
                .fold(proc_macro2::TokenStream::new(), |mut tokens, field| {
                    if let Some(field_name) = field.ident.as_ref() {
                        if is_phantom_data(&field.ty) {
                            tokens.extend(quote! {
                                #field_name: std::marker::PhantomData,
                            });
                        } else {
                            let field_deconstruct = if is_external(&field.attrs) {
                                let external_type = ExternalType::wrapper_type(&field.ty);
                                let wrapper_type_simple_name = type_path_simple_name(&external_type);
                                quote!(#wrapper_type_simple_name(#field_name))
                            } else {
                                quote!(#field_name)
                            };

                            tokens.extend(quote! {
                                #field_name: #peace_params_path::ValueSpec::Value(#field_deconstruct),
                            });
                        }
                    }
                    tokens
                })
        }
        Fields::Unnamed(fields_unnamed) => {
            // Generates:
            //
            // ```rust
            // #peace_params_path::ValueSpec::Value(_0),
            // #peace_params_path::ValueSpec::Value(_1),
            // PhantomData,
            //
            // // or
            // // #peace_params_path::ValueSpec::Value(Wrapper(_0)),
            // // #peace_params_path::ValueSpec::Value(Wrapper(_1)),
            // ```
            fields_unnamed.unnamed.iter().enumerate().fold(
                proc_macro2::TokenStream::new(),
                |mut tokens, (field_index, field)| {
                    let field_name = tuple_ident_from_field_index(field_index);

                    if is_phantom_data(&field.ty) {
                        tokens.extend(quote!(std::marker::PhantomData,));
                    } else {
                        let field_deconstruct = if is_external(&field.attrs) {
                            let external_type = ExternalType::wrapper_type(&field.ty);
                            let wrapper_type_simple_name = type_path_simple_name(&external_type);
                            quote!(#wrapper_type_simple_name(#field_name))
                        } else {
                            quote!(#field_name)
                        };

                        tokens.extend(
                            quote!(#peace_params_path::ValueSpec::Value(#field_deconstruct),),
                        );
                    }

                    tokens
                },
            )
        }
        Fields::Unit => quote!(),
    }
}
