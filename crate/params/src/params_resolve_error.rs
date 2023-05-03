/// Failed to resolve values for a `Params` object from `resources`.
//
// TODO: Help text could be generated based on the type of `Params` -- named fields struct, tuple
// struct, enum -- instead of assuming it's always a named fields struct.
#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "error_reporting", derive(miette::Diagnostic))]
pub enum ParamsResolveError {
    /// Failed to resolve a field value from `resources`.
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_params::params_resolve_error::from),
            help("Make sure `{field_type_name}` has been inserted into `resources`.")
        )
    )]
    #[error(
        r#"Failed to resolve `{field_type_name}` to populate:

```rust
{params_type_name} {{
    {field_name}: {field_type_name},
    ..
}}
```"#
    )]
    From {
        /// Name of the `Params` type whose value could not be resolved.
        params_type_name: &'static str,
        /// Field name within `Params` whose value could not be resolved.
        field_name: &'static str,
        /// Name of the field type in `Params`.
        field_type_name: &'static str,
    },

    /// Failed to borrow a field value from `resources`.
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_params::params_resolve_error::from_borrow_conflict),
            help("By design `{field_type_name}` must not be borrowed mutably.")
        )
    )]
    #[error(
        r#"Borrow conflict on `{field_type_name}` to populate:

```rust
{params_type_name} {{
    {field_name}: {field_type_name},
    ..
}}
```"#
    )]
    FromBorrowConflict {
        /// Name of the `Params` type.
        params_type_name: &'static str,
        /// Field name within `Params`.
        field_name: &'static str,
        /// Name of the field type in `Params`.
        field_type_name: &'static str,
    },

    /// Failed to resolve a from value from `resources`.
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_params::params_resolve_error::from_map),
            help("Make sure `{from_type_name}` has been inserted into `resources`.")
        )
    )]
    #[error(
        r#"Failed to resolve `{from_type_name}` to populate:

```rust
{params_type_name} {{
    {field_name}: {field_type_name},
    ..
}}
```"#
    )]
    FromMap {
        /// Name of the `Params` type whose value could not be resolved.
        params_type_name: &'static str,
        /// Field name within `Params` whose value could not be resolved.
        field_name: &'static str,
        /// Name of the field type in `Params`.
        ///
        /// Corresponds to `T` in `Fn(&U) -> T`.
        field_type_name: &'static str,
        /// Name of the type from which to map the field value from.
        ///
        /// Corresponds to `U` in `Fn(&U) -> T`.
        from_type_name: &'static str,
    },

    /// Failed to borrow a value to map to a field from `resources`.
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_params::params_resolve_error::from_map_borrow_conflict),
            help("By design `{from_type_name}` must not be borrowed mutably.")
        )
    )]
    #[error(
        r#"Borrow conflict on `{from_type_name}` to populate:

```rust
{params_type_name} {{
    {field_name}: {field_type_name},
    ..
}}
```"#
    )]
    FromMapBorrowConflict {
        /// Name of the `Params` type.
        params_type_name: &'static str,
        /// Field name within `Params`.
        field_name: &'static str,
        /// Name of the field type in `Params`.
        field_type_name: &'static str,
        /// Name of the type from which to map the field value from.
        ///
        /// Corresponds to `U` in `Fn(&U) -> T`.
        from_type_name: &'static str,
    },
}
