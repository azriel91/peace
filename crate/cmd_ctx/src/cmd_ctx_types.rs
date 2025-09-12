use std::fmt::Debug;

use peace_params::{MappingFns, ParamsKey};
use peace_rt_model::output::OutputWrite;

/// Trait so that a single type parameter can be used in `CmdCtx` and `Scopes`.
///
/// The associated types linked to the concrete type can all be queried through
/// this trait.
pub trait CmdCtxTypes: Debug + Unpin + 'static {
    /// Error type of the automation software.
    type AppError: Debug
        + std::error::Error
        + From<peace_rt_model::Error>
        + From<<Self::Output as OutputWrite>::Error>
        + Send
        + Sync
        + Unpin
        + 'static;
    /// Output to write progress or outcome to.
    type Output: OutputWrite;
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

    /// Enum to give names to mapping functions, so that params specs and value
    /// specs can be serialized.
    ///
    /// Item parameters may be mapped from other items' state, and that logic
    /// exists as code. However, we want the ability to store (remember) those
    /// mappings across command executions. If a closure is held in the params
    /// specs and value specs, then they cannot be serialized. However, if we
    /// place that logic elsewhere (like in the `CmdCtxTypes` implementation),
    /// and have an intermediate enum to represent the mapping functions, we can
    /// serialize the enum instead of the closure.
    type MappingFns: MappingFns;
}
