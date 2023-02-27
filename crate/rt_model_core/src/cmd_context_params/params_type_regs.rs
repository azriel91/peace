use std::fmt::Debug;

use type_reg::untagged::{BoxDt, TypeReg};

use crate::cmd_context_params::{
    KeyMaybe, KeyUnknown, ParamsKeys, ParamsKeysImpl, ParamsTypeRegsBuilder,
};

/// Type registries to deserialize [`WorkspaceParamsFile`],
/// [`ProfileParamsFile`] and [`FlowParamsFile`].
///
/// [`WorkspaceParamsFile`]: peace_resources::internal::WorkspaceParamsFile
/// [`ProfileParamsFile`]: peace_resources::internal::ProfileParamsFile
/// [`FlowParamsFile`]: peace_resources::internal::FlowParamsFile
#[derive(Debug)]
pub struct ParamsTypeRegs<PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Type registry for [`WorkspaceParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: crate::cmd_context_params::WorkspaceParams
    workspace_params_type_reg: TypeReg<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key, BoxDt>,
    /// Type registry for [`ProfileParams`] deserialization.
    ///
    /// [`ProfileParams`]: crate::cmd_context_params::ProfileParams
    profile_params_type_reg: TypeReg<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key, BoxDt>,
    /// Type registry for [`FlowParams`] deserialization.
    ///
    /// [`FlowParams`]: crate::cmd_context_params::FlowParams
    flow_params_type_reg: TypeReg<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key, BoxDt>,
}

impl ParamsTypeRegs<ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>> {
    /// Returns a new `ParamsTypeRegsBuilder`.
    pub fn builder() -> ParamsTypeRegsBuilder<ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>> {
        ParamsTypeRegsBuilder::new()
    }
}

impl<PKeys> ParamsTypeRegs<PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Returns a new `ParamsTypeRegs`.
    pub(crate) fn new(
        workspace_params_type_reg: TypeReg<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key, BoxDt>,
        profile_params_type_reg: TypeReg<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key, BoxDt>,
        flow_params_type_reg: TypeReg<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key, BoxDt>,
    ) -> Self {
        Self {
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
        }
    }

    /// Returns a reference to the workspace params type registry.
    pub fn workspace_params_type_reg(
        &self,
    ) -> &TypeReg<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key, BoxDt> {
        &self.workspace_params_type_reg
    }

    /// Returns a mutable reference to the workspace params type registry.
    pub fn workspace_params_type_reg_mut(
        &mut self,
    ) -> &mut TypeReg<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key, BoxDt> {
        &mut self.workspace_params_type_reg
    }

    /// Returns a reference to the profile params type registry.
    pub fn profile_params_type_reg(
        &self,
    ) -> &TypeReg<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key, BoxDt> {
        &self.profile_params_type_reg
    }

    /// Returns a mutable reference to the profile params type registry.
    pub fn profile_params_type_reg_mut(
        &mut self,
    ) -> &mut TypeReg<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key, BoxDt> {
        &mut self.profile_params_type_reg
    }

    /// Returns a reference to the flow params type registry.
    pub fn flow_params_type_reg(
        &self,
    ) -> &TypeReg<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key, BoxDt> {
        &self.flow_params_type_reg
    }

    /// Returns a mutable reference to the flow params type registry.
    pub fn flow_params_type_reg_mut(
        &mut self,
    ) -> &mut TypeReg<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key, BoxDt> {
        &mut self.flow_params_type_reg
    }
}

impl<PKeys> Default for ParamsTypeRegs<PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    fn default() -> Self {
        Self {
            workspace_params_type_reg: TypeReg::default(),
            profile_params_type_reg: TypeReg::default(),
            flow_params_type_reg: TypeReg::default(),
        }
    }
}
