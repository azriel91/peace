mod debug {
    use peace::resources::states::ts::{
        Cleaned, CleanedDry, Current, Desired, Ensured, EnsuredDry, Saved,
    };

    #[test]
    fn states_saved() {
        assert_eq!("Saved", format!("{Saved:?}"))
    }

    #[test]
    fn states_current() {
        assert_eq!("Current", format!("{Current:?}"))
    }

    #[test]
    fn states_desired() {
        assert_eq!("Desired", format!("{Desired:?}"))
    }

    #[test]
    fn states_ensured() {
        assert_eq!("Ensured", format!("{Ensured:?}"))
    }

    #[test]
    fn states_ensured_dry() {
        assert_eq!("EnsuredDry", format!("{EnsuredDry:?}"))
    }

    #[test]
    fn states_cleaned() {
        assert_eq!("Cleaned", format!("{Cleaned:?}"))
    }

    #[test]
    fn states_cleaned_dry() {
        assert_eq!("CleanedDry", format!("{CleanedDry:?}"))
    }
}

mod serde {
    use peace::resources::states::ts::{
        Cleaned, CleanedDry, Current, Desired, Ensured, EnsuredDry, Saved,
    };

    #[test]
    fn saved() {
        let s = serde_yaml::to_string(&Saved).unwrap();

        assert!(serde_yaml::from_str::<Saved>(&s).is_ok());
    }

    #[test]
    fn current() {
        let s = serde_yaml::to_string(&Current).unwrap();

        assert!(serde_yaml::from_str::<Current>(&s).is_ok());
    }

    #[test]
    fn desired() {
        let s = serde_yaml::to_string(&Desired).unwrap();

        assert!(serde_yaml::from_str::<Desired>(&s).is_ok());
    }

    #[test]
    fn ensured() {
        let s = serde_yaml::to_string(&Ensured).unwrap();

        assert!(serde_yaml::from_str::<Ensured>(&s).is_ok());
    }

    #[test]
    fn ensured_dry() {
        let s = serde_yaml::to_string(&EnsuredDry).unwrap();

        assert!(serde_yaml::from_str::<EnsuredDry>(&s).is_ok());
    }

    #[test]
    fn cleaned() {
        let s = serde_yaml::to_string(&Cleaned).unwrap();

        assert!(serde_yaml::from_str::<Cleaned>(&s).is_ok());
    }

    #[test]
    fn cleaned_dry() {
        let s = serde_yaml::to_string(&CleanedDry).unwrap();

        assert!(serde_yaml::from_str::<CleanedDry>(&s).is_ok());
    }
}
