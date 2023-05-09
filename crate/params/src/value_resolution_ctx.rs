use std::fmt;

use peace_core::ItemSpecId;

use crate::FieldNameAndType;

/// Collects information about how a value is resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueResolutionCtx {
    /// ID of the item spec whose params are being resolved.
    item_spec_id: ItemSpecId,
    /// Name of the `ItemSpec::Params` type.
    params_type_name: &'static str,
    /// Hierarchy of fields traversed to resolve this value.
    resolution_chain: Vec<FieldNameAndType>,
}

impl ValueResolutionCtx {
    pub fn new(item_spec_id: ItemSpecId, params_type_name: &'static str) -> Self {
        Self {
            item_spec_id,
            params_type_name,
            resolution_chain: Vec::new(),
        }
    }

    pub fn item_spec_id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    pub fn params_type_name(&self) -> &str {
        self.params_type_name
    }

    pub fn resolution_chain(&self) -> &[FieldNameAndType] {
        self.resolution_chain.as_ref()
    }

    /// Appends a field name and type to the resolution chain.
    pub fn push(&mut self, field_name_and_type: FieldNameAndType) {
        self.resolution_chain.push(field_name_and_type);
    }

    /// Removes a field name and type from the resolution chain.
    pub fn pop(&mut self) {
        self.resolution_chain.pop();
    }
}

impl fmt::Display for ValueResolutionCtx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params_type_name = self.params_type_name();
        write!(f, "{params_type_name} {{\n}}")?;

        self.resolution_chain().iter().enumerate().try_for_each(
            |(indentation, field_name_and_type)| {
                (0..indentation).try_for_each(|_| write!(f, "    "))?;

                let field_name = field_name_and_type.field_name();
                let type_name = field_name_and_type.type_name();
                write!(f, "{field_name}: {type_name} {{")?;

                (0..indentation).try_for_each(|_| write!(f, "    "))?;
                write!(f, "..\n")
            },
        )?;
        (0..self.resolution_chain().len())
            .rev()
            .skip(1)
            .try_for_each(|indentation| {
                (0..indentation).try_for_each(|_| write!(f, "    "))?;
                write!(f, "}}")
            })?;

        write!(f, "\n}}")
    }
}
