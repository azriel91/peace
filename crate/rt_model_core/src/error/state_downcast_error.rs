use type_reg::untagged::{BoxDtDisplay, DataType};

/// Error downcasting a `BoxDtDisplay` into a step's concrete state type.
#[cfg_attr(feature = "error_reporting", derive(miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum StateDowncastError {
    /// Both step states could not be downcasted.
    #[error(
        "Step states could not be downcasted to `{ty_name}`.\n\
        Boxed type are:\n\
        \n\
        * `{boxed_ty_a:?}`.\n\
        * `{boxed_ty_b:?}`.\n\
        ",
        ty_name = ty_name,
        boxed_ty_a = state_a.type_name(),
        boxed_ty_b = state_b.type_name(),
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::state_downcast_error::both),
            help(
                "\
                This error happens when the boxed states could not be downcasted to
                this step's state, which indicates one of the following:\n\
                \n\
                * Peace contains a bug, and passed an incorrect box to this step.\n\
                * Step IDs were swapped, such that `StepA`'s state is passed to `StepB`.\n\
                \n\
                This needs some rework on how step IDs are implemented -- as in,
                whether we should use a string newtype for `StepId`s, or redesign
                how `Step`s or related types are keyed.\n\
                "
            ),
        )
    )]
    Both {
        /// Type name of the state type.
        ty_name: String,
        /// First state parameter.
        state_a: BoxDtDisplay,
        /// Second state parameter.
        state_b: BoxDtDisplay,
    },
    /// First step state could not be downcasted.
    #[error(
        "First step state could not be downcasted to `{ty_name}`.\n\
        Boxed type is `{boxed_ty:?}`.",
        ty_name = ty_name,
        boxed_ty = state_a.type_name(),
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::state_downcast_error::first),
            help(
                "\
                This error happens when the boxed states could not be downcasted to
                this step's state, which indicates one of the following:\n\
                \n\
                * Peace contains a bug, and passed an incorrect box to this step.\n\
                * Step IDs were swapped, such that `StepA`'s state is passed to `StepB`.\n\
                \n\
                This needs some rework on how step IDs are implemented -- as in,
                whether we should use a string newtype for `StepId`s, or redesign
                how `Step`s or related types are keyed.\n\
                "
            ),
        )
    )]
    First {
        /// Type name of the state type.
        ty_name: String,
        /// First state parameter.
        state_a: BoxDtDisplay,
    },
    /// Second step state could not be downcasted.
    #[error(
        "Second step state could not be downcasted to `{ty_name}`.\n\
        Boxed type is `{boxed_ty:?}`.",
        ty_name = ty_name,
        boxed_ty = state_b.type_name(),
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_rt_model::state_downcast_error::second),
            help(
                "\
                This error happens when the boxed states could not be downcasted to
                this step's state, which indicates one of the following:\n\
                \n\
                * Peace contains a bug, and passed an incorrect box to this step.\n\
                * Step IDs were swapped, such that `StepA`'s state is passed to `StepB`.\n\
                \n\
                This needs some rework on how step IDs are implemented -- as in,
                whether we should use a string newtype for `StepId`s, or redesign
                how `Step`s or related types are keyed.\n\
                "
            ),
        )
    )]
    Second {
        /// Type name of the state type.
        ty_name: String,
        /// Second state parameter.
        state_b: BoxDtDisplay,
    },
}
