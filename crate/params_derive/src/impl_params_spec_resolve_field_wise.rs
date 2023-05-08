use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, DeriveInput, Fields, Ident, ImplGenerics, LitInt, Path, Type,
    TypeGenerics, Variant, WhereClause,
};

use crate::util::{fields_deconstruct, is_phantom_data, variant_match_arm};

/// `impl ValueSpecRt for ParamsSpec`, so that Peace can resolve the params type
/// as well as its values from the spec.
pub fn impl_params_spec_resolve_field_wise(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    peace_params_path: &Path,
    peace_resources_path: &Path,
    params_name: &Ident,
    params_field_wise_name: &Ident,
    params_partial_name: &Ident,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics_split;

    let resolve_body = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_resolve(
                ty_generics,
                params_name,
                params_field_wise_name,
                fields,
                peace_params_path,
                peace_resources_path,
                ResolveMode::Full { params_name },
            )
        }
        syn::Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_resolve(
                ty_generics,
                params_name,
                params_field_wise_name,
                variants,
                peace_params_path,
                peace_resources_path,
                ResolveMode::Full { params_name },
            )
        }
        syn::Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_resolve(
                ty_generics,
                params_name,
                params_field_wise_name,
                &fields,
                peace_params_path,
                peace_resources_path,
                ResolveMode::Full { params_name },
            )
        }
    };

    let resolve_partial_body = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_resolve(
                ty_generics,
                params_name,
                params_field_wise_name,
                fields,
                peace_params_path,
                peace_resources_path,
                ResolveMode::Partial {
                    params_partial_name,
                },
            )
        }
        syn::Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_resolve(
                ty_generics,
                params_name,
                params_field_wise_name,
                variants,
                peace_params_path,
                peace_resources_path,
                ResolveMode::Partial {
                    params_partial_name,
                },
            )
        }
        syn::Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_resolve(
                ty_generics,
                params_name,
                params_field_wise_name,
                &fields,
                peace_params_path,
                peace_resources_path,
                ResolveMode::Partial {
                    params_partial_name,
                },
            )
        }
    };

    quote! {
        impl #impl_generics #params_field_wise_name #ty_generics
        #where_clause
        {
            pub fn resolve(
                &self,
                resources: &#peace_resources_path::Resources<#peace_resources_path::resources::ts::SetUp>
            ) -> Result<#params_name #ty_generics, #peace_params_path::ParamsResolveError> {
                #resolve_body
            }

            pub fn resolve_partial(
                &self,
                resources: &#peace_resources_path::Resources<#peace_resources_path::resources::ts::SetUp>
            ) -> Result<#params_partial_name #ty_generics, #peace_params_path::ParamsResolveError> {
                #resolve_partial_body
            }
        }
    }
}

fn struct_fields_resolve(
    ty_generics: &TypeGenerics,
    params_name: &Ident,
    params_field_wise_name: &Ident,
    fields: &Fields,
    peace_params_path: &Path,
    peace_resources_path: &Path,
    resolve_mode: ResolveMode,
) -> proc_macro2::TokenStream {
    let fields_resolution = fields_resolution(
        ty_generics,
        params_name,
        fields,
        peace_params_path,
        peace_resources_path,
        resolve_mode,
    );
    let fields_deconstructed = fields_deconstruct(fields);
    let params_return_type_name = resolve_mode.params_return_type_name();

    match fields {
        Fields::Named(_fields_named) => {
            // Generates:
            //
            // ```rust
            // let #params_field_wise_name {
            //     field_1,
            //     field_2,
            //     marker: PhantomData,
            // } = self;
            //
            // #fields_resolution
            //
            // let params = #params_return_type_name {
            //     field_1,
            //     field_2,
            //     marker: PhantomData,
            // };
            //
            // Ok(params)
            // ```

            quote! {
                let #params_field_wise_name {
                    #(#fields_deconstructed),*
                } = self;

                #fields_resolution

                let params = #params_return_type_name {
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
            // let params = #params_return_type_name(_0, _1, PhantomData,);
            // Ok(params)
            // ```

            quote! {
                let #params_field_wise_name(#(#fields_deconstructed),*) = self;

                #fields_resolution

                let params = #params_return_type_name(#(#fields_deconstructed),*);
                Ok(params)
            }
        }
        Fields::Unit => quote!(Ok(#params_return_type_name)),
    }
}

fn variants_resolve(
    ty_generics: &TypeGenerics,
    params_name: &Ident,
    params_field_wise_name: &Ident,
    variants: &Punctuated<Variant, Token![,]>,
    peace_params_path: &Path,
    peace_resources_path: &Path,
    resolve_mode: ResolveMode,
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
                    &variant.ident,
                    &variant.fields,
                    &fields_deconstructed,
                    peace_params_path,
                    peace_resources_path,
                    resolve_mode,
                );
                tokens.extend(variant_match_arm(
                    params_field_wise_name,
                    variant,
                    &fields_deconstructed,
                    variant_fields_resolve,
                ));

                tokens
            });

    quote! {
        match self {
            #variant_resolve_arms
        }
    }
}

// Code gen, not user facing
#[allow(clippy::too_many_arguments)]
fn variant_fields_resolve(
    ty_generics: &TypeGenerics,
    params_name: &Ident,
    variant_name: &Ident,
    fields: &Fields,
    fields_deconstructed: &[proc_macro2::TokenStream],
    peace_params_path: &Path,
    peace_resources_path: &Path,
    resolve_mode: ResolveMode,
) -> proc_macro2::TokenStream {
    let fields_resolution = fields_resolution(
        ty_generics,
        params_name,
        fields,
        peace_params_path,
        peace_resources_path,
        resolve_mode,
    );
    let params_return_type_name = resolve_mode.params_return_type_name();

    match fields {
        Fields::Named(_fields_named) => {
            // Generates:
            //
            // ```rust
            // #fields_resolution
            //
            // let params = #params_return_type_name::Variant {
            //     field_1,
            //     field_2,
            //     marker: PhantomData,
            // };
            //
            // Ok(params)
            // ```

            quote! {
                #fields_resolution

                let params = #params_return_type_name::#variant_name {
                    #(#fields_deconstructed),*
                };
                Ok(params)
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            // Generates:
            //
            // ```rust
            // #fields_resolution
            //
            // let params = #params_return_type_name::Variant(_0, _1, PhantomData,);
            // Ok(params)
            // ```

            quote! {
                #fields_resolution

                let params = #params_return_type_name::#variant_name(#(#fields_deconstructed),*);
                Ok(params)
            }
        }
        Fields::Unit => quote!(Ok(#params_return_type_name::#variant_name)),
    }
}

fn fields_resolution(
    ty_generics: &TypeGenerics,
    params_name: &Ident,
    fields: &Fields,
    peace_params_path: &Path,
    peace_resources_path: &Path,
    resolve_mode: ResolveMode,
) -> proc_macro2::TokenStream {
    let value_clone_expr = resolve_mode.value_clone_expr();
    let from_clone_expr = resolve_mode.from_clone_expr();
    let mapping_fn_name = resolve_mode.mapping_fn_name();

    match fields {
        Fields::Named(fields_named) => {
            // Generates:
            //
            // ```rust
            // let field_1 = match field_1 {
            //     ValueSpec::Value(t) => Ok(t.clone()), // or `Ok(Some(t.clone()))`
            //     ValueSpec::From => {
            //         match resources.try_borrow::<#field_ty>() {
            //             Ok(t) => Ok((*t).clone()),    // or `Ok(Some((*t).clone()))`
            //             Err(borrow_fail) => match borrow_fail {
            //                 BorrowFail::ValueNotFound => {
            //                     // either
            //                     Ok(None)
            //
            //                     // or
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
            //         // either
            //         mapping_fn.try_map
            //
            //         // or
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

                    let value_none_expr = resolve_mode.value_none_expr(
                        ty_generics,
                        peace_params_path,
                        field_name,
                        field_ty,
                    );

                    tokens.extend(quote! {
                        let #field_name = match #field_name {
                            #peace_params_path::ValueSpec::Value(t) => Ok(#value_clone_expr),
                            #peace_params_path::ValueSpec::From => {
                                match resources.try_borrow::<#field_ty>() {
                                    Ok(t) => Ok(#from_clone_expr),
                                    Err(borrow_fail) => match borrow_fail {
                                        #peace_resources_path::BorrowFail::ValueNotFound => {
                                            #value_none_expr
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
                                mapping_fn.#mapping_fn_name(
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
            //     ValueSpec::Value(t) => Ok(t.clone()), // or `Ok(Some(t.clone()))`
            //     ValueSpec::From => {
            //         match resources.try_borrow::<#field_ty>() {
            //             Ok(t) => Ok((*t).clone()),    // or `Ok(Some((*t).clone()))`
            //             Err(borrow_fail) => match borrow_fail {
            //                 BorrowFail::ValueNotFound => {
            //                     // either
            //                     Ok(None)
            //
            //                     // or
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
            //         // either
            //         mapping_fn.try_map
            //
            //         // or
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
                    let field_ident = Ident::new(&format!("_{field_index}"), Span::call_site());
                    // Need to convert this to a `LitInt`,
                    // because `quote` outputs the index as `0usize` instead of `0`
                    let field_index = LitInt::new(&format!("{field_index}"), Span::call_site());
                    let field_ty = &field.ty;

                    let value_none_expr = resolve_mode.value_none_expr(
                        ty_generics,
                        peace_params_path,
                        &field_ident,
                        field_ty,
                    );

                    tokens.extend(quote! {
                        let #field_ident = match #field_ident {
                            #peace_params_path::ValueSpec::Value(t) => Ok(#value_clone_expr),
                            #peace_params_path::ValueSpec::From => {
                                match resources.try_borrow::<#field_ty>() {
                                    Ok(t) => Ok(#from_clone_expr),
                                    Err(borrow_fail) => match borrow_fail {
                                        #peace_resources_path::BorrowFail::ValueNotFound => {
                                            #value_none_expr
                                        }
                                        #peace_resources_path::BorrowFail::BorrowConflictImm |
                                        #peace_resources_path::BorrowFail::BorrowConflictMut => {
                                            Err(#peace_params_path::ParamsResolveError::FromBorrowConflict {
                                                params_type_name: std::any::type_name::<#params_name #ty_generics>(),
                                                field_name: stringify!(#field_index),
                                                field_type_name: std::any::type_name::<#field_ty>(),
                                            })
                                        }
                                    },
                                }
                            }
                            #peace_params_path::ValueSpec::FromMap(mapping_fn) => {
                                mapping_fn.#mapping_fn_name(
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

/// Whether all `Params` values must be resolved.
#[derive(Clone, Copy, Debug)]
enum ResolveMode<'name> {
    /// Resolving all values for params.
    Full { params_name: &'name Ident },
    /// Resolving whatever values are available.
    Partial { params_partial_name: &'name Ident },
}

impl<'name> ResolveMode<'name> {
    /// Returns `params_name` for `Params` and `params_partial_name` for
    /// `Params::Partial`.
    fn params_return_type_name(self) -> &'name Ident {
        match self {
            Self::Full { params_name } => params_name,
            Self::Partial {
                params_partial_name,
            } => params_partial_name,
        }
    }

    /// Returns `t.clone()` for `Params` and `Some(t.clone())` for
    /// `Params::Partial`.
    fn value_clone_expr(self) -> proc_macro2::TokenStream {
        match self {
            Self::Full { .. } => quote!(t.clone()),
            Self::Partial { .. } => quote!(Some(t.clone())),
        }
    }

    /// Returns `(*t).clone()` for `Params` and `Some((*t).clone())` for
    /// `Params::Partial`.
    // this is the *from* clone expression, not something from "clone_expression"
    #[allow(clippy::wrong_self_convention)]
    fn from_clone_expr(self) -> proc_macro2::TokenStream {
        match self {
            Self::Full { .. } => quote!((*t).clone()),
            Self::Partial { .. } => quote!(Some((*t).clone())),
        }
    }

    /// Returns `map` or `try_map` to call on the `MappingFn`.
    fn mapping_fn_name(self) -> Ident {
        match self {
            Self::Full { .. } => Ident::new("map", Span::call_site()),
            Self::Partial { .. } => Ident::new("try_map", Span::call_site()),
        }
    }

    /// Returns the expression to use when the value is not in `resources`.
    fn value_none_expr(
        self,
        ty_generics: &TypeGenerics,
        peace_params_path: &Path,
        field_name: &Ident,
        field_ty: &Type,
    ) -> proc_macro2::TokenStream {
        match self {
            Self::Full { params_name } => quote! {
                Err(#peace_params_path::ParamsResolveError::From {
                    params_type_name: std::any::type_name::<#params_name #ty_generics>(),
                    field_name: stringify!(#field_name),
                    field_type_name: std::any::type_name::<#field_ty>(),
                })
            },
            Self::Partial { .. } => quote! {
                Ok(None)
            },
        }
    }
}
