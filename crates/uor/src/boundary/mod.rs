//! Toroidal boundary substrate.
//!
//! This module contains the categorical computation substrate:
//!
//! - [`substrate::ToroidalBoundary`]: 12,288-cell toroidal execution substrate
//! - [`cell::Cell`]: Cell address type (wrapping to 12,288 cells)
//! - [`mutation::BoundaryMutation`]: Atomic mutation generators (Set, Add, Mirror, Swap, Multiply)

pub mod cell;
pub mod mutation;
pub mod substrate;
