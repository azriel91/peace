use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Unique identifier for a [`Step`], `Cow<'static, str>` newtype.
///
/// Must begin with a letter or underscore, and contain only letters, numbers,
/// and underscores.
///
/// # Examples
///
/// The following are all examples of valid `StepId`s:
///
/// ```rust
/// # use peace_core::{step_id, StepId};
/// #
/// let _snake = step_id!("snake_case");
/// let _camel = step_id!("camelCase");
/// let _pascal = step_id!("PascalCase");
/// ```
///
/// # Design Note
///
/// TODO: Experiment with upgrades.
///
/// For backward compatibility and migrating steps from old IDs to new IDs, e.g.
/// when they were deployed with an old version of the automation software,
/// there needs to be a way to:
///
/// * Read state using the old ID.
/// * Either clean up that state, or migrate that state into a Step with the
///   new ID.
///
/// [`Step`]: https://docs.rs/peace_cfg/latest/peace_cfg/trait.Step.html
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct StepId(Cow<'static, str>);

crate::id_newtype!(StepId, StepIdInvalidFmt, step_id, code_inline);
