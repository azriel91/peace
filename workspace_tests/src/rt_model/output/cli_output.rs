use peace::{
    cfg::{item_spec_id, ItemSpecId, State},
    resources::{
        internal::{StateDiffsMut, StatesMut},
        states::{
            StateDiffs, StatesCleaned, StatesCleanedDry, StatesDesired, StatesEnsured,
            StatesEnsuredDry, StatesSaved,
        },
    },
    rt_model::output::{CliOutput, OutputFormat, OutputWrite},
};

#[cfg(feature = "output_colorized")]
use peace::rt_model::output::CliColorize;

#[tokio::test]
async fn outputs_states_saved_as_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Text);
    let states_saved = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesSaved::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_saved(&mut cli_output, &states_saved)
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
async fn outputs_states_desired_as_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Text);
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
async fn outputs_state_diffs_as_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Text);
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
async fn outputs_states_ensured_dry_as_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Text);
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
async fn outputs_states_ensured_as_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Text);
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
async fn outputs_states_cleaned_dry_as_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Text);
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
async fn outputs_states_cleaned_as_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Text);
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
async fn outputs_error_as_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Text);
    let error = Error::CliOutputTest;

    <CliOutput<_> as OutputWrite<Error>>::write_err(&mut cli_output, &error).await?;

    assert_eq!(
        r#"CliOutputTest display message.
"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[cfg(feature = "output_colorized")]
#[tokio::test]
async fn outputs_states_saved_as_text_colorized() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output_colorized(&mut buffer, OutputFormat::Text);
    let states_saved = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesSaved::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_saved(&mut cli_output, &states_saved)
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

#[cfg(feature = "output_colorized")]
#[tokio::test]
async fn outputs_states_desired_as_text_colorized() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output_colorized(&mut buffer, OutputFormat::Text);
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

#[cfg(feature = "output_colorized")]
#[tokio::test]
async fn outputs_state_diffs_as_text_colorized() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output_colorized(&mut buffer, OutputFormat::Text);
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

#[cfg(feature = "output_colorized")]
#[tokio::test]
async fn outputs_states_ensured_dry_as_text_colorized() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output_colorized(&mut buffer, OutputFormat::Text);
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

#[cfg(feature = "output_colorized")]
#[tokio::test]
async fn outputs_states_ensured_as_text_colorized() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output_colorized(&mut buffer, OutputFormat::Text);
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

#[cfg(feature = "output_colorized")]
#[tokio::test]
async fn outputs_states_cleaned_dry_as_text_colorized() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output_colorized(&mut buffer, OutputFormat::Text);
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

#[cfg(feature = "output_colorized")]
#[tokio::test]
async fn outputs_states_cleaned_as_text_colorized() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output_colorized(&mut buffer, OutputFormat::Text);
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

#[cfg(feature = "output_colorized")]
#[tokio::test]
async fn outputs_error_as_text_colorized() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output_colorized(&mut buffer, OutputFormat::Text);
    let error = Error::CliOutputTest;

    <CliOutput<_> as OutputWrite<Error>>::write_err(&mut cli_output, &error).await?;

    assert_eq!(
        r#"CliOutputTest display message.
"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[tokio::test]
async fn outputs_states_saved_as_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Yaml);
    let states_saved = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesSaved::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_saved(&mut cli_output, &states_saved)
        .await?;

    assert_eq!(
        r#"item_0:
  logical: logical
  physical: 1.1
item_1:
  logical: 1
  physical: true
"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[tokio::test]
async fn outputs_states_desired_as_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Yaml);
    let states_desired = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesDesired::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_desired(&mut cli_output, &states_desired)
        .await?;

    assert_eq!(
        r#"item_0:
  logical: logical
  physical: 1.1
item_1:
  logical: 1
  physical: true
"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[tokio::test]
async fn outputs_state_diffs_as_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Yaml);
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
async fn outputs_states_ensured_dry_as_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Yaml);
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
        r#"item_0:
  logical: logical
  physical: 1.1
item_1:
  logical: 1
  physical: true
"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[tokio::test]
async fn outputs_states_ensured_as_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Yaml);
    let states_ensured = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesEnsured::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_ensured(&mut cli_output, &states_ensured)
        .await?;

    assert_eq!(
        r#"item_0:
  logical: logical
  physical: 1.1
item_1:
  logical: 1
  physical: true
"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[tokio::test]
async fn outputs_states_cleaned_dry_as_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Yaml);
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
        r#"item_0:
  logical: logical
  physical: 1.1
item_1:
  logical: 1
  physical: true
"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[tokio::test]
async fn outputs_states_cleaned_as_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Yaml);
    let states_cleaned = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesCleaned::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_cleaned(&mut cli_output, &states_cleaned)
        .await?;

    assert_eq!(
        r#"item_0:
  logical: logical
  physical: 1.1
item_1:
  logical: 1
  physical: true
"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[tokio::test]
async fn outputs_error_as_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Yaml);
    let error = Error::CliOutputTest;

    <CliOutput<_> as OutputWrite<Error>>::write_err(&mut cli_output, &error).await?;

    assert_eq!(
        r#"CliOutputTest display message.
"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[cfg(feature = "output_json")]
#[tokio::test]
async fn outputs_states_saved_as_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Json);
    let states_saved = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesSaved::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_saved(&mut cli_output, &states_saved)
        .await?;

    assert_eq!(
        r#"{"item_0":{"logical":"logical","physical":1.1},"item_1":{"logical":1,"physical":true}}"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[cfg(feature = "output_json")]
#[tokio::test]
async fn outputs_states_desired_as_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Json);
    let states_desired = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesDesired::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_desired(&mut cli_output, &states_desired)
        .await?;

    assert_eq!(
        r#"{"item_0":{"logical":"logical","physical":1.1},"item_1":{"logical":1,"physical":true}}"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[cfg(feature = "output_json")]
#[tokio::test]
async fn outputs_state_diffs_as_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Json);
    let state_diffs = {
        let mut state_diffs_mut = StateDiffsMut::new();
        state_diffs_mut.insert(item_spec_id!("item_0"), "need one more server");
        state_diffs_mut.insert(item_spec_id!("item_1"), 1);
        StateDiffs::from(state_diffs_mut)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_state_diffs(&mut cli_output, &state_diffs).await?;

    assert_eq!(
        r#"{"item_0":"need one more server","item_1":1}"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[cfg(feature = "output_json")]
#[tokio::test]
async fn outputs_states_ensured_dry_as_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Json);
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
        r#"{"item_0":{"logical":"logical","physical":1.1},"item_1":{"logical":1,"physical":true}}"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[cfg(feature = "output_json")]
#[tokio::test]
async fn outputs_states_ensured_as_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Json);
    let states_ensured = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesEnsured::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_ensured(&mut cli_output, &states_ensured)
        .await?;

    assert_eq!(
        r#"{"item_0":{"logical":"logical","physical":1.1},"item_1":{"logical":1,"physical":true}}"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[cfg(feature = "output_json")]
#[tokio::test]
async fn outputs_states_cleaned_dry_as_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Json);
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
        r#"{"item_0":{"logical":"logical","physical":1.1},"item_1":{"logical":1,"physical":true}}"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[cfg(feature = "output_json")]
#[tokio::test]
async fn outputs_states_cleaned_as_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Json);
    let states_cleaned = {
        let mut states = StatesMut::new();
        states.insert(item_spec_id!("item_0"), State::new("logical", 1.1));
        states.insert(item_spec_id!("item_1"), State::new(1u8, true));
        StatesCleaned::from(states)
    };

    <CliOutput<_> as OutputWrite<Error>>::write_states_cleaned(&mut cli_output, &states_cleaned)
        .await?;

    assert_eq!(
        r#"{"item_0":{"logical":"logical","physical":1.1},"item_1":{"logical":1,"physical":true}}"#,
        String::from_utf8(buffer)?
    );
    Ok(())
}

#[cfg(feature = "output_json")]
#[tokio::test]
async fn outputs_error_as_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output(&mut buffer, OutputFormat::Json);
    let error = Error::CliOutputTest;

    <CliOutput<_> as OutputWrite<Error>>::write_err(&mut cli_output, &error).await?;

    assert_eq!(
        r#""CliOutputTest display message.""#,
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

fn cli_output(buffer: &mut Vec<u8>, output_format: OutputFormat) -> CliOutput<&mut Vec<u8>> {
    CliOutput::new_with_writer(buffer).with_output_format(output_format)
}

#[cfg(feature = "output_colorized")]
fn cli_output_colorized(
    buffer: &mut Vec<u8>,
    output_format: OutputFormat,
) -> CliOutput<&mut Vec<u8>> {
    CliOutput::new_with_writer(buffer)
        .with_output_format(output_format)
        .with_colorize(CliColorize::Always)
}
