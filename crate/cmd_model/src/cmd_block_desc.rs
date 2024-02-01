/// String representation of the `CmdBlock` in a `CmdExecution`.
///
/// This is used to provide a well-formatted error message so that developers
/// can identify where a bug lies more easily.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CmdBlockDesc {
    /// Short name of the command block.
    cmd_block_name: String,
    /// Short type names of `CmdBlock::InputT`.
    ///
    /// * If `InputT` is the unit struct `()`, this should be empty.
    /// * If `InputT` is a named struct, this should contain just one `String`.
    /// * If `InputT` is a tuple, this should contain one `String` per type
    ///   within the tuple.
    cmd_block_input_names: Vec<String>,
    /// Short type names of `CmdBlock::Outcome`.
    ///
    /// * If `Outcome` is the unit struct `()`, this should be empty.
    /// * If `Outcome` is a named struct, this should contain just one `String`.
    /// * If `Outcome` is a tuple, this should contain one `String` per type
    ///   within the tuple.
    cmd_block_outcome_names: Vec<String>,
}

impl CmdBlockDesc {
    /// Returns a new `CmdBlockDesc`
    pub fn new(
        cmd_block_name: String,
        cmd_block_input_names: Vec<String>,
        cmd_block_outcome_names: Vec<String>,
    ) -> Self {
        Self {
            cmd_block_name,
            cmd_block_input_names,
            cmd_block_outcome_names,
        }
    }

    /// Returns the short name of the command block, e.g.
    /// `"StatesCurrentReadCmdBlock"`.
    pub fn cmd_block_name(&self) -> &str {
        self.cmd_block_name.as_ref()
    }

    /// Returns the short type names of `CmdBlock::InputT`, e.g.
    /// `["States<ItemIdT, Current>", "States<ItemIdT, Goal>"]`.
    ///
    /// * If `InputT` is the unit struct `()`, this should be empty.
    /// * If `InputT` is a named struct, this should contain just one `String`.
    /// * If `InputT` is a tuple, this should contain one `String` per type
    ///   within the tuple.
    pub fn cmd_block_input_names(&self) -> &[String] {
        self.cmd_block_input_names.as_ref()
    }

    /// Returns the short type names of `CmdBlock::Outcome`, e.g.
    /// `["StateDiffs"]`.
    ///
    /// * If `Outcome` is the unit struct `()`, this should be empty.
    /// * If `Outcome` is a named struct, this should contain just one `String`.
    /// * If `Outcome` is a tuple, this should contain one `String` per type
    ///   within the tuple.
    pub fn cmd_block_outcome_names(&self) -> &[String] {
        self.cmd_block_outcome_names.as_ref()
    }
}
