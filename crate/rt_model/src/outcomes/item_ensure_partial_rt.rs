use peace_cfg::OpCheckStatus;
use peace_resources::type_reg::untagged::{DataType, DataTypeDisplay};

/// Trait to allow inspecting a type-erased `ItemEnsurePartial`.
pub trait ItemEnsurePartialRt: DataType {
    /// Returns `state_saved` as type-erased data.
    fn state_saved(&self) -> Option<Box<dyn DataTypeDisplay>>;

    /// Returns `state_current` as type-erased data.
    fn state_current(&self) -> Option<Box<dyn DataTypeDisplay>>;

    /// Returns `state_desired` as type-erased data.
    fn state_desired(&self) -> Option<Box<dyn DataTypeDisplay>>;

    /// Returns `state_diff` as type-erased data.
    fn state_diff(&self) -> Option<Box<dyn DataTypeDisplay>>;

    /// Returns `op_check_status` as type-erased data.
    fn op_check_status(&self) -> Option<OpCheckStatus>;

    /// Returns self as a `&dyn DataType`;
    fn as_data_type(&self) -> &dyn DataType;

    /// Returns self as a `&mut dyn DataType`;
    fn as_data_type_mut(&mut self) -> &mut dyn DataType;
}

dyn_clone::clone_trait_object!(ItemEnsurePartialRt);

impl ItemEnsurePartialRt for Box<dyn ItemEnsurePartialRt> {
    fn state_saved(&self) -> Option<Box<dyn DataTypeDisplay>> {
        self.as_ref().state_saved()
    }

    fn state_current(&self) -> Option<Box<dyn DataTypeDisplay>> {
        self.as_ref().state_current()
    }

    fn state_desired(&self) -> Option<Box<dyn DataTypeDisplay>> {
        self.as_ref().state_desired()
    }

    fn state_diff(&self) -> Option<Box<dyn DataTypeDisplay>> {
        self.as_ref().state_diff()
    }

    fn op_check_status(&self) -> Option<OpCheckStatus> {
        self.as_ref().op_check_status()
    }

    fn as_data_type(&self) -> &dyn DataType {
        self.as_ref().as_data_type()
    }

    fn as_data_type_mut(&mut self) -> &mut dyn DataType {
        self.as_mut().as_data_type_mut()
    }
}

impl<'a> serde::Serialize for dyn ItemEnsurePartialRt + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self, serializer)
    }
}