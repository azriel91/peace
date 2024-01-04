use std::{fmt::Debug, marker::PhantomData};

use futures::FutureExt;
use peace_cmd::scopes::SingleProfileSingleFlowView;
use peace_cmd_model::CmdBlockOutcome;
use peace_cmd_rt::{async_trait, CmdBlock};
use peace_resources::{
    internal::StatesMut,
    resources::ts::SetUp,
    states::{ts::Clean, StatesClean},
    ResourceFetchError, Resources,
};
use peace_rt_model::{fn_graph::StreamOpts, params::ParamsKeys, Error};
use peace_rt_model_core::IndexMap;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::progress::CmdProgressUpdate;
        use tokio::sync::mpsc::Sender;
    }
}

/// Inserts [`StatesClean`]s for each item.
///
/// This calls [`Item::state_clean`] for each item, and groups them together
/// into `StatesClean`.
#[derive(Debug)]
pub struct StatesCleanInsertionCmdBlock<E, PKeys>(PhantomData<(E, PKeys)>);

impl<E, PKeys> StatesCleanInsertionCmdBlock<E, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Returns a new `StatesCleanInsertionCmdBlock`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<E, PKeys> Default for StatesCleanInsertionCmdBlock<E, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[async_trait(?Send)]
impl<E, PKeys> CmdBlock for StatesCleanInsertionCmdBlock<E, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    type Error = E;
    type InputT = ();
    type Outcome = StatesClean;
    type PKeys = PKeys;

    fn input_fetch(&self, _resources: &mut Resources<SetUp>) -> Result<(), ResourceFetchError> {
        Ok(())
    }

    fn input_type_names(&self) -> Vec<String> {
        vec![]
    }

    async fn exec(
        &self,
        _input: Self::InputT,
        cmd_view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        #[cfg(feature = "output_progress")] _progress_tx: &Sender<CmdProgressUpdate>,
    ) -> Result<CmdBlockOutcome<Self::Outcome, Self::Error>, Self::Error> {
        let SingleProfileSingleFlowView {
            interruptibility_state,
            flow,
            params_specs,
            resources,
            ..
        } = cmd_view;

        let params_specs = &*params_specs;
        let resources = &*resources;
        let (stream_outcome, errors) = flow
            .graph()
            .fold_async_with(
                (StatesMut::<Clean>::new(), IndexMap::new()),
                StreamOpts::new()
                    .interruptibility_state(interruptibility_state.reborrow())
                    .interrupted_next_item_include(false),
                |(mut states_clean_mut, mut errors), item_rt| {
                    async move {
                        let item_id = item_rt.id().clone();
                        let state_clean_boxed_result =
                            item_rt.state_clean(params_specs, resources).await;

                        match state_clean_boxed_result {
                            Ok(state_clean_boxed) => {
                                states_clean_mut.insert_raw(item_id, state_clean_boxed);
                            }
                            Err(error) => {
                                errors.insert(item_id, error);
                            }
                        }

                        (states_clean_mut, errors)
                    }
                    .boxed_local()
                },
            )
            .await
            .replace_with(std::convert::identity);

        let stream_outcome = stream_outcome.map(StatesClean::from);

        Ok(CmdBlockOutcome::ItemWise {
            stream_outcome,
            errors,
        })
    }
}
