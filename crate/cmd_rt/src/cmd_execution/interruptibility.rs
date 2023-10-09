use std::fmt::Debug;

use interruptible::InterruptSignal;
use tokio::sync::mpsc;

/// Marks a `CmdExecution` as interruptible.
#[derive(Debug)]
pub struct Interruptible<'rx>(pub(crate) &'rx mut mpsc::Receiver<InterruptSignal>);

/// Marks a `CmdExecution` as not interruptible.
#[derive(Debug)]
pub struct NonInterruptible;

/// Interruptible type states.
pub trait InterruptibleT<'rx>: Debug + Send + Sync + 'rx {}

impl<'rx> InterruptibleT<'rx> for Interruptible<'rx> {}
impl<'rx> InterruptibleT<'rx> for NonInterruptible {}
