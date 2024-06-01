use peace_cfg::ApplyCheck;
use peace_resources::type_reg::untagged::{BoxDtDisplay, DataType};

/// Trait to allow inspecting a type-erased `StepApply`.
pub trait StepApplyRt: DataType {
    /// Returns `state_current_stored` as type-erased data.
    fn state_current_stored(&self) -> Option<BoxDtDisplay>;

    /// Returns `state_current` as type-erased data.
    fn state_current(&self) -> BoxDtDisplay;

    /// Returns `state_target` as type-erased data.
    fn state_target(&self) -> BoxDtDisplay;

    /// Returns `state_diff` as type-erased data.
    fn state_diff(&self) -> BoxDtDisplay;

    /// Returns `apply_check` as type-erased data.
    fn apply_check(&self) -> ApplyCheck;

    /// Returns `state_applied` as type-erased data.
    fn state_applied(&self) -> Option<BoxDtDisplay>;

    /// Returns self as a `&dyn DataType`;
    fn as_data_type(&self) -> &dyn DataType;

    /// Returns self as a `&mut dyn DataType`;
    fn as_data_type_mut(&mut self) -> &mut dyn DataType;
}

dyn_clone::clone_trait_object!(StepApplyRt);

impl StepApplyRt for Box<dyn StepApplyRt> {
    fn state_current_stored(&self) -> Option<BoxDtDisplay> {
        self.as_ref().state_current_stored()
    }

    fn state_current(&self) -> BoxDtDisplay {
        self.as_ref().state_current()
    }

    fn state_target(&self) -> BoxDtDisplay {
        self.as_ref().state_target()
    }

    fn state_diff(&self) -> BoxDtDisplay {
        self.as_ref().state_diff()
    }

    fn apply_check(&self) -> ApplyCheck {
        self.as_ref().apply_check()
    }

    fn state_applied(&self) -> Option<BoxDtDisplay> {
        self.as_ref().state_applied()
    }

    fn as_data_type(&self) -> &dyn DataType {
        self.as_ref().as_data_type()
    }

    fn as_data_type_mut(&mut self) -> &mut dyn DataType {
        self.as_mut().as_data_type_mut()
    }
}

impl<'a> serde::Serialize for dyn StepApplyRt + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self, serializer)
    }
}
