use peace_cfg::OpCheckStatus;
use peace_resources::type_reg::untagged::{BoxDtDisplay, DataType};

/// Trait to allow inspecting a type-erased `ItemEnsure`.
pub trait ItemEnsureRt: DataType {
    /// Returns `state_saved` as type-erased data.
    fn state_saved(&self) -> Option<BoxDtDisplay>;

    /// Returns `state_current` as type-erased data.
    fn state_current(&self) -> BoxDtDisplay;

    /// Returns `state_desired` as type-erased data.
    fn state_desired(&self) -> BoxDtDisplay;

    /// Returns `state_diff` as type-erased data.
    fn state_diff(&self) -> BoxDtDisplay;

    /// Returns `op_check_status` as type-erased data.
    fn op_check_status(&self) -> OpCheckStatus;

    /// Returns `state_ensured` as type-erased data.
    fn state_ensured(&self) -> Option<BoxDtDisplay>;

    /// Returns self as a `&dyn DataType`;
    fn as_data_type(&self) -> &dyn DataType;

    /// Returns self as a `&mut dyn DataType`;
    fn as_data_type_mut(&mut self) -> &mut dyn DataType;
}

dyn_clone::clone_trait_object!(ItemEnsureRt);

impl ItemEnsureRt for Box<dyn ItemEnsureRt> {
    fn state_saved(&self) -> Option<BoxDtDisplay> {
        self.as_ref().state_saved()
    }

    fn state_current(&self) -> BoxDtDisplay {
        self.as_ref().state_current()
    }

    fn state_desired(&self) -> BoxDtDisplay {
        self.as_ref().state_desired()
    }

    fn state_diff(&self) -> BoxDtDisplay {
        self.as_ref().state_diff()
    }

    fn op_check_status(&self) -> OpCheckStatus {
        self.as_ref().op_check_status()
    }

    fn state_ensured(&self) -> Option<BoxDtDisplay> {
        self.as_ref().state_ensured()
    }

    fn as_data_type(&self) -> &dyn DataType {
        self.as_ref().as_data_type()
    }

    fn as_data_type_mut(&mut self) -> &mut dyn DataType {
        self.as_mut().as_data_type_mut()
    }
}

impl<'a> serde::Serialize for dyn ItemEnsureRt + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self, serializer)
    }
}
