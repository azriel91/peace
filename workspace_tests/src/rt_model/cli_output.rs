use peace::{
    cfg::{item_spec_id, ItemSpecId, State},
    resources::{
        internal::{StateDiffsMut, StatesMut},
        states::{
            StateDiffs, StatesCleaned, StatesCleanedDry, StatesCurrent, StatesDesired,
            StatesEnsured, StatesEnsuredDry,
        },
    },
    rt_model::{CliOutput, OutputWrite},
};

#[tokio::test]
async fn outputs_states_current() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer);
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
async fn outputs_states_desired() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer);
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
async fn outputs_state_diffs() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer);
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
async fn outputs_states_ensured_dry() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer);
    let states_ensured_dry = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesEnsuredDry::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_ensured_dry(
        &mut cli_output,
        &states_ensured_dry,
    )
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
async fn outputs_states_ensured() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer);
    let states_ensured = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesEnsured::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_ensured(&mut cli_output, &states_ensured)
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
async fn outputs_states_cleaned_dry() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer);
    let states_cleaned_dry = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesCleanedDry::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_cleaned_dry(
        &mut cli_output,
        &states_cleaned_dry,
    )
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
async fn outputs_states_cleaned() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer);
    let states_cleaned = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesCleaned::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_cleaned(&mut cli_output, &states_cleaned)
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
async fn outputs_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer);
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

#[cfg(not(feature = "output_colorized"))]
fn cli_output(buffer: &mut Vec<u8>) -> CliOutput<&mut Vec<u8>> {
    CliOutput::new_with_writer(buffer)
}

#[cfg(feature = "output_colorized")]
fn cli_output(buffer: &mut Vec<u8>) -> CliOutput<&mut Vec<u8>> {
    CliOutput::new_with_writer(buffer).colorized()
}
