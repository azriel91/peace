use std::fmt::Debug;

use peace_params::ParamsKey;
use peace_rt_model::output::OutputWrite;
use type_reg::untagged::TypeReg;

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

    /// Specifies the data types of the values stored against each key in the
    /// workspace parameters.
    ///
    /// This corresponds to registering types for [deserialization] in an
    /// untagged type registry.
    ///
    /// [deserialization]: https://docs.rs/type_reg/latest/type_reg/#deserialization
    ///
    /// # Examples
    ///
    /// ## `String` as the key type
    ///
    /// ```rust,ignore
    /// use peace_cmd_ctx::type_reg::untagged::TypeReg;
    ///
    /// #[derive(Debug)]
    /// pub struct MyCmdCtxTypes;
    ///
    /// impl CmdCtxTypes for MyCmdCtxTypes {
    ///     // ..
    ///     type WorkspaceParamsKey = String;
    ///
    ///     fn workspace_params_register(type_reg: &mut TypeReg<Self::WorkspaceParamsKey>) {
    ///         type_reg.register::<u32>(String::from("one"));
    ///         type_reg.register::<u64>(String::from("two"));
    ///         type_reg.register::<A>(String::from("three"));
    ///     }
    /// }
    /// ```
    ///
    /// ## Enum as the key type
    ///
    /// ```rust,ignore
    /// use serde::{Serialize, Deserialize};
    /// use peace_cmd_ctx::type_reg::untagged::TypeReg;
    ///
    /// #[derive(Debug)]
    /// pub struct MyCmdCtxTypes;
    ///
    /// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    /// pub enum MyWorkspaceParamsKey {
    ///     One,
    ///     Two,
    ///     Three,
    /// }
    ///
    /// impl CmdCtxTypes for MyCmdCtxTypes {
    ///     // ..
    ///     type WorkspaceParamsKey = MyWorkspaceParamsKey;
    ///
    ///     fn workspace_params_register(type_reg: &mut TypeReg<Self::WorkspaceParamsKey>) {
    ///         type_reg.register::<u32>(MyWorkspaceParamsKey::One);
    ///         type_reg.register::<u64>(MyWorkspaceParamsKey::Two);
    ///         type_reg.register::<A>(MyWorkspaceParamsKey::Three);
    ///     }
    /// }
    /// ```
    fn workspace_params_register(type_reg: &mut TypeReg<Self::WorkspaceParamsKey>);

    /// Specifies the data types of the values stored against each key in the
    /// profile parameters.
    ///
    /// This corresponds to registering types for [deserialization] in an
    /// untagged type registry.
    ///
    /// [deserialization]: https://docs.rs/type_reg/latest/type_reg/#deserialization
    ///
    /// # Examples
    ///
    /// ## `String` as the key type
    ///
    /// ```rust,ignore
    /// use peace_cmd_ctx::type_reg::untagged::TypeReg;
    ///
    /// #[derive(Debug)]
    /// pub struct MyCmdCtxTypes;
    ///
    /// impl CmdCtxTypes for MyCmdCtxTypes {
    ///     // ..
    ///     type ProfileParamsKey = String;
    ///
    ///     fn profile_params_register(type_reg: &mut TypeReg<Self::ProfileParamsKey>) {
    ///         type_reg.register::<u32>(String::from("one"));
    ///         type_reg.register::<u64>(String::from("two"));
    ///         type_reg.register::<A>(String::from("three"));
    ///     }
    /// }
    /// ```
    ///
    /// ## Enum as the key type
    ///
    /// ```rust,ignore
    /// use serde::{Serialize, Deserialize};
    /// use peace_cmd_ctx::type_reg::untagged::TypeReg;
    ///
    /// #[derive(Debug)]
    /// pub struct MyCmdCtxTypes;
    ///
    /// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    /// pub enum MyProfileParamsKey {
    ///     One,
    ///     Two,
    ///     Three,
    /// }
    ///
    /// impl CmdCtxTypes for MyCmdCtxTypes {
    ///     // ..
    ///     type ProfileParamsKey = MyProfileParamsKey;
    ///
    ///     fn profile_params_register(type_reg: &mut TypeReg<Self::ProfileParamsKey>) {
    ///         type_reg.register::<u32>(MyProfileParamsKey::One);
    ///         type_reg.register::<u64>(MyProfileParamsKey::Two);
    ///         type_reg.register::<A>(MyProfileParamsKey::Three);
    ///     }
    /// }
    /// ```
    fn profile_params_register(type_reg: &mut TypeReg<Self::ProfileParamsKey>);

    /// Specifies the data types of the values stored against each key in the
    /// flow parameters.
    ///
    /// This corresponds to registering types for [deserialization] in an
    /// untagged type registry.
    ///
    /// [deserialization]: https://docs.rs/type_reg/latest/type_reg/#deserialization
    ///
    /// # Examples
    ///
    /// ## `String` as the key type
    ///
    /// ```rust,ignore
    /// use peace_cmd_ctx::type_reg::untagged::TypeReg;
    ///
    /// #[derive(Debug)]
    /// pub struct MyCmdCtxTypes;
    ///
    /// impl CmdCtxTypes for MyCmdCtxTypes {
    ///     // ..
    ///     type FlowParamsKey = String;
    ///
    ///     fn flow_params_register(type_reg: &mut TypeReg<Self::FlowParamsKey>) {
    ///         type_reg.register::<u32>(String::from("one"));
    ///         type_reg.register::<u64>(String::from("two"));
    ///         type_reg.register::<A>(String::from("three"));
    ///     }
    /// }
    /// ```
    ///
    /// ## Enum as the key type
    ///
    /// ```rust,ignore
    /// use serde::{Serialize, Deserialize};
    /// use peace_cmd_ctx::type_reg::untagged::TypeReg;
    ///
    /// #[derive(Debug)]
    /// pub struct MyCmdCtxTypes;
    ///
    /// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    /// pub enum MyFlowParamsKey {
    ///     One,
    ///     Two,
    ///     Three,
    /// }
    ///
    /// impl CmdCtxTypes for MyCmdCtxTypes {
    ///     // ..
    ///     type FlowParamsKey = MyFlowParamsKey;
    ///
    ///     fn flow_params_register(type_reg: &mut TypeReg<Self::FlowParamsKey>) {
    ///         type_reg.register::<u32>(MyFlowParamsKey::One);
    ///         type_reg.register::<u64>(MyFlowParamsKey::Two);
    ///         type_reg.register::<A>(MyFlowParamsKey::Three);
    ///     }
    /// }
    /// ```
    fn flow_params_register(type_reg: &mut TypeReg<Self::FlowParamsKey>);
}
