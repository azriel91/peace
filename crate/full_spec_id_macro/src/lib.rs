use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Returns a `const FullSpecId` validated at compile time.
///
/// # Examples
///
/// Instantiate a valid `FullSpecId` at compile time:
///
/// ```rust
/// # use peace_full_spec_id_macro::full_spec_id;
/// // use peace::cfg::{full_spec_id, FullSpecId};
///
/// let _my_full_spec_id: FullSpecId = full_spec_id!("valid_id"); // Ok!
///
/// # struct FullSpecId(&'static str);
/// # impl FullSpecId {
/// #     fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// # }
/// ```
///
/// If the ID is invalid, a compilation error is produced:
///
/// ```rust,compile_fail
/// # use peace_full_spec_id_macro::full_spec_id;
/// // use peace::cfg::{full_spec_id, FullSpecId};
///
/// let _my_full_spec_id: FullSpecId = full_spec_id!("-invalid_id"); // Compile error
/// //                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// // error: "-invalid_id" is not a valid `FullSpecId`.
/// //        `FullSpecId`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.
/// #
/// # struct FullSpecId(&'static str);
/// # impl FullSpecId {
/// #     fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// # }
/// ```
#[proc_macro]
pub fn full_spec_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let proposed_id = parse_macro_input!(input as LitStr).value();

    if is_valid_id(&proposed_id) {
        quote!( FullSpecId::new_unchecked( #proposed_id )).into()
    } else {
        let message = format!(
            "\"{proposed_id}\" is not a valid `FullSpecId`.\n\
            `FullSpecId`s must begin with a letter or underscore, and contain only letters, numbers, or underscores."
        );
        quote! {
            compile_error!(#message)
        }
        .into()
    }
}

fn is_valid_id(proposed_id: &str) -> bool {
    let mut chars = proposed_id.chars();
    let first_char = chars.next();
    let first_char_valid = first_char
        .map(|c| c.is_ascii_alphabetic() || c == '_')
        .unwrap_or(false);
    let remainder_chars_valid =
        chars.all(|c| c.is_ascii_alphabetic() || c == '_' || c.is_ascii_digit());

    first_char_valid && remainder_chars_valid
}
