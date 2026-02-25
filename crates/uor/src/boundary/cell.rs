//! Boundary cell address type for toroidal computation.
//!
//! A cell is an address in the 12,288-cell toroidal boundary substrate.
//! The boundary has toroidal topology, so arithmetic wraps around.

use core::fmt;

use crate::core::constants::BOUNDARY_SIZE;

/// A cell address in the toroidal boundary.
///
/// Cell indices are in the range [0, 12287] (BOUNDARY_SIZE - 1).
/// The boundary has toroidal topology, so arithmetic wraps around.
///
/// # Properties
///
/// - Size: 2 bytes (u16)
/// - Range: [0, 12287]
/// - Topology: Toroidal (wraps around)
///
/// # Example
///
/// ```
/// use uor::Cell;
///
/// let cell = Cell::new(100);
/// assert_eq!(cell.index(), 100);
///
/// // Wrap-around behavior
/// let wrapped = Cell::new(12288);
/// assert_eq!(wrapped.index(), 0);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Cell(u16);

impl Cell {
    /// The first cell (index 0).
    pub const ZERO: Self = Self(0);

    /// The last cell (index 12287).
    pub const MAX: Self = Self((BOUNDARY_SIZE - 1) as u16);

    /// Creates a new Cell from any usize value, wrapping modulo BOUNDARY_SIZE.
    #[inline]
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self((index % BOUNDARY_SIZE) as u16)
    }

    /// Creates a Cell from a u16 value, wrapping modulo BOUNDARY_SIZE.
    #[inline]
    #[must_use]
    pub const fn from_u16(index: u16) -> Self {
        Self(index % (BOUNDARY_SIZE as u16))
    }

    /// Creates a Cell from a raw u16 value without bounds checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure `index < BOUNDARY_SIZE`.
    #[inline]
    #[must_use]
    pub const unsafe fn from_u16_unchecked(index: u16) -> Self {
        debug_assert!((index as usize) < BOUNDARY_SIZE);
        Self(index)
    }

    /// Returns the cell index as usize.
    #[inline]
    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    /// Returns the cell index as u16.
    #[inline]
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0
    }

    /// Returns the next cell, wrapping at boundary end.
    #[inline]
    #[must_use]
    pub const fn next(self) -> Self {
        Self::new(self.0 as usize + 1)
    }

    /// Returns the previous cell, wrapping at boundary start.
    #[inline]
    #[must_use]
    pub const fn prev(self) -> Self {
        if self.0 == 0 {
            Self::MAX
        } else {
            Self(self.0 - 1)
        }
    }

    /// Adds an offset to the cell index, wrapping modulo BOUNDARY_SIZE.
    #[inline]
    #[must_use]
    pub const fn offset(self, delta: isize) -> Self {
        let index = self.0 as isize;
        let new_index = index + delta;
        // Handle negative wrap-around
        let wrapped = new_index.rem_euclid(BOUNDARY_SIZE as isize);
        Self(wrapped as u16)
    }

    /// Returns the distance from this cell to another (always positive).
    ///
    /// Takes the shorter path around the torus.
    #[inline]
    #[must_use]
    pub const fn distance_to(self, other: Self) -> usize {
        let a = self.0 as isize;
        let b = other.0 as isize;
        let diff = (b - a).unsigned_abs();
        // Take the shorter path around the torus
        if diff <= BOUNDARY_SIZE / 2 {
            diff
        } else {
            BOUNDARY_SIZE - diff
        }
    }

    /// Returns a range of cells starting from this cell.
    #[inline]
    #[must_use]
    pub const fn range(self, count: usize) -> CellRange {
        CellRange { start: self, count }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cell({})", self.0)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for Cell {
    #[inline]
    fn from(index: usize) -> Self {
        Self::new(index)
    }
}

impl From<u16> for Cell {
    #[inline]
    fn from(index: u16) -> Self {
        Self::from_u16(index)
    }
}

impl From<u32> for Cell {
    #[inline]
    fn from(index: u32) -> Self {
        Self::new(index as usize)
    }
}

/// A range of consecutive cells in the boundary.
///
/// Ranges wrap around the boundary edge, providing toroidal iteration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellRange {
    start: Cell,
    count: usize,
}

impl CellRange {
    /// Creates a new cell range.
    #[inline]
    #[must_use]
    pub const fn new(start: Cell, count: usize) -> Self {
        Self { start, count }
    }

    /// Returns the starting cell.
    #[inline]
    #[must_use]
    pub const fn start(&self) -> Cell {
        self.start
    }

    /// Returns the number of cells in the range.
    #[inline]
    #[must_use]
    pub const fn count(&self) -> usize {
        self.count
    }

    /// Returns true if the range is empty.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns the cell at a given offset within the range.
    #[inline]
    #[must_use]
    pub const fn get(&self, offset: usize) -> Option<Cell> {
        if offset < self.count {
            Some(self.start.offset(offset as isize))
        } else {
            None
        }
    }
}

impl IntoIterator for CellRange {
    type Item = Cell;
    type IntoIter = CellRangeIter;

    fn into_iter(self) -> Self::IntoIter {
        CellRangeIter {
            current: self.start,
            remaining: self.count,
        }
    }
}

/// Iterator over a cell range.
pub struct CellRangeIter {
    current: Cell,
    remaining: usize,
}

impl Iterator for CellRangeIter {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            let cell = self.current;
            self.current = self.current.next();
            self.remaining -= 1;
            Some(cell)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for CellRangeIter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Cell::new(0).index(), 0);
        assert_eq!(Cell::new(100).index(), 100);
        assert_eq!(Cell::new(12287).index(), 12287);
        assert_eq!(Cell::new(12288).index(), 0);
        assert_eq!(Cell::new(12289).index(), 1);
        assert_eq!(Cell::new(24576).index(), 0);
    }

    #[test]
    fn test_next_prev() {
        assert_eq!(Cell::new(0).next().index(), 1);
        assert_eq!(Cell::new(12287).next().index(), 0);
        assert_eq!(Cell::new(0).prev().index(), 12287);
        assert_eq!(Cell::new(1).prev().index(), 0);
    }

    #[test]
    fn test_offset() {
        assert_eq!(Cell::new(100).offset(10).index(), 110);
        assert_eq!(Cell::new(100).offset(-10).index(), 90);
        assert_eq!(Cell::new(0).offset(-1).index(), 12287);
        assert_eq!(Cell::new(12287).offset(1).index(), 0);
        assert_eq!(Cell::new(12287).offset(100).index(), 99);
    }

    #[test]
    fn test_distance() {
        assert_eq!(Cell::new(0).distance_to(Cell::new(100)), 100);
        assert_eq!(Cell::new(100).distance_to(Cell::new(0)), 100);
        // Shorter path around the torus
        assert_eq!(Cell::new(0).distance_to(Cell::new(12000)), 288);
        assert_eq!(Cell::new(12000).distance_to(Cell::new(0)), 288);
    }

    #[test]
    fn test_range() {
        let range = Cell::new(100).range(5);
        assert_eq!(range.start().index(), 100);
        assert_eq!(range.count(), 5);
        assert!(!range.is_empty());

        let cells: Vec<_> = range.into_iter().collect();
        assert_eq!(cells.len(), 5);
        assert_eq!(cells[0].index(), 100);
        assert_eq!(cells[4].index(), 104);
    }

    #[test]
    fn test_range_wrap() {
        let range = Cell::new(12285).range(5);
        let cells: Vec<_> = range.into_iter().collect();
        assert_eq!(cells.len(), 5);
        assert_eq!(cells[0].index(), 12285);
        assert_eq!(cells[1].index(), 12286);
        assert_eq!(cells[2].index(), 12287);
        assert_eq!(cells[3].index(), 0);
        assert_eq!(cells[4].index(), 1);
    }

    #[test]
    fn test_empty_range() {
        let range = Cell::new(100).range(0);
        assert!(range.is_empty());
        assert_eq!(range.into_iter().count(), 0);
    }

    #[test]
    fn test_constants() {
        assert_eq!(Cell::ZERO.index(), 0);
        assert_eq!(Cell::MAX.index(), 12287);
    }
}
