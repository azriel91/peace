mod cmd;

/// Generates the command context builder implementation for the given scope.
///
/// # Examples
///
/// Instantiate a valid `AppName` at compile time:
///
/// ```rust
/// use peace_code_gen::cmd_ctx_builder_impl;
///
/// cmd_ctx_builder_impl!(SingleProfileSingleFlow);
/// ```
#[proc_macro_attribute]
pub fn cmd_ctx_builder_impl(
    _attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    crate::cmd::cmd_ctx_builder_impl(input)
}
