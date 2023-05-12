use crate::{FieldNameAndType, ValueResolutionCtx};

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
            help("Make sure `{field_type_name}` has been inserted into `resources`.",
                field_type_name = value_resolution_ctx
                    .resolution_chain()
                    .last()
                    .map(FieldNameAndType::type_name)
                    .unwrap_or(value_resolution_ctx.params_type_name())
            )
        )
    )]
    #[error("Failed to resolve `{field_type_name}` to populate:\n\
        \n\
        ```rust\n\
        {value_resolution_ctx}\n\
        ```",
        field_type_name = value_resolution_ctx
            .resolution_chain()
            .last()
            .map(FieldNameAndType::type_name)
            .unwrap_or(value_resolution_ctx.params_type_name()))]
    From {
        /// Hierarchy of fields traversed to resolve the value.
        value_resolution_ctx: ValueResolutionCtx,
    },

    /// Failed to borrow a field value from `resources`.
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_params::params_resolve_error::from_borrow_conflict),
            help("By design `{field_type_name}` must not be borrowed mutably.",
                field_type_name = value_resolution_ctx
                    .resolution_chain()
                    .last()
                    .map(FieldNameAndType::type_name)
                    .unwrap_or(value_resolution_ctx.params_type_name())
            )
        )
    )]
    #[error("Borrow conflict on `{field_type_name}` to populate:\n\
        \n\
        ```rust\n\
        {value_resolution_ctx}\n\
        ```",
        field_type_name = value_resolution_ctx
            .resolution_chain()
            .last()
            .map(FieldNameAndType::type_name)
            .unwrap_or(value_resolution_ctx.params_type_name())
        )
    ]
    FromBorrowConflict {
        /// Hierarchy of fields traversed to resolve the value.
        value_resolution_ctx: ValueResolutionCtx,
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
        "Failed to resolve `{from_type_name}` to populate:\n\
        \n\
        ```rust\n\
        {value_resolution_ctx}\n\
        ```"
    )]
    FromMap {
        /// Hierarchy of fields traversed to resolve the value.
        value_resolution_ctx: ValueResolutionCtx,
        /// Name of the type from which to map the field value from.
        ///
        /// Corresponds to `U` in `Fn(&U) -> T`.
        from_type_name: String,
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
        "Borrow conflict on `{from_type_name}` to populate:\n\
        \n\
        ```rust\n\
        {value_resolution_ctx}\n\
        ```"
    )]
    FromMapBorrowConflict {
        /// Hierarchy of fields traversed to resolve the value.
        value_resolution_ctx: ValueResolutionCtx,
        /// Name of the type from which to map the field value from.
        ///
        /// Corresponds to `U` in `Fn(&U) -> T`.
        from_type_name: String,
    },
}
