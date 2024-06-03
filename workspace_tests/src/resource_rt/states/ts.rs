mod debug {
    use peace::resource_rt::states::ts::{
        Clean, Cleaned, CleanedDry, Current, CurrentStored, Ensured, EnsuredDry, Goal, GoalStored,
        Previous,
    };

    #[test]
    fn states_clean() {
        assert_eq!("Clean", format!("{Clean:?}"))
    }

    #[test]
    fn states_current_stored() {
        assert_eq!("CurrentStored", format!("{CurrentStored:?}"))
    }

    #[test]
    fn states_current() {
        assert_eq!("Current", format!("{Current:?}"))
    }

    #[test]
    fn states_goal_stored() {
        assert_eq!("GoalStored", format!("{GoalStored:?}"))
    }

    #[test]
    fn states_goal() {
        assert_eq!("Goal", format!("{Goal:?}"))
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

    #[test]
    fn states_previous() {
        assert_eq!("Previous", format!("{Previous:?}"))
    }
}

mod serde {
    use peace::resource_rt::states::ts::{
        Clean, Cleaned, CleanedDry, Current, CurrentStored, Ensured, EnsuredDry, Goal, GoalStored,
        Previous,
    };

    #[test]
    fn clean() {
        let s = serde_yaml::to_string(&Clean).unwrap();

        assert!(serde_yaml::from_str::<Clean>(&s).is_ok());
    }

    #[test]
    fn current_stored() {
        let s = serde_yaml::to_string(&CurrentStored).unwrap();

        assert!(serde_yaml::from_str::<CurrentStored>(&s).is_ok());
    }

    #[test]
    fn current() {
        let s = serde_yaml::to_string(&Current).unwrap();

        assert!(serde_yaml::from_str::<Current>(&s).is_ok());
    }

    #[test]
    fn goal_stored() {
        let s = serde_yaml::to_string(&GoalStored).unwrap();

        assert!(serde_yaml::from_str::<GoalStored>(&s).is_ok());
    }

    #[test]
    fn goal() {
        let s = serde_yaml::to_string(&Goal).unwrap();

        assert!(serde_yaml::from_str::<Goal>(&s).is_ok());
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

    #[test]
    fn previous() {
        let s = serde_yaml::to_string(&Previous).unwrap();

        assert!(serde_yaml::from_str::<Previous>(&s).is_ok());
    }
}
