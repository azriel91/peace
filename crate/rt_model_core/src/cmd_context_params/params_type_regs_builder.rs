use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use serde::{de::DeserializeOwned, Serialize};
use type_reg::untagged::{BoxDt, TypeReg};

use crate::cmd_context_params::{
    KeyKnown, KeyMaybe, KeyUnknown, ParamsKeys, ParamsKeysImpl, ParamsTypeRegs,
};

/// Type registries to deserialize [`WorkspaceParamsFile`],
/// [`ProfileParamsFile`] and [`FlowParamsFile`].
///
/// [`WorkspaceParamsFile`]: peace_resources::internal::WorkspaceParamsFile
/// [`ProfileParamsFile`]: peace_resources::internal::ProfileParamsFile
/// [`FlowParamsFile`]: peace_resources::internal::FlowParamsFile
#[derive(Debug)]
pub struct ParamsTypeRegsBuilder<PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Marker.
    marker: PhantomData<PKeys>,
}

impl ParamsTypeRegsBuilder<ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>> {
    /// Returns a new `ParamsTypeRegsBuilder`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsKMaybe>
    ParamsTypeRegsBuilder<
        ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsKMaybe>,
    >
where
    WorkspaceParamsKMaybe: KeyMaybe,
    ProfileParamsKMaybe: KeyMaybe,
    FlowParamsKMaybe: KeyMaybe,
{
    /// Returns a `ParamsTypeRegs` with the registered keys.
    pub fn build(
        self,
    ) -> ParamsTypeRegs<ParamsKeysImpl<WorkspaceParamsKMaybe, ProfileParamsKMaybe, FlowParamsKMaybe>>
    {
        let workspace_params_type_reg = TypeReg::<WorkspaceParamsKMaybe::Key, BoxDt>::new();
        let profile_params_type_reg = TypeReg::<ProfileParamsKMaybe::Key, BoxDt>::new();
        let flow_params_type_reg = TypeReg::<FlowParamsKMaybe::Key, BoxDt>::new();

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
        WorkspaceParamsK:
            Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        ParamsTypeRegsBuilder {
            marker: PhantomData,
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
        ProfileParamsK:
            Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        ParamsTypeRegsBuilder {
            marker: PhantomData,
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
        FlowParamsK:
            Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        ParamsTypeRegsBuilder {
            marker: PhantomData,
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
        ParamsTypeRegsBuilder {
            marker: PhantomData,
        }
    }
}
