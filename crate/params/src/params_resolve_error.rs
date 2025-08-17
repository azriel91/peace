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
    InMemory {
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
    InMemoryBorrowConflict {
        /// Hierarchy of fields traversed to resolve the value.
        value_resolution_ctx: ValueResolutionCtx,
    },

    /// Failed to resolve a from value from `resources`.
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_params::params_resolve_error::from_map),
            help(
                "Make sure `{from_type_name}` has been inserted into `resources`.\n\
                Value resolution mode is: {value_resolution_mode:?}",
                value_resolution_mode = value_resolution_ctx.value_resolution_mode()
            )
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

    #[error(
        "Failed to downcast resolved `BoxDt` into `{to_type_name}` to populate:\n\
        \n\
        ```rust\n\
        {value_resolution_ctx}\n\
        ```"
    )]
    FromMapDowncast {
        /// Hierarchy of fields traversed to resolve the value.
        value_resolution_ctx: ValueResolutionCtx,
        /// Name of the type that is being resolved.
        ///
        /// Usually one of the `Item::Params` types.
        ///
        /// Corresponds to `T` in `Fn(&U) -> T`.
        to_type_name: String,
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

    /// Failed to resolve a mapping function from the registry.
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_params::params_resolve_error::mapping_fn_resolve),
            help(
                "Mapping function variants are intended to be stable, so if you renamed the mapping function, you may have to edit the stored param spec to use the new name."
            )
        )
    )]
    #[error(
        "Failed to resolve mapping function `{mapping_fn:?}` to populate:\n\
        \n\
        ```rust\n\
        {value_resolution_ctx}\n\
        ```"
    )]
    MappingFnResolve {
        /// Hierarchy of fields traversed to resolve the value.
        value_resolution_ctx: ValueResolutionCtx,
        /// String representation of the mapping function.
        ///
        /// In practice, this is a YAML serialized string representation of the
        /// `MappingFns` variant.
        mapping_fn: String,
    },
}
