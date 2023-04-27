use proc_macro2::Span;
use syn::{Ident, Type, TypePath};

/// Returns whether the given field is a `PhantomData`.
pub fn is_phantom_data(field_ty: &Type) -> bool {
    matches!(&field_ty, Type::Path(TypePath { path, .. })
        if matches!(path.segments.last(), Some(segment) if segment.ident == "PhantomData"))
}

/// Returns tuple idents as `_n` where `n` is the index of the field.
pub fn tuple_ident_from_field_index(field_index: usize) -> Ident {
    Ident::new(&format!("_{field_index}"), Span::call_site())
}
