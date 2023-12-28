use peace::{
    cmd_model::CmdOutcome,
    rt_model::{fn_graph::StreamOutcome, IndexMap},
};

#[test]
fn value() {
    assert_eq!(Some(123), cmd_outcome_complete(123).value().copied());
    assert_eq!(
        Some(123),
        cmd_outcome_block_interrupted(123).value().copied()
    );
    assert_eq!(
        Some(123),
        cmd_outcome_execution_interrupted(Some(123))
            .value()
            .copied()
    );
    assert_eq!(Some(123), cmd_outcome_item_error(123).value().copied());
}

#[test]
fn is_complete() {
    assert!(cmd_outcome_complete(123).is_complete());
    assert!(!cmd_outcome_block_interrupted(123).is_complete());
    assert!(!cmd_outcome_execution_interrupted(Some(123)).is_complete());
    assert!(!cmd_outcome_item_error(123).is_complete());
}

#[test]
fn is_interrupted() {
    assert!(!cmd_outcome_complete(123).is_interrupted());
    assert!(cmd_outcome_block_interrupted(123).is_interrupted());
    assert!(cmd_outcome_execution_interrupted(Some(123)).is_interrupted());
    assert!(!cmd_outcome_item_error(123).is_interrupted());
}

#[test]
fn is_err() {
    assert!(!cmd_outcome_complete(123).is_err());
    assert!(!cmd_outcome_block_interrupted(123).is_err());
    assert!(!cmd_outcome_execution_interrupted(Some(123)).is_err());
    assert!(cmd_outcome_item_error(123).is_err());
}

#[test]
fn map() {
    assert_eq!(
        cmd_outcome_complete(124),
        cmd_outcome_complete(123).map(|value| value + 1)
    );
    assert_eq!(
        cmd_outcome_block_interrupted(124),
        cmd_outcome_block_interrupted(123).map(|value| value + 1)
    );
    assert_eq!(
        cmd_outcome_execution_interrupted(Some(124)),
        cmd_outcome_execution_interrupted(Some(123)).map(|value| value + 1)
    );
    assert_eq!(
        cmd_outcome_execution_interrupted(None::<u32>),
        cmd_outcome_execution_interrupted(None::<u32>).map(|value| value + 1)
    );
    assert_eq!(
        cmd_outcome_item_error(124),
        cmd_outcome_item_error(123).map(|value| value + 1)
    );
}

#[tokio::test]
async fn map_async() {
    assert_eq!(
        cmd_outcome_complete(124),
        cmd_outcome_complete(123)
            .map_async(|value| async move { value + 1 })
            .await
    );
    assert_eq!(
        cmd_outcome_block_interrupted(124),
        cmd_outcome_block_interrupted(123)
            .map_async(|value| async move { value + 1 })
            .await
    );
    assert_eq!(
        cmd_outcome_execution_interrupted(Some(124)),
        cmd_outcome_execution_interrupted(Some(123))
            .map_async(|value| async move { value + 1 })
            .await
    );
    assert_eq!(
        cmd_outcome_execution_interrupted(None::<u32>),
        cmd_outcome_execution_interrupted(None::<u32>)
            .map_async(|value| async move { value + 1 })
            .await
    );
    assert_eq!(
        cmd_outcome_item_error(124),
        cmd_outcome_item_error(123)
            .map_async(|value| async move { value + 1 })
            .await
    );
}

#[test]
fn transpose() {
    assert_eq!(
        Ok(cmd_outcome_complete(123)),
        cmd_outcome_complete(Ok(123)).transpose()
    );
    assert_eq!(
        Err("err".to_string()),
        cmd_outcome_complete(Err::<u32, _>("err".to_string())).transpose()
    );
    assert_eq!(
        Ok(cmd_outcome_block_interrupted(123)),
        cmd_outcome_block_interrupted(Ok(123)).transpose()
    );
    assert_eq!(
        Err("err".to_string()),
        cmd_outcome_block_interrupted(Err::<u32, _>("err".to_string())).transpose()
    );
    assert_eq!(
        Ok(cmd_outcome_execution_interrupted(Some(123))),
        cmd_outcome_execution_interrupted(Some(Ok(123))).transpose()
    );
    assert_eq!(
        Err("err".to_string()),
        cmd_outcome_execution_interrupted(Some(Err::<u32, _>("err".to_string()))).transpose()
    );
    assert_eq!(
        Ok(cmd_outcome_execution_interrupted(None::<u32>)),
        cmd_outcome_execution_interrupted(None::<Result<u32, String>>).transpose()
    );
    assert_eq!(
        Ok(cmd_outcome_item_error(123)),
        cmd_outcome_item_error(Ok(123)).transpose()
    );
    assert_eq!(
        Err("err".to_string()),
        cmd_outcome_item_error(Err::<u32, _>("err".to_string())).transpose()
    );
}

#[test]
fn clone() {
    let cmd_outcome = cmd_outcome_complete(123);

    assert_eq!(cmd_outcome, Clone::clone(&cmd_outcome));
}

#[test]
fn debug() {
    let cmd_outcome = cmd_outcome_complete(123);

    assert_eq!(
        "Complete { value: 123, cmd_blocks_processed: [] }",
        format!("{cmd_outcome:?}")
    );
}

fn cmd_outcome_complete<T>(value: T) -> CmdOutcome<T, String> {
    CmdOutcome::<T, String>::Complete {
        value,
        cmd_blocks_processed: vec![],
    }
}

fn cmd_outcome_block_interrupted<T>(value: T) -> CmdOutcome<T, String> {
    CmdOutcome::<T, String>::BlockInterrupted {
        stream_outcome: StreamOutcome::finished_with(value, Vec::new()),
        cmd_blocks_processed: vec![],
        cmd_blocks_not_processed: vec![],
    }
}

fn cmd_outcome_execution_interrupted<T>(value: Option<T>) -> CmdOutcome<T, String> {
    CmdOutcome::<T, String>::ExecutionInterrupted {
        value,
        cmd_blocks_processed: vec![],
        cmd_blocks_not_processed: vec![],
    }
}

fn cmd_outcome_item_error<T>(value: T) -> CmdOutcome<T, String> {
    CmdOutcome::<T, String>::ItemError {
        stream_outcome: StreamOutcome::finished_with(value, Vec::new()),
        cmd_blocks_processed: vec![],
        cmd_blocks_not_processed: vec![],
        errors: IndexMap::new(),
    }
}
