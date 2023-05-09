use syn::{Attribute, DeriveInput, Ident, ImplGenerics, TypeGenerics, WhereClause};

/// Generates a type based off an external `Params` type.
///
/// ```rust,ignore
/// pub struct Generated(Option<MyParams>);
/// ```
///
/// This is meant to be used for external `ParamsPartial` and maybe
/// `ParamsFieldWise`.
///
/// # Parameters
///
/// * `ast`: The `Params` type.
/// * `generics_split`: Generics of the `Params` type.
/// * `params_name`: Name of the params type.
/// * `type_name`: Name of the type to generate.
/// * `attrs`: Attributes to attach to the generated type.
pub fn type_gen_external(
    ast: &DeriveInput,
    generics_split: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    params_name: &Ident,
    type_name: &Ident,
    attrs: &[Attribute],
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics_split;

    let constructor_doc = format!("Returns a new `{type_name}`.");

    let mut generics_for_ref = ast.generics.clone();
    generics_for_ref.params.insert(0, parse_quote!('generated));
    let (impl_generics_for_ref, _type_generics, _where_clause) = generics_for_ref.split_for_impl();

    quote! {
        #(#attrs)*
        pub struct #type_name #ty_generics(Option<#params_name #ty_generics>);

        impl #impl_generics #type_name #ty_generics {
            #[doc = #constructor_doc]
            pub fn new(value: Option<#params_name #ty_generics>) -> Self {
                Self(value)
            }

            pub fn into_inner(self) -> Option<#params_name #ty_generics> {
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
            type Target = Option<#params_name #ty_generics>;

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

        // impl TryFrom<ParamsPartial> for Params
        impl #impl_generics ::std::convert::TryFrom<#type_name #ty_generics>
        for #params_name #ty_generics
        #where_clause
        {
            type Error = #type_name #ty_generics;

            fn try_from(generated: #type_name #ty_generics) -> Result<Self, Self::Error> {
                if let Some(params) = generated.0 {
                    Ok(params)
                } else {
                    Err(generated)
                }
            }
        }

        impl #impl_generics_for_ref ::std::convert::TryFrom<&'generated #type_name #ty_generics>
        for #params_name #ty_generics
        #where_clause
        {
            type Error = &'generated #type_name #ty_generics;

            fn try_from(generated: &'generated #type_name #ty_generics) -> Result<Self, Self::Error> {
                if let Some(params) = generated.0.as_ref() {
                    Ok(params.clone())
                } else {
                    Err(generated)
                }
            }
        }

        // impl From<Params> for ParamsPartial
        impl #impl_generics ::std::convert::From<#params_name #ty_generics>
        for #type_name #ty_generics
        #where_clause
        {
            fn from(params: #params_name #ty_generics) -> Self {
                Self::new(Some(params))
            }
        }
    }
}
