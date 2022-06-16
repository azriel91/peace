use std::convert::Infallible;

/// Runs jobs.
#[derive(Debug)]
pub struct JobRunner;

impl JobRunner {
    /// Runs the job.
    pub fn run() -> Result<(), Infallible> {
        Ok(())
    }
}
