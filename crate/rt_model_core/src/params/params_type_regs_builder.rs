use std::{fmt::Debug, hash::Hash};

use serde::{de::DeserializeOwned, Serialize};
use type_reg::untagged::{BoxDt, TypeReg};

use crate::params::{KeyKnown, KeyMaybe, KeyUnknown, ParamsKeys, ParamsKeysImpl, ParamsTypeRegs};

/// Type registries to deserialize [`WorkspaceParamsFile`],
/// [`ProfileParamsFile`] and [`FlowParamsFile`].
///
/// [`WorkspaceParamsFile`]: peace_resource_rt::internal::WorkspaceParamsFile
/// [`ProfileParamsFile`]: peace_resource_rt::internal::ProfileParamsFile
/// [`FlowParamsFile`]: peace_resource_rt::internal::FlowParamsFile
#[derive(Debug)]
pub struct ParamsTypeRegsBuilder<PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Type registry for [`WorkspaceParams`] deserialization.
    ///
    /// [`WorkspaceParams`]: peace_rt_model::params::WorkspaceParams
    workspace_params_type_reg: TypeReg<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key, BoxDt>,
    /// Type registry for [`ProfileParams`] deserialization.
    ///
    /// [`ProfileParams`]: peace_rt_model::params::ProfileParams
    profile_params_type_reg: TypeReg<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key, BoxDt>,
    /// Type registry for [`FlowParams`] deserialization.
    ///
    /// [`FlowParams`]: peace_rt_model::params::FlowParams
    flow_params_type_reg: TypeReg<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key, BoxDt>,
}

impl ParamsTypeRegsBuilder<ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>> {
    /// Returns a new `ParamsTypeRegsBuilder`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<PKeys> ParamsTypeRegsBuilder<PKeys>
where
    PKeys: ParamsKeys + 'static,
{
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

    /// Returns a `ParamsTypeRegs` with the registered keys.
    pub fn build(
        self,
    ) -> ParamsTypeRegs<
        ParamsKeysImpl<
            PKeys::WorkspaceParamsKMaybe,
            PKeys::ProfileParamsKMaybe,
            PKeys::FlowParamsKMaybe,
        >,
    > {
        let workspace_params_type_reg =
            TypeReg::<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key, BoxDt>::new();
        let profile_params_type_reg =
            TypeReg::<<PKeys::ProfileParamsKMaybe as KeyMaybe>::Key, BoxDt>::new();
        let flow_params_type_reg =
            TypeReg::<<PKeys::FlowParamsKMaybe as KeyMaybe>::Key, BoxDt>::new();

        ParamsTypeRegs::new(
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
        )
    }
}

impl<ProfileParamsKMaybe, FlowParamsKMaybe>
    ParamsTypeRegsBuilder<ParamsKeysImpl<KeyUnknown, ProfileParamsKMaybe, FlowParamsKMaybe>>
where
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    pub fn with_workspace_params_k<WorkspaceParamsK>(
        self,
    ) -> ParamsTypeRegsBuilder<
        ParamsKeysImpl<KeyKnown<WorkspaceParamsK>, ProfileParamsKMaybe, FlowParamsKMaybe>,
    >
    where
        WorkspaceParamsK: Clone
            + Debug
            + Eq
            + Hash
            + DeserializeOwned
            + Serialize
            + Send
            + Sync
            + Unpin
            + 'static,
    {
        let ParamsTypeRegsBuilder {
            workspace_params_type_reg: _,
            profile_params_type_reg,
            flow_params_type_reg,
        } = self;

        let workspace_params_type_reg = TypeReg::<WorkspaceParamsK, BoxDt>::new();

        ParamsTypeRegsBuilder {
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
        }
    }
}

impl<WorkspaceParamsKMaybe, FlowParamsKMaybe>
    ParamsTypeRegsBuilder<ParamsKeysImpl<WorkspaceParamsKMaybe, KeyUnknown, FlowParamsKMaybe>>
where
    WorkspaceParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    pub fn with_profile_params_k<ProfileParamsK>(
        self,
    ) -> ParamsTypeRegsBuilder<
        ParamsKeysImpl<WorkspaceParamsKMaybe, KeyKnown<ProfileParamsK>, FlowParamsKMaybe>,
    >
    where
        ProfileParamsK: Clone
            + Debug
            + Eq
            + Hash
            + DeserializeOwned
            + Serialize
            + Send
            + Sync
            + Unpin
            + 'static,
    {
        let ParamsTypeRegsBuilder {
            workspace_params_type_reg,
            profile_params_type_reg: _,
            flow_params_type_reg,
        } = self;

        let profile_params_type_reg = TypeReg::<ProfileParamsK, BoxDt>::new();

        ParamsTypeRegsBuilder {
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
        }
    }
}

impl<WorkspaceParamsKMaybe, ProfileParamsKMaybe>
    ParamsTypeRegsBuilder<ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, KeyUnknown>>
where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
{
    pub fn with_flow_params_k<FlowParamsK>(
        self,
    ) -> ParamsTypeRegsBuilder<
        ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, KeyKnown<FlowParamsK>>,
    >
    where
        FlowParamsK: Clone
            + Debug
            + Eq
            + Hash
            + DeserializeOwned
            + Serialize
            + Send
            + Sync
            + Unpin
            + 'static,
    {
        let ParamsTypeRegsBuilder {
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg: _,
        } = self;

        let flow_params_type_reg = TypeReg::<FlowParamsK, BoxDt>::new();

        ParamsTypeRegsBuilder {
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
        }
    }
}

impl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsKMaybe> Default
    for ParamsTypeRegsBuilder<
        ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsKMaybe>,
    >
where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    fn default() -> Self {
        let workspace_params_type_reg = TypeReg::<WorkspaceParamsKMaybe::Key, BoxDt>::new();
        let profile_params_type_reg = TypeReg::<ProfileParamsKMaybe::Key, BoxDt>::new();
        let flow_params_type_reg = TypeReg::<FlowParamsKMaybe::Key, BoxDt>::new();

        ParamsTypeRegsBuilder {
            workspace_params_type_reg,
            profile_params_type_reg,
            flow_params_type_reg,
        }
    }
}
