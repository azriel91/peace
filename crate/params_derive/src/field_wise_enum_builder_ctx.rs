use syn::{Generics, Ident};

#[derive(Clone, Debug)]
pub struct FieldWiseEnumBuilderCtx {
    /// The `EnumParams`' generics with `VariantSelection` inserted beforehand.
    pub generics: Generics,
    /// Type parameter struct name when no variant has been selected.
    pub variant_none: Ident,
    /// Type parameters without the angle brackets: `T1, T2`.
    pub ty_generics_idents: proc_macro2::TokenStream,
    /// `<#enum_params_variant_selection_none, #ty_generics_idents>`
    pub type_params_with_variant_none: proc_macro2::TokenStream,
}
