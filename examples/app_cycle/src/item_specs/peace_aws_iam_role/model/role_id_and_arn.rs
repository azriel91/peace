use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct RoleIdAndArn {
    /// The stable and unique string identifying the role. For more information
    /// about IDs, see [IAM identifiers] in the *IAM User Guide*.
    ///
    /// [IAM identifiers]: https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html
    id: String,
    /// The Amazon Resource Name (ARN) specifying the role. For more information
    /// about ARNs and how to use them in policies, see [IAM identifiers] in the
    /// *IAM User Guide*.
    ///
    /// [IAM identifiers]: https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html
    arn: String,
}

impl RoleIdAndArn {
    pub fn new(id: String, arn: String) -> Self {
        Self { id, arn }
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn arn(&self) -> &str {
        self.arn.as_ref()
    }
}
