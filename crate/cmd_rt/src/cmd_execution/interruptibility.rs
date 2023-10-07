use std::fmt::Debug;

use interruptible::InterruptSignal;
use tokio::sync::oneshot;

/// Marks a `CmdExecution` as interruptible.
#[derive(Debug)]
pub struct Interruptible(pub(crate) oneshot::Receiver<InterruptSignal>);

/// Marks a `CmdExecution` as not interruptible.
#[derive(Debug)]
pub struct NonInterruptible;

/// Interruptible type states.
pub trait InterruptibleT: Debug + Send + Sync + 'static {}

impl InterruptibleT for Interruptible {}
impl InterruptibleT for NonInterruptible {}
