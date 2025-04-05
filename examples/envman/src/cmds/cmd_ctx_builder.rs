#[rustfmt::skip] // https://github.com/rust-lang/rustfmt/issues/4609
#[macro_export]
macro_rules! interruptibility_augment {
    ($cmd_ctx_builder:ident) => {
        let (interrupt_tx, interrupt_rx) = tokio::sync::mpsc::channel::<
            peace::cmd_ctx::interruptible::InterruptSignal
        >(16);
        tokio::spawn(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to listen for `SIGINT`.");
            let _ = interrupt_tx.send(peace::cmd_ctx::interruptible::InterruptSignal).await;
        });

        let $cmd_ctx_builder = $cmd_ctx_builder
            .with_interruptibility(peace::cmd_ctx::interruptible::Interruptibility::new(
                interrupt_rx.into(),
                peace::cmd_ctx::interruptible::InterruptStrategy::FinishCurrent,
            ));
    };
}

pub use interruptibility_augment;
