use serde::{de::DeserializeOwned, Serialize};

use crate::OpSpec;

/// Encapsulates all the operations of a work item.
///
/// A work item encapsulates the data and logic needed to manage the lifecycle
/// operations of *something* that is user defined. The *something* may be:
///
/// * Downloading a file.
/// * Installing an application.
/// * Launching some servers.
///
/// The lifecycle operations are defined as:
///
/// 1. Status discovery.
/// 2. Execution -- dry run and actual.
/// 3. Backup.
/// 4. Restoration.
/// 5. Clean up / deletion.
pub trait WorkSpec {
    /// IDs of resources produced by the operation.
    ///
    /// This is provided to the clean up logic to determine what to clean up.
    ///
    /// These should be physical IDs, not logical IDs. A logical resource ID is
    /// defined by code, and does not change. A physical resource ID is one
    /// generated during execution, which generally is random or computed.
    ///
    /// # Examples
    ///
    /// The following are examples of logical IDs and corresponding physical
    /// IDs:
    ///
    /// * If the operation creates a file, the ID *may* be the full file path,
    ///   or it may be the file name, assuming the file path may be deduced by
    ///   the clean up logic from [`Params`].
    ///
    /// * If the operation instantiates a virtual machine on a cloud platform,
    ///   this may be the ID of the instance so that it may be terminated.
    ///
    /// | Logical ID               | Physical ID                            |
    /// | ------------------------ | -------------------------------------- |
    /// | `app.file_path`          | `/mnt/data/app.zip`                    |
    /// | `app_server_instance_id` | `ef34a9a4-0c02-45a6-96ec-a4db06d4980c` |
    /// | `app_server.address`     | `10.0.0.1`                             |
    ///
    /// [`Params`]: crate::OpSpec::Params
    type ResIds: Serialize + DeserializeOwned;

    /// State of the data or resources that this `WorkSpec` manages.
    ///
    /// This is intended as a serializable summary of the state, so it should be
    /// relatively lightweight.
    ///
    /// This is the type returned by the [`StatusSpec`], and is used by
    /// [`EnsureOpSpec`] and [`CleanOpSpec`] to determine if their [`exec`]
    /// function needs to be run.
    ///
    /// # Examples
    ///
    /// ## `WorkSpec` that manages application configuration:
    ///
    /// The state is not necessarily the configuration itself, but may be a
    /// content hash, commit hash or version of the configuration. If the
    /// configuration is small, then one may consider making that the state.
    ///
    /// * This status operation should return this, and it should be renderable
    ///   in a human readable format.
    ///
    /// * The ensure operation's check function should be able to compare the
    ///   desired configuration with this to determine if the configuration is
    ///   already in the correct state or needs to be altered.
    ///
    /// * The clean operation's check function should be able to use this to
    ///   determine if the configuration needs to be undone.
    ///
    /// * The backup operation's work function should produce this as a record
    ///   of the current state.
    ///
    /// * The restore operation's work function should be able to read this and
    ///   determine how to alter the system to match this state. If this were a
    ///   commit hash, then restoring would be applying the configuration at
    ///   that commit hash.
    ///
    /// ## `WorkSpec` that manages servers:
    ///
    /// The state may be the number of server instances, the boot image, and
    /// their hardware capacity.
    ///
    /// * This status operation should return this, and it should be renderable
    ///   in a human readable format.
    ///
    /// * The ensure operation's check function should be able to use this to
    ///   determine if there are enough servers using the desired image.
    ///
    /// * The clean operation's check function should be able to use this to
    ///   determine if the servers that need to be removed.
    ///
    /// * The backup operation's work function should produce this as a record
    ///   of the current state.
    ///
    /// * The restore operation's work function should be able to read this and
    ///   launch servers using the recorded image and hardware capacity.
    ///
    /// [`StatusSpec`]: Self::StatusSpec
    /// [`EnsureOpSpec`]: Self::EnsureOpSpec
    /// [`CleanOpSpec`]: Self::CleanOpSpec
    /// [`exec`]: crate::OpSpec::exec
    type State: Serialize + DeserializeOwned;

    /// Specification of the status function.
    ///
    /// The `state` used by the Status function should include:
    ///
    /// * Execution ID
    /// * Last status query time
    ///
    /// This allows the check function to tell: if the status has been queried
    /// within the past day, don't query it again.
    ///
    /// The output is the state that this `WorkSpec` manages.
    type StatusSpec: OpSpec<State = Self::State, Output = Self::State>;

    /// Specification of the ensure operation.
    ///
    /// The output is the IDs of resources produced by the operation.
    type EnsureOpSpec: OpSpec<State = Self::State, Output = Self::ResIds>;

    /// Specification of the clean operation.
    ///
    /// The output is the IDs of resources cleaned by the operation.
    type CleanOpSpec: OpSpec<State = Self::State, Output = Self::ResIds>;
}
