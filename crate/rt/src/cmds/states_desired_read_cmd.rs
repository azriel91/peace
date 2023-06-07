use std::{fmt::Debug, marker::PhantomData};

use futures::FutureExt;
use peace_cfg::{FlowId, ItemId};
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    paths::{FlowDir, StatesDesiredFile},
    resources::ts::SetUp,
    states::StatesDesired,
    type_reg::untagged::{BoxDtDisplay, TypeReg},
    Resources,
};
use peace_rt_model::{params::ParamsKeys, Error, StatesSerializer, Storage};
use peace_rt_model_core::output::OutputWrite;

use crate::cmds::{cmd_ctx_internal::CmdIndependence, CmdBase};

/// Reads [`StatesDesired`]s from storage.
#[derive(Debug)]
pub struct StatesDesiredReadCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> StatesDesiredReadCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    O: OutputWrite<E>,
    PKeys: ParamsKeys + 'static,
{
    /// Reads [`StatesDesired`]s from storage.
    ///
    /// [`StatesDiscoverCmd`] must have run prior to this command to read the
    /// state.
    ///
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StatesDesired, E> {
        let SingleProfileSingleFlowView {
            states_type_reg,
            resources,
            ..
        } = cmd_ctx.scope_mut().view();

        let states_desired = Self::deserialize_internal(resources, states_type_reg).await?;

        Ok(states_desired)
    }

    /// Reads [`StatesDesired`]s from storage.
    ///
    /// See [`Self::exec`] for full documentation.
    ///
    /// This function exists so that this command can be executed as sub
    /// functionality of another command.
    pub async fn exec_with(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
    ) -> Result<StatesDesired, E> {
        CmdBase::oneshot(cmd_independence, |cmd_view| {
            async move {
                let SingleProfileSingleFlowView {
                    states_type_reg,
                    resources,
                    ..
                } = cmd_view;

                Self::deserialize_internal(resources, states_type_reg).await
            }
            .boxed_local()
        })
        .await
    }

    pub(crate) async fn deserialize_internal(
        resources: &mut Resources<SetUp>,
        states_type_reg: &TypeReg<ItemId, BoxDtDisplay>,
    ) -> Result<StatesDesired, E> {
        let flow_id = resources.borrow::<FlowId>();
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_desired_file = StatesDesiredFile::from(&*flow_dir);

        let states_desired = StatesSerializer::deserialize_desired(
            &flow_id,
            &storage,
            states_type_reg,
            &states_desired_file,
        )
        .await?;

        drop(storage);
        drop(flow_dir);
        drop(flow_id);

        resources.insert(states_desired_file);

        Ok(states_desired)
    }
}

impl<E, O, PKeys> Default for StatesDesiredReadCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
