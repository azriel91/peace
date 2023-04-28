use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, DeriveInput, Fields, Ident, ImplGenerics, LitInt, Path, TypeGenerics,
    Variant, WhereClause,
};

use crate::util::{fields_deconstruct, is_phantom_data, variant_match_arm};

/// `impl ParamsSpec for ParamsSpec`, so that Peace can resolve the params type
/// as well as its values from the spec.
pub fn impl_params_spec_for_params_spec(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    peace_params_path: &Path,
    peace_resources_path: &Path,
    params_name: &Ident,
    params_spec_name: &Ident,
    params_partial_name: &Ident,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics_split;

    let resolve_body = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_resolve(
                ty_generics,
                params_name,
                params_spec_name,
                fields,
                peace_params_path,
                peace_resources_path,
            )
        }
        syn::Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_resolve(
                ty_generics,
                params_name,
                params_spec_name,
                variants,
                peace_params_path,
                peace_resources_path,
            )
        }
        syn::Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_resolve(
                ty_generics,
                params_name,
                params_spec_name,
                &fields,
                peace_params_path,
                peace_resources_path,
            )
        }
    };

    quote! {
        impl #impl_generics #peace_params_path::ParamsSpec
        for #params_spec_name #ty_generics
        #where_clause
        {
            type Params = #params_name #ty_generics;
            type Partial = #params_partial_name #ty_generics;

            fn resolve(
                &self,
                resources: &#peace_resources_path::Resources<#peace_resources_path::resources::ts::SetUp>
            ) -> Result<Self::Params, #peace_params_path::ParamsResolveError> {
                #resolve_body
            }

            fn resolve_partial(
                &self,
                resources: &#peace_resources_path::Resources<#peace_resources_path::resources::ts::SetUp>
            ) -> Self::Partial {
                todo!()
            }
        }
    }
}

fn struct_fields_resolve(
    ty_generics: &TypeGenerics,
    params_name: &Ident,
    params_spec_name: &Ident,
    fields: &Fields,
    peace_params_path: &Path,
    peace_resources_path: &Path,
) -> proc_macro2::TokenStream {
    let fields_resolution = fields_resolution(
        ty_generics,
        params_name,
        fields,
        peace_params_path,
        peace_resources_path,
    );
    let fields_deconstructed = fields_deconstruct(fields);

    match fields {
        Fields::Named(_fields_named) => {
            // Generates:
            //
            // ```rust
            // let #params_spec_name {
            //     field_1,
            //     field_2,
            //     marker: PhantomData,
            // } = self;
            //
            // #fields_resolution
            //
            // let params = #params_name {
            //     field_1,
            //     field_2,
            //     marker: PhantomData,
            // };
            //
            // Ok(params)
            // ```

            quote! {
                let #params_spec_name {
                    #(#fields_deconstructed),*
                } = self;

                #fields_resolution

                let params = #params_name {
                    #(#fields_deconstructed),*
                };
                Ok(params)
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            // Generates:
            //
            // ```rust
            // let #params_name(_0, _1, PhantomData,) = self;
            //
            // #fields_resolution
            //
            // let params = #params_name(_0, _1, PhantomData,);
            // Ok(params)
            // ```

            quote! {
                let #params_spec_name(#(#fields_deconstructed),*) = self;

                #fields_resolution

                let params = #params_name(#(#fields_deconstructed),*);
                Ok(params)
            }
        }
        Fields::Unit => quote!(Ok(#params_name)),
    }
}

fn variants_resolve(
    ty_generics: &TypeGenerics,
    params_name: &Ident,
    params_spec_name: &Ident,
    variants: &Punctuated<Variant, Token![,]>,
    peace_params_path: &Path,
    peace_resources_path: &Path,
) -> proc_macro2::TokenStream {
    // Generates:
    //
    // ```rust
    // match self {
    //     ParamsSpec::Variant1 => Ok(Params::Variant1),
    //     ParamsSpec::Variant2(_0, _1, PhantomData) => {
    //         let _0 = ..?;
    //         let _1 = ..?;
    //         let params = Params::Variant2(_0, _1, PhantomData);
    //         Ok(params)
    //     }
    //     ParamsSpec::Variant3 { field_1, field_2, marker: PhantomData } => {
    //         #variant_fields_resolve
    //     }
    // }
    // ```

    let variant_resolve_arms =
        variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                let fields_deconstructed = fields_deconstruct(&variant.fields);

                let variant_fields_resolve = variant_fields_resolve(
                    ty_generics,
                    params_name,
                    params_spec_name,
                    &variant.ident,
                    &variant.fields,
                    &fields_deconstructed,
                    peace_params_path,
                    peace_resources_path,
                );
                tokens.extend(variant_match_arm(
                    params_spec_name,
                    variant,
                    &fields_deconstructed,
                    variant_fields_resolve,
                ));

                tokens
            });

    quote! {
        match params {
            #variant_resolve_arms
        }
    }
}

fn variant_fields_resolve(
    ty_generics: &TypeGenerics,
    params_name: &Ident,
    params_spec_name: &Ident,
    variant_name: &Ident,
    fields: &Fields,
    fields_deconstructed: &[proc_macro2::TokenStream],
    peace_params_path: &Path,
    peace_resources_path: &Path,
) -> proc_macro2::TokenStream {
    let fields_resolution = fields_resolution(
        ty_generics,
        params_name,
        fields,
        peace_params_path,
        peace_resources_path,
    );

    match fields {
        Fields::Named(_fields_named) => {
            // Generates:
            //
            // ```rust
            // let #params_spec_name::Variant {
            //     field_1,
            //     field_2,
            //     marker: PhantomData,
            // } = self;
            //
            // #fields_resolution
            //
            // let params = #params_name::Variant {
            //     field_1,
            //     field_2,
            //     marker: PhantomData,
            // };
            //
            // Ok(params)
            // ```

            quote! {
                let #params_spec_name::#variant_name {
                    #(#fields_deconstructed),*
                } = self;

                #fields_resolution

                let params = #params_name::#variant_name {
                    #(#fields_deconstructed),*
                };
                Ok(params)
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            // Generates:
            //
            // ```rust
            // let #params_name::Variant(_0, _1, PhantomData,) = self;
            //
            // #fields_resolution
            //
            // let params = #params_name::Variant(_0, _1, PhantomData,);
            // Ok(params)
            // ```

            quote! {
                let #params_spec_name::#variant_name(#(#fields_deconstructed),*) = self;

                #fields_resolution

                let params = #params_name::#variant_name(#(#fields_deconstructed),*);
                Ok(params)
            }
        }
        Fields::Unit => quote!(Ok(#params_name::#variant_name)),
    }
}

fn fields_resolution(
    ty_generics: &TypeGenerics,
    params_name: &Ident,
    fields: &Fields,
    peace_params_path: &Path,
    peace_resources_path: &Path,
) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => {
            // Generates:
            //
            // ```rust
            // let field_1 = match field_1 {
            //     ValueSpec::Value(t) => Ok(t.clone()),
            //     ValueSpec::From => {
            //         match resources.try_borrow::<#field_ty>() {
            //             Ok(t) => Ok((*t).clone()),
            //             Err(borrow_fail) => match borrow_fail {
            //                 BorrowFail::ValueNotFound => {
            //                     Err(ParamsResolveError::From {
            //                         params_type_name: std::any::type_name::<#params_name #ty_generics>(),
            //                         field_name: stringify!(#field_name),
            //                         field_type_name: std::any::type_name::<#field_ty>(),
            //                     })
            //                 }
            //                 BorrowFail::BorrowConflictImm |
            //                 BorrowFail::BorrowConflictMut => {
            //                     Err(ParamsResolveError::FromBorrowConflict {
            //                         params_type_name: std::any::type_name::<#params_name #ty_generics>(),
            //                         field_name: stringify!(#field_name),
            //                         field_type_name: std::any::type_name::<#field_ty>(),
            //                     })
            //                 }
            //             },
            //         }
            //     }
            //     ValueSpec::FromMap(mapping_fn) => {
            //         mapping_fn.map(
            //             resources,
            //             std::any::type_name::<#params_name #ty_generics>, // params_type_name_fn
            //             stringify!(#field_name),
            //         )
            //     }
            // };
            // ```

            fields_named.named.iter()
                .filter(|field| !is_phantom_data(&field.ty))
                .filter_map(|field| field.ident.as_ref().map(|field_name| (field, field_name)))
                .fold(
                proc_macro2::TokenStream::new(),
                |mut tokens, (field, field_name)| {
                    let field_ty = &field.ty;

                    tokens.extend(quote! {
                        let #field_name = match #field_name {
                            #peace_params_path::ValueSpec::Value(t) => Ok(t.clone()),
                            #peace_params_path::ValueSpec::From => {
                                match resources.try_borrow::<#field_ty>() {
                                    Ok(t) => Ok((*t).clone()),
                                    Err(borrow_fail) => match borrow_fail {
                                        #peace_resources_path::BorrowFail::ValueNotFound => {
                                            Err(#peace_params_path::ParamsResolveError::From {
                                                params_type_name: std::any::type_name::<#params_name #ty_generics>(),
                                                field_name: stringify!(#field_name),
                                                field_type_name: std::any::type_name::<#field_ty>(),
                                            })
                                        }
                                        #peace_resources_path::BorrowFail::BorrowConflictImm |
                                        #peace_resources_path::BorrowFail::BorrowConflictMut => {
                                            Err(#peace_params_path::ParamsResolveError::FromBorrowConflict {
                                                params_type_name: std::any::type_name::<#params_name #ty_generics>(),
                                                field_name: stringify!(#field_name),
                                                field_type_name: std::any::type_name::<#field_ty>(),
                                            })
                                        }
                                    },
                                }
                            }
                            #peace_params_path::ValueSpec::FromMap(mapping_fn) => {
                                mapping_fn.map(
                                    resources,
                                    std::any::type_name::<#params_name #ty_generics>, // params_type_name_fn
                                    stringify!(#field_name),
                                )
                            }
                        }?;
                    });
                    tokens
                },
            )
        }
        Fields::Unnamed(fields_unnamed) => {
            // Generates:
            //
            // ```rust
            // let _0 = match _0 {
            //     ValueSpec::Value(t) => Ok(t.clone()),
            //     ValueSpec::From => {
            //         match resources.try_borrow::<#field_ty>() {
            //             Ok(t) => Ok((*t).clone()),
            //             Err(borrow_fail) => match borrow_fail {
            //                 BorrowFail::ValueNotFound => {
            //                     Err(ParamsResolveError::From {
            //                         params_type_name: std::any::type_name::<#params_name #ty_generics>(),
            //                         field_name: stringify!(#field_name),
            //                         field_type_name: std::any::type_name::<#field_ty>(),
            //                     })
            //                 }
            //                 BorrowFail::BorrowConflictImm |
            //                 BorrowFail::BorrowConflictMut => {
            //                     Err(ParamsResolveError::FromBorrowConflict {
            //                         params_type_name: std::any::type_name::<#params_name #ty_generics>(),
            //                         field_name: stringify!(#field_name),
            //                         field_type_name: std::any::type_name::<#field_ty>(),
            //                     })
            //                 }
            //             },
            //         }
            //     }
            //     ValueSpec::FromMap(mapping_fn) => {
            //         mapping_fn.map(
            //             resources,
            //             std::any::type_name::<#params_name #ty_generics>, // params_type_name_fn
            //             stringify!(#field_name),
            //         )
            //     }
            // };
            // ```
            fields_unnamed.unnamed.iter().enumerate()
                .filter(|(_field_index, field)| !is_phantom_data(&field.ty))
                .fold(
                proc_macro2::TokenStream::new(),
                |mut tokens, (field_index, field)| {
                    let field_ident = LitInt::new(&format!("_{field_index}"), Span::call_site());
                    // Need to convert this to a `LitInt`,
                    // because `quote` outputs the index as `0usize` instead of `0`
                    let field_index = LitInt::new(&format!("{field_index}"), Span::call_site());
                    let field_ty = &field.ty;

                    tokens.extend(quote! {
                        let #field_ident = match &self.#field_index {
                            #peace_params_path::ValueSpec::Value(t) => Ok(t.clone()),
                            #peace_params_path::ValueSpec::From => {
                                match resources.try_borrow::<#field_ty>() {
                                    Ok(t) => Ok((*t).clone()),
                                    Err(borrow_fail) => match borrow_fail {
                                        #peace_resources_path::BorrowFail::ValueNotFound => {
                                            Err(#peace_params_path::ParamsResolveError::From {
                                                params_type_name: std::any::type_name::<#params_name #ty_generics>(),
                                                field_index: stringify!(#field_index),
                                                field_type_name: std::any::type_name::<#field_ty>(),
                                            })
                                        }
                                        #peace_resources_path::BorrowFail::BorrowConflictImm |
                                        #peace_resources_path::BorrowFail::BorrowConflictMut => {
                                            Err(#peace_params_path::ParamsResolveError::FromBorrowConflict {
                                                params_type_name: std::any::type_name::<#params_name #ty_generics>(),
                                                field_index: stringify!(#field_index),
                                                field_type_name: std::any::type_name::<#field_ty>(),
                                            })
                                        }
                                    },
                                }
                            }
                            #peace_params_path::ValueSpec::FromMap(mapping_fn) => {
                                mapping_fn.map(
                                    resources,
                                    std::any::type_name::<#params_name #ty_generics>, // params_type_name_fn
                                    stringify!(#field_index),
                                )
                            }
                        }?;
                    });
                    tokens
                },
            )
        }
        Fields::Unit => proc_macro2::TokenStream::new(),
    }
}
