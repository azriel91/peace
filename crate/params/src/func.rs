/// Provides a simpler constraint for the compiler to report to the developer.
///
/// Implemented for up to five arguments.
///
/// Instead of the detailed error message from the `From` trait:
///
/// ```md
/// the trait `From<(
///     std::option::Option<std::string::String>,
///     [closure@examples/envman/src/flows/app_upload_flow.rs:101:40: 101:73],
/// )>`
/// is not implemented for
/// `MappingFnImpl<
///     std::string::String,
///     [closure@examples/envman/src/flows/app_upload_flow.rs:101:40: 101:73],
///     _,
/// >`
/// ```
///
/// we get the much clearer:
///
/// ```md
/// the trait `Func<std::option::Option<std::string::String>, _>`
/// is not implemented for closure `[closure@examples/envman/src/flows/app_upload_flow.rs:101:40: 101:73]`
/// ```
pub trait Func<ReturnType, Args> {}

macro_rules! impl_func_for_f {
    ($($Arg:ident),+) => {
        impl<T, F, $($Arg,)+> Func<T, ($($Arg,)+)> for F
        where F: Fn($(&$Arg,)+) -> T {}
    };
}

impl_func_for_f!(A0);
impl_func_for_f!(A0, A1);
impl_func_for_f!(A0, A1, A2);
impl_func_for_f!(A0, A1, A2, A3);
impl_func_for_f!(A0, A1, A2, A3, A4);

/// Provides a simpler constraint for the compiler to report to the developer.
///
/// Instead of the detailed error message from the `From` trait:
///
/// ```md
/// the trait `From<(
///     std::option::Option<std::string::String>,
///     [closure@examples/envman/src/flows/app_upload_flow.rs:101:40: 101:73],
/// )>`
/// is not implemented for
/// `MappingFnImpl<
///     std::string::String,
///     [closure@examples/envman/src/flows/app_upload_flow.rs:101:40: 101:73],
///     _,
/// >`
/// ```
///
/// we get:
///
/// ```md
/// the trait `FromFunc<[closure@examples/envman/src/flows/app_upload_flow.rs:101:40: 101:73]>`
/// is not implemented for `MappingFnImpl<
///     std::string::String,
///     [closure@examples/envman/src/flows/app_upload_flow.rs:101:40: 101:73],
///     _,
/// >`
/// ```
pub trait FromFunc<F> {
    fn from_func(field_name: Option<String>, f: F) -> Self;
}
