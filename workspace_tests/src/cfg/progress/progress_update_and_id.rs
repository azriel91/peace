use peace::cfg::{
    item_spec_id,
    progress::{
        ProgressComplete, ProgressDelta, ProgressLimit, ProgressUpdate, ProgressUpdateAndId,
    },
    ItemSpecId,
};

#[test]
fn clone() {
    let progress_update_and_id_0 = ProgressUpdateAndId {
        item_spec_id: item_spec_id!("test_item_spec_id"),
        progress_update: ProgressUpdate::Delta(ProgressDelta::Tick),
    };
    let progress_update_and_id_1 = progress_update_and_id_0.clone();

    assert_eq!(progress_update_and_id_0, progress_update_and_id_1);
}

#[test]
fn deserialize() {
    assert_eq!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
        },
        serde_yaml::from_str(
            r#"item_spec_id: test_item_spec_id
progress_update: !Delta
  Inc: 3
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
"#,
        serde_yaml::to_string(&ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
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
        },
        serde_json::from_str(
            r#"{"item_spec_id":"test_item_spec_id","progress_update":{"Delta":{"Inc":3}}}"#
        )
        .unwrap()
    )
}

#[test]
#[cfg(feature = "output_json")]
fn serialize_json() {
    assert_eq!(
        r#"{"item_spec_id":"test_item_spec_id","progress_update":{"Delta":{"Inc":3}}}"#,
        serde_json::to_string(&ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
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
        },
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Limit(ProgressLimit::Steps(3)),
        }
    );
    assert_eq!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
        },
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
        }
    );
    assert_eq!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Complete(ProgressComplete::Success),
        },
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Complete(ProgressComplete::Success),
        }
    );
}

#[test]
fn ne() {
    assert_ne!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Limit(ProgressLimit::Steps(3)),
        },
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Limit(ProgressLimit::Steps(4)),
        }
    );
    assert_ne!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
        },
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(4)),
        }
    );
    assert_ne!(
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Complete(ProgressComplete::Success),
        },
        ProgressUpdateAndId {
            item_spec_id: item_spec_id!("test_item_spec_id"),
            progress_update: ProgressUpdate::Complete(ProgressComplete::Fail),
        }
    );
}

#[test]
fn debug() {
    assert_eq!(
        r#"ProgressUpdateAndId { item_spec_id: ItemSpecId("test_item_spec_id"), progress_update: Limit(Steps(3)) }"#,
        format!(
            "{:?}",
            ProgressUpdateAndId {
                item_spec_id: item_spec_id!("test_item_spec_id"),
                progress_update: ProgressUpdate::Limit(ProgressLimit::Steps(3)),
            }
        )
    );
    assert_eq!(
        r#"ProgressUpdateAndId { item_spec_id: ItemSpecId("test_item_spec_id"), progress_update: Delta(Inc(3)) }"#,
        format!(
            "{:?}",
            ProgressUpdateAndId {
                item_spec_id: item_spec_id!("test_item_spec_id"),
                progress_update: ProgressUpdate::Delta(ProgressDelta::Inc(3)),
            }
        )
    );
    assert_eq!(
        r#"ProgressUpdateAndId { item_spec_id: ItemSpecId("test_item_spec_id"), progress_update: Complete(Success) }"#,
        format!(
            "{:?}",
            ProgressUpdateAndId {
                item_spec_id: item_spec_id!("test_item_spec_id"),
                progress_update: ProgressUpdate::Complete(ProgressComplete::Success),
            }
        )
    );
}
