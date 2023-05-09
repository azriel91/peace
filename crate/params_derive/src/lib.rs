#![recursion_limit = "256"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

use syn::{
    DeriveInput, Generics, Ident, ImplGenerics, Path, TypeGenerics, WhereClause, WherePredicate,
};
use type_gen_external::External;

use crate::{
    external_type::ExternalType,
    fields_map::{fields_to_optional, fields_to_value_spec},
    impl_default::impl_default,
    impl_field_wise_spec_rt_for_field_wise::impl_field_wise_spec_rt_for_field_wise,
    impl_field_wise_spec_rt_for_field_wise_external::impl_field_wise_spec_rt_for_field_wise_external,
    impl_from_params_for_params_field_wise::impl_from_params_for_params_field_wise,
    impl_from_params_for_params_partial::impl_from_params_for_params_partial,
    impl_try_from_params_partial_for_params::impl_try_from_params_partial_for_params,
    impl_value_spec_rt_for_field_wise::impl_value_spec_rt_for_field_wise,
    type_gen::TypeGen,
    type_gen_external::type_gen_external,
    util::is_external,
};

mod external_type;
mod fields_map;
mod impl_default;
mod impl_field_wise_spec_rt_for_field_wise;
mod impl_field_wise_spec_rt_for_field_wise_external;
mod impl_from_params_for_params_field_wise;
mod impl_from_params_for_params_partial;
mod impl_try_from_params_partial_for_params;
mod impl_value_spec_rt_for_field_wise;
mod type_gen;
mod type_gen_external;
mod util;

/// Used to `#[derive]` the `Params` and `Value` traits.
///
/// For regular usage, use `#[derive(Params)]`
///
/// For peace crates, also add the `#[peace_internal]` attribute, which
/// references the `peace_params` crate instead of the `peace::params`
/// re-export.
///
/// For types derived from `struct` `Param`s -- `Spec`, `Partial` -- we also:
///
/// * Generate getters and mut getters for non-`pub`, non-`PhantomData` fields.
/// * Generate a constructor if not all fields are `pub`.
///
/// Maybe we should also generate a `SpecBuilder` -- see commit `10f63611` which
/// removed builder generation.
///
/// # Attributes:
///
/// * `peace_internal`: Type level attribute indicating the `peace_params` crate
///   is referenced by `peace_params` instead of the default `peace::params`.
///
/// * `crate_internal`: Type level attribute indicating the `peace_params` crate
///   is referenced by `crate` instead of the default `peace::params`.
///
/// * `params(external)`: Used as either of:
///
///     - Type level attribute indicating fields are not known, and so
///       `ParamsPartial` will instead hold an `Option<Params>` field.
///     - Field level attribute indicating a third party type is in use, and a
///       newtype wrapper should be generated to implement the
///       `peace_params::Value` trait.
///
/// * `default`: Enum variant attribute to indicate which variant to instantiate
///   for `ParamsPartial::default()`.
#[proc_macro_derive(Params, attributes(peace_internal, crate_internal, params, default))]
pub fn params_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input)
        .expect("`Params` derive: Failed to parse item as struct, enum, or union.");

    let gen = impl_params(&ast);

    gen.into()
}

/// Used to `#[derive]` the `Value` trait.
///
/// For regular usage, use `#[derive(Value)]`
///
/// For peace crates, also add the `#[peace_internal]` attribute, which
/// references the `peace_params` crate instead of the `peace::params`
/// re-export.
///
/// # Attributes:
///
/// * `peace_internal`: Type level attribute indicating the `peace_params` crate
///   is referenced by `peace_params` instead of the default `peace::params`.
///
/// * `crate_internal`: Type level attribute indicating the `peace_params` crate
///   is referenced by `crate` instead of the default `peace::params`.
///
/// * `params(external)`: Type level attribute indicating fields are not known,
///   and so `ParamsPartial` will instead hold an `Option<Params>` field.
///
/// * `default`: Enum variant attribute to indicate which variant to instantiate
///   for `ParamsPartial::default()`.
#[proc_macro_derive(Value, attributes(peace_internal, crate_internal, params, default))]
pub fn value_derive(input: TokenStream) -> TokenStream {
    let ast =
        syn::parse(input).expect("`Value` derive: Failed to parse item as struct, enum, or union.");

    let gen = impl_value(&ast);

    gen.into()
}

#[proc_macro]
pub fn value_impl(input: TokenStream) -> TokenStream {
    let ast =
        syn::parse(input).expect("`Value` impl: Failed to parse item as struct, enum, or union.");

    let gen = impl_value(&ast);

    gen.into()
}

fn impl_params(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let params_name = &ast.ident;

    let (peace_params_path, peace_resources_path): (Path, Path) = ast
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.path().is_ident("peace_internal") {
                Some((parse_quote!(peace_params), parse_quote!(peace_resources)))
            } else if attr.path().is_ident("crate_internal") {
                Some((parse_quote!(crate), parse_quote!(peace_resources)))
            } else {
                None
            }
        })
        .unwrap_or_else(|| (parse_quote!(peace::params), parse_quote!(peace::resources)));

    let mut generics = ast.generics.clone();
    type_parameters_constrain(&mut generics);
    let generics_split = generics.split_for_impl();

    // MyParams -> MyParamsPartial
    let t_partial_name = {
        let mut t_partial_name = ast.ident.to_string();
        t_partial_name.push_str("Partial");
        Ident::new(&t_partial_name, ast.ident.span())
    };

    // MyParams -> MyParamsFieldWise
    let t_field_wise_name = {
        let mut t_field_wise_name = ast.ident.to_string();
        t_field_wise_name.push_str("FieldWise");
        Ident::new(&t_field_wise_name, ast.ident.span())
    };

    let (t_partial, t_field_wise) = if is_external(&ast.attrs) {
        let t_partial = t_partial_external(ast, &generics_split, params_name, &t_partial_name);
        let t_field_wise = t_field_wise_external(
            ast,
            &generics_split,
            &peace_params_path,
            &peace_resources_path,
            params_name,
            &t_field_wise_name,
            &t_partial_name,
        );

        (t_partial, t_field_wise)
    } else {
        let t_partial = t_partial(ast, &generics_split, params_name, &t_partial_name);
        let t_field_wise = t_field_wise(
            ast,
            &generics_split,
            &peace_params_path,
            &peace_resources_path,
            params_name,
            &t_field_wise_name,
            &t_partial_name,
        );

        (t_partial, t_field_wise)
    };
    let (impl_generics, ty_generics, where_clause) = &generics_split;

    let external_wrapper_types = ExternalType::external_wrapper_types(ast, &peace_params_path);

    quote! {
        impl #impl_generics #peace_params_path::Params
        for #params_name #ty_generics
        #where_clause
        {
            type Spec = #peace_params_path::ParamsSpec<#params_name #ty_generics>;
            type Partial = #t_partial_name #ty_generics;
            type FieldWiseSpec = #t_field_wise_name #ty_generics;
        }

        impl #impl_generics #peace_params_path::Value
        for #params_name #ty_generics
        #where_clause
        {
            type Spec = #peace_params_path::ValueSpec<#params_name #ty_generics>;
            type Partial = #t_partial_name #ty_generics;
        }

        #t_field_wise

        #t_partial

        #external_wrapper_types
    }
}

fn impl_value(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let value_name = &ast.ident;

    let (peace_params_path, peace_resources_path): (Path, Path) = ast
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.path().is_ident("peace_internal") {
                Some((parse_quote!(peace_params), parse_quote!(peace_resources)))
            } else if attr.path().is_ident("crate_internal") {
                Some((parse_quote!(crate), parse_quote!(peace_resources)))
            } else {
                None
            }
        })
        .unwrap_or_else(|| (parse_quote!(peace::params), parse_quote!(peace::resources)));

    let mut generics = ast.generics.clone();
    type_parameters_constrain(&mut generics);
    let generics_split = generics.split_for_impl();

    // MyValue -> MyValuePartial
    let t_partial_name = {
        let mut t_partial_name = ast.ident.to_string();
        t_partial_name.push_str("Partial");
        Ident::new(&t_partial_name, ast.ident.span())
    };

    // MyValue -> MyValueFieldWise
    let t_field_wise_name = {
        let mut t_field_wise_name = ast.ident.to_string();
        t_field_wise_name.push_str("FieldWise");
        Ident::new(&t_field_wise_name, ast.ident.span())
    };

    let (t_partial, t_field_wise) = if is_external(&ast.attrs) {
        let t_partial = t_partial_external(ast, &generics_split, value_name, &t_partial_name);
        let t_field_wise = t_field_wise_external(
            ast,
            &generics_split,
            &peace_params_path,
            &peace_resources_path,
            value_name,
            &t_field_wise_name,
            &t_partial_name,
        );

        (t_partial, t_field_wise)
    } else {
        let t_partial = t_partial(ast, &generics_split, value_name, &t_partial_name);
        let t_field_wise = t_field_wise(
            ast,
            &generics_split,
            &peace_params_path,
            &peace_resources_path,
            value_name,
            &t_field_wise_name,
            &t_partial_name,
        );

        (t_partial, t_field_wise)
    };
    let (impl_generics, ty_generics, where_clause) = &generics_split;

    let external_wrapper_types = ExternalType::external_wrapper_types(ast, &peace_params_path);

    quote! {
        impl #impl_generics #peace_params_path::Value
        for #value_name #ty_generics
        #where_clause
        {
            type Spec = #peace_params_path::ValueSpec<#value_name #ty_generics>;
            type Partial = #t_partial_name #ty_generics;
        }

        #t_field_wise

        #t_partial

        #external_wrapper_types
    }
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
fn t_partial(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    params_name: &Ident,
    t_partial_name: &Ident,
) -> proc_macro2::TokenStream {
    let mut t_partial = TypeGen::gen_from_value_type(
        ast,
        generics_split,
        t_partial_name,
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
    );

    t_partial.extend(impl_try_from_params_partial_for_params(
        ast,
        generics_split,
        params_name,
        t_partial_name,
    ));

    t_partial.extend(impl_default(ast, generics_split, t_partial_name));

    t_partial.extend(impl_from_params_for_params_partial(
        ast,
        generics_split,
        params_name,
        t_partial_name,
    ));

    t_partial
}

/// Generates something like the following:
///
/// ```rust,ignore
/// #[derive(Clone, Debug, PartialEq, Eq)]
/// struct MyParamsPartial(Option<MyParams>)
/// ```
fn t_partial_external(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    params_name: &Ident,
    t_partial_name: &Ident,
) -> proc_macro2::TokenStream {
    type_gen_external(
        ast,
        generics_split,
        External::Direct {
            value_name: params_name,
        },
        t_partial_name,
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
/// struct MyParamsFieldWise {
///     src: peace_params::ValueSpec<PathBuf>,
///     dest_ip: peace_params::ValueSpec<IpAddr>,
///     dest_path: peace_params::ValueSpec<PathBuf>,
/// }
/// ```
fn t_field_wise(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    peace_params_path: &Path,
    peace_resources_path: &Path,
    params_name: &Ident,
    t_field_wise_name: &Ident,
    t_partial_name: &Ident,
) -> proc_macro2::TokenStream {
    let mut t_field_wise = TypeGen::gen_from_value_type(
        ast,
        generics_split,
        t_field_wise_name,
        |fields| fields_to_value_spec(fields, peace_params_path),
        &[
            parse_quote! {
                #[doc="Specification of how to look up values for an item spec's parameters."]
            },
            // `Clone` and `Debug` are implemented manually, so that type parameters do not receive
            // the `Clone` and `Debug` bounds.
            parse_quote!(#[derive(serde::Serialize, serde::Deserialize)]),
        ],
    );

    t_field_wise.extend(impl_field_wise_spec_rt_for_field_wise(
        ast,
        generics_split,
        peace_params_path,
        peace_resources_path,
        params_name,
        t_field_wise_name,
        t_partial_name,
    ));

    t_field_wise.extend(impl_value_spec_rt_for_field_wise(
        ast,
        generics_split,
        peace_params_path,
        peace_resources_path,
        params_name,
        t_field_wise_name,
    ));

    t_field_wise.extend(impl_from_params_for_params_field_wise(
        ast,
        generics_split,
        peace_params_path,
        params_name,
        t_field_wise_name,
    ));

    t_field_wise
}

/// Generates something like the following:
///
/// ```rust,ignore
/// struct MyParamsFieldWise(peace_params::ValueSpec<MyParams>)
/// ```
fn t_field_wise_external(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    peace_params_path: &Path,
    peace_resources_path: &Path,
    params_name: &Ident,
    t_field_wise_name: &Ident,
    t_partial_name: &Ident,
) -> proc_macro2::TokenStream {
    let mut t_field_wise = type_gen_external(
        ast,
        generics_split,
        External::Direct {
            value_name: params_name,
        },
        t_field_wise_name,
        &[
            parse_quote! {
                #[doc="Specification of how to look up values for an item spec's parameters."]
            },
            parse_quote!(#[derive(serde::Serialize, serde::Deserialize)]),
        ],
    );

    t_field_wise.extend(impl_field_wise_spec_rt_for_field_wise_external(
        generics_split,
        peace_params_path,
        peace_resources_path,
        params_name,
        t_field_wise_name,
        t_partial_name,
    ));

    // TODO: Do we need this?
    // t_field_wise.extend(impl_value_spec_rt_for_field_wise_external(
    //     ast,
    //     generics_split,
    //     peace_params_path,
    //     peace_resources_path,
    //     params_name,
    //     t_field_wise_name,
    // ));

    t_field_wise
}
