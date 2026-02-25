use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;

use super::cell::Cell;
use super::mutation::BoundaryMutation;
use crate::core::constants::BOUNDARY_SIZE;

#[derive(Clone)]
pub struct ToroidalBoundary {
    cells: Box<[u8; BOUNDARY_SIZE]>,
}

impl ToroidalBoundary {
    #[must_use]
    pub fn new_zero() -> Self {
        Self {
            cells: Box::new([0u8; BOUNDARY_SIZE]),
        }
    }

    #[must_use]
    pub fn new_one() -> Self {
        Self {
            cells: Box::new([1u8; BOUNDARY_SIZE]),
        }
    }

    #[must_use]
    pub fn new_fill(value: u8) -> Self {
        Self {
            cells: Box::new([value; BOUNDARY_SIZE]),
        }
    }

    #[must_use]
    pub fn from_fn<F>(f: F) -> Self
    where
        F: Fn(usize) -> u8,
    {
        let mut cells = Box::new([0u8; BOUNDARY_SIZE]);
        for (i, cell) in cells.iter_mut().enumerate() {
            *cell = f(i);
        }
        Self { cells }
    }

    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut cells = Box::new([0u8; BOUNDARY_SIZE]);
        for (i, &byte) in bytes.iter().take(BOUNDARY_SIZE).enumerate() {
            cells[i] = byte;
        }
        Self { cells }
    }

    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.cells.to_vec()
    }

    #[must_use]
    pub fn get(&self, cell: Cell) -> u8 {
        self.cells[cell.index()]
    }

    pub fn set(&mut self, cell: Cell, value: u8) {
        self.cells[cell.index()] = value;
    }

    #[must_use]
    pub fn get_index(&self, index: usize) -> u8 {
        self.cells[index % BOUNDARY_SIZE]
    }

    pub fn set_index(&mut self, index: usize, value: u8) {
        self.cells[index % BOUNDARY_SIZE] = value;
    }

    #[must_use]
    pub fn cells(&self) -> &[u8; BOUNDARY_SIZE] {
        &self.cells
    }

    #[must_use]
    pub fn cells_mut(&mut self) -> &mut [u8; BOUNDARY_SIZE] {
        &mut self.cells
    }

    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.cells[..]
    }

    pub fn apply(&mut self, mutation: &BoundaryMutation) {
        match mutation {
            BoundaryMutation::Set { cell, value } => {
                self.cells[cell.index()] = *value;
            }
            BoundaryMutation::Add { cell, delta } => {
                let idx = cell.index();
                if *delta >= 0 {
                    self.cells[idx] = self.cells[idx].wrapping_add(*delta as u8);
                } else {
                    self.cells[idx] = self.cells[idx].wrapping_sub((-*delta) as u8);
                }
            }
            BoundaryMutation::Mirror { cell } => {
                let idx = cell.index();
                self.cells[idx] = !self.cells[idx];
            }
            BoundaryMutation::Swap { cell1, cell2 } => {
                self.cells.swap(cell1.index(), cell2.index());
            }
            BoundaryMutation::Multiply { cell, scalar }
            | BoundaryMutation::Mul { cell, scalar } => {
                let idx = cell.index();
                self.cells[idx] = self.cells[idx].wrapping_mul(*scalar);
            }
            BoundaryMutation::MulFrom { source, target } => {
                let s = self.cells[source.index()];
                let t = target.index();
                self.cells[t] = self.cells[t].wrapping_mul(s);
            }
            BoundaryMutation::Fma { a, b, target } => {
                let product = self.cells[a.index()].wrapping_mul(self.cells[b.index()]);
                let t = target.index();
                self.cells[t] = self.cells[t].wrapping_add(product);
            }
            BoundaryMutation::BitwiseAnd { source, target } => {
                self.cells[target.index()] &= self.cells[source.index()];
            }
            BoundaryMutation::BitwiseOr { source, target } => {
                self.cells[target.index()] |= self.cells[source.index()];
            }
            BoundaryMutation::BitwiseXor { source, target } => {
                self.cells[target.index()] ^= self.cells[source.index()];
            }
            BoundaryMutation::BitwiseNot { cell } => {
                let idx = cell.index();
                self.cells[idx] = !self.cells[idx];
            }
            BoundaryMutation::ShiftLeft { cell, amount } => {
                let idx = cell.index();
                self.cells[idx] = self.cells[idx].wrapping_shl((*amount).into());
            }
            BoundaryMutation::ShiftRight { cell, amount } => {
                let idx = cell.index();
                self.cells[idx] = self.cells[idx].wrapping_shr((*amount).into());
            }
            BoundaryMutation::IndexedWrite { index_cell, value } => {
                let target = self.cells[index_cell.index()] as usize % BOUNDARY_SIZE;
                self.cells[target] = *value;
            }
            BoundaryMutation::IndexedCopy { index_cell, source } => {
                let value = self.cells[source.index()];
                let target = self.cells[index_cell.index()] as usize % BOUNDARY_SIZE;
                self.cells[target] = value;
            }
            BoundaryMutation::IndirectRead {
                base_cell,
                offset,
                target,
            } => {
                let index = self.cells[base_cell.index()] as usize;
                let effective_addr = (index + *offset) % BOUNDARY_SIZE;
                self.cells[target.index()] = self.cells[effective_addr];
            }
            BoundaryMutation::IndirectMulFrom {
                base_cell,
                offset,
                target,
            } => {
                let index = self.cells[base_cell.index()] as usize;
                let effective_addr = (index + *offset) % BOUNDARY_SIZE;
                let target_idx = target.index();
                self.cells[target_idx] =
                    self.cells[target_idx].wrapping_mul(self.cells[effective_addr]);
            }
        }
    }

    pub fn apply_batch(&mut self, mutations: &[BoundaryMutation]) {
        for mutation in mutations {
            self.apply(mutation);
        }
    }

    #[must_use]
    pub fn total_budget(&self) -> i64 {
        self.cells.iter().map(|&c| i64::from(c)).sum()
    }

    #[must_use]
    pub fn signed_budget(&self) -> i64 {
        self.cells.iter().map(|&c| i64::from(c as i8)).sum()
    }

    pub fn clear(&mut self) {
        self.cells.fill(0);
    }

    pub fn fill_range(&mut self, start: Cell, count: usize, value: u8) {
        for i in 0..count {
            self.set(start.offset(i as isize), value);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Cell, u8)> + '_ {
        self.cells
            .iter()
            .enumerate()
            .map(|(i, v)| (Cell::new(i), *v))
    }

    pub fn iter_range(&self, start: Cell, count: usize) -> impl Iterator<Item = (Cell, u8)> + '_ {
        (0..count).map(move |i| {
            let cell = start.offset(i as isize);
            (cell, self.get(cell))
        })
    }
}

impl Default for ToroidalBoundary {
    fn default() -> Self {
        Self::new_zero()
    }
}

impl fmt::Debug for ToroidalBoundary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let non_zero_count = self.cells.iter().filter(|&&v| v != 0).count();
        write!(
            f,
            "ToroidalBoundary {{ {} non-zero cells, budget: {} }}",
            non_zero_count,
            self.total_budget()
        )
    }
}

impl PartialEq for ToroidalBoundary {
    fn eq(&self, other: &Self) -> bool {
        self.cells[..] == other.cells[..]
    }
}

impl Eq for ToroidalBoundary {}
