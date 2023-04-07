use std::fmt;

use async_trait::async_trait;
use dyn_clone::DynClone;
use peace_core::{ItemSpecId, OpCheckStatus};
use peace_data::Data;
use peace_resources::{resources::ts::Empty, Resources};
use serde::{de::DeserializeOwned, Serialize};

use crate::OpCtx;

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
pub trait ItemSpec: DynClone {
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

    /// Diff between the current and target [`State`]s.
    ///
    /// # Design Note
    ///
    /// Initially I thought the field-wise diff between two [`State`]s is
    /// suitable, but:
    ///
    /// * Externally controlled state may not be known ahead of time.
    /// * It isn't easy or necessarily desired to compare every single field.
    /// * `state.apply(diff) = state_desired` may not be meaningful for a field
    ///   level diff, and the `apply` may be a complex process.
    ///
    /// [`State`]: Self::State
    type StateDiff: Clone + fmt::Display + Serialize + DeserializeOwned;

    /// Data that the function reads from, or writes to.
    ///
    /// These may be parameters to the function, or information calculated from
    /// previous functions.
    type Data<'op>: Data<'op>
    where
        Self: 'op;

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
    /// must be inserted into the map so that the [`check`] and [`apply`]
    /// functions of each operation can borrow the instance of that type.
    ///
    /// [`check`]: crate::ApplyFns::check
    /// [`apply`]: crate::ApplyFns::apply
    async fn setup(&self, data: &mut Resources<Empty>) -> Result<(), Self::Error>;

    /// Returns the current state of the managed item, if possible.
    ///
    /// This should return `Ok(None)` if the state is not able to be queried,
    /// such as when failing to connect to a remote host, instead of returning
    /// an error.
    async fn try_state_current(
        op_ctx: OpCtx<'_>,
        data: Self::Data<'_>,
    ) -> Result<Option<Self::State>, Self::Error>;

    /// Returns the current state of the managed item.
    ///
    /// This is *expected* to successfully discover the current state, so errors
    /// will be presented to the user.
    async fn state_current(
        op_ctx: OpCtx<'_>,
        data: Self::Data<'_>,
    ) -> Result<Self::State, Self::Error>;

    /// Returns the desired state of the managed item, if possible.
    ///
    /// This should return `Ok(None)` if the state is not able to be queried,
    /// such as when failing to read a potentially non-existent file to
    /// determine its content hash, instead of returning an error.
    async fn try_state_desired(
        op_ctx: OpCtx<'_>,
        data: Self::Data<'_>,
    ) -> Result<Option<Self::State>, Self::Error>;

    /// Returns the desired state of the managed item.
    ///
    /// This is *expected* to successfully discover the desired state, so errors
    /// will be presented to the user.
    ///
    /// # Examples
    ///
    /// * For a file download operation, the desired state could be the
    ///   destination path and a content hash.
    ///
    /// * For a web application service operation, the desired state could be
    ///   the web service is running on the latest version.
    async fn state_desired(
        op_ctx: OpCtx<'_>,
        data: Self::Data<'_>,
    ) -> Result<Self::State, Self::Error>;

    /// Returns the difference between the current state and desired state.
    ///
    /// # Implementors
    ///
    /// When this type is serialized, it should provide "just enough" /
    /// meaningful information to the user on what has changed. So instead of
    /// including the complete desired [`State`], it should only include the
    /// parts that changed.
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
    async fn state_diff(
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_desired: &Self::State,
    ) -> Result<Self::StateDiff, Self::Error>;

    /// Returns the representation of a clean `State`.
    ///
    /// # Implementors
    ///
    /// This should return essentially the `None` concept of the item spec
    /// state. The diff between this and the current state will be shown to the
    /// user when they want to see what would be cleaned up by the clean
    /// command.
    async fn state_clean(data: Self::Data<'_>) -> Result<Self::State, Self::Error>;

    /// Returns whether `apply` needs to be executed.
    ///
    /// If the current state is already in sync with the target state, then
    /// `apply` does not have to be executed.
    ///
    /// # Examples
    ///
    /// * For a file download operation, if the destination file differs from
    ///   the file on the server, then the file needs to be downloaded.
    ///
    /// * For a web application service operation, if the web service is
    ///   running, but reports a previous version, then the service may need to
    ///   be restarted.
    ///
    /// # Implementors
    ///
    /// This function call is intended to be cheap and fast.
    ///
    /// # Parameters
    ///
    /// * `data`: Runtime data that the operation reads from, or writes to.
    /// * `state_current`: Current [`State`] of the managed item, returned from
    ///   [`state_current`].
    /// * `state_target`: Target [`State`] of the managed item, either
    ///   [`state_clean`] or [`state_desired`].
    /// * `state_diff`: Desired [`State`] of the managed item, returned from
    ///   [`state_diff`].
    ///
    /// [`state_clean`]: crate::ItemSpec::state_clean
    /// [`state_current`]: crate::ItemSpec::state_current
    /// [`state_desired`]: crate::ItemSpec::state_desired
    /// [`State`]: Self::State
    /// [`state_diff`]: crate::ItemSpec::state_diff
    async fn apply_check(
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<OpCheckStatus, Self::Error>;

    /// Dry-run transform of the current state to the target state.
    ///
    /// This will only be called if [`check`] returns [`ExecRequired`].
    ///
    /// This should mirror the logic in [`apply`], with the following
    /// differences:
    ///
    /// * When state will actually be altered, this would skip the logic.
    ///
    /// * Where there would be IDs received from an external system, a
    ///   placeholder ID should still be inserted into the runtime data. This
    ///   should allow subsequent `ItemSpec`s that rely on this one to use those
    ///   placeholders in their logic.
    ///
    /// # Implementors
    ///
    /// This function call is intended to be read-only and cheap.
    ///
    /// # Parameters
    ///
    /// * `data`: Runtime data that the operation reads from, or writes to.
    /// * `state_current`: Current [`State`] of the managed item, returned from
    ///   [`state_current`].
    /// * `state_target`: Target [`State`] of the managed item, either
    ///   [`state_clean`] or [`state_desired`].
    /// * `state_diff`: Desired [`State`] of the managed item, returned from
    ///   [`state_diff`].
    ///
    /// [`check`]: Self::check
    /// [`ExecRequired`]: crate::OpCheckStatus::ExecRequired
    /// [`state_clean`]: crate::ItemSpec::state_clean
    /// [`state_current`]: crate::ItemSpec::state_current
    /// [`state_desired`]: crate::ItemSpec::state_desired
    /// [`State`]: Self::State
    /// [`state_diff`]: crate::ItemSpec::state_diff
    async fn apply_dry(
        op_ctx: OpCtx<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error>;

    /// Transforms the current state to the target state.
    ///
    /// This will only be called if [`check`] returns [`ExecRequired`].
    ///
    /// # Parameters
    ///
    /// * `data`: Runtime data that the operation reads from, or writes to.
    /// * `state_current`: Current [`State`] of the managed item, returned from
    ///   [`state_current`].
    /// * `state_target`: Target [`State`] of the managed item, either
    ///   [`state_clean`] or [`state_desired`].
    /// * `state_diff`: Desired [`State`] of the managed item, returned from
    ///   [`state_diff`].
    ///
    /// [`check`]: Self::check
    /// [`ExecRequired`]: crate::OpCheckStatus::ExecRequired
    /// [`state_clean`]: crate::ItemSpec::state_clean
    /// [`state_current`]: crate::ItemSpec::state_current
    /// [`state_desired`]: crate::ItemSpec::state_desired
    /// [`State`]: Self::State
    /// [`state_diff`]: crate::ItemSpec::state_diff
    async fn apply(
        op_ctx: OpCtx<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error>;
}
