use acvm::FieldElement;

use super::map::Id;

/// Represents a numeric constant in Ssa. Constants themselves are
/// uniqued in the DataFlowGraph and immutable.
///
/// This is just a thin wrapper around FieldElement so that
/// we can use Id<NumericConstant> without it getting confused
/// with a possible future use of Id<FieldElement>.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct NumericConstant(FieldElement);

impl NumericConstant {
    pub(crate) fn new(value: FieldElement) -> Self {
        Self(value)
    }

    pub(crate) fn value(&self) -> FieldElement {
        self.0
    }
}

pub(crate) type NumericConstantId = Id<NumericConstant>;

impl std::ops::Add for NumericConstant {
    type Output = NumericConstant;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}

impl std::ops::Sub for NumericConstant {
    type Output = NumericConstant;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}

impl std::ops::Mul for NumericConstant {
    type Output = NumericConstant;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

impl std::ops::Div for NumericConstant {
    type Output = NumericConstant;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.0 / rhs.0)
    }
}
