use peace::cfg::{
    item_spec_id,
    progress::{
        ProgressComplete, ProgressDelta, ProgressLimit, ProgressMsgUpdate, ProgressUpdate,
        ProgressUpdateAndId,
    },
    ItemSpecId,
};

#[test]
fn clone() {
    let progress_update_and_id_0 = ProgressUpdateAndId {
        item_spec_id: item_spec_id!("test_item_spec_id"),
        progress_update: ProgressUpdate::Delta(ProgressDelta::Tick),
        msg_update: ProgressMsgUpdate::NoChange,
    };
    #[allow(clippy::redundant_clone)] // https://github.com/rust-lang/rust-clippy/issues/9011
    let progress_update_and_id_1 = progress_update_and_id_0.clone();

    assert_eq!(progress_update_and_id_0, progress_update_and_id_1);
}

#[test]
fn deserialize() {
    assert_eq!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
            msg_update: ProgressMsgUpdate::NoChange,
        },
        serde_yaml::from_str(
            r#"item_spec_id: test_item_spec_id
progress_update: !Delta
  Inc: 3
msg_update: NoChange
"#
        )
        .unwrap()
    )
}

#[test]
fn serialize() {
    assert_eq!(
        r#"item_spec_id: test_item_spec_id
progress_update: !Delta
  Inc: 3
msg_update: NoChange
"#,
        serde_yaml::to_string(&ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
            msg_update: ProgressMsgUpdate::NoChange,
        })
        .unwrap()
    )
}

#[test]
#[cfg(feature = "output_json")]
fn deserialize_json() {
    assert_eq!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
            msg_update: ProgressMsgUpdate::NoChange,
        },
        serde_json::from_str(
            r#"{"item_spec_id":"test_item_spec_id","progress_update":{"Delta":{"Inc":3}},"msg_update":"NoChange"}"#
        )
        .unwrap()
    )
}

#[test]
#[cfg(feature = "output_json")]
fn serialize_json() {
    assert_eq!(
        r#"{"item_spec_id":"test_item_spec_id","progress_update":{"Delta":{"Inc":3}},"msg_update":"NoChange"}"#,
        serde_json::to_string(&ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
            msg_update: ProgressMsgUpdate::NoChange,
        })
        .unwrap()
    )
}

#[test]
fn eq() {
    assert_eq!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Limit(ProgressLimit::Steps(3)),
            msg_update: ProgressMsgUpdate::NoChange,
        },
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Limit(ProgressLimit::Steps(3)),
            msg_update: ProgressMsgUpdate::NoChange,
        }
    );
    assert_eq!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
            msg_update: ProgressMsgUpdate::NoChange,
        },
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
            msg_update: ProgressMsgUpdate::NoChange,
        }
    );
    assert_eq!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Complete(ProgressComplete::Success),
            msg_update: ProgressMsgUpdate::NoChange,
        },
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Complete(ProgressComplete::Success),
            msg_update: ProgressMsgUpdate::NoChange,
        }
    );
}

#[test]
fn ne() {
    assert_ne!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Limit(ProgressLimit::Steps(3)),
            msg_update: ProgressMsgUpdate::NoChange,
        },
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Limit(ProgressLimit::Steps(4)),
            msg_update: ProgressMsgUpdate::Clear,
        }
    );
    assert_ne!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
            msg_update: ProgressMsgUpdate::NoChange,
        },
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(4)),
            msg_update: ProgressMsgUpdate::Clear,
        }
    );
    assert_ne!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Complete(ProgressComplete::Success),
            msg_update: ProgressMsgUpdate::NoChange,
        },
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Complete(ProgressComplete::Fail),
            msg_update: ProgressMsgUpdate::Clear,
        }
    );
}

#[test]
fn debug() {
    assert_eq!(
        r#"ProgressUpdateAndId { item_spec_id: ItemSpecId("test_item_spec_id"), progress_update: Limit(Steps(3)), msg_update: NoChange }"#,
        format!(
            "{:?}",
            ProgressUpdateAndId {
                item_spec_id: item_spec_id!("test_item_spec_id"),
                progress_update: ProgressUpdate::Limit(ProgressLimit::Steps(3)),
                msg_update: ProgressMsgUpdate::NoChange,
            }
        )
    );
    assert_eq!(
        r#"ProgressUpdateAndId { item_spec_id: ItemSpecId("test_item_spec_id"), progress_update: Delta(Inc(3)), msg_update: NoChange }"#,
        format!(
            "{:?}",
            ProgressUpdateAndId {
                item_spec_id: item_spec_id!("test_item_spec_id"),
                progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
                msg_update: ProgressMsgUpdate::NoChange,
            }
        )
    );
    assert_eq!(
        r#"ProgressUpdateAndId { item_spec_id: ItemSpecId("test_item_spec_id"), progress_update: Complete(Success), msg_update: NoChange }"#,
        format!(
            "{:?}",
            ProgressUpdateAndId {
                item_spec_id: item_spec_id!("test_item_spec_id"),
                progress_update: ProgressUpdate::Complete(ProgressComplete::Success),
                msg_update: ProgressMsgUpdate::NoChange,
            }
        )
    );
}
