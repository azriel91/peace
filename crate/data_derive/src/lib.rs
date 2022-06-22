#![recursion_limit = "256"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Literal;
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, DataStruct, DeriveInput, Field, Fields,
    FieldsNamed, FieldsUnnamed, Ident, Lifetime, Type, WhereClause, WherePredicate,
};

/// Used to `#[derive]` the `Data` trait.
///
/// For regular usage, use `#[derive(Data)]`
///
/// For peace crates, also add the `#[peace_internal]` attribute, which
/// references the `peace_data` crate instead of the `peace::data` re-export.
#[proc_macro_derive(Data, attributes(peace_internal))]
pub fn data_access(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Data derive: Code failed to be parsed.");

    let gen = impl_data_access(&ast);

    gen.into()
}

fn impl_data_access(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let module_pfx = ast
        .attrs
        .iter()
        .find(peace_internal)
        .map(|_| quote!(peace_data))
        .unwrap_or_else(|| quote!(peace::data));

    let mut generics = ast.generics.clone();

    let (tys, field_names, borrow_return) = gen_from_body(&ast.data, name);
    let tys = &tys;
    // Assumes that the first lifetime is the borrow lifetime
    let def_borrow_lt = ast
        .generics
        .lifetimes()
        .next()
        .expect("Struct must have at least one lifetime");
    let impl_borrow_lt = &def_borrow_lt.lifetime;

    {
        let where_clause = generics.make_where_clause();
        constrain_data_access_types(where_clause, impl_borrow_lt, tys);
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics #module_pfx::DataAccess
            for #name #ty_generics
            #where_clause
        {
            fn borrows() -> #module_pfx::TypeIds {
                let mut r = #module_pfx::TypeIds::new();

                #( {
                        let mut borrows = <#tys as #module_pfx::DataAccess>::borrows();
                        r.append(&mut borrows);
                    } )*

                r
            }

            fn borrow_muts() -> #module_pfx::TypeIds {
                let mut r = #module_pfx::TypeIds::new();

                #( {
                        let mut borrow_muts = <#tys as #module_pfx::DataAccess>::borrow_muts();
                        r.append(&mut borrow_muts);
                    } )*

                r
            }
        }

        impl #impl_generics #module_pfx::DataAccessDyn
            for #name #ty_generics
            #where_clause
        {
            fn borrows(&self) -> #module_pfx::TypeIds {
                let mut r = #module_pfx::TypeIds::new();

                #( {
                        let mut borrows = <#tys as #module_pfx::DataAccessDyn>::borrows(&self.#field_names);
                        r.append(&mut borrows);
                    } )*

                r
            }

            fn borrow_muts(&self) -> #module_pfx::TypeIds {
                let mut r = #module_pfx::TypeIds::new();

                #( {
                        let mut borrow_muts = <#tys as #module_pfx::DataAccessDyn>::borrow_muts(&self.#field_names);
                        r.append(&mut borrow_muts);
                    } )*

                r
            }
        }

        impl #impl_generics #module_pfx::Data< #impl_borrow_lt >
            for #name #ty_generics
            #where_clause
        {
            fn borrow(resources: & #impl_borrow_lt #module_pfx::Resources) -> Self {
                #borrow_return
            }
        }
    }
}

fn peace_internal(attr: &&Attribute) -> bool {
    attr.path.is_ident("peace_internal")
}

fn collect_field_types(fields: &Punctuated<Field, Comma>) -> Vec<Type> {
    fields.iter().map(|x| x.ty.clone()).collect()
}

fn gen_identifiers(fields: &Punctuated<Field, Comma>) -> Vec<Ident> {
    fields
        .iter()
        .map(|x| x.ident.clone().expect("Data derive: Failed to clone ident"))
        .collect()
}

/// Adds a `Data<'lt>` bound on each of the system data types.
fn constrain_data_access_types(clause: &mut WhereClause, borrow_lt: &Lifetime, tys: &[Type]) {
    for ty in tys.iter() {
        let where_predicate: WherePredicate = parse_quote!(#ty : Data< #borrow_lt >);
        clause.predicates.push(where_predicate);
    }
}

fn gen_from_body(
    ast: &syn::Data,
    name: &Ident,
) -> (
    Vec<Type>,
    Vec<proc_macro2::TokenStream>,
    proc_macro2::TokenStream,
) {
    enum DataType {
        Struct,
        Tuple,
    }

    let (body, fields) = match *ast {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named: ref x, .. }),
            ..
        }) => (DataType::Struct, x),

        syn::Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { unnamed: ref x, .. }),
            ..
        }) => (DataType::Tuple, x),

        _ => panic!("Enums are not supported"),
    };

    let tys = collect_field_types(fields);

    let (field_names, borrow_return) = match body {
        DataType::Struct => {
            let identifiers = gen_identifiers(fields);

            let field_names = identifiers
                .iter()
                .map(|ident| quote!(#ident))
                .collect::<Vec<_>>();

            let borrow_return = quote! {
                #name {
                    #( #identifiers: Data::borrow(resources) ),*
                }
            };

            (field_names, borrow_return)
        }
        DataType::Tuple => {
            let count = tys.len();
            let field_names = (0..count)
                .map(Literal::usize_unsuffixed)
                .map(|n| quote!(#n))
                .collect::<Vec<_>>();

            let borrow = vec![quote! { Data::borrow(resources) }; count];
            let borrow_return = quote! {
                #name ( #( #borrow ),* )
            };

            (field_names, borrow_return)
        }
    };

    (tys, field_names, borrow_return)
}
