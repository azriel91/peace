mod debug {
    use peace::resources::resources::ts::{
        Cleaned, CleanedDry, Empty, Ensured, EnsuredDry, SetUp, WithStatesCurrent,
        WithStatesCurrentAndDesired, WithStatesCurrentDiffs, WithStatesDesired, WithStatesSaved,
        WithStatesSavedAndDesired, WithStatesSavedDiffs,
    };

    #[test]
    fn cleaned() {
        assert_eq!("Cleaned", format!("{Cleaned:?}"));
    }

    #[test]
    fn cleaned_dry() {
        assert_eq!("CleanedDry", format!("{CleanedDry:?}"));
    }

    #[test]
    fn empty() {
        assert_eq!("Empty", format!("{Empty:?}"));
    }

    #[test]
    fn ensured() {
        assert_eq!("Ensured", format!("{Ensured:?}"));
    }

    #[test]
    fn ensured_dry() {
        assert_eq!("EnsuredDry", format!("{EnsuredDry:?}"));
    }

    #[test]
    fn set_up() {
        assert_eq!("SetUp", format!("{SetUp:?}"));
    }

    #[test]
    fn with_states_current() {
        assert_eq!("WithStatesCurrent", format!("{WithStatesCurrent:?}"));
    }

    #[test]
    fn with_states_current_and_desired() {
        assert_eq!(
            "WithStatesCurrentAndDesired",
            format!("{WithStatesCurrentAndDesired:?}")
        );
    }

    #[test]
    fn with_states_current_diffs() {
        assert_eq!(
            "WithStatesCurrentDiffs",
            format!("{WithStatesCurrentDiffs:?}")
        );
    }

    #[test]
    fn with_states_desired() {
        assert_eq!("WithStatesDesired", format!("{WithStatesDesired:?}"));
    }

    #[test]
    fn with_states_saved() {
        assert_eq!("WithStatesSaved", format!("{WithStatesSaved:?}"));
    }

    #[test]
    fn with_states_saved_and_desired() {
        assert_eq!(
            "WithStatesSavedAndDesired",
            format!("{WithStatesSavedAndDesired:?}")
        );
    }

    #[test]
    fn with_states_saved_diffs() {
        assert_eq!("WithStatesSavedDiffs", format!("{WithStatesSavedDiffs:?}"));
    }
}
