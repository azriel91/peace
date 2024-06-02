use peace_cfg::ApplyCheck;
use peace_resource_rt::type_reg::untagged::{BoxDtDisplay, DataType};

/// Trait to allow inspecting a type-erased `ItemApplyPartial`.
pub trait ItemApplyPartialRt: DataType {
    /// Returns `state_current_stored` as type-erased data.
    fn state_current_stored(&self) -> Option<BoxDtDisplay>;

    /// Returns `state_current` as type-erased data.
    fn state_current(&self) -> Option<BoxDtDisplay>;

    /// Returns `state_target` as type-erased data.
    fn state_target(&self) -> Option<BoxDtDisplay>;

    /// Returns `state_diff` as type-erased data.
    fn state_diff(&self) -> Option<BoxDtDisplay>;

    /// Returns `apply_check` as type-erased data.
    fn apply_check(&self) -> Option<ApplyCheck>;

    /// Returns self as a `&dyn DataType`;
    fn as_data_type(&self) -> &dyn DataType;

    /// Returns self as a `&mut dyn DataType`;
    fn as_data_type_mut(&mut self) -> &mut dyn DataType;
}

dyn_clone::clone_trait_object!(ItemApplyPartialRt);

impl ItemApplyPartialRt for Box<dyn ItemApplyPartialRt> {
    fn state_current_stored(&self) -> Option<BoxDtDisplay> {
        self.as_ref().state_current_stored()
    }

    fn state_current(&self) -> Option<BoxDtDisplay> {
        self.as_ref().state_current()
    }

    fn state_target(&self) -> Option<BoxDtDisplay> {
        self.as_ref().state_target()
    }

    fn state_diff(&self) -> Option<BoxDtDisplay> {
        self.as_ref().state_diff()
    }

    fn apply_check(&self) -> Option<ApplyCheck> {
        self.as_ref().apply_check()
    }

    fn as_data_type(&self) -> &dyn DataType {
        self.as_ref().as_data_type()
    }

    fn as_data_type_mut(&mut self) -> &mut dyn DataType {
        self.as_mut().as_data_type_mut()
    }
}

impl<'a> serde::Serialize for dyn ItemApplyPartialRt + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self, serializer)
    }
}
