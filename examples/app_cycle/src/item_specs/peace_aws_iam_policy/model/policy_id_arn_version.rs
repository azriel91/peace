use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PolicyIdArnVersion {
    /// The stable and unique string identifying the policy. For more
    /// information about IDs, see [IAM identifiers] in the *IAM User
    /// Guide*.
    ///
    /// [IAM identifiers]: https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html
    id: String,
    /// The Amazon Resource Name (ARN) specifying the policy. For more
    /// information about ARNs and how to use them in policies, see [IAM
    /// identifiers] in the *IAM User Guide*.
    ///
    /// [IAM identifiers]: https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html
    arn: String,
    /// Policy version.
    version: String,
}

impl PolicyIdArnVersion {
    pub fn new(id: String, arn: String, version: String) -> Self {
        Self { id, arn, version }
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn arn(&self) -> &str {
        self.arn.as_ref()
    }

    pub fn version(&self) -> &str {
        self.version.as_ref()
    }
}
