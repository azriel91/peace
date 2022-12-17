use peace_cfg::ItemSpecId;
use peace_resources::type_reg::untagged::{BoxDtDisplay, TypeReg};

/// Type registries to deserialize `StatesPreviousFile` and `StatesDesiredFile`.
#[derive(Debug, Default)]
pub struct StatesTypeRegs {
    /// Type registry for each item spec's `State<StateLogical, StatePhysical>`.
    states_current_type_reg: TypeReg<ItemSpecId, BoxDtDisplay>,
    /// Type registry for each item spec's `StateLogical`.
    states_desired_type_reg: TypeReg<ItemSpecId, BoxDtDisplay>,
}

impl StatesTypeRegs {
    /// Returns new `StatesTypeRegs`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to the current states type registry.
    ///
    /// This maps from each item spec's ID to `State<StateLogical,
    /// StatePhysical>`.
    pub fn states_current_type_reg(&self) -> &TypeReg<ItemSpecId, BoxDtDisplay> {
        &self.states_current_type_reg
    }

    /// Returns a mutable reference to the current states type registry.
    ///
    /// This maps from each item spec's ID to `State<StateLogical,
    /// StatePhysical>`.
    pub fn states_current_type_reg_mut(&mut self) -> &mut TypeReg<ItemSpecId, BoxDtDisplay> {
        &mut self.states_current_type_reg
    }

    /// Returns a reference to the desired states type registry.
    ///
    /// This maps from each item spec's ID to `StateLogical`.
    pub fn states_desired_type_reg(&self) -> &TypeReg<ItemSpecId, BoxDtDisplay> {
        &self.states_desired_type_reg
    }

    /// Returns a mutable reference to the desired states type registry.
    ///
    /// This maps from each item spec's ID to `StateLogical`.
    pub fn states_desired_type_reg_mut(&mut self) -> &mut TypeReg<ItemSpecId, BoxDtDisplay> {
        &mut self.states_desired_type_reg
    }
}
