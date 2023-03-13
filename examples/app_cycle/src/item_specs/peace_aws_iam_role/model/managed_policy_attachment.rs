use serde::{Deserialize, Serialize};

/// Managed policy to attach to the role.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ManagedPolicyAttachment {
    /// ARN of the managed policy to attach to the role.
    arn: String,
    /// Whether the policy has been attached to the role.
    attached: bool,
}

impl ManagedPolicyAttachment {
    pub fn new(arn: String, attached: bool) -> Self {
        Self { arn, attached }
    }

    pub fn arn(&self) -> &str {
        self.arn.as_ref()
    }

    pub fn attached(&self) -> bool {
        self.attached
    }
}
