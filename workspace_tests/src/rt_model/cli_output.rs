use peace::{
    cfg::{item_spec_id, ItemSpecId, State},
    resources::{
        internal::{StateDiffsMut, StatesMut},
        states::{StateDiffs, StatesCurrent, StatesDesired},
    },
    rt_model::{CliOutput, OutputWrite},
};

#[tokio::test]
async fn outputs_states_current_verbose_mode() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = CliOutput::new_with_writer(&mut buffer);
    let states_current = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesCurrent::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_current(&mut cli_output, &states_current)
        .await?;

    assert_eq!(
        "\
        item_0: logical, 1.1\n\
        item_1: 1, true\n\
        ",
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[tokio::test]
async fn outputs_states_desired_verbose_mode() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = CliOutput::new_with_writer(&mut buffer);
    let states_desired = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesDesired::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_desired(&mut cli_output, &states_desired)
        .await?;

    assert_eq!(
        "\
        item_0: logical, 1.1\n\
        item_1: 1, true\n\
        ",
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[tokio::test]
async fn outputs_state_diffs_verbose_mode() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = CliOutput::new_with_writer(&mut buffer);
    let state_diffs = {
        let mut state_diffs_mut = StateDiffsMut::new();
        state_diffs_mut.insert(item_spec_id!("item_0"), "need one more server");
        state_diffs_mut.insert(item_spec_id!("item_1"), 1);
        StateDiffs::from(state_diffs_mut)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_state_diffs(&mut cli_output, &state_diffs).await?;

    assert_eq!(
        r#"item_0: need one more server
item_1: 1
"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[tokio::test]
async fn outputs_error_one_line_mode() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = CliOutput::new_with_writer(&mut buffer);
    let error = Error::CliOutputTest;

    <CliOutput<_> as OutputWrite<Error>>::write_err(&mut cli_output, &error).await?;

    assert_eq!(
        r#"CliOutputTest display message.
"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    /// CliOutputTest display message.
    #[error("CliOutputTest display message.")]
    CliOutputTest,

    // Framework errors
    /// A `peace` runtime error occurred.
    #[error("A `peace` runtime error occurred.")]
    PeaceRtError(#[from] peace::rt_model::Error),
}
