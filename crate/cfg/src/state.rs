pub use self::{
    external::{External, Fetched, Generated, Timestamped},
    external_opt::{ExternalOpt, FetchedOpt, GeneratedOpt, TimestampedOpt},
    nothing::Nothing,
};

mod external;
mod external_opt;
mod nothing;

use std::{any::TypeId, fmt};

use serde::{Deserialize, Serialize};

/// Logical and physical states of a managed item.
///
/// This type can be used when the managed item has both logical and physical
/// states. Otherwise, a type that represents the fully logical / fully physical
/// state is sufficient.
///
/// In `peace`, [`State`] represents the values of an item, and has the
/// following usages:
///
/// * Showing users the state of an item.
/// * Allowing users to describe the state that an item should be.
/// * Determining what needs to change between the current state and the goal
///   state.
///
/// Therefore, `State` should be:
///
/// * Serializable
/// * Displayable in a human readable format
/// * Relatively lightweight &ndash; e.g. does not necessarily contain file
///   contents, but a hash of it.
///
///
/// ## Logical and Physical State
///
/// State can be separated into two parts:
///
/// * **Logical state:** Information that is functionally important, and can be
///   specified by the user ahead of time.
///
///     Examples of logical state are:
///
///     - File contents
///     - An application version
///     - Server operating system version
///     - Server CPU capacity
///     - Server RAM capacity
///
/// * **Physical state:** Information that is discovered / produced when the
///   automation is executed.
///
///     Examples of physical state are:
///
///     - ETag of a downloaded file.
///     - Execution time of a command.
///     - Server ID that is generated on launch.
///     - Server IP address.
///
///
/// ## Defining State
///
/// ### Fully Logical
///
/// If an item's state can be fully described before the item exists, and can be
/// made to happen without interacting with an external service, then the state
/// is fully logical.
///
/// For example, copying a file from one directory to another. The state of the
/// file in the source directory and destination directories are fully
/// discoverable, and there is no information generated during automation that
/// is needed to determine if the states are equivalent.
///
///
/// ### Logical and Physical
///
/// If an item's goal state can be described before the item exists, but
/// interacts with an external service which produces additional information to
/// bring that goal state into existence, then the state has both logical and
/// physical parts.
///
/// For example, launching a server or virtual machine. The operating system,
/// CPU capacity, and RAM are logical information, and can be determined ahead
/// of time. However, the server ID and IP address are produced by the virtual
/// machine service provider, which is physical state.
///
///
/// ### Fully Physical
///
/// If an item's goal state is simply, "automation has been executed after these
/// files have been modified", then the state has no logical component.
///
/// For example, running a compilation command only if the compilation artifact
/// doesn't exist, or the source files have changed since the last time the
/// compilation has been executed.
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
