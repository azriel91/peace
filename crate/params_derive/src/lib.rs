#![recursion_limit = "256"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, Attribute, DeriveInput, Fields, Generics, Ident, ImplGenerics, LitInt,
    Path, Type, TypeGenerics, TypePath, Variant, WhereClause, WherePredicate,
};

/// Used to `#[derive]` the `Params` trait.
///
/// For regular usage, use `#[derive(Params)]`
///
/// For peace crates, also add the `#[peace_internal]` attribute, which
/// references the `peace_params` crate instead of the `peace::params`
/// re-export.
#[proc_macro_derive(Params, attributes(peace_internal))]
pub fn data_access(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Data derive: Code failed to be parsed.");

    let gen = impl_data_access(&ast);

    gen.into()
}

fn impl_data_access(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let params_name = &ast.ident;

    let peace_params_path: Path = ast
        .attrs
        .iter()
        .find(peace_internal)
        .map(|_| parse_quote!(peace_params))
        .unwrap_or_else(|| parse_quote!(peace::params));

    let mut generics = ast.generics.clone();
    type_parameters_constrain(&mut generics);
    let generics_split = generics.split_for_impl();

    // MyParams -> MyParamsPartial
    let params_partial_name = {
        let mut params_partial_name = ast.ident.to_string();
        params_partial_name.push_str("Partial");
        Ident::new(&params_partial_name, ast.ident.span())
    };

    // MyParams -> MyParamsSpec
    let params_spec_name = {
        let mut params_spec_name = ast.ident.to_string();
        params_spec_name.push_str("Spec");
        Ident::new(&params_spec_name, ast.ident.span())
    };

    // MyParams -> MyParamsSpecBuilder
    let params_spec_builder_name = {
        let mut params_spec_builder_name = ast.ident.to_string();
        params_spec_builder_name.push_str("SpecBuilder");
        Ident::new(&params_spec_builder_name, ast.ident.span())
    };

    let params_partial = params_partial(ast, &generics_split, &params_partial_name);
    let params_spec = params_spec(ast, &generics_split, &params_spec_name, &peace_params_path);
    let params_spec_builder = params_spec_builder(
        ast,
        &generics_split,
        &params_spec_builder_name,
        &peace_params_path,
        &params_spec_name,
    );

    let (impl_generics, ty_generics, where_clause) = generics_split;

    quote! {
        impl #impl_generics #peace_params_path::Params
        for #params_name #ty_generics
        #where_clause
        {
            type Spec = #params_spec_name #ty_generics;
            type SpecBuilder = #params_spec_builder_name #ty_generics;
            type Partial = #params_partial_name #ty_generics;
        }

        #params_partial

        #params_spec

        #params_spec_builder
    }
}

fn peace_internal(attr: &&Attribute) -> bool {
    attr.path().is_ident("peace_internal")
}

/// Adds a `Send + Sync + 'static` bound on each of the type parameters.
fn type_parameters_constrain(generics: &mut Generics) {
    let generic_params = &generics.params;

    let where_predicates = generic_params
        .iter()
        .filter_map(|generic_param| match generic_param {
            syn::GenericParam::Lifetime(_) => None,
            syn::GenericParam::Type(type_param) => Some(type_param),
            syn::GenericParam::Const(_) => None,
        })
        .map(|type_param| parse_quote!(#type_param: Send + Sync + 'static))
        .collect::<Vec<WherePredicate>>();

    let where_clause = generics.make_where_clause();
    where_predicates
        .into_iter()
        .for_each(|where_predicate| where_clause.predicates.push(where_predicate));
}

/// Generates something like the following:
///
/// ```rust,ignore
/// #[derive(Clone, Debug, PartialEq, Eq)]
/// struct MyParamsPartial {
///     src: Option<PathBuf>,
///     dest_ip: Option<IpAddr>,
///     dest_path: Option<PathBuf>,
/// }
/// ```
fn params_partial(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    params_partial_name: &Ident,
) -> proc_macro2::TokenStream {
    type_gen(
        ast,
        generics_split,
        params_partial_name,
        fields_to_optional,
        &[
            parse_quote! {
                #[doc="\
                    Item spec parameters that may not necessarily have values.\n\
                    \n\
                    This is used for `try_state_current` and `try_state_desired` where values \n\
                    could be referenced from predecessors, which may not yet be available, such \n\
                    as the IP address of a server that is yet to be launched, or may change, \n\
                    such as the content hash of a file which is to be re-downloaded.\n\
                "]
            },
            parse_quote!(#[derive(PartialEq, Eq, serde::Serialize, serde::Deserialize)]),
        ],
    )
}

/// Generates something like the following:
///
/// ```rust,ignore
/// struct MyParamsSpec {
///     src: peace_params::ValueSpec<PathBuf>,
///     dest_ip: peace_params::ValueSpec<IpAddr>,
///     dest_path: peace_params::ValueSpec<PathBuf>,
/// }
/// ```
fn params_spec(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    params_spec_name: &Ident,
    peace_params_path: &Path,
) -> proc_macro2::TokenStream {
    type_gen(
        ast,
        generics_split,
        params_spec_name,
        |fields| fields_to_value_spec(fields, peace_params_path),
        &[
            parse_quote! {
                #[doc="Specification of how to look up the values for an item spec's parameters."]
            },
            // `Clone` and `Debug` are implemented manually, so that type parameters do not receive
            // the `Clone` and `Debug` bounds.
            //
            // `serde::Deserialize` is not derived, as `ValueSpec` is generic, and `ValueSpecDe` is
            // used to deserialize the serialized value.
            parse_quote!(#[derive(serde::Serialize)]),
        ],
    )
}

/// Generates something like the following:
///
/// ```rust,ignore
/// struct MyParamsSpecBuilder {
///     src: Option<peace_params::ValueSpec<PathBuf>>,
///     dest_ip: Option<peace_params::ValueSpec<IpAddr>>,
///     dest_path: Option<peace_params::ValueSpec<PathBuf>>,
/// }
/// ```
fn params_spec_builder(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    params_spec_builder_name: &Ident,
    peace_params_path: &Path,
    params_spec_name: &Ident,
) -> proc_macro2::TokenStream {
    let params_spec_builder_type = type_gen(
        ast,
        generics_split,
        params_spec_builder_name,
        |fields| fields_to_optional_value_spec(fields, peace_params_path),
        &[parse_quote! {
            #[doc="\
                Builder for specification of how to look up the values for an item spec's \n\
                parameters.\n\
            "]
        }],
    );

    let (impl_generics, ty_generics, where_clause) = generics_split;

    quote! {
        #params_spec_builder_type

        impl #impl_generics #peace_params_path::ParamsSpecBuilder
        for #params_spec_builder_name #ty_generics
        #where_clause
        {
            type Output = #params_spec_name #ty_generics;
        }
    }
}

fn type_gen<F>(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    type_name: &Ident,
    fields_map: F,
    attrs: &[Attribute],
) -> proc_macro2::TokenStream
where
    F: Fn(&mut Fields),
{
    let (impl_generics, ty_generics, _where_clause) = generics_split;

    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let mut fields = data_struct.fields.clone();
            fields_map(&mut fields);
            let semi_colon_maybe = if matches!(&fields, Fields::Unnamed(_)) {
                quote!(;)
            } else {
                quote!()
            };

            let struct_fields_clone = struct_fields_clone(type_name, &fields);
            let struct_fields_debug = struct_fields_debug(type_name, &fields);

            quote! {
                #(#attrs)*
                pub struct #type_name #ty_generics #fields #semi_colon_maybe

                impl #impl_generics ::std::clone::Clone for #type_name #ty_generics {
                    fn clone(&self) -> Self {
                        #struct_fields_clone
                    }
                }

                impl #impl_generics ::std::fmt::Debug for #type_name #ty_generics {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        #struct_fields_debug
                    }
                }
            }
        }
        syn::Data::Enum(data_enum) => {
            let mut variants = data_enum.variants.clone();
            variants.iter_mut().for_each(|variant| {
                fields_map(&mut variant.fields);
            });

            let variants_clone = variants_clone(&variants);
            let variants_debug = variants_debug(&variants);

            quote! {
                #(#attrs)*
                pub enum #type_name #ty_generics {
                    #variants
                }

                impl #impl_generics ::std::clone::Clone for #type_name #ty_generics {
                    fn clone(&self) -> Self {
                        #variants_clone
                    }
                }

                impl #impl_generics ::std::fmt::Debug for #type_name #ty_generics {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        #variants_debug
                    }
                }
            }
        }
        syn::Data::Union(data_union) => {
            let mut fields = Fields::from(data_union.fields.clone());
            fields_map(&mut fields);

            quote! {
                #(#attrs)*
                pub union #type_name #ty_generics #fields
            }
        }
    }
}

fn fields_to_optional(fields: &mut Fields) {
    fields_map(fields, |field_ty| {
        if is_phantom_data(field_ty) {
            field_ty.clone()
        } else {
            parse_quote!(Option<#field_ty>)
        }
    })
}

fn fields_to_value_spec(fields: &mut Fields, peace_params_path: &Path) {
    fields_map(fields, |field_ty| {
        if is_phantom_data(field_ty) {
            field_ty.clone()
        } else {
            parse_quote!(#peace_params_path::ValueSpec<#field_ty>)
        }
    })
}

fn fields_to_optional_value_spec(fields: &mut Fields, peace_params_path: &Path) {
    fields_map(fields, |field_ty| {
        if is_phantom_data(field_ty) {
            field_ty.clone()
        } else {
            parse_quote!(Option<#peace_params_path::ValueSpec<#field_ty>>)
        }
    })
}

fn is_phantom_data(field_ty: &Type) -> bool {
    matches!(&field_ty, Type::Path(TypePath { path, .. })
        if matches!(path.segments.last(), Some(segment) if segment.ident == "PhantomData"))
}

fn fields_map<F>(fields: &mut Fields, f: F)
where
    F: Fn(&Type) -> Type,
{
    match fields {
        Fields::Named(fields_named) => {
            fields_named.named.iter_mut().for_each(|field| {
                // Don't copy across attributes, e.g. `#[serde(default)].
                field.attrs.clear();

                let field_ty = &mut field.ty;
                field.ty = f(field_ty);
            });
        }
        Fields::Unnamed(fields_unnamed) => {
            fields_unnamed.unnamed.iter_mut().for_each(|field| {
                // Don't copy across attributes, e.g. `#[serde(default)].
                field.attrs.clear();

                let field_ty = &field.ty;
                field.ty = f(field_ty);
            });
        }
        Fields::Unit => {}
    }
}

fn variants_debug(variants: &Punctuated<Variant, Token![,]>) -> proc_macro2::TokenStream {
    // Generates:
    //
    // match self {
    //     Self::Variant1 => f.debug_struct("Variant1").finish(),
    //     Self::Variant2 => f.debug_tuple("Variant2").finish(),
    //     Self::Variant3 { .. } => f.debug_struct("Variant3").field(..).finish(),
    // }

    let variant_debug_arms =
        variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                let variant_name = &variant.ident;
                let variant_fields = &variant
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(field_index, field)| {
                        field
                            .ident
                            .clone()
                            .unwrap_or_else(|| tuple_ident_from_field_index(field_index))
                    })
                    .collect::<Vec<Ident>>();
                let variant_fields_debug = variant_fields_debug(&variant.ident, &variant.fields);

                match &variant.fields {
                    Fields::Named(_fields_named) => {
                        tokens.extend(quote! {
                            Self::#variant_name { #(#variant_fields),* } => {
                                #variant_fields_debug
                            }
                        });
                    }
                    Fields::Unnamed(_) => {
                        tokens.extend(quote! {
                            Self::#variant_name(#(#variant_fields),*) => {
                                #variant_fields_debug
                            }
                        });
                    }
                    Fields::Unit => {
                        tokens.extend(quote! {
                            Self::#variant_name => {
                                #variant_fields_debug
                            }
                        });
                    }
                }

                tokens
            });

    quote! {
        match self {
            #variant_debug_arms
        }
    }
}

fn struct_fields_debug(type_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
    let type_name = &type_name.to_string();
    match fields {
        Fields::Named(fields_named) => {
            // Generates:
            //
            // let mut debug_struct = f.debug_struct(#type_name);
            // debug_struct.field("field_0", &self.field_0);
            // debug_struct.field("field_1", &self.field_1);
            // debug_struct.finish()

            let tokens = quote! {
                let mut debug_struct = f.debug_struct(#type_name);
            };

            let mut tokens = fields_named.named.iter().fold(tokens, |mut tokens, field| {
                if let Some(field_name) = field.ident.as_ref() {
                    let field_name_str = &field_name.to_string();
                    tokens.extend(quote! {
                        debug_struct.field(#field_name_str, &self.#field_name);
                    });
                }
                tokens
            });

            tokens.extend(quote!(debug_struct.finish()));

            tokens
        }
        Fields::Unnamed(fields_unnamed) => {
            // Generates:
            //
            // let mut debug_tuple = f.debug_tuple(#type_name);
            // debug_tuple.field(&self.0);
            // debug_tuple.field(&self.1);
            // debug_tuple.finish()

            let tokens = quote! {
                let mut debug_tuple = f.debug_tuple(#type_name);
            };

            let mut tokens =
                (0..fields_unnamed.unnamed.len()).fold(tokens, |mut tokens, field_index| {
                    // Need to convert this to a `LitInt`,
                    // because `quote` outputs the index as `0usize` instead of `0`
                    let field_index = LitInt::new(&format!("{field_index}"), Span::call_site());
                    tokens.extend(quote! {
                        debug_tuple.field(&self.#field_index);
                    });
                    tokens
                });

            tokens.extend(quote!(debug_tuple.finish()));

            tokens
        }
        Fields::Unit => quote!(f.debug_struct(#type_name).finish()),
    }
}

fn variant_fields_debug(variant_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
    let variant_name = &variant_name.to_string();
    match fields {
        Fields::Named(fields_named) => {
            // Generates:
            //
            // let mut debug_struct = f.debug_struct(#variant_name);
            // debug_struct.field("field_0", &field_0);
            // debug_struct.field("field_1", &field_1);
            // debug_struct.finish()

            let tokens = quote! {
                let mut debug_struct = f.debug_struct(#variant_name);
            };

            let mut tokens = fields_named.named.iter().fold(tokens, |mut tokens, field| {
                if let Some(field_name) = field.ident.as_ref() {
                    let field_name_str = &field_name.to_string();
                    tokens.extend(quote! {
                        debug_struct.field(#field_name_str, &#field_name);
                    });
                }
                tokens
            });

            tokens.extend(quote!(debug_struct.finish()));

            tokens
        }
        Fields::Unnamed(fields_unnamed) => {
            // Generates:
            //
            // let mut debug_tuple = f.debug_tuple(#variant_name);
            // debug_tuple.field(&_0);
            // debug_tuple.field(&_1);
            // debug_tuple.finish()

            let tokens = quote! {
                let mut debug_tuple = f.debug_tuple(#variant_name);
            };

            let mut tokens = (0..fields_unnamed.unnamed.len())
                .map(tuple_ident_from_field_index)
                .fold(tokens, |mut tokens, field_index| {
                    tokens.extend(quote! {
                        debug_tuple.field(&#field_index);
                    });
                    tokens
                });

            tokens.extend(quote!(debug_tuple.finish()));

            tokens
        }
        Fields::Unit => quote!(f.debug_struct(#variant_name).finish()),
    }
}

fn struct_fields_clone(type_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => {
            // Generates:
            //
            // #type_name {
            //     field_1: self.field_1.clone(),
            //     field_2: self.field_2.clone(),
            //     marker: PhantomData,
            // }

            let fields_clone = fields_named.named.iter().fold(
                proc_macro2::TokenStream::new(),
                |mut tokens, field| {
                    if let Some(field_name) = field.ident.as_ref() {
                        if is_phantom_data(&field.ty) {
                            tokens.extend(quote! {
                                #field_name: std::marker::PhantomData,
                            });
                        } else {
                            tokens.extend(quote! {
                                #field_name: self.#field_name.clone(),
                            });
                        }
                    }
                    tokens
                },
            );

            quote! {
                #type_name {
                    #fields_clone
                }
            }
        }
        Fields::Unnamed(fields_unnamed) => {
            // Generates:
            //
            // #type_name(self.0.clone(), self.1.clone(), PhantomData)
            let fields_clone = fields_unnamed.unnamed.iter().enumerate().fold(
                proc_macro2::TokenStream::new(),
                |mut tokens, (field_index, field)| {
                    // Need to convert this to a `LitInt`,
                    // because `quote` outputs the index as `0usize` instead of `0`
                    let field_index = LitInt::new(&format!("{field_index}"), Span::call_site());

                    if is_phantom_data(&field.ty) {
                        tokens.extend(quote!(std::marker::PhantomData,));
                    } else {
                        tokens.extend(quote!(self.#field_index.clone(),));
                    }

                    tokens
                },
            );

            quote! {
                #type_name(#fields_clone)
            }
        }
        Fields::Unit => quote!(#type_name),
    }
}

fn variants_clone(variants: &Punctuated<Variant, Token![,]>) -> proc_macro2::TokenStream {
    // Generates:
    //
    // match self {
    //     Self::Variant1 => Self::Variant1,
    //     Self::Variant2(_0, _1, PhantomData) => {
    //         Self::Variant2(_0.clone(), _1.clone(), PhantomData)
    //     }
    //     Self::Variant3 { field_1, field_2, marker: PhantomData } => {
    //         Self::Variant3 {
    //             field_1: field_1.clone(),
    //             field_2: field_2.clone(),
    //             marker: PhantomData,
    //         }
    //     }
    // }

    let variant_clone_arms =
        variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |mut tokens, variant| {
                let variant_name = &variant.ident;
                let variant_fields = &variant
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(field_index, field)| {
                        if is_phantom_data(&field.ty) {
                            if let Some(field_ident) = field.ident.as_ref() {
                                quote!(#field_ident: ::std::marker::PhantomData)
                            } else {
                                quote!(::std::marker::PhantomData)
                            }
                        } else if let Some(field_ident) = field.ident.as_ref() {
                            quote!(#field_ident)
                        } else {
                            let field_ident = tuple_ident_from_field_index(field_index);
                            quote!(#field_ident)
                        }
                    })
                    .collect::<Vec<proc_macro2::TokenStream>>();
                let variant_fields_clone = variant_fields_clone(&variant.ident, &variant.fields);

                match &variant.fields {
                    Fields::Named(_fields_named) => {
                        tokens.extend(quote! {
                            Self::#variant_name { #(#variant_fields),* } => {
                                #variant_fields_clone
                            }
                        });
                    }
                    Fields::Unnamed(_) => {
                        tokens.extend(quote! {
                            Self::#variant_name(#(#variant_fields),*) => {
                                #variant_fields_clone
                            }
                        });
                    }
                    Fields::Unit => {
                        tokens.extend(quote! {
                            Self::#variant_name => {
                                #variant_fields_clone
                            }
                        });
                    }
                }

                tokens
            });

    quote! {
        match self {
            #variant_clone_arms
        }
    }
}

fn variant_fields_clone(variant_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => {
            // Generates:
            //
            // Self {
            //     field_1: field_1.clone(),
            //     field_2: field_2.clone(),
            //     marker: PhantomData,
            // }

            let fields_clone = fields_named.named.iter().fold(
                proc_macro2::TokenStream::new(),
                |mut tokens, field| {
                    if let Some(field_name) = field.ident.as_ref() {
                        if is_phantom_data(&field.ty) {
                            tokens.extend(quote! {
                                #field_name: std::marker::PhantomData,
                            });
                        } else {
                            tokens.extend(quote! {
                                #field_name: #field_name.clone(),
                            });
                        }
                    }
                    tokens
                },
            );

            quote! {
                Self::#variant_name {
                    #fields_clone
                }
            }
        }
        Fields::Unnamed(fields_unnamed) => {
            // Generates:
            //
            // Self(_0.clone(), _1.clone(), PhantomData)
            let fields_clone = fields_unnamed
                .unnamed
                .iter()
                .enumerate()
                .map(|(field_index, field)| (tuple_ident_from_field_index(field_index), field))
                .fold(
                    proc_macro2::TokenStream::new(),
                    |mut tokens, (field_index, field)| {
                        if is_phantom_data(&field.ty) {
                            tokens.extend(quote!(std::marker::PhantomData,));
                        } else {
                            tokens.extend(quote!(#field_index.clone(),));
                        }

                        tokens
                    },
                );

            quote! {
                Self::#variant_name(#fields_clone)
            }
        }
        Fields::Unit => quote!(Self::#variant_name),
    }
}

fn tuple_ident_from_field_index(field_index: usize) -> Ident {
    Ident::new(&format!("_{field_index}"), Span::call_site())
}
