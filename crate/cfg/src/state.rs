use peace_diff::Diff;
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
#[derive(Clone, Debug, Diff, Deserialize, Serialize)]
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
