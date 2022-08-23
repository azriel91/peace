use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Returns a `const ItemSpecId` validated at compile time.
///
/// # Examples
///
/// Instantiate a valid `ItemSpecId` at compile time:
///
/// ```rust
/// # use peace_static_check_macros::item_spec_id;
/// // use peace::cfg::{item_spec_id, ItemSpecId};
///
/// let _my_item_spec_id: ItemSpecId = item_spec_id!("valid_id"); // Ok!
///
/// # struct ItemSpecId(&'static str);
/// # impl ItemSpecId {
/// #     fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// # }
/// ```
///
/// If the ID is invalid, a compilation error is produced:
///
/// ```rust,compile_fail
/// # use peace_static_check_macros::item_spec_id;
/// // use peace::cfg::{item_spec_id, ItemSpecId};
///
/// let _my_item_spec_id: ItemSpecId = item_spec_id!("-invalid_id"); // Compile error
/// //                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// // error: "-invalid_id" is not a valid `ItemSpecId`.
/// //        `ItemSpecId`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.
/// #
/// # struct ItemSpecId(&'static str);
/// # impl ItemSpecId {
/// #     fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// # }
/// ```
#[proc_macro]
pub fn item_spec_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let proposed_id = parse_macro_input!(input as LitStr).value();

    if is_valid_id(&proposed_id) {
        quote!( ItemSpecId::new_unchecked( #proposed_id )).into()
    } else {
        let message = format!(
            "\"{proposed_id}\" is not a valid `ItemSpecId`.\n\
            `ItemSpecId`s must begin with a letter or underscore, and contain only letters, numbers, or underscores."
        );
        quote! {
            compile_error!(#message)
        }
        .into()
    }
}

/// Returns a `const Profile` validated at compile time.
///
/// # Examples
///
/// Instantiate a valid `Profile` at compile time:
///
/// ```rust
/// # use peace_static_check_macros::profile;
/// // use peace::cfg::{profile, Profile};
///
/// let _my_profile: Profile = profile!("valid_id"); // Ok!
///
/// # struct Profile(&'static str);
/// # impl Profile {
/// #     fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// # }
/// ```
///
/// If the ID is invalid, a compilation error is produced:
///
/// ```rust,compile_fail
/// # use peace_static_check_macros::profile;
/// // use peace::cfg::{profile, Profile};
///
/// let _my_profile: Profile = profile!("-invalid_id"); // Compile error
/// //                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// // error: "-invalid_id" is not a valid `Profile`.
/// //        `Profile`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.
/// #
/// # struct Profile(&'static str);
/// # impl Profile {
/// #     fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// # }
/// ```
#[proc_macro]
pub fn profile(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let proposed_id = parse_macro_input!(input as LitStr).value();

    if is_valid_id(&proposed_id) {
        quote!( Profile::new_unchecked( #proposed_id )).into()
    } else {
        let message = format!(
            "\"{proposed_id}\" is not a valid `Profile`.\n\
            `Profile`s must begin with a letter or underscore, and contain only letters, numbers, or underscores."
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
