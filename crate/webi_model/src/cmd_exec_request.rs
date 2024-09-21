use serde::{Deserialize, Serialize};

/// Request for a command execution.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CmdExecRequest {
    // TODO: which command
}
