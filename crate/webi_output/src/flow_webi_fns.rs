use std::fmt::{self, Debug};

use dot_ix_model::info_graph::InfoGraph;
use futures::future::LocalBoxFuture;
use peace_params::ParamsSpecs;
use peace_resource_rt::{resources::ts::SetUp, Resources};
use peace_rt_model::Flow;

use crate::{CmdExecSpawnCtx, WebiOutput};

/// Functions to work with `Flow` from the [`WebiOutput`].
///
/// [`WebiOutput`]: crate::WebiOutput
pub struct FlowWebiFns<E> {
    /// Flow to work with.
    pub flow: Flow<E>,
    /// Function to create an `InfoGraph`.
    ///
    /// # Design
    ///
    /// This circumvents the need to pass around the specific `CmdCtx` type by
    /// getting the tool developer to instantiate the `CmdCtx`, then pass the
    /// relevant parameters to the function that we pass in.
    pub outcome_info_graph_fn: Box<
        dyn Fn(
            &mut WebiOutput,
            fn(&Flow<E>, &ParamsSpecs, &Resources<SetUp>) -> InfoGraph,
        ) -> LocalBoxFuture<InfoGraph>,
    >,
    /// Function to spawn a `CmdExecution`.
    ///
    /// # Design
    ///
    /// Because passing around a `CmdCtx` with all its type parameters is
    /// technically high cost, all of the `CmdCtx` instantiation logic, and
    /// `*Cmd::run` invocations are hidden behind a plain function interface.
    ///
    /// Currently we only take in one function. In the future this should take
    /// in a `Map<CmdExecutionRequest, CmdExecutionSpawnFn>`
    pub cmd_exec_spawn_fn: Box<dyn Fn(WebiOutput) -> CmdExecSpawnCtx>,
}

impl<E> fmt::Debug for FlowWebiFns<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlowWebiFns")
            .field("flow", &self.flow)
            .field(
                "outcome_info_graph_fn",
                &stringify!(
                    Box<
                        dyn Fn(
                            &mut WebiOutput,
                            fn(&Flow<E>, &ParamsSpecs, &Resources<SetUp>) -> InfoGraph,
                        ) -> LocalBoxFuture<InfoGraph>,
                    >
                ),
            )
            .field(
                "cmd_exec_spawn_fn",
                &stringify!(Box<dyn Fn(WebiOutput) -> CmdExecSpawnCtx>),
            )
            .finish()
    }
}
