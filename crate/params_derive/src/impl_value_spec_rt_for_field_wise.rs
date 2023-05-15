use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, DeriveInput, Field, Fields, Ident, ImplGenerics, LitInt, Path,
    TypeGenerics, Variant, WhereClause,
};

use crate::util::{
    field_spec_ty, fields_deconstruct, is_phantom_data, t_value_and_try_from_partial_bounds,
    variant_match_arm,
};

/// `impl ValueSpecRt for ValueSpec`, so that Peace can resolve the params type
/// as well as its values from the spec.
pub fn impl_value_spec_rt_for_field_wise(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    peace_params_path: &Path,
    peace_resources_path: &Path,
    params_name: &Ident,
    params_field_wise_name: &Ident,
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

    let resolve_body = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_resolve(
                ast,
                params_name,
                params_field_wise_name,
                fields,
                peace_params_path,
                ResolveMode::Resolve,
            )
        }
        syn::Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_resolve(
                ast,
                params_name,
                params_field_wise_name,
                variants,
                peace_params_path,
                ResolveMode::Resolve,
            )
        }
        syn::Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_resolve(
                ast,
                params_name,
                params_field_wise_name,
                &fields,
                peace_params_path,
                ResolveMode::Resolve,
            )
        }
    };

    let try_resolve_body = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            struct_fields_resolve(
                ast,
                params_name,
                params_field_wise_name,
                fields,
                peace_params_path,
                ResolveMode::TryResolve,
            )
        }
        syn::Data::Enum(data_enum) => {
            let variants = &data_enum.variants;

            variants_resolve(
                ast,
                params_name,
                params_field_wise_name,
                variants,
                peace_params_path,
                ResolveMode::TryResolve,
            )
        }
        syn::Data::Union(data_union) => {
            let fields = Fields::from(data_union.fields.clone());

            struct_fields_resolve(
                ast,
                params_name,
                params_field_wise_name,
                &fields,
                peace_params_path,
                ResolveMode::TryResolve,
            )
        }
    };

    quote! {
        impl #impl_generics #peace_params_path::ValueSpecRt
        for #params_field_wise_name #ty_generics
        #where_clause
        {
            type ValueType = #params_name #ty_generics;

            fn resolve(
                &self,
                resources: &#peace_resources_path::Resources<#peace_resources_path::resources::ts::SetUp>,
                value_resolution_ctx: &mut #peace_params_path::ValueResolutionCtx,
            ) -> Result<#params_name #ty_generics, #peace_params_path::ParamsResolveError> {
                #resolve_body
            }

            fn try_resolve(
                &self,
                resources: &#peace_resources_path::Resources<#peace_resources_path::resources::ts::SetUp>,
                value_resolution_ctx: &mut #peace_params_path::ValueResolutionCtx,
            ) -> Result<Option<#params_name #ty_generics>, #peace_params_path::ParamsResolveError> {
                #try_resolve_body
            }
        }
    }
}

fn struct_fields_resolve(
    parent_ast: &DeriveInput,
    params_name: &Ident,
    params_field_wise_name: &Ident,
    fields: &Fields,
    peace_params_path: &Path,
    resolve_mode: ResolveMode,
) -> proc_macro2::TokenStream {
    let fields_resolution = fields_resolution(parent_ast, fields, peace_params_path, resolve_mode);
    let fields_deconstructed = fields_deconstruct(fields);
    let return_expr = resolve_mode.return_expr();

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
            // let params = #params_name {
            //     field_1,
            //     field_2,
            //     marker: PhantomData,
            // };
            //
            // Ok(params) // or Ok(Some(params))
            // ```

            quote! {
                let #params_field_wise_name {
                    #(#fields_deconstructed),*
                } = self;

                #fields_resolution

                let params = #params_name {
                    #(#fields_deconstructed),*
                };

                #return_expr
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
            // Ok(params) // or Ok(Some(params))
            // ```

            quote! {
                let #params_field_wise_name(#(#fields_deconstructed),*) = self;

                #fields_resolution

                let params = #params_name(#(#fields_deconstructed),*);

                #return_expr
            }
        }
        Fields::Unit => quote! {
            let params = #params_name;
            #return_expr
        },
    }
}

fn variants_resolve(
    parent_ast: &DeriveInput,
    params_name: &Ident,
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
    //         Ok(params) // or Ok(Some(params))
    //     }
    //     ValueSpec::Variant3 { field_1, field_2, marker: PhantomData } => {
    //         #variant_fields_resolve
    //         Ok(params) // or Ok(Some(params))
    //     }
    // }
    // ```

    let variant_resolve_arms =
        variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                let fields_deconstructed = fields_deconstruct(&variant.fields);

                let variant_fields_resolve = variant_fields_resolve(
                    parent_ast,
                    params_name,
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
    parent_ast: &DeriveInput,
    params_name: &Ident,
    variant_name: &Ident,
    fields: &Fields,
    fields_deconstructed: &[proc_macro2::TokenStream],
    peace_params_path: &Path,
    resolve_mode: ResolveMode,
) -> proc_macro2::TokenStream {
    let fields_resolution = fields_resolution(parent_ast, fields, peace_params_path, resolve_mode);
    let return_expr = resolve_mode.return_expr();

    match fields {
        Fields::Named(_fields_named) => {
            // Generates:
            //
            // ```rust
            // #fields_resolution
            //
            // let params = #params_name::Variant {
            //     field_1,
            //     field_2,
            //     marker: PhantomData,
            // };
            //
            // Ok(params) // or Ok(Some(params))
            // ```

            quote! {
                #fields_resolution

                let params = #params_name::#variant_name {
                    #(#fields_deconstructed),*
                };

                #return_expr
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            // Generates:
            //
            // ```rust
            // #fields_resolution
            //
            // let params = #params_name::Variant(_0, _1, PhantomData,);
            // Ok(params) // or Ok(Some(params))
            // ```

            quote! {
                #fields_resolution

                let params = #params_name::#variant_name(#(#fields_deconstructed),*);

                #return_expr
            }
        }
        Fields::Unit => quote! {
            let params = #params_name::#variant_name;

            #return_expr
        },
    }
}

fn fields_resolution(
    parent_ast: &DeriveInput,
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
                        let resolve_value = resolve_mode.resolve_value(
                            parent_ast,
                            peace_params_path,
                            field_name,
                            field,
                        );

                        tokens.extend(quote! {
                            value_resolution_ctx.push(#peace_params_path::FieldNameAndType::new(
                                String::from(stringify!(#field_name)),
                                String::from(#peace_params_path::tynm::type_name::<#field_ty>())),
                            );
                            #resolve_value
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
                        let resolve_value = resolve_mode.resolve_value(
                            parent_ast,
                            peace_params_path,
                            &field_ident,
                            field,
                        );

                        tokens.extend(quote! {
                            value_resolution_ctx.push(#peace_params_path::FieldNameAndType::new(
                                String::from(stringify!(#field_index)),
                                String::from(#peace_params_path::tynm::type_name::<#field_ty>())),
                            );
                            #resolve_value
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
enum ResolveMode {
    /// Resolving all values for params.
    Resolve,
    /// Resolving whatever values are available.
    TryResolve,
}

impl ResolveMode {
    // Returns `resolve` for `Full` resolution, and `try_resolve` for `Partial`
    // resolution.
    fn resolve_value(
        self,
        parent_ast: &DeriveInput,
        peace_params_path: &Path,
        field_name: &Ident,
        field: &Field,
    ) -> proc_macro2::TokenStream {
        let field_spec_ty = field_spec_ty(Some(parent_ast), peace_params_path, field);

        match self {
            Self::Resolve => quote! {
                let #field_name = <#field_spec_ty as #peace_params_path::ValueSpecRt>
                    ::resolve(#field_name, resources, value_resolution_ctx)?.into();
            },
            Self::TryResolve => quote! {
                let #field_name = <#field_spec_ty as #peace_params_path::ValueSpecRt>
                    ::try_resolve(#field_name, resources, value_resolution_ctx)?
                    .map(::std::convert::TryFrom::try_from)
                    .and_then(::std::result::Result::ok);

                let Some(#field_name) = #field_name else {
                    return Ok(None);
                };
            },
        }
    }

    fn return_expr(self) -> proc_macro2::TokenStream {
        match self {
            Self::Resolve => quote! {
                Ok(params)
            },
            Self::TryResolve => quote! {
                Ok(Some(params))
            },
        }
    }
}
