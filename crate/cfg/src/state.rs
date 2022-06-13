use peace_diff::Diff;

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
#[derive(Debug, Diff)]
pub struct State<Logical, Physical> {
    /// Logical state
    logical: Logical,
    /// Physical state
    physical: Physical,
}

impl<Logical, Physical> State<Logical, Physical> {
    /// Returns a new `State`.
    pub fn new(logical: Logical, physical: Physical) -> Self {
        Self { logical, physical }
    }

    /// Returns a reference to the logical state.
    pub fn logical(&self) -> &Logical {
        &self.logical
    }

    /// Returns a mutable reference to the logical state.
    pub fn logical_mut(&mut self) -> &mut Logical {
        &mut self.logical
    }

    /// Returns a reference to the physical state.
    pub fn physical(&self) -> &Physical {
        &self.physical
    }

    /// Returns a mutable reference to the physical state.
    pub fn physical_mut(&mut self) -> &mut Physical {
        &mut self.physical
    }
}
