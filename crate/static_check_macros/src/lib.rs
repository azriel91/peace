use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Ident};

use self::lit_str_maybe::LitStrMaybe;

mod lit_str_maybe;

/// Returns a `const AppName` validated at compile time.
///
/// This defaults to the crate name.
///
/// # Examples
///
/// Instantiate a valid `AppName` at compile time:
///
/// ```rust
/// # use peace_static_check_macros::app_name;
/// // use peace::cfg::{app_name, AppName};
///
/// let _my_flow: AppName = app_name!("valid_id"); // Ok!
///
/// # struct AppName(&'static str);
/// # impl AppName {
/// #     fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// # }
/// ```
///
/// If the ID is invalid, a compilation error is produced:
///
/// ```rust,compile_fail
/// # use peace_static_check_macros::app_name;
/// // use peace::cfg::{app_name, AppName};
///
/// let _my_flow: AppName = app_name!("-invalid"); // Compile error
/// //                     ^^^^^^^^^^^^^^^^^^^^^^^
/// // error: "-invalid" is not a valid `AppName`.
/// //        `AppName`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.
/// #
/// # struct AppName(&'static str);
/// # impl AppName {
/// #     fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// # }
/// ```
#[proc_macro]
pub fn app_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let crate_name = std::env::var("CARGO_PKG_NAME")
        .expect("Failed to read `CARGO_PKG_NAME` environmental variable to infer `AppName`.");
    ensure_valid_id(input, "AppName", Some(crate_name))
}

/// Returns a `const ItemId` validated at compile time.
///
/// # Examples
///
/// Instantiate a valid `ItemId` at compile time:
///
/// ```rust
/// # use peace_static_check_macros::item_id;
/// // use peace::cfg::{item_id, ItemId};
///
/// let _my_item_id: ItemId = item_id!("valid_id"); // Ok!
///
/// # struct ItemId(&'static str);
/// # impl ItemId {
/// #     fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// # }
/// ```
///
/// If the ID is invalid, a compilation error is produced:
///
/// ```rust,compile_fail
/// # use peace_static_check_macros::item_id;
/// // use peace::cfg::{item_id, ItemId};
///
/// let _my_item_id: ItemId = item_id!("-invalid_id"); // Compile error
/// //                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// // error: "-invalid_id" is not a valid `ItemId`.
/// //        `ItemId`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.
/// #
/// # struct ItemId(&'static str);
/// # impl ItemId {
/// #     fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// # }
/// ```
#[proc_macro]
pub fn item_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ensure_valid_id(input, "ItemId", None)
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
/// //                         ^^^^^^^^^^^^^^^^^^^^^^^
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
    ensure_valid_id(input, "Profile", None)
}

/// Returns a `const FlowId` validated at compile time.
///
/// # Examples
///
/// Instantiate a valid `FlowId` at compile time:
///
/// ```rust
/// # use peace_static_check_macros::flow_id;
/// // use peace::cfg::{flow_id, FlowId};
///
/// let _my_flow: FlowId = flow_id!("valid_id"); // Ok!
///
/// # struct FlowId(&'static str);
/// # impl FlowId {
/// #     fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// # }
/// ```
///
/// If the ID is invalid, a compilation error is produced:
///
/// ```rust,compile_fail
/// # use peace_static_check_macros::flow_id;
/// // use peace::cfg::{flow_id, FlowId};
///
/// let _my_flow: FlowId = flow_id!("-invalid_id"); // Compile error
/// //                     ^^^^^^^^^^^^^^^^^^^^^^^
/// // error: "-invalid_id" is not a valid `FlowId`.
/// //        `FlowId`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.
/// #
/// # struct FlowId(&'static str);
/// # impl FlowId {
/// #     fn new_unchecked(s: &'static str) -> Self { Self(s) }
/// # }
/// ```
#[proc_macro]
pub fn flow_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ensure_valid_id(input, "FlowId", None)
}

fn ensure_valid_id(
    input: proc_macro::TokenStream,
    ty_name: &str,
    default: Option<String>,
) -> proc_macro::TokenStream {
    let proposed_id = parse_macro_input!(input as LitStrMaybe)
        .as_ref()
        .map(|lit_str| lit_str.value())
        .or(default);

    if let Some(proposed_id) = proposed_id.as_deref() {
        if is_valid_id(proposed_id) {
            let ty_name = Ident::new(ty_name, Span::call_site());
            quote!( #ty_name ::new_unchecked( #proposed_id )).into()
        } else {
            let message = format!(
                "\"{proposed_id}\" is not a valid `{ty_name}`.\n\
                `{ty_name}`s must begin with a letter or underscore, and contain only letters, numbers, or underscores."
            );
            quote! {
                compile_error!(#message)
            }
            .into()
        }
    } else {
        let message = format!(
            "`` is not a valid `{ty_name}`.\n\
            `{ty_name}`s must begin with a letter or underscore, and contain only letters, numbers, or underscores."
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
