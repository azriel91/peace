use std::{fmt::Debug, hash::Hash};

use enum_iterator::Sequence;
use peace_resource_rt::type_reg::untagged::TypeReg;
use serde::{de::DeserializeOwned, Serialize};

/// Marker trait for a parameter key type.
///
/// This trait is automatically implemented for types that are `Clone + Debug +
/// Eq + Hash + Deserialize + Serialize`.
///
/// # Examples
///
/// ```rust,ignore
/// use peace::{
///     cmd_ctx::type_reg::untagged::TypeReg, enum_iterator::Sequence, params::ParamsKey,
///     profile_model::Profile,
/// };
/// use serde::{Deserialize, Serialize};
///
/// /// Keys for workspace parameters.
/// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Sequence)]
/// #[enum_iterator(crate = peace::enum_iterator)]
/// #[serde(rename_all = "snake_case")]
/// pub enum WorkspaceParamsKey {
///     /// Default profile to use.
///     Profile,
///     /// Which flow this workspace is using.
///     Flow,
/// }
///
/// impl ParamsKey for WorkspaceParamsKey {
///     fn register_value_type(self, type_reg: &mut TypeReg<Self>) {
///         match self {
///             Self::Profile => type_reg.register::<Profile>(self),
///             Self::Flow => type_reg.register::<EnvManFlow>(self),
///         }
///     }
/// }
///
/// impl CmdCtxTypes for MyCmdCtxTypes {
///     // ..
///     type WorkspaceParamsKey = WorkspaceParam;
/// }
/// ```
pub trait ParamsKey:
    Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Sequence + Send + Sync + 'static
{
    /// Registers the type of the value stored against this params key.
    ///
    /// This informs the type registry how to deserialize the value when
    /// encountering this key.
    fn register_value_type(self, type_reg: &mut TypeReg<Self>);
}

impl ParamsKey for () {
    fn register_value_type(self, _type_reg: &mut TypeReg<Self>) {}
}
