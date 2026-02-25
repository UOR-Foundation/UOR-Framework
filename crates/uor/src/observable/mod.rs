//! Monodromy observables for computation path analysis.
//!
//! This module provides observables that measure topological properties of
//! computation paths in the ring Z/(2^n)Z. These observables characterize paths
//! by their monodromy invariants: winding number, total variation, and cascade
//! spectrum.
//!
//! # Monodromy Signature
//!
//! Two paths with the same monodromy signature are **monodromically equivalent**:
//! they traverse the same topological structure even if visiting different states.
//!
//! ```text
//! M(path) = (WindingNumber, TotalVariation, CascadeSpectrum)
//! ```
//!
//! # Example
//!
//! ```
//! use uor::observable::{winding_number, total_variation, cascade_spectrum};
//!
//! // A simple path: 0 → 1 → 2 → 3
//! let path = [0u64, 1, 2, 3];
//! let quantum = 8;
//!
//! let winding = winding_number(&path, quantum);
//! let variation = total_variation(&path, quantum);
//! let spectrum = cascade_spectrum(&path, quantum);
//!
//! assert_eq!(winding, 0); // No complete ring traversal
//! ```

mod cascade;
mod signature;
mod variation;
mod winding;

pub use cascade::{cascade_length, cascade_spectrum, CascadeSpectrum};
pub use signature::MonodromySignature;
pub use variation::{normalized_total_variation, total_variation, TotalVariation};
pub use winding::{winding_number, WindingNumber};

/// Trait for observables computed over computation paths.
///
/// Path observables measure global properties of a sequence of values
/// in the ring Z/(2^n)Z. Unlike point observables (like Hamming weight),
/// path observables capture the *trajectory* through the algebraic space.
pub trait PathObservable {
    /// The type of value this observable produces.
    type Value;

    /// Compute the observable over a path at the given quantum level.
    ///
    /// # Arguments
    ///
    /// * `path` - Sequence of values representing the computation path
    /// * `quantum` - Bit width of the ring (e.g., 8 for Z/256Z, 16 for Z/65536Z)
    fn compute(path: &[u64], quantum: u32) -> Self::Value;
}

/// Compute the stratum (Hamming weight) of a value at the given quantum level.
///
/// This is a helper function used by several observables.
/// Uses O(1) LUT lookup for Q0 (8-bit) and Q1 (16-bit) quantum levels.
#[inline]
pub fn stratum(value: u64, quantum: u32) -> u32 {
    use crate::lut::{stratum_q0, stratum_q1};

    match quantum {
        // O(1) LUT for Q0
        8 => stratum_q0(value as u8) as u32,
        // O(1) LUT for Q1
        16 => stratum_q1(value as u16) as u32,
        // General case for larger quantum levels
        _ => {
            let mask = if quantum >= 64 {
                u64::MAX
            } else {
                (1u64 << quantum) - 1
            };
            (value & mask).count_ones()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stratum() {
        assert_eq!(stratum(0, 8), 0);
        assert_eq!(stratum(1, 8), 1);
        assert_eq!(stratum(3, 8), 2);
        assert_eq!(stratum(255, 8), 8);
        assert_eq!(stratum(256, 8), 0); // Masked to 8 bits
        assert_eq!(stratum(257, 8), 1); // 257 & 255 = 1

        // Q1 (16-bit)
        assert_eq!(stratum(0xFFFF, 16), 16);
        assert_eq!(stratum(0x10000, 16), 0);
    }
}
