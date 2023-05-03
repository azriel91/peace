use peace_cfg::ItemSpecId;
use peace_rt_model_core::IndexMap;

#[derive(Clone, Debug)]
pub struct CmdOutcome<T, E> {
    /// The outcome value.
    pub value: T,
    /// Errors from the command execution.
    pub errors: IndexMap<ItemSpecId, E>,
}

impl<T, E> CmdOutcome<T, E> {
    /// Returns whether the command ran successfully.
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns whether the command encountered any errors during execution.
    pub fn is_err(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn map<F, U>(self, f: F) -> CmdOutcome<U, E>
    where
        F: FnOnce(T) -> U,
    {
        let CmdOutcome { value: t, errors } = self;
        let u = f(t);

        CmdOutcome { value: u, errors }
    }
}
