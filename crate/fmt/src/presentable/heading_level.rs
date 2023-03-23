use serde::{Deserialize, Serialize};

/// Level of conceptual detail for a given topic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum HeadingLevel {
    /// Top level heading.
    Level1,
    /// Second level heading.
    Level2,
    /// Third level heading.
    Level3,
    /// Fourth level heading.
    Level4,
    /// Fifth level heading.
    Level5,
    /// Lowest level of conceptual detail.
    Level6,
}
