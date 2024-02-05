use peace::{
    cli::output::{CliColorizeOpt, CliMdPresenter},
    fmt::{
        presentable::{Bold, CodeInline, ListNumberedAligned},
        Presentable,
    },
};

use crate::fmt::cli_output;

#[tokio::test]
async fn present() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut cli_output = cli_output(&mut buffer, CliColorizeOpt::Always);
    let mut presenter = CliMdPresenter::new(&mut cli_output);

    let list_numbered_aligned = ListNumberedAligned::new(
        (1..=11)
            .map(|n| {
                (
                    Bold::new(format!("Item {n}")),
                    (
                        String::from("description with "),
                        CodeInline::new("code".into()),
                    ),
                )
            })
            .collect::<Vec<_>>(),
    );
    list_numbered_aligned.present(&mut presenter).await?;

    let output = String::from_utf8(buffer)?;
    assert_eq!(
        " \u{1b}[38;5;15m1.\u{1b}[0m **\u{1b}[1mItem 1\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m2.\u{1b}[0m **\u{1b}[1mItem 2\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m3.\u{1b}[0m **\u{1b}[1mItem 3\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m4.\u{1b}[0m **\u{1b}[1mItem 4\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m5.\u{1b}[0m **\u{1b}[1mItem 5\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m6.\u{1b}[0m **\u{1b}[1mItem 6\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m7.\u{1b}[0m **\u{1b}[1mItem 7\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m8.\u{1b}[0m **\u{1b}[1mItem 8\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
 \u{1b}[38;5;15m9.\u{1b}[0m **\u{1b}[1mItem 9\u{1b}[0m** : description with \u{1b}[38;5;75m`code`\u{1b}[0m
\u{1b}[38;5;15m10.\u{1b}[0m **\u{1b}[1mItem 10\u{1b}[0m**: description with \u{1b}[38;5;75m`code`\u{1b}[0m
\u{1b}[38;5;15m11.\u{1b}[0m **\u{1b}[1mItem 11\u{1b}[0m**: description with \u{1b}[38;5;75m`code`\u{1b}[0m
",
        output
    );
    assert_eq!(
        " 1. **Item 1** : description with `code`
 2. **Item 2** : description with `code`
 3. **Item 3** : description with `code`
 4. **Item 4** : description with `code`
 5. **Item 5** : description with `code`
 6. **Item 6** : description with `code`
 7. **Item 7** : description with `code`
 8. **Item 8** : description with `code`
 9. **Item 9** : description with `code`
10. **Item 10**: description with `code`
11. **Item 11**: description with `code`
",
        console::strip_ansi_codes(&output)
    );
    Ok(())
}

#[test]
fn debug() {
    assert_eq!(
        r#"ListNumberedAligned([("abc", "def")])"#,
        format!("{:?}", ListNumberedAligned::new(vec![("abc", "def")]))
    )
}

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        r#"- - abc
  - def
"#,
        serde_yaml::to_string(&ListNumberedAligned::new(vec![("abc", "def")]))?
    );
    Ok(())
}

#[test]
fn deserialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        ListNumberedAligned::new(vec![(String::from("abc"), String::from("def"))]),
        serde_yaml::from_str("- [abc, def]")?,
    );
    Ok(())
}

#[test]
fn from() {
    assert_eq!(
        ListNumberedAligned::new(vec![("abc", "def")]),
        ListNumberedAligned::from(vec![("abc", "def")])
    );
}
