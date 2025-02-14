use std::fmt::Debug;

use peace_params::ParamsKey;

/// Trait so that a single type parameter can be used in `CmdCtx` and `Scopes`.
///
/// The associated types linked to the concrete type can all be queried through
/// this trait.
pub trait CmdCtxTypes {
    /// Error type of the automation software.
    type AppError: Debug;
    /// Output to write progress or outcome to.
    type Output;
    /// Key type for parameters that are common for the workspace.
    ///
    /// If this is not needed, you may use the [`!` never type][never_type].
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
    /// pub enum WorkspaceParam {
    ///     UserEmail,
    ///     Profile,
    /// }
    ///
    /// impl CmdCtxTypes for MyCmdCtxTypes {
    ///     // ..
    ///     type WorkspaceParamsKey = WorkspaceParam;
    /// }
    /// ```
    ///
    /// [never_type]: https://doc.rust-lang.org/std/primitive.never.html
    type WorkspaceParamsKey: ParamsKey;
    /// Key type for parameters that differ between profiles.
    ///
    /// If this is not needed, you may use the [`!` never type][never_type].
    ///
    /// # Examples
    ///
    /// Store an instance type that will be used as a parameter to an item that
    /// launches a virtual machine.
    ///
    /// ```rust,ignore
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
    /// pub enum ProfileParam {
    ///     /// Default instance type to use across all flows within the same profile.
    ///     InstanceType,
    /// }
    ///
    /// impl CmdCtxTypes for MyCmdCtxTypes {
    ///     // ..
    ///     type ProfileParamsKey = ProfileParam;
    /// }
    /// ```
    ///
    /// [never_type]: https://doc.rust-lang.org/std/primitive.never.html
    type ProfileParamsKey: ParamsKey;
    /// Key type for parameters that differ between flows.
    ///
    /// If this is not needed, you may use the [`!` never type][never_type].
    ///
    /// # Examples
    ///
    /// Store an instance type that will be used as a parameter to an item that
    /// launches a virtual machine.
    ///
    /// ```rust,ignore
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
    /// pub enum FlowParam {
    ///     /// Instance type to use within this flow.
    ///     ///
    ///     /// Overrides `ProfileParam::InstanceType` if set.
    ///     InstanceType,
    /// }
    ///
    /// impl CmdCtxTypes for MyCmdCtxTypes {
    ///     // ..
    ///     type FlowParamsKey = FlowParam;
    /// }
    /// ```
    ///
    /// [never_type]: https://doc.rust-lang.org/std/primitive.never.html
    type FlowParamsKey: ParamsKey;
}
