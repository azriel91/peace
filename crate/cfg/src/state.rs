pub use self::nothing::Nothing;

mod nothing;

use std::{any::TypeId, fmt};

use serde::{Deserialize, Serialize};

/// Controlled and uncontrolled state of the managed item.
///
/// The logical state is what is controlled, such as:
///
/// * OS image to boot the server.
/// * Application version to install and run.
///
/// The physical state is what is not controlled, such as:
///
/// * Virtual machine instance ID.
/// * Last modification time of configuration.
///
/// This type can be used to represent the current state of the managed item, or
/// the desired state. The `Diff` between the current and desired state
/// indicates whether an operation should be executed.
///
/// If there is no separate physical state -- i.e. the state is fully
/// pre-calculatable and definable in logical state -- the [`Nothing`] type can
/// be used,
///
/// [`Nothing`]: crate::state::Nothing
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct State<Logical, Physical> {
    /// Logical state
    pub logical: Logical,
    /// Physical state
    pub physical: Physical,
}

impl<Logical, Physical> State<Logical, Physical> {
    /// Returns a new `State`.
    pub fn new(logical: Logical, physical: Physical) -> Self {
        Self { logical, physical }
    }
}

impl<Logical, Physical> fmt::Display for State<Logical, Physical>
where
    Logical: fmt::Display,
    Physical: fmt::Display + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let State { logical, physical } = self;

        // Perhaps we should provide a separate trait instead of using `Display`, which
        // returns an optional function for each logical / physical state.
        if TypeId::of::<Physical>() == TypeId::of::<Nothing>() {
            write!(f, "{logical}")
        } else {
            write!(f, "{logical}, {physical}")
        }
    }
}
