use syn::{Attribute, DeriveInput, Ident, ImplGenerics, Type, TypeGenerics, WhereClause};

use crate::util::is_serde_bound_attr;

/// Generates a type based off an external `Value` type.
///
/// A `MyValue` wrapped type will produce:
///
/// ```rust,ignore
/// pub struct Generated(Option<MyValue>);
/// ```
///
/// This is meant to be used for external `ValuePartial` and maybe
/// `ValueFieldWise`.
///
/// # Parameters
///
/// * `ast`: The `Value` type.
/// * `generics_split`: Generics of the `Value` type.
/// * `value_name`: Name of the params / value type.
/// * `type_name`: Name of the type to generate.
/// * `attrs`: Attributes to attach to the generated type.
pub fn type_gen_external(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    value_ty: &Type,
    type_name: &Ident,
    attrs: &[Attribute],
) -> proc_macro2::TokenStream {
    let mut serde_bound_attrs = ast
        .attrs
        .iter()
        .filter(|attr| is_serde_bound_attr(attr))
        .collect::<Vec<&Attribute>>();
    let serde_bound_empty = parse_quote!(#[serde(bound = "")]);
    if serde_bound_attrs.is_empty() {
        serde_bound_attrs.push(&serde_bound_empty);
    }

    let (impl_generics, ty_generics, where_clause) = generics_split;

    let constructor_doc = format!("Returns a new `{type_name}`.");

    let mut generics_for_ref = ast.generics.clone();
    generics_for_ref.params.insert(0, parse_quote!('generated));
    let (impl_generics_for_ref, _type_generics, _where_clause) = generics_for_ref.split_for_impl();

    let mut tokens = quote! {
        #(#attrs)*
        #(#serde_bound_attrs)*
        pub struct #type_name #ty_generics(Option<#value_ty>)
        #where_clause;

        impl #impl_generics #type_name #ty_generics
        #where_clause
        {
            #[doc = #constructor_doc]
            pub fn new(value: Option<#value_ty>) -> Self {
                Self(value)
            }

            pub fn into_inner(self) -> Option<#value_ty> {
                self.0
            }
        }

        impl #impl_generics ::std::clone::Clone
        for #type_name #ty_generics
        #where_clause
        {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl #impl_generics ::std::fmt::Debug
        for #type_name #ty_generics
        #where_clause
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.debug_tuple(stringify!(#type_name))
                    .field(&self.0)
                    .finish()
            }
        }

        impl #impl_generics ::std::default::Default
        for #type_name #ty_generics
        #where_clause
        {
            fn default() -> Self {
                Self(None)
            }
        }

        impl #impl_generics ::std::ops::Deref
        for #type_name #ty_generics
        #where_clause
        {
            type Target = Option<#value_ty>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl #impl_generics ::std::ops::DerefMut
        for #type_name #ty_generics
        #where_clause
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        // impl From<Value> for ValuePartial
        impl #impl_generics ::std::convert::From<#value_ty>
        for #type_name #ty_generics
        #where_clause
        {
            fn from(value: #value_ty) -> Self {
                Self::new(Some(value))
            }
        }
    };

    tokens.extend(quote! {
        // impl TryFrom<ValuePartial> for Value
        impl #impl_generics ::std::convert::TryFrom<#type_name #ty_generics>
        for #value_ty
        #where_clause
        {
            type Error = #type_name #ty_generics;

            fn try_from(generated: #type_name #ty_generics) -> Result<Self, Self::Error> {
                if let Some(value) = generated.0 {
                    Ok(value)
                } else {
                    Err(generated)
                }
            }
        }

        impl #impl_generics_for_ref ::std::convert::TryFrom<&'generated #type_name #ty_generics>
        for #value_ty
        #where_clause
        {
            type Error = &'generated #type_name #ty_generics;

            fn try_from(generated: &'generated #type_name #ty_generics) -> Result<Self, Self::Error> {
                if let Some(value) = generated.0.as_ref() {
                    Ok(value.clone())
                } else {
                    Err(generated)
                }
            }
        }
    });

    tokens
}
