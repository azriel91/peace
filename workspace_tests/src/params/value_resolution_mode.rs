use peace::params::ValueResolutionMode;

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        "Current\n",
        serde_yaml::to_string(&ValueResolutionMode::Current)?
    );
    Ok(())
}

#[test]
fn deserialize() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        ValueResolutionMode::Current,
        serde_yaml::from_str("Current")?
    );
    Ok(())
}
