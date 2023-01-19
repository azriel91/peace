use peace::cfg::state::ExternalOpt;

#[test]
fn display() {
    assert_eq!(
        "u8 not yet determined",
        format!("{}", ExternalOpt::<u8>::Tbd)
    );
    assert_eq!("u8 is non-existent", format!("{}", ExternalOpt::<u8>::None));
    assert_eq!("u8: 123", format!("{}", ExternalOpt::<u8>::Value(123)));
}

#[test]
fn clone() {
    let external_opt = &ExternalOpt::<u8>::Tbd;

    assert_eq!(ExternalOpt::<u8>::Tbd, external_opt.clone());
}

#[test]
fn debug() {
    assert_eq!("Tbd", format!("{:?}", ExternalOpt::<u8>::Tbd));
}

#[test]
fn serialize() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        "!Tbd null\n",
        serde_yaml::to_string(&ExternalOpt::<u8>::Tbd)?
    );
    assert_eq!(
        "!None null\n",
        serde_yaml::to_string(&ExternalOpt::<u8>::None)?
    );
    assert_eq!(
        "!Value 123\n",
        serde_yaml::to_string(&ExternalOpt::<u8>::Value(123))?
    );
    Ok(())
}

#[test]
fn deserialize() -> Result<(), Box<dyn std::error::Error>> {
    let external = serde_yaml::from_str::<ExternalOpt<u8>>("!Tbd null\n")?;
    assert_eq!(ExternalOpt::<u8>::Tbd, external);

    let external = serde_yaml::from_str::<ExternalOpt<u8>>("!None null\n")?;
    assert_eq!(ExternalOpt::<u8>::None, external);

    let external = serde_yaml::from_str::<ExternalOpt<u8>>("!Value 123\n")?;
    assert_eq!(ExternalOpt::<u8>::Value(123), external);
    Ok(())
}
