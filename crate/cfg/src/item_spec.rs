use std::fmt;

use async_trait::async_trait;
use peace_core::ItemSpecId;
use peace_resources::{resources::ts::Empty, Resources};
use serde::{de::DeserializeOwned, Serialize};

use crate::{CleanOpSpec, EnsureOpSpec, StateDiffFnSpec, TryFnSpec};

/// Defines all of the data and logic to manage an item.
///
/// The item may be simple or complex, ranging from:
///
/// * File download.
/// * Application installation.
/// * Server launching / initialization.
/// * Multiple cloud resource management.
///
/// The lifecycle operations include:
///
/// 1. Status discovery.
/// 2. Execution.
/// 3. Backup.
/// 4. Restoration.
/// 5. Clean up / deletion.
///
/// Since the latter four operations are write-operations, their specification
/// includes a dry run function.
///
/// # Logical IDs vs Physical IDs
///
/// A logical ID is defined by code, and does not change. A physical ID is one
/// generated during execution, which may be random or computed.
///
/// ## Examples
///
/// The following are examples of logical IDs and corresponding physical
/// IDs:
///
/// * If the operation creates a file, the ID *may* be the full file path, or it
///   may be the file name, assuming the file path may be deduced by the clean
///   up logic from [`Data`].
///
/// * If the operation instantiates a virtual machine on a cloud platform, this
///   may be the ID of the instance so that it may be terminated.
///
/// | Logical ID               | Physical ID                            |
/// | ------------------------ | -------------------------------------- |
/// | `app.file_path`          | `/mnt/data/app.zip`                    |
/// | `app_server_instance_id` | `ef34a9a4-0c02-45a6-96ec-a4db06d4980c` |
/// | `app_server.address`     | `10.0.0.1`                             |
///
/// [`Data`]: crate::CleanOpSpec::Data
#[async_trait(?Send)]
pub trait ItemSpec {
    /// Consumer provided error type.
    type Error: std::error::Error;

    /// Summary of the managed item's state.
    ///
    /// **For an extensive explanation of state, and how to define it, please
    /// see the [state concept] as well as the [`State`] type.**
    ///
    /// This type is used to represent the current state of the item (if it
    /// exists), the desired state of the item (what is intended to exist), and
    /// is used in the *diff* calculation -- what is the difference between the
    /// current and desired states.
    ///
    /// # Examples
    ///
    /// * A file's state may be its path, and a hash of its contents.
    /// * A server's state may be its operating system, CPU and memory capacity,
    ///   IP address, and ID.
    ///
    /// [state concept]: https://peace.mk/book/technical_concepts/state.html
    /// [`State`]: crate::state::State
    type State: Clone + fmt::Display + Serialize + DeserializeOwned;

    /// Diff between the current [`State`] and the desired [`State`].
    ///
    /// This may be the difference between two [`StateLogical`]s, since it may
    /// be impossible to compute / control what [`StatePhysical`] will be.
    /// However, the type may include whether [`StatePhysical`] will be
    /// replaced, even if it cannot tell what it will be replaced with.
    ///
    /// # Design Note
    ///
    /// Initially I thought the field-wise diff between two [`StateLogical`]s is
    /// suitable, but:
    ///
    /// * It does not capture that `StatePhysical` may change.
    /// * It isn't easy or necessarily desired to compare every single field.
    /// * `state.logical.apply(diff) = state_desired` may not be meaningful for
    ///   a field level diff, and the `apply` may be a complex process.
    ///
    /// [`StateLogical`]: Self::StateLogical
    /// [`StatePhysical`]: Self::StatePhysical
    type StateDiff: Clone + fmt::Display + Serialize + DeserializeOwned;

    /// Function that returns the current state of the managed item.
    type StateCurrentFnSpec: TryFnSpec<Error = Self::Error, Output = Self::State>;

    /// Function that returns the desired state of the managed item.
    ///
    /// # Examples
    ///
    /// * For a file download operation, the desired state could be the
    ///   destination path and a content hash.
    ///
    /// * For a web application service operation, the desired state could be
    ///   the web service is running on the latest version.
    type StateDesiredFnSpec: TryFnSpec<Error = Self::Error, Output = Self::State>;

    /// Returns the difference between the current state and desired state.
    ///
    /// # Implementors
    ///
    /// When this type is serialized, it should provide "just enough" /
    /// meaningful information to the user on what has changed. So instead of
    /// including the complete [`State`] and [`StateDesired`], it should only
    /// include the parts that matter.
    ///
    /// This function call is intended to be cheap and fast.
    ///
    /// # Examples
    ///
    /// * For a file download operation, the difference could be the content
    ///   hash changes from `abcd` to `efgh`.
    ///
    /// * For a web application service operation, the desired state could be
    ///   the application version changing from 1 to 2.
    type StateDiffFnSpec: StateDiffFnSpec<Error = Self::Error, State = Self::State, StateDiff = Self::StateDiff>;

    /// Specification of the ensure operation.
    ///
    /// The output is the IDs of resources produced by the operation.
    type EnsureOpSpec: EnsureOpSpec<Error = Self::Error, State = Self::State, StateDiff = Self::StateDiff>;

    /// Specification of the clean operation.
    ///
    /// The output is the IDs of resources cleaned by the operation.
    type CleanOpSpec: CleanOpSpec<Error = Self::Error, State = Self::State>;

    /// Returns the ID of this full spec.
    ///
    /// # Implementors
    ///
    /// The ID should be a unique value that does not change over the lifetime
    /// of the managed item.
    ///
    /// [`ItemSpecId`]s must begin with a letter or underscore, and contain only
    /// letters, numbers, and underscores.  The [`item_spec_id!`] macro provides
    /// a compile time check to ensure that these conditions are upheld.
    ///
    /// ```rust
    /// # use peace_cfg::{item_spec_id, ItemSpecId};
    /// const fn id() -> ItemSpecId {
    ///     item_spec_id!("my_item_spec")
    /// }
    /// # fn main() { let _id = id(); }
    /// ```
    ///
    /// # Design Note
    ///
    /// This is an instance method as logic for an `ItemSpec` may be used for
    /// multiple tasks. For example, an `ItemSpec` implemented to download a
    /// file may be instantiated with different files to download, and each
    /// instance of the `ItemSpec` should have its own ID.
    ///
    /// [`item_spec_id!`]: peace_static_check_macros::item_spec_id
    fn id(&self) -> &ItemSpecId;

    /// Inserts an instance of each data type in [`Resources`].
    ///
    /// # Implementors
    ///
    /// [`Resources`] is the map of any type, and an instance of each data type
    /// must be inserted into the map so that the [`check`] and [`exec`]
    /// functions of each operation can borrow the instance of that type.
    ///
    /// [`check`]: crate::EnsureOpSpec::check
    /// [`exec`]: crate::EnsureOpSpec::exec
    async fn setup(&self, data: &mut Resources<Empty>) -> Result<(), Self::Error>;
}
