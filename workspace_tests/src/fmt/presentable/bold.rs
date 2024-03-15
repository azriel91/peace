use peace::{
    cli::output::{CliColorizeOpt, CliMdPresenter},
    fmt::{presentable::Bold, Presentable},
};

use crate::fmt::cli_output;

#[tokio::test]
async fn present() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut cli_output = cli_output(&mut buffer, CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    Bold::new(String::from("bold"))
        .present(&mut presenter)
        .await?;

    let output = String::from_utf8(buffer)?;
    assert_eq!("**\u{1b}[1mbold\u{1b}[0m**", output);
    assert_eq!("**bold**", console::strip_ansi_codes(&output));
    Ok(())
}

#[test]
fn debug() {
    assert_eq!("Bold(\"abc\")", format!("{:?}", Bold::new("abc")))
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        "abc\n\
        ",
        serde_yaml::to_string(&Bold::new("abc"))?
    );
    Ok(())
}

#[test]
fn deserialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(Bold::new("abc"), serde_yaml::from_str("abc")?,);
    Ok(())
}
