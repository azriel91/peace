use peace::{
    cfg::{
        item_spec_id,
        progress::{ProgressDelta, ProgressSender, ProgressUpdateAndId},
        ItemSpecId,
    },
    rt_model::ProgressUpdate,
};
use tokio::sync::mpsc::{self, error::TryRecvError};

#[test]
fn inc_sends_progress_update() -> Result<(), Box<dyn std::error::Error>> {
    let item_spec_id = item_spec_id!("test_item_spec_id");
    let (progress_tx, mut progress_rx) = mpsc::channel(10);
    let progress_sender = ProgressSender::new(&item_spec_id, &progress_tx);

    progress_sender.inc(123);

    let progress_update_and_id = progress_rx.try_recv().unwrap();

    assert_eq!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(123))
        },
        progress_update_and_id
    );
    let error = progress_rx.try_recv().unwrap_err();
    assert_eq!(TryRecvError::Empty, error);
    Ok(())
}

#[test]
fn inc_is_received_if_sent_before_progress_channel_is_closed()
-> Result<(), Box<dyn std::error::Error>> {
    let item_spec_id = item_spec_id!("test_item_spec_id");
    let (progress_tx, mut progress_rx) = mpsc::channel(10);
    let progress_sender = ProgressSender::new(&item_spec_id, &progress_tx);

    progress_sender.inc(123);
    progress_rx.close();

    let progress_update_and_id = progress_rx.try_recv().unwrap();

    assert_eq!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(123))
        },
        progress_update_and_id
    );
    let error = progress_rx.try_recv().unwrap_err();
    assert_eq!(TryRecvError::Empty, error);
    Ok(())
}

#[test]
fn inc_does_not_panic_when_progress_channel_is_closed() -> Result<(), Box<dyn std::error::Error>> {
    let item_spec_id = item_spec_id!("test_item_spec_id");
    let (progress_tx, mut progress_rx) = mpsc::channel(10);
    let progress_sender = ProgressSender::new(&item_spec_id, &progress_tx);

    progress_rx.close();
    progress_sender.inc(123);

    let error = progress_rx.try_recv().unwrap_err();
    assert_eq!(TryRecvError::Empty, error);
    Ok(())
}

#[test]
fn tick_sends_progress_update() -> Result<(), Box<dyn std::error::Error>> {
    let item_spec_id = item_spec_id!("test_item_spec_id");
    let (progress_tx, mut progress_rx) = mpsc::channel(10);
    let progress_sender = ProgressSender::new(&item_spec_id, &progress_tx);

    progress_sender.tick();

    let progress_update_and_id = progress_rx.try_recv().unwrap();

    assert_eq!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Tick)
        },
        progress_update_and_id
    );
    let error = progress_rx.try_recv().unwrap_err();
    assert_eq!(TryRecvError::Empty, error);
    Ok(())
}

#[test]
fn tick_is_received_if_sent_before_progress_channel_is_closed()
-> Result<(), Box<dyn std::error::Error>> {
    let item_spec_id = item_spec_id!("test_item_spec_id");
    let (progress_tx, mut progress_rx) = mpsc::channel(10);
    let progress_sender = ProgressSender::new(&item_spec_id, &progress_tx);

    progress_sender.tick();
    progress_rx.close();

    let progress_update_and_id = progress_rx.try_recv().unwrap();

    assert_eq!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Tick)
        },
        progress_update_and_id
    );
    let error = progress_rx.try_recv().unwrap_err();
    assert_eq!(TryRecvError::Empty, error);
    Ok(())
}

#[test]
fn tick_does_not_panic_when_progress_channel_is_closed() -> Result<(), Box<dyn std::error::Error>> {
    let item_spec_id = item_spec_id!("test_item_spec_id");
    let (progress_tx, mut progress_rx) = mpsc::channel(10);
    let progress_sender = ProgressSender::new(&item_spec_id, &progress_tx);

    progress_rx.close();
    progress_sender.tick();

    let error = progress_rx.try_recv().unwrap_err();
    assert_eq!(TryRecvError::Empty, error);
    Ok(())
}

#[test]
fn debug() {
    let item_spec_id = item_spec_id!("test_item_spec_id");
    let (progress_tx, _progress_rx) = mpsc::channel(10);
    let progress_sender = ProgressSender::new(&item_spec_id, &progress_tx);

    assert!(format!("{progress_sender:?}").starts_with(
        r#"ProgressSender { item_spec_id: ItemSpecId("test_item_spec_id"), progress_tx: Sender"#
    ));
}
