use fn_graph::StreamOutcome;
use peace_cmd_model::StepStreamOutcome;
use peace_rt_model::Flow;

/// Maps a `StreamOutcome<T>` to a `StepStreamOutcome<T>`.
///
/// # Design Note
///
/// This resides in the `cmd_rt` package as the `Flow` type is needed for
/// mapping, and adding `rt_model` as a dependency of `cmd_model` creates a
/// dependency cycle with `rt_model_core`:
///
/// ```text
/// cmd_model -> rt_model
///    ^         /
///   /         v
/// rt_model_core
/// ```
pub struct StepStreamOutcomeMapper;

impl StepStreamOutcomeMapper {
    /// Maps `FnId`s into `StepId`s for a better information abstraction level.
    pub fn map<T, E>(flow: &Flow<E>, stream_outcome: StreamOutcome<T>) -> StepStreamOutcome<T>
    where
        E: 'static,
    {
        let StreamOutcome {
            value,
            state,
            fn_ids_processed,
            fn_ids_not_processed,
        } = stream_outcome;

        let step_ids_processed = fn_ids_processed
            .into_iter()
            .filter_map(|fn_id| {
                flow.graph()
                    .node_weight(fn_id)
                    .map(|step| step.id())
                    .cloned()
            })
            .collect::<Vec<_>>();
        let step_ids_not_processed = fn_ids_not_processed
            .into_iter()
            .filter_map(|fn_id| {
                flow.graph()
                    .node_weight(fn_id)
                    .map(|step| step.id())
                    .cloned()
            })
            .collect::<Vec<_>>();

        StepStreamOutcome {
            value,
            state,
            step_ids_processed,
            step_ids_not_processed,
        }
    }
}
