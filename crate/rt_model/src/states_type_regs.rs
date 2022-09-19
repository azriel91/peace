use peace_cfg::ItemSpecId;
use peace_resources::type_reg::untagged::TypeReg;

/// Type registries to deserialize `StatesCurrentFile` and `StatesDesiredFile`.
#[derive(Debug, Default)]
pub struct StatesTypeRegs {
    /// Type registry for each item spec's `State<StateLogical, StatePhysical>`.
    states_current_type_reg: TypeReg<ItemSpecId>,
    /// Type registry for each item spec's `StateLogical`.
    states_desired_type_reg: TypeReg<ItemSpecId>,
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
    pub fn states_current_type_reg(&self) -> &TypeReg<ItemSpecId> {
        &self.states_current_type_reg
    }

    /// Returns a mutable reference to the current states type registry.
    ///
    /// This maps from each item spec's ID to `State<StateLogical,
    /// StatePhysical>`.
    pub fn states_current_type_reg_mut(&mut self) -> &mut TypeReg<ItemSpecId> {
        &mut self.states_current_type_reg
    }

    /// Returns a reference to the desired states type registry.
    ///
    /// This maps from each item spec's ID to `StateLogical`.
    pub fn states_desired_type_reg(&self) -> &TypeReg<ItemSpecId> {
        &self.states_desired_type_reg
    }

    /// Returns a mutable reference to the desired states type registry.
    ///
    /// This maps from each item spec's ID to `StateLogical`.
    pub fn states_desired_type_reg_mut(&mut self) -> &mut TypeReg<ItemSpecId> {
        &mut self.states_desired_type_reg
    }
}
