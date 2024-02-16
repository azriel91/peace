use fn_graph::GraphInfo;
use peace_core::FlowId;

use serde::{Deserialize, Serialize};

use crate::ItemSpecInfo;

/// Serializable representation of how a [`Flow`] is configured.
///
/// [`Flow`]: https://docs.rs/peace_rt_model/latest/peace_rt_model/struct.Flow.html
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FlowSpecInfo {
    /// ID of the flow.
    pub flow_id: FlowId,
    /// Serialized representation of the flow graph.
    pub graph_info: GraphInfo<ItemSpecInfo>,
}
