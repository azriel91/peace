use async_trait::async_trait;
use peace_core::FullSpecId;
use peace_diff::Diff;
use peace_resources::{resources_type_state::Empty, Resources};
use serde::{de::DeserializeOwned, Serialize};

use crate::{CleanOpSpec, EnsureOpSpec, FnSpec, State};

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
#[async_trait]
#[nougat::gat]
pub trait FullSpec {
    /// Consumer provided error type.
    type Error: std::error::Error;

    /// State of the managed item that is controlled.
    ///
    /// Examples are a server boot image, or application configuration version,
    /// but not virtual machine instance IDs, or file timestamps. For
    /// those, see [`StatePhysical`].
    ///
    /// This is intended as a serializable summary of the state, so it should be
    /// relatively lightweight.
    ///
    /// This is returned by [`StateNowFnSpec`], and is used by [`EnsureOpSpec`]
    /// and [`CleanOpSpec`] to determine if their `exec` functions need to
    /// be run.
    ///
    /// # Examples
    ///
    /// ## `FullSpec` that manages servers:
    ///
    /// The `StateLogical` may be the number of server instances, the boot
    /// image, and their hardware capacity.
    ///
    /// * The [`StateNowFnSpec`] returns this, and it should be renderable in a
    ///   human readable format.
    ///
    /// * The [`EnsureOpSpec::check`] function should be able to use this to
    ///   determine if there are enough servers using the desired image. The
    ///   [`EnsureOpSpec::exec`] function returns the physical IDs of any
    ///   launched servers.
    ///
    /// * The [`CleanOpSpec::check`] function should be able to use this to
    ///   determine if the servers that need to be removed. The
    ///   [`EnsureOpSpec::exec`] function should be able to remove the servers.
    ///
    /// * The backup [`EnsureOpSpec::exec`] function should produce this as a
    ///   record of the current state.
    ///
    /// * The restore [`EnsureOpSpec::exec`] function should be able to read
    ///   this and launch servers using the recorded image and hardware
    ///   capacity.
    ///
    /// ## `FullSpec` that manages application configuration:
    ///
    /// The `StateLogical` is not necessarily the configuration itself, but may
    /// be a content hash, commit hash or version of the configuration. If
    /// the configuration is small, then one may consider making that the
    /// state.
    ///
    /// * The [`StateNowFnSpec`] returns this, and it should be renderable in a
    ///   human readable format.
    ///
    /// * The [`EnsureOpSpec::check`] function should be able to compare the
    ///   desired configuration with this to determine if the configuration is
    ///   already in the correct state or needs to be altered.
    ///
    /// * The [`CleanOpSpec::check`] function should be able to use this to
    ///   determine if the configuration needs to be undone. The
    ///   [`EnsureOpSpec::exec`] function should be able to remove the
    ///   configuration.
    ///
    /// * The backup [`EnsureOpSpec::exec`] function should produce this as a
    ///   record of the current state.
    ///
    /// * The restore [`EnsureOpSpec::exec`] function should be able to read
    ///   this and determine how to alter the system to match this state. If
    ///   this were a commit hash, then restoring would be applying the
    ///   configuration at that commit hash.
    ///
    /// [`StateNowFnSpec`]: Self::StateNowFnSpec
    /// [`StatePhysical`]: Self::StatePhysical
    type StateLogical: Clone + Diff + Serialize + DeserializeOwned;

    /// State of the managed item that is not controlled.
    ///
    /// Examples are virtual machine instance IDs, or generated values.
    ///
    /// Physical IDs of *things* produced by the operation will be part of
    /// `StatePhysical`. Even though they are not controlled, they still matter:
    ///
    /// * Environmental configuration: Providing these to servers to communicate
    ///   with each other.
    /// * Cleaning up resources: VMs, reserved tokens etcetera.
    ///
    /// [`Data`]: crate::EnsureOpSpec::Data
    /// [`StateLogical`]: Self::State
    /// [`EnsureOpSpec::desired`]: crate::EnsureOpSpec::desired
    type StatePhysical: Clone + Serialize + DeserializeOwned;

    /// Function that returns the current state of the managed item.
    ///
    /// # Future Development
    ///
    /// The `StateNowFnSpec` may decide to not check for state if it caches
    /// state. For that use case, the `state` used by the StateNowFnSpec
    /// should include:
    ///
    /// * Execution ID
    /// * Last state query time
    ///
    /// This allows the check function to tell if the state has been queried
    /// within the past day, don't query it again.
    type StateNowFnSpec: FnSpec<
        Error = Self::Error,
        Output = State<Self::StateLogical, Self::StatePhysical>,
    >;

    /// Function that returns the desired state of the managed item.
    ///
    /// # Examples
    ///
    /// * For a file download operation, the desired state could be the
    ///   destination path and a content hash.
    ///
    /// * For a web application service operation, the desired state could be
    ///   the web service is running on the latest version.
    ///
    /// # Implementors
    ///
    /// This function call is intended to be cheap and fast.
    type StateDesiredFnSpec: FnSpec<Error = Self::Error, Output = Self::StateLogical>;

    // TODO: DiffFnSpec:
    //
    // Shows the [`Diff`] between the [`StateLogical`] returned from
    // [`StateNowFnSpec`].

    /// Specification of the ensure operation.
    ///
    /// The output is the IDs of resources produced by the operation.
    type EnsureOpSpec: EnsureOpSpec<
        Error = Self::Error,
        StateLogical = Self::StateLogical,
        StatePhysical = Self::StatePhysical,
    >;

    /// Specification of the clean operation.
    ///
    /// The output is the IDs of resources cleaned by the operation.
    type CleanOpSpec: CleanOpSpec<
        Error = Self::Error,
        StateLogical = Self::StateLogical,
        StatePhysical = Self::StatePhysical,
    >;

    /// Returns the ID of this full spec.
    ///
    /// # Implementors
    ///
    /// The ID should be a unique value that does not change over the lifetime
    /// of the managed item.
    ///
    /// [`FullSpecId`]s must begin with a letter or underscore, and contain only
    /// letters, numbers, and underscores.  The [`full_spec_id!`] macro provides
    /// a compile time check to ensure that these conditions are upheld.
    ///
    /// ```rust
    /// # use peace_cfg::{full_spec_id, FullSpecId};
    /// const fn id() -> FullSpecId {
    ///     full_spec_id!("my_full_spec")
    /// }
    /// # fn main() { let _id = id(); }
    /// ```
    ///
    /// # Design Note
    ///
    /// This is an instance method as logic for a `FullSpec` may be used for
    /// multiple tasks. For example, a `FullSpec` implemented to download a
    /// file may be instantiated with different files to download, and each
    /// instance of the `FullSpec` should have its own ID.
    ///
    /// [`full_spec_id!`]: peace_full_spec_id_macro::full_spec_id
    fn id(&self) -> FullSpecId;

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
