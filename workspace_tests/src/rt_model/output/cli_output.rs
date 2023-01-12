use peace::{
    cfg::{item_spec_id, ItemSpecId, State},
    resources::{
        internal::{StateDiffsMut, StatesMut},
        states::{
            StateDiffs, StatesCleaned, StatesCleanedDry, StatesDesired, StatesEnsured,
            StatesEnsuredDry, StatesSaved,
        },
    },
    rt_model::output::{CliOutput, CliOutputBuilder, OutputFormat, OutputWrite},
};

#[cfg(feature = "output_colorized")]
use peace::rt_model::output::CliColorizeOpt;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use std::collections::HashMap;

        use peace::{
            cfg::progress::{
                ProgressComplete,
                ProgressDelta,
                ProgressLimit,
                ProgressStatus,
                ProgressTracker,
                ProgressUpdate,
            },
            rt_model::{
                indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget},
                output::{CliOutputTarget, CliProgressFormatOpt},
                CmdProgressTracker,
            },
        };
    }
}

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

    let output = String::from_utf8(buffer)?;
    assert_eq!(
        "\u{1b}[38;5;69mitem_0\u{1b}[0m: logical, 1.1\n\
        \u{1b}[38;5;69mitem_1\u{1b}[0m: 1, true\n",
        output
    );
    assert_eq!(
        "\
        item_0: logical, 1.1\n\
        item_1: 1, true\n\
        ",
        console::strip_ansi_codes(&output)
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

    let output = String::from_utf8(buffer)?;
    assert_eq!(
        "\u{1b}[38;5;69mitem_0\u{1b}[0m: logical, 1.1\n\
        \u{1b}[38;5;69mitem_1\u{1b}[0m: 1, true\n",
        output
    );
    assert_eq!(
        "\
        item_0: logical, 1.1\n\
        item_1: 1, true\n\
        ",
        console::strip_ansi_codes(&output)
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

    let output = String::from_utf8(buffer)?;
    assert_eq!(
        "\u{1b}[38;5;69mitem_0\u{1b}[0m: need one more server\n\
        \u{1b}[38;5;69mitem_1\u{1b}[0m: 1\n",
        output
    );
    assert_eq!(
        "\
        item_0: need one more server\n\
        item_1: 1\n\
        ",
        console::strip_ansi_codes(&output)
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

    let output = String::from_utf8(buffer)?;
    assert_eq!(
        "\u{1b}[38;5;69mitem_0\u{1b}[0m: logical, 1.1\n\
        \u{1b}[38;5;69mitem_1\u{1b}[0m: 1, true\n",
        output
    );
    assert_eq!(
        "\
        item_0: logical, 1.1\n\
        item_1: 1, true\n\
        ",
        console::strip_ansi_codes(&output)
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

    let output = String::from_utf8(buffer)?;
    assert_eq!(
        "\u{1b}[38;5;69mitem_0\u{1b}[0m: logical, 1.1\n\
        \u{1b}[38;5;69mitem_1\u{1b}[0m: 1, true\n",
        output
    );
    assert_eq!(
        "\
        item_0: logical, 1.1\n\
        item_1: 1, true\n\
        ",
        console::strip_ansi_codes(&output)
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

    let output = String::from_utf8(buffer)?;
    assert_eq!(
        "\u{1b}[38;5;69mitem_0\u{1b}[0m: logical, 1.1\n\
        \u{1b}[38;5;69mitem_1\u{1b}[0m: 1, true\n",
        output
    );
    assert_eq!(
        "\
        item_0: logical, 1.1\n\
        item_1: 1, true\n\
        ",
        console::strip_ansi_codes(&output)
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

    let output = String::from_utf8(buffer)?;
    assert_eq!(
        "\u{1b}[38;5;69mitem_0\u{1b}[0m: logical, 1.1\n\
        \u{1b}[38;5;69mitem_1\u{1b}[0m: 1, true\n",
        output
    );
    assert_eq!(
        "\
        item_0: logical, 1.1\n\
        item_1: 1, true\n\
        ",
        console::strip_ansi_codes(&output)
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

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn progress_begin_sets_prefix_and_progress_bar_style() {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output_progress(
        &mut buffer,
        OutputFormat::Text,
        CliProgressFormatOpt::ProgressBar,
    );
    let (cmd_progress_tracker, progress_bar) = cmd_progress_tracker(&cli_output);

    <CliOutput<_> as OutputWrite<Error>>::progress_begin(&mut cli_output, &cmd_progress_tracker)
        .await;

    // We can't inspect `ProgressStyle`'s fields, so we have to render the progress
    // and compare the output.
    assert_eq!("test_item_spec_id", progress_bar.prefix());
    let CliOutputTarget::InMemory(in_memory_term) = cli_output.progress_target() else {
        unreachable!("This is set in `cli_output_progress`.");
    };
    assert_eq!(
        r#"test_item_spec_id    ▕▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▏"#,
        // ^                   ^ ^                                      ^
        // '---- 20 chars -----' '-------------- 40 chars --------------'
        in_memory_term.contents()
    );
}

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn progress_update_with_limit_sets_progress_bar_style() {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output_progress(
        &mut buffer,
        OutputFormat::Text,
        CliProgressFormatOpt::ProgressBar,
    );
    let (mut cmd_progress_tracker, progress_bar) = cmd_progress_tracker(&cli_output);

    <CliOutput<_> as OutputWrite<Error>>::progress_begin(&mut cli_output, &cmd_progress_tracker)
        .await;

    let progress_trackers = cmd_progress_tracker.progress_trackers_mut();
    let progress_tracker = progress_trackers
        .get_mut(&item_spec_id!("test_item_spec_id"))
        .unwrap();

    // Adjust progress_bar length and units.
    progress_tracker.set_progress_status(ProgressStatus::Running);
    progress_bar.set_length(100);

    let progress_update = ProgressUpdate::Limit(ProgressLimit::Steps(100));
    <CliOutput<_> as OutputWrite<Error>>::progress_update(
        &mut cli_output,
        &progress_tracker,
        progress_update,
    )
    .await;

    assert_eq!("test_item_spec_id", progress_bar.prefix());
    let CliOutputTarget::InMemory(in_memory_term) = cli_output.progress_target() else {
        unreachable!("This is set in `cli_output_progress`.");
    };
    progress_bar.set_position(20);
    assert_eq!(
        r#"test_item_spec_id    ▕████████                                ▏"#,
        in_memory_term.contents()
    );
    progress_bar.set_position(21);
    assert_eq!(
        r#"test_item_spec_id    ▕████████▍                               ▏"#,
        in_memory_term.contents()
    );
    progress_bar.set_position(22);
    assert_eq!(
        r#"test_item_spec_id    ▕████████▊                               ▏"#,
        in_memory_term.contents()
    );
}

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn progress_update_with_complete_success_finishes_progress_bar() {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output_progress(
        &mut buffer,
        OutputFormat::Text,
        CliProgressFormatOpt::ProgressBar,
    );
    let (mut cmd_progress_tracker, progress_bar) = cmd_progress_tracker(&cli_output);

    <CliOutput<_> as OutputWrite<Error>>::progress_begin(&mut cli_output, &cmd_progress_tracker)
        .await;

    let progress_trackers = cmd_progress_tracker.progress_trackers_mut();
    let progress_tracker = progress_trackers
        .get_mut(&item_spec_id!("test_item_spec_id"))
        .unwrap();

    // Adjust progress_bar length and units.
    progress_tracker.set_progress_status(ProgressStatus::Running);
    progress_bar.set_length(100);

    let progress_update = ProgressUpdate::Limit(ProgressLimit::Steps(100));
    <CliOutput<_> as OutputWrite<Error>>::progress_update(
        &mut cli_output,
        &progress_tracker,
        progress_update,
    )
    .await;

    // Check current position
    let CliOutputTarget::InMemory(in_memory_term) = cli_output.progress_target() else {
        unreachable!("This is set in `cli_output_progress`.");
    };
    progress_bar.set_position(20);
    assert_eq!(
        r#"test_item_spec_id    ▕████████                                ▏"#,
        in_memory_term.contents()
    );

    let progress_update = ProgressUpdate::Complete(ProgressComplete::Success);
    <CliOutput<_> as OutputWrite<Error>>::progress_update(
        &mut cli_output,
        &progress_tracker,
        progress_update,
    )
    .await;
    let CliOutputTarget::InMemory(in_memory_term) = cli_output.progress_target() else {
        unreachable!("This is set in `cli_output_progress`.");
    };
    assert_eq!(
        r#"test_item_spec_id    ▕████████████████████████████████████████▏"#,
        in_memory_term.contents()
    );
}

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn progress_update_with_complete_fail_abandons_progress_bar() {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output_progress(
        &mut buffer,
        OutputFormat::Text,
        CliProgressFormatOpt::ProgressBar,
    );
    let (mut cmd_progress_tracker, progress_bar) = cmd_progress_tracker(&cli_output);

    <CliOutput<_> as OutputWrite<Error>>::progress_begin(&mut cli_output, &cmd_progress_tracker)
        .await;

    let progress_trackers = cmd_progress_tracker.progress_trackers_mut();
    let progress_tracker = progress_trackers
        .get_mut(&item_spec_id!("test_item_spec_id"))
        .unwrap();

    // Adjust progress_bar length and units.
    progress_tracker.set_progress_status(ProgressStatus::Running);
    progress_bar.set_length(100);

    let progress_update = ProgressUpdate::Limit(ProgressLimit::Steps(100));
    <CliOutput<_> as OutputWrite<Error>>::progress_update(
        &mut cli_output,
        &progress_tracker,
        progress_update,
    )
    .await;

    // Check current position
    let CliOutputTarget::InMemory(in_memory_term) = cli_output.progress_target() else {
        unreachable!("This is set in `cli_output_progress`.");
    };
    progress_bar.set_position(20);
    assert_eq!(
        r#"test_item_spec_id    ▕████████                                ▏"#,
        in_memory_term.contents()
    );

    let progress_update = ProgressUpdate::Complete(ProgressComplete::Fail);
    <CliOutput<_> as OutputWrite<Error>>::progress_update(
        &mut cli_output,
        &progress_tracker,
        progress_update,
    )
    .await;
    let CliOutputTarget::InMemory(in_memory_term) = cli_output.progress_target() else {
        unreachable!("This is set in `cli_output_progress`.");
    };
    assert_eq!(
        r#"test_item_spec_id    ▕████████                                ▏"#,
        in_memory_term.contents()
    );
}

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn progress_update_delta_with_progress_format_outcome_writes_yaml() {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output_progress(
        &mut buffer,
        OutputFormat::Text,
        CliProgressFormatOpt::Outcome,
    );
    let (mut cmd_progress_tracker, progress_bar) = cmd_progress_tracker(&cli_output);

    <CliOutput<_> as OutputWrite<Error>>::progress_begin(&mut cli_output, &cmd_progress_tracker)
        .await;

    let progress_trackers = cmd_progress_tracker.progress_trackers_mut();
    let progress_tracker = progress_trackers
        .get_mut(&item_spec_id!("test_item_spec_id"))
        .unwrap();

    let progress_update = ProgressUpdate::Limit(ProgressLimit::Steps(100));
    <CliOutput<_> as OutputWrite<Error>>::progress_update(
        &mut cli_output,
        &progress_tracker,
        progress_update,
    )
    .await;

    // Adjust progress_bar length and units.
    progress_tracker.set_progress_status(ProgressStatus::Running);
    progress_bar.set_length(100);

    let progress_update = ProgressUpdate::Delta(ProgressDelta::Inc(21));
    <CliOutput<_> as OutputWrite<Error>>::progress_update(
        &mut cli_output,
        &progress_tracker,
        progress_update,
    )
    .await;
    let CliOutputTarget::InMemory(in_memory_term) = cli_output.progress_target() else {
        unreachable!("This is set in `cli_output_progress`.");
    };
    // TODO: send in `ProgressUpdateAndId`.
    assert_eq!(
        r#"!Limit
Steps: 100
!Delta
Inc: 21"#,
        in_memory_term.contents()
    );
}

#[cfg(all(feature = "output_json", feature = "output_progress"))]
#[tokio::test]
async fn progress_update_delta_with_progress_format_outcome_writes_json() {
    let mut buffer = Vec::with_capacity(128);
    let mut cli_output = cli_output_progress(
        &mut buffer,
        OutputFormat::Json,
        CliProgressFormatOpt::Outcome,
    );
    let (mut cmd_progress_tracker, progress_bar) = cmd_progress_tracker(&cli_output);

    <CliOutput<_> as OutputWrite<Error>>::progress_begin(&mut cli_output, &cmd_progress_tracker)
        .await;

    let progress_trackers = cmd_progress_tracker.progress_trackers_mut();
    let progress_tracker = progress_trackers
        .get_mut(&item_spec_id!("test_item_spec_id"))
        .unwrap();

    let progress_update = ProgressUpdate::Limit(ProgressLimit::Steps(100));
    <CliOutput<_> as OutputWrite<Error>>::progress_update(
        &mut cli_output,
        &progress_tracker,
        progress_update,
    )
    .await;

    // Adjust progress_bar length and units.
    progress_tracker.set_progress_status(ProgressStatus::Running);
    progress_bar.set_length(100);

    let progress_update = ProgressUpdate::Delta(ProgressDelta::Inc(21));
    <CliOutput<_> as OutputWrite<Error>>::progress_update(
        &mut cli_output,
        &progress_tracker,
        progress_update,
    )
    .await;
    let CliOutputTarget::InMemory(in_memory_term) = cli_output.progress_target() else {
        unreachable!("This is set in `cli_output_progress`.");
    };
    // TODO: send in `ProgressUpdateAndId`.
    assert_eq!(
        r#"{"Limit":{"Steps":100}}
{"Delta":{"Inc":21}}"#,
        in_memory_term.contents()
    );
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
    CliOutputBuilder::new_with_writer(buffer)
        .with_outcome_format(output_format)
        .build()
}

#[cfg(feature = "output_colorized")]
fn cli_output_colorized(
    buffer: &mut Vec<u8>,
    output_format: OutputFormat,
) -> CliOutput<&mut Vec<u8>> {
    CliOutputBuilder::new_with_writer(buffer)
        .with_outcome_format(output_format)
        .with_colorize(CliColorizeOpt::Always)
        .build()
}

#[cfg(feature = "output_progress")]
fn cli_output_progress(
    buffer: &mut Vec<u8>,
    output_format: OutputFormat,
    progress_format: CliProgressFormatOpt,
) -> CliOutput<&mut Vec<u8>> {
    CliOutputBuilder::new_with_writer(buffer)
        .with_outcome_format(output_format)
        .with_progress_target(CliOutputTarget::in_memory(50, 120))
        .with_progress_format(progress_format)
        .build()
}

#[cfg(feature = "output_progress")]
fn cmd_progress_tracker(cli_output: &CliOutput<&mut Vec<u8>>) -> (CmdProgressTracker, ProgressBar) {
    let CliOutputTarget::InMemory(in_memory_term) = cli_output.progress_target() else {
        unreachable!("This is set in `cli_output_progress`.");
    };
    let multi_progress = MultiProgress::with_draw_target(ProgressDrawTarget::term_like(Box::new(
        in_memory_term.clone(),
    )));
    let mut progress_trackers = HashMap::new();
    let progress_bar = multi_progress.add(ProgressBar::hidden());
    let progress_tracker = ProgressTracker::new(progress_bar.clone());
    progress_trackers.insert(item_spec_id!("test_item_spec_id"), progress_tracker);
    let cmd_progress_tracker = CmdProgressTracker::new(multi_progress, progress_trackers);

    (cmd_progress_tracker, progress_bar)
}
