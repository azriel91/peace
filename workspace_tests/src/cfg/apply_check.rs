use peace::cfg::ApplyCheck;

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        "ExecNotRequired\n",
        serde_yaml::to_string(&ApplyCheck::ExecNotRequired)?
    );
    Ok(())
}

#[test]
fn deserialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        ApplyCheck::ExecNotRequired,
        serde_yaml::from_str("ExecNotRequired")?
    );
    Ok(())
}
