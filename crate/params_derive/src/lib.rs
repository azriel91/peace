#![recursion_limit = "256"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use quote::ToTokens;

use syn::{
    Data, DeriveInput, GenericParam, Ident, ImplGenerics, Path, Type, TypeGenerics, WhereClause,
    WherePredicate,
};

use crate::{
    field_wise_enum_builder_ctx::FieldWiseEnumBuilderCtx,
    fields_map::{fields_to_optional, fields_to_value_spec},
    impl_any_spec_rt_for_field_wise::impl_any_spec_rt_for_field_wise,
    impl_default::impl_default,
    impl_field_wise_builder::impl_field_wise_builder,
    impl_field_wise_spec_rt_for_field_wise::impl_field_wise_spec_rt_for_field_wise,
    impl_field_wise_spec_rt_for_field_wise_external::impl_field_wise_spec_rt_for_field_wise_external,
    impl_from_params_for_params_field_wise::impl_from_params_for_params_field_wise,
    impl_from_params_for_params_partial::impl_from_params_for_params_partial,
    impl_try_from_params_partial_for_params::impl_try_from_params_partial_for_params,
    impl_value_spec_rt_for_field_wise::impl_value_spec_rt_for_field_wise,
    type_gen::TypeGen,
    type_gen_external::type_gen_external,
    util::{is_fieldless_type, serde_bounds_for_type_params, ImplMode},
};

mod field_wise_enum_builder_ctx;
mod fields_map;
mod impl_any_spec_rt_for_field_wise;
mod impl_default;
mod impl_field_wise_builder;
mod impl_field_wise_spec_rt_for_field_wise;
mod impl_field_wise_spec_rt_for_field_wise_external;
mod impl_from_params_for_params_field_wise;
mod impl_from_params_for_params_partial;
mod impl_try_from_params_partial_for_params;
mod impl_value_spec_rt_for_field_wise;
mod spec_is_usable;
mod spec_merge;
mod type_gen;
mod type_gen_external;
mod util;

/// Used to `#[derive]` the `ValueSpec` and `ValueSpecRt` traits.
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
/// * `value_spec(fieldless)`: Used as either of:
///
///     - Type level attribute indicating fields are not known, and so
///       `ParamsPartial` will instead hold an `Option<Params>` field.
///     - Field level attribute indicating a third party type is in use, and a
///       newtype wrapper should be generated to implement the
///       `peace_params::Value` trait.
///
/// * `default`: Enum variant attribute to indicate which variant to instantiate
///   for `ParamsPartial::default()`.
#[proc_macro_derive(
    Params,
    attributes(peace_internal, crate_internal, value_spec, default, serde)
)]
pub fn value_spec(input: TokenStream) -> TokenStream {
    let mut ast = syn::parse(input)
        .expect("`Params` derive: Failed to parse item as struct, enum, or union.");

    let gen = impl_value(&mut ast, ImplMode::Fieldwise);

    gen.into()
}

/// Used to `#[derive]` the `ParamsSpecFieldless` and `ValueSpecRt` traits.
///
/// For regular usage, use `#[derive(ParamsFieldless)]`
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
/// * `value_spec(fieldless)`: Type level attribute indicating fields are not
///   known, and so `ParamsPartial` will instead hold an `Option<Value>` field.
///
/// * `default`: Enum variant attribute to indicate which variant to instantiate
///   for `ParamsPartial::default()`.
#[proc_macro_derive(
    ParamsFieldless,
    attributes(peace_internal, crate_internal, value_spec, default)
)]
pub fn value_spec_fieldless(input: TokenStream) -> TokenStream {
    let mut ast = syn::parse(input)
        .expect("`ParamsFieldless` derive: Failed to parse item as struct, enum, or union.");

    let gen = impl_value(&mut ast, ImplMode::Fieldless);

    gen.into()
}

#[proc_macro]
pub fn value_impl(input: TokenStream) -> TokenStream {
    let mut ast = syn::parse(input)
        .expect("`peace_params::value_impl`: Failed to parse item as struct, enum, or union.");

    let gen = impl_value(&mut ast, ImplMode::Fieldless);

    gen.into()
}

fn impl_value(ast: &mut DeriveInput, impl_mode: ImplMode) -> proc_macro2::TokenStream {
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

    type_parameters_constrain(ast);
    let value_name = &ast.ident;
    let generics = &ast.generics;
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

    // MyValue -> MyValueFieldWiseBuilder
    let t_field_wise_builder_name = {
        let mut t_field_wise_builder_name = ast.ident.to_string();
        t_field_wise_builder_name.push_str("FieldWiseBuilder");
        Ident::new(&t_field_wise_builder_name, ast.ident.span())
    };

    let field_wise_enum_builder_ctx = {
        // `EnumParams`' generics with `VariantSelection` inserted beforehand.
        let generics = {
            let mut generics = ast.generics.clone();
            generics.params.insert(0, parse_quote!(VariantSelection));
            generics
        };
        let variant_none = format_ident!("{}VariantNone", t_field_wise_builder_name);
        // Used for PhantomData type parameters, as well as `Default` impl type
        // parameters.
        let ty_generics_idents = ast.generics.params.iter().fold(
            proc_macro2::TokenStream::new(),
            |mut tokens, generic_param| {
                match generic_param {
                    GenericParam::Lifetime(_) => {
                        panic!("Lifetime generics are not supported in Params derive.")
                    }
                    GenericParam::Type(type_param) => {
                        tokens.extend(type_param.ident.to_token_stream())
                    }
                    GenericParam::Const(const_param) => {
                        tokens.extend(const_param.ident.to_token_stream())
                    }
                }
                tokens
            },
        );
        let type_params_with_variant_none = quote!(<#variant_none, #ty_generics_idents>);

        FieldWiseEnumBuilderCtx {
            generics,
            variant_none,
            ty_generics_idents,
            type_params_with_variant_none,
        }
    };

    let builder_generics = if matches!(&ast.data, Data::Enum(_)) {
        #[allow(clippy::redundant_clone)] // False positive 2023-05-19
        field_wise_enum_builder_ctx
            .type_params_with_variant_none
            .clone()
    } else {
        let ty_generics = &generics_split.1;
        quote!(#ty_generics)
    };

    let (t_partial, t_field_wise, t_field_wise_builder) =
        if is_fieldless_type(ast) || impl_mode == ImplMode::Fieldless {
            let ty_generics = &generics_split.1;
            let value_ty: Type = parse_quote!(#value_name #ty_generics);
            let t_partial = t_partial_external(ast, &generics_split, &value_ty, &t_partial_name);
            let t_field_wise = t_field_wise_external(
                ast,
                &generics_split,
                &peace_params_path,
                &peace_resources_path,
                &value_ty,
                value_name,
                &t_field_wise_name,
                &t_partial_name,
            );

            (t_partial, t_field_wise, None)
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

            let t_field_wise_builder = impl_field_wise_builder(
                ast,
                &generics_split,
                &peace_params_path,
                &t_field_wise_name,
                &t_field_wise_builder_name,
                impl_mode,
                &field_wise_enum_builder_ctx,
            );

            (t_partial, t_field_wise, Some(t_field_wise_builder))
        };
    let (impl_generics, ty_generics, where_clause) = &generics_split;

    let mut impl_value_tokens = proc_macro2::TokenStream::new();
    match impl_mode {
        ImplMode::Fieldwise => impl_value_tokens.extend(quote! {
            impl #impl_generics #peace_params_path::Params
            for #value_name #ty_generics
            #where_clause
            {
                type Spec = #peace_params_path::ParamsSpec<#value_name #ty_generics>;
                type Partial = #t_partial_name #ty_generics;
                type FieldWiseSpec = #t_field_wise_name #ty_generics;
                type FieldWiseBuilder = #t_field_wise_builder_name #builder_generics;

                fn field_wise_spec() -> Self::FieldWiseBuilder {
                    Self::FieldWiseBuilder::default()
                }
            }
        }),
        ImplMode::Fieldless => {}
    }

    impl_value_tokens.extend(quote! {
        impl #impl_generics #peace_params_path::ParamsFieldless
        for #value_name #ty_generics
        #where_clause
        {
            type Spec = #peace_params_path::ParamsSpecFieldless<#value_name #ty_generics>;
            type Partial = #t_partial_name #ty_generics;
        }

        #t_partial

        #t_field_wise

        #t_field_wise_builder
    });

    impl_value_tokens
}

/// Adds trait bounds on each of the type parameters.
///
/// * `Send + Sync + 'static` is always added
/// * If a type has `#[serde(bound = "<Bound>")]`s, those bounds are used.
/// * If a type does not have `#[serde(bound = "")]`, `Serialize +
///   DeserializeOwned` is added for each type parameter.
fn type_parameters_constrain(ast: &mut DeriveInput) {
    let serde_bounds_for_type_params = serde_bounds_for_type_params(ast);
    let additional_bounds = ast
        .generics
        .params
        .iter()
        .filter_map(|generic_param| match generic_param {
            GenericParam::Lifetime(_) => None,
            GenericParam::Type(type_param) => Some(type_param),
            GenericParam::Const(_) => None,
        })
        .map(|type_param| parse_quote!(#type_param: Send + Sync + 'static))
        .collect::<Vec<WherePredicate>>();

    let generics = &mut ast.generics;
    let where_predicates = generics.make_where_clause();
    where_predicates.predicates.extend(additional_bounds);
    where_predicates
        .predicates
        .extend(serde_bounds_for_type_params);
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
                    Item parameters that may not necessarily have values.\n\
                    \n\
                    This is used for `try_state_current` and `try_state_desired` where values \n\
                    could be referenced from predecessors, which may not yet be available, such \n\
                    as the IP address of a server that is yet to be launched, or may change, \n\
                    such as the content hash of a file which is to be re-downloaded.\n\
                "]
            },
            parse_quote!(#[derive(PartialEq, Eq, serde::Serialize, serde::Deserialize)]),
        ],
        true,
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
    value_ty: &Type,
    t_partial_name: &Ident,
) -> proc_macro2::TokenStream {
    type_gen_external(
        ast,
        generics_split,
        value_ty,
        t_partial_name,
        &[
            parse_quote! {
                #[doc="\
                    Item parameters that may not necessarily have values.\n\
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
///     src: peace_params::ParamsSpecFieldless<PathBuf>,
///     dest_ip: peace_params::ParamsSpecFieldless<IpAddr>,
///     dest_path: peace_params::ParamsSpecFieldless<PathBuf>,
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
                #[doc="Specification of how to look up values for an item's parameters."]
            },
            // `Clone` and `Debug` are implemented manually, so that type parameters do not receive
            // the `Clone` and `Debug` bounds.
            parse_quote!(#[derive(serde::Serialize, serde::Deserialize)]),
        ],
        true,
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

    t_field_wise.extend(impl_any_spec_rt_for_field_wise(
        ast,
        generics_split,
        peace_params_path,
        t_field_wise_name,
    ));

    t_field_wise
}

/// Generates something like the following:
///
/// ```rust,ignore
/// struct MyParamsFieldWise(peace_params::ParamsSpecFieldless<MyParams>)
/// ```
// TODO: Refactor this crate to not pass redundant information, or use a context object.
#[allow(clippy::too_many_arguments)]
fn t_field_wise_external(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    peace_params_path: &Path,
    peace_resources_path: &Path,
    params_ty: &Type,
    params_name: &Ident,
    t_field_wise_name: &Ident,
    t_partial_name: &Ident,
) -> proc_macro2::TokenStream {
    let mut t_field_wise = type_gen_external(
        ast,
        generics_split,
        params_ty,
        t_field_wise_name,
        &[
            parse_quote! {
                #[doc="Specification of how to look up values for an item's parameters."]
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

    t_field_wise.extend(impl_any_spec_rt_for_field_wise(
        ast,
        generics_split,
        peace_params_path,
        t_field_wise_name,
    ));

    t_field_wise
}
