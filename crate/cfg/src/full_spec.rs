use async_trait::async_trait;
use fn_graph::Resources;
use peace_diff::Diff;
use serde::{de::DeserializeOwned, Serialize};

use crate::{CleanOpSpec, EnsureOpSpec, FnSpec};

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
#[async_trait]
pub trait FullSpec<'op> {
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
    /// This is returned by [`StatusFnSpec`], and is used by [`EnsureOpSpec`]
    /// and [`CleanOpSpec`] to determine if their [`exec`] function needs to
    /// be run.
    ///
    /// # Examples
    ///
    /// ## `FullSpec` that manages servers:
    ///
    /// The `StateLogical` may be the number of server instances, the boot
    /// image, and their hardware capacity.
    ///
    /// * The [`StatusFnSpec`] returns this, and it should be renderable in a
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
    /// * The [`StatusFnSpec`] returns this, and it should be renderable in a
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
    /// [`StatusFnSpec`]: Self::StatusFnSpec
    /// [`StatePhysical`]: Self::StatePhysical
    type StateLogical: Diff + Serialize + DeserializeOwned;

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
    type StatePhysical: Serialize + DeserializeOwned;

    /// Function that returns the current status of the managed item.
    ///
    /// # Future Development
    ///
    /// The `StatusFnSpec` may decide to not check for status if it caches
    /// status. For that use case, the `state` used by the StatusFnSpec
    /// should include:
    ///
    /// * Execution ID
    /// * Last status query time
    ///
    /// This allows the check function to tell if the status has been queried
    /// within the past day, don't query it again.
    type StatusFnSpec: FnSpec<'op, Error = Self::Error, Output = Self::StateLogical>;

    // TODO: DiffFnSpec:
    //
    // Shows the [`Diff`] between the [`StateLogical`] returned from
    // [`StatusFnSpec`].

    /// Specification of the ensure operation.
    ///
    /// The output is the IDs of resources produced by the operation.
    type EnsureOpSpec: EnsureOpSpec<
        'op,
        Error = Self::Error,
        StateLogical = Self::StateLogical,
        StatePhysical = Self::StatePhysical,
    >;

    /// Specification of the clean operation.
    ///
    /// The output is the IDs of resources cleaned by the operation.
    type CleanOpSpec: CleanOpSpec<'op, Error = Self::Error, StatePhysical = Self::StatePhysical>;

    /// Returns the `StatusFnSpec` for this `FullSpec`.
    fn status_fn_spec(&self) -> &Self::StatusFnSpec;

    /// Returns the `EnsureOpSpec` for this `FullSpec`.
    fn ensure_op_spec(&self) -> &Self::EnsureOpSpec;

    /// Returns the `CleanOpSpec` for this `FullSpec`.
    fn clean_op_spec(&self) -> &Self::CleanOpSpec;

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
    async fn setup(data: &mut Resources) -> Result<(), Self::Error>;
}
