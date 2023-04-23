#![recursion_limit = "256"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use syn::{
    Attribute, DeriveInput, Fields, Ident, ImplGenerics, Path, Type, TypeGenerics, WhereClause,
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

    let generics_split = ast.generics.split_for_impl();

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

    let params_partial = params_partial(&ast, &generics_split, &params_partial_name);
    let params_spec = params_spec(&ast, &generics_split, &params_spec_name, &peace_params_path);
    let params_spec_builder = params_spec_builder(
        &ast,
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
            parse_quote!(#[derive(Clone, Debug, PartialEq, Eq)]),
        ],
    )
}

/// Generates something like the following:
///
/// ```rust,ignore
/// #[derive(Clone, Debug /*, serde::Serialize, serde::Deserialize */)]
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
            // TODO: Figure out how to encode `ValueSpec::FromMap` into a serializable form.
            //
            // Can't derive any of the following because `ValueSpec` contains `Box<dyn Fn..>`
            //
            // parse_quote!(#[derive(Clone, serde::Serialize, serde::Deserialize)]),
            parse_quote!(#[derive(Debug)]),
        ],
    )
}

/// Generates something like the following:
///
/// ```rust,ignore
/// #[derive(Debug)]
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
        &[
            parse_quote! {
                #[doc="\
                    Builder for specification of how to look up the values for an item spec's \n\
                    parameters.\n\
                "]
            },
            parse_quote!(#[derive(Debug)]),
        ],
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
    let (_impl_generics, ty_generics, _where_clause) = generics_split;

    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let mut fields = data_struct.fields.clone();
            fields_map(&mut fields);

            quote! {
                #(#attrs)*
                struct #type_name #ty_generics #fields
            }
        }
        syn::Data::Enum(data_enum) => {
            let mut variants = data_enum.variants.clone();
            variants.iter_mut().for_each(|variant| {
                fields_map(&mut variant.fields);
            });

            quote! {
                #(#attrs)*
                enum #type_name #ty_generics #variants
            }
        }
        syn::Data::Union(data_union) => {
            let mut fields = Fields::from(data_union.fields.clone());
            fields_map(&mut fields);

            quote! {
                #(#attrs)*
                union #type_name #ty_generics #fields
            }
        }
    }
}

fn fields_to_optional(fields: &mut Fields) {
    field_map(fields, |field_ty| parse_quote!(Option<#field_ty>))
}

fn fields_to_value_spec(fields: &mut Fields, peace_params_path: &Path) {
    field_map(
        fields,
        |field_ty| parse_quote!(#peace_params_path::ValueSpec<#field_ty>),
    )
}

fn fields_to_optional_value_spec(fields: &mut Fields, peace_params_path: &Path) {
    field_map(
        fields,
        |field_ty| parse_quote!(Option<#peace_params_path::ValueSpec<#field_ty>>),
    )
}

fn field_map<F>(fields: &mut Fields, f: F)
where
    F: Fn(&Type) -> Type,
{
    match fields {
        Fields::Named(fields_named) => {
            fields_named.named.iter_mut().for_each(|field| {
                let field_ty = &mut field.ty;
                field.ty = f(field_ty);
            });
        }
        Fields::Unnamed(fields_unnamed) => {
            fields_unnamed.unnamed.iter_mut().for_each(|field| {
                let field_ty = &field.ty;
                field.ty = f(field_ty);
            });
        }
        Fields::Unit => {}
    }
}
