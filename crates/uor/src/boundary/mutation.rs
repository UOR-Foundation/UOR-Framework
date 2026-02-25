use alloc::vec::Vec;
use core::fmt;

use super::cell::Cell;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundaryMutation {
    Set {
        cell: Cell,
        value: u8,
    },
    Add {
        cell: Cell,
        delta: i8,
    },
    Mirror {
        cell: Cell,
    },
    Swap {
        cell1: Cell,
        cell2: Cell,
    },
    Multiply {
        cell: Cell,
        scalar: u8,
    },
    Mul {
        cell: Cell,
        scalar: u8,
    },
    MulFrom {
        source: Cell,
        target: Cell,
    },
    Fma {
        a: Cell,
        b: Cell,
        target: Cell,
    },
    BitwiseAnd {
        source: Cell,
        target: Cell,
    },
    BitwiseOr {
        source: Cell,
        target: Cell,
    },
    BitwiseXor {
        source: Cell,
        target: Cell,
    },
    BitwiseNot {
        cell: Cell,
    },
    ShiftLeft {
        cell: Cell,
        amount: u8,
    },
    ShiftRight {
        cell: Cell,
        amount: u8,
    },
    IndexedWrite {
        index_cell: Cell,
        value: u8,
    },
    IndexedCopy {
        index_cell: Cell,
        source: Cell,
    },
    IndirectRead {
        base_cell: Cell,
        offset: usize,
        target: Cell,
    },
    IndirectMulFrom {
        base_cell: Cell,
        offset: usize,
        target: Cell,
    },
}

impl BoundaryMutation {
    #[must_use]
    pub const fn set(cell: Cell, value: u8) -> Self {
        Self::Set { cell, value }
    }

    #[must_use]
    pub const fn add(cell: Cell, delta: i8) -> Self {
        Self::Add { cell, delta }
    }

    #[must_use]
    pub const fn mirror(cell: Cell) -> Self {
        Self::Mirror { cell }
    }

    #[must_use]
    pub const fn swap(cell1: Cell, cell2: Cell) -> Self {
        Self::Swap { cell1, cell2 }
    }

    #[must_use]
    pub const fn multiply(cell: Cell, scalar: u8) -> Self {
        Self::Multiply { cell, scalar }
    }

    #[must_use]
    pub const fn indirect_read(base_cell: Cell, offset: usize, target: Cell) -> Self {
        Self::IndirectRead {
            base_cell,
            offset,
            target,
        }
    }

    #[must_use]
    pub const fn indirect_mul_from(base_cell: Cell, offset: usize, target: Cell) -> Self {
        Self::IndirectMulFrom {
            base_cell,
            offset,
            target,
        }
    }

    #[must_use]
    pub const fn primary_cell(&self) -> Cell {
        match self {
            Self::Set { cell, .. }
            | Self::Add { cell, .. }
            | Self::Mirror { cell }
            | Self::Multiply { cell, .. }
            | Self::Mul { cell, .. }
            | Self::BitwiseNot { cell }
            | Self::ShiftLeft { cell, .. }
            | Self::ShiftRight { cell, .. } => *cell,
            Self::Swap { cell1, .. } => *cell1,
            Self::MulFrom { target, .. }
            | Self::Fma { target, .. }
            | Self::BitwiseAnd { target, .. }
            | Self::BitwiseOr { target, .. }
            | Self::BitwiseXor { target, .. }
            | Self::IndirectRead { target, .. }
            | Self::IndirectMulFrom { target, .. } => *target,
            Self::IndexedWrite { index_cell, .. } | Self::IndexedCopy { index_cell, .. } => {
                *index_cell
            }
        }
    }

    #[must_use]
    pub const fn affected_cells(&self) -> (Cell, Option<Cell>) {
        match self {
            Self::Set { cell, .. }
            | Self::Add { cell, .. }
            | Self::Mirror { cell }
            | Self::Multiply { cell, .. }
            | Self::Mul { cell, .. }
            | Self::BitwiseNot { cell }
            | Self::ShiftLeft { cell, .. }
            | Self::ShiftRight { cell, .. } => (*cell, None),
            Self::Swap { cell1, cell2 } => (*cell1, Some(*cell2)),
            Self::MulFrom { source, target }
            | Self::BitwiseAnd { source, target }
            | Self::BitwiseOr { source, target }
            | Self::BitwiseXor { source, target }
            | Self::IndexedCopy {
                index_cell: target,
                source,
            } => (*target, Some(*source)),
            Self::Fma { a, target, .. } => (*target, Some(*a)),
            Self::IndexedWrite { index_cell, .. }
            | Self::IndirectRead {
                target: index_cell, ..
            }
            | Self::IndirectMulFrom {
                target: index_cell, ..
            } => (*index_cell, None),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Set { .. } => "Set",
            Self::Add { .. } => "Add",
            Self::Mirror { .. } => "Mirror",
            Self::Swap { .. } => "Swap",
            Self::Multiply { .. } => "Multiply",
            Self::Mul { .. } => "Mul",
            Self::MulFrom { .. } => "MulFrom",
            Self::Fma { .. } => "Fma",
            Self::BitwiseAnd { .. } => "BitwiseAnd",
            Self::BitwiseOr { .. } => "BitwiseOr",
            Self::BitwiseXor { .. } => "BitwiseXor",
            Self::BitwiseNot { .. } => "BitwiseNot",
            Self::ShiftLeft { .. } => "ShiftLeft",
            Self::ShiftRight { .. } => "ShiftRight",
            Self::IndexedWrite { .. } => "IndexedWrite",
            Self::IndexedCopy { .. } => "IndexedCopy",
            Self::IndirectRead { .. } => "IndirectRead",
            Self::IndirectMulFrom { .. } => "IndirectMulFrom",
        }
    }
}

impl fmt::Display for BoundaryMutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Set { cell, value } => write!(f, "set({cell}) = {value}"),
            Self::Add { cell, delta } => write!(f, "add({cell}) += {delta}"),
            Self::Mirror { cell } => write!(f, "mirror({cell})"),
            Self::Swap { cell1, cell2 } => write!(f, "swap({cell1}, {cell2})"),
            Self::Multiply { cell, scalar } | Self::Mul { cell, scalar } => {
                write!(f, "mul({cell}) *= {scalar}")
            }
            Self::MulFrom { source, target } => write!(f, "mul_from({target}, {source})"),
            Self::Fma { a, b, target } => write!(f, "fma({target}) += [{a}] * [{b}]"),
            Self::BitwiseAnd { source, target } => write!(f, "and({target}) &= [{source}]"),
            Self::BitwiseOr { source, target } => write!(f, "or({target}) |= [{source}]"),
            Self::BitwiseXor { source, target } => write!(f, "xor({target}) ^= [{source}]"),
            Self::BitwiseNot { cell } => write!(f, "not({cell})"),
            Self::ShiftLeft { cell, amount } => write!(f, "shl({cell}) <<= {amount}"),
            Self::ShiftRight { cell, amount } => write!(f, "shr({cell}) >>= {amount}"),
            Self::IndexedWrite { index_cell, value } => {
                write!(f, "idx_write([{index_cell}], {value})")
            }
            Self::IndexedCopy { index_cell, source } => {
                write!(f, "idx_copy([{index_cell}], [{source}])")
            }
            Self::IndirectRead {
                base_cell,
                offset,
                target,
            } => write!(f, "indirect_read({target}, [{base_cell}] + {offset})"),
            Self::IndirectMulFrom {
                base_cell,
                offset,
                target,
            } => write!(f, "indirect_mul({target}, [{base_cell}] + {offset})"),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MutationBatch {
    mutations: Vec<BoundaryMutation>,
}

impl MutationBatch {
    #[must_use]
    pub fn new() -> Self {
        Self {
            mutations: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            mutations: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, mutation: BoundaryMutation) {
        self.mutations.push(mutation);
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.mutations.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.mutations.is_empty()
    }

    #[must_use]
    pub fn as_slice(&self) -> &[BoundaryMutation] {
        &self.mutations
    }

    pub fn clear(&mut self) {
        self.mutations.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &BoundaryMutation> {
        self.mutations.iter()
    }
}

impl FromIterator<BoundaryMutation> for MutationBatch {
    fn from_iter<T: IntoIterator<Item = BoundaryMutation>>(iter: T) -> Self {
        Self {
            mutations: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for MutationBatch {
    type Item = BoundaryMutation;
    type IntoIter = alloc::vec::IntoIter<BoundaryMutation>;

    fn into_iter(self) -> Self::IntoIter {
        self.mutations.into_iter()
    }
}

impl<'a> IntoIterator for &'a MutationBatch {
    type Item = &'a BoundaryMutation;
    type IntoIter = core::slice::Iter<'a, BoundaryMutation>;

    fn into_iter(self) -> Self::IntoIter {
        self.mutations.iter()
    }
}
