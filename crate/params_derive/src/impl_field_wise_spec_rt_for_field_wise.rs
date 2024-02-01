use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, Data, DeriveInput, Field, Fields, Ident, ImplGenerics, LitInt, Path,
    TypeGenerics, Variant, WhereClause,
};

use crate::util::{field_spec_ty_path, fields_deconstruct, is_phantom_data, variant_match_arm};

/// `impl FieldWiseSpecRt for ValueSpec`, so that Peace can resolve the params
/// type as well as its values from the spec.
pub fn impl_field_wise_spec_rt_for_field_wise(
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
        Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_resolve(
                params_field_wise_name,
                fields,
                peace_params_path,
                ResolveMode::Full { params_name },
            )
        }
        Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_resolve(
                params_field_wise_name,
                variants,
                peace_params_path,
                ResolveMode::Full { params_name },
            )
        }
        Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_resolve(
                params_field_wise_name,
                &fields,
                peace_params_path,
                ResolveMode::Full { params_name },
            )
        }
    };

    let resolve_partial_body = match &ast.data {
        Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_resolve(
                params_field_wise_name,
                fields,
                peace_params_path,
                ResolveMode::Partial {
                    params_partial_name,
                },
            )
        }
        Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_resolve(
                params_field_wise_name,
                variants,
                peace_params_path,
                ResolveMode::Partial {
                    params_partial_name,
                },
            )
        }
        Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_resolve(
                params_field_wise_name,
                &fields,
                peace_params_path,
                ResolveMode::Partial {
                    params_partial_name,
                },
            )
        }
    };

    quote! {
        impl #impl_generics #peace_params_path::FieldWiseSpecRt
        for #params_field_wise_name #ty_generics
        #where_clause
        {
            type ValueType = #params_name #ty_generics;
            type Partial = #params_partial_name #ty_generics;

            fn resolve(
                &self,
                resources: &#peace_resources_path::Resources<#peace_resources_path::resources::ts::SetUp>,
                value_resolution_ctx: &mut #peace_params_path::ValueResolutionCtx<ItemIdT>,
            ) -> Result<#params_name #ty_generics, #peace_params_path::ParamsResolveError<ItemIdT>> {
                #resolve_body
            }

            fn resolve_partial(
                &self,
                resources: &#peace_resources_path::Resources<#peace_resources_path::resources::ts::SetUp>,
                value_resolution_ctx: &mut #peace_params_path::ValueResolutionCtx<ItemIdT>,
            ) -> Result<#params_partial_name #ty_generics, #peace_params_path::ParamsResolveError<ItemIdT>> {
                #resolve_partial_body
            }
        }
    }
}

fn struct_fields_resolve(
    params_field_wise_name: &Ident,
    fields: &Fields,
    peace_params_path: &Path,
    resolve_mode: ResolveMode,
) -> proc_macro2::TokenStream {
    let fields_resolution = fields_resolution(fields, peace_params_path, resolve_mode);
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
    params_field_wise_name: &Ident,
    variants: &Punctuated<Variant, Token![,]>,
    peace_params_path: &Path,
    resolve_mode: ResolveMode,
) -> proc_macro2::TokenStream {
    // Generates:
    //
    // ```rust
    // match self {
    //     ValueSpec::Variant1 => Ok(Params::Variant1),
    //     ValueSpec::Variant2(_0, _1, PhantomData) => {
    //         let _0 = ..?;
    //         let _1 = ..?;
    //         let params = Params::Variant2(_0, _1, PhantomData);
    //         Ok(params)
    //     }
    //     ValueSpec::Variant3 { field_1, field_2, marker: PhantomData } => {
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
                    &variant.ident,
                    &variant.fields,
                    &fields_deconstructed,
                    peace_params_path,
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
    variant_name: &Ident,
    fields: &Fields,
    fields_deconstructed: &[proc_macro2::TokenStream],
    peace_params_path: &Path,
    resolve_mode: ResolveMode,
) -> proc_macro2::TokenStream {
    let fields_resolution = fields_resolution(fields, peace_params_path, resolve_mode);
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
    fields: &Fields,
    peace_params_path: &Path,
    resolve_mode: ResolveMode,
) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => {
            // Generates:
            //
            // ```rust
            // value_resolution_ctx.push(#peace_params_path::FieldNameAndType::new(
            //     String::from(stringify!(field_1)),
            //     String::from(#peace_params_path::tynm::type_name::<#field_ty>())),
            // );
            // let field_1 = field_1.resolve(resources, value_resolution_ctx)?;
            // value_resolution_ctx.pop();
            // ```

            fields_named
                .named
                .iter()
                .filter(|field| !is_phantom_data(&field.ty))
                .filter_map(|field| field.ident.as_ref().map(|field_name| (field, field_name)))
                .fold(
                    proc_macro2::TokenStream::new(),
                    |mut tokens, (field, field_name)| {
                        let field_ty = &field.ty;
                        let resolve_method =
                            resolve_mode.resolve_method(peace_params_path, field_name, field);

                        tokens.extend(quote! {
                            value_resolution_ctx.push(#peace_params_path::FieldNameAndType::new(
                                String::from(stringify!(#field_name)),
                                String::from(#peace_params_path::tynm::type_name::<#field_ty>())),
                            );
                            #resolve_method
                            value_resolution_ctx.pop();
                        });
                        tokens
                    },
                )
        }
        Fields::Unnamed(fields_unnamed) => {
            // Generates:
            //
            // ```rust
            // value_resolution_ctx.push(#peace_params_path::FieldNameAndType::new(
            //     String::from(stringify!(1)),
            //     String::from(#peace_params_path::tynm::type_name::<#field_ty>())),
            // );
            // let _1 = _1.resolve(resources, value_resolution_ctx)?;
            // value_resolution_ctx.pop();
            // ```
            fields_unnamed
                .unnamed
                .iter()
                .enumerate()
                .filter(|(_field_index, field)| !is_phantom_data(&field.ty))
                .fold(
                    proc_macro2::TokenStream::new(),
                    |mut tokens, (field_index, field)| {
                        let field_ident = Ident::new(&format!("_{field_index}"), Span::call_site());
                        // Need to convert this to a `LitInt`,
                        // because `quote` outputs the index as `0usize` instead of `0`
                        let field_index = LitInt::new(&format!("{field_index}"), Span::call_site());
                        let field_ty = &field.ty;
                        let resolve_method =
                            resolve_mode.resolve_method(peace_params_path, &field_ident, field);

                        tokens.extend(quote! {
                            value_resolution_ctx.push(#peace_params_path::FieldNameAndType::new(
                                String::from(stringify!(#field_index)),
                                String::from(#peace_params_path::tynm::type_name::<#field_ty>())),
                            );
                            #resolve_method
                            value_resolution_ctx.pop();
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

    // Returns `resolve` for `Full` resolution, and `resolve_partial` for `Partial`
    // resolution.
    fn resolve_method(
        self,
        peace_params_path: &Path,
        field_name: &Ident,
        field: &Field,
    ) -> proc_macro2::TokenStream {
        let field_ty = &field.ty;
        let field_spec_ty_path = field_spec_ty_path(peace_params_path, field_ty);
        match self {
            Self::Full { .. } => quote! {
                let #field_name = #field_spec_ty_path::resolve(
                    #field_name,
                    resources,
                    value_resolution_ctx,
                )?;
            },
            Self::Partial { .. } => quote! {
                let #field_name = #field_spec_ty_path::resolve_partial(
                    #field_name,
                    resources,
                    value_resolution_ctx,
                )?;
            },
        }
    }
}
