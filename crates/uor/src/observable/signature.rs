//! Monodromy signature - composite observable for path classification.
//!
//! The monodromy signature combines winding number, total variation, and cascade
//! spectrum into a single classification invariant. Two paths with equal signatures
//! are monodromically equivalent.

use super::{cascade_spectrum, total_variation, winding_number, PathObservable};
use alloc::vec::Vec;

/// Complete monodromy classification of a computation path.
///
/// Two paths with equal monodromy signatures are **monodromically equivalent**:
/// they exhibit the same topological traversal pattern even if visiting
/// different specific states.
///
/// # Components
///
/// - **Winding number**: Net cycles around the ring (signed integer)
/// - **Total variation**: Cumulative stratum change (unsigned integer)
/// - **Cascade spectrum**: Histogram of cascade lengths (vector)
///
/// # Example
///
/// ```
/// use uor::observable::MonodromySignature;
///
/// let path: Vec<u64> = (0..=255).collect();
/// let sig = MonodromySignature::from_path(&path, 8);
///
/// assert_eq!(sig.winding, 0); // Almost one cycle but not complete
/// assert!(sig.total_variation > 0);
/// assert_eq!(sig.cascade_spectrum[1], 128); // Half are single-bit flips
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MonodromySignature {
    /// Winding number: net cycles around the ring.
    pub winding: i64,

    /// Total variation: cumulative stratum change.
    pub total_variation: u64,

    /// Cascade spectrum: histogram of cascade lengths.
    pub cascade_spectrum: Vec<u32>,
}

impl MonodromySignature {
    /// Compute the monodromy signature for a path.
    ///
    /// # Arguments
    ///
    /// * `path` - Sequence of values representing the computation path
    /// * `quantum` - Bit width of the ring
    ///
    /// # Returns
    ///
    /// The complete monodromy signature.
    pub fn from_path(path: &[u64], quantum: u32) -> Self {
        Self {
            winding: winding_number(path, quantum),
            total_variation: total_variation(path, quantum),
            cascade_spectrum: cascade_spectrum(path, quantum),
        }
    }

    /// Check if two signatures are equal (paths are monodromically equivalent).
    pub fn is_equivalent(&self, other: &Self) -> bool {
        self == other
    }

    /// Compute the mean cascade length from the spectrum.
    ///
    /// Returns `None` if the spectrum is empty or has no transitions.
    pub fn mean_cascade_length(&self) -> Option<f64> {
        let total_transitions: u32 = self.cascade_spectrum.iter().sum();
        if total_transitions == 0 {
            return None;
        }

        let weighted_sum: u64 = self
            .cascade_spectrum
            .iter()
            .enumerate()
            .map(|(k, &count)| k as u64 * count as u64)
            .sum();

        Some(weighted_sum as f64 / total_transitions as f64)
    }

    /// Compute the normalized total variation (per transition).
    ///
    /// Returns `None` if the path has fewer than 2 elements.
    pub fn normalized_total_variation(&self) -> Option<f64> {
        let total_transitions: u32 = self.cascade_spectrum.iter().sum();
        if total_transitions == 0 {
            return None;
        }

        Some(self.total_variation as f64 / total_transitions as f64)
    }
}

impl PathObservable for MonodromySignature {
    type Value = Self;

    fn compute(path: &[u64], quantum: u32) -> Self::Value {
        Self::from_path(path, quantum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monodromy_signature_basic() {
        let path = [0u64, 1, 2, 3];
        let sig = MonodromySignature::from_path(&path, 8);

        assert_eq!(sig.winding, 0);
        assert!(sig.total_variation > 0);
        assert_eq!(sig.cascade_spectrum[1], 2); // 0→1 and 2→3
        assert_eq!(sig.cascade_spectrum[2], 1); // 1→2
    }

    #[test]
    fn test_monodromy_signature_full_cycle() {
        let path: Vec<u64> = (0..=255).collect();
        let sig = MonodromySignature::from_path(&path, 8);

        assert_eq!(sig.winding, 0); // 255 steps = not quite one cycle
        assert!(sig.total_variation > 0);

        // Verify cascade distribution
        assert_eq!(sig.cascade_spectrum[1], 128);
        assert_eq!(sig.cascade_spectrum[2], 64);
    }

    #[test]
    fn test_monodromy_signature_full_wrap() {
        let path: Vec<u64> = (0..=256).collect();
        let sig = MonodromySignature::from_path(&path, 8);

        assert_eq!(sig.winding, 1); // 256 steps = one complete cycle
    }

    #[test]
    fn test_equivalence() {
        // Two different paths with same monodromy signature
        // (same winding, same TV, same cascade spectrum)
        let path1 = [0u64, 1, 2, 3];
        let path2 = [100u64, 101, 102, 103]; // Same structure, different start

        let sig1 = MonodromySignature::from_path(&path1, 8);
        let sig2 = MonodromySignature::from_path(&path2, 8);

        // These should have the same cascade spectrum and winding
        assert!(sig1.is_equivalent(&sig2));
    }

    #[test]
    fn test_mean_cascade_length() {
        let path: Vec<u64> = (0..=255).collect();
        let sig = MonodromySignature::from_path(&path, 8);

        let mean = sig.mean_cascade_length().unwrap();
        // Theoretical mean for full cycle is approximately 2.0
        // Actual for 0..255 is (128*1 + 64*2 + 32*3 + 16*4 + 8*5 + 4*6 + 2*7 + 1*8) / 255
        // = (128 + 128 + 96 + 64 + 40 + 24 + 14 + 8) / 255 = 502 / 255 ≈ 1.969
        assert!(mean > 1.9 && mean < 2.1, "Mean cascade: {}", mean);
    }

    #[test]
    fn test_normalized_tv() {
        let path = [0u64, 1, 3, 7]; // TV = 3, 3 transitions
        let sig = MonodromySignature::from_path(&path, 8);

        let ntv = sig.normalized_total_variation().unwrap();
        assert!((ntv - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_empty_path() {
        let sig = MonodromySignature::from_path(&[], 8);

        assert_eq!(sig.winding, 0);
        assert_eq!(sig.total_variation, 0);
        assert_eq!(sig.mean_cascade_length(), None);
    }

    #[test]
    fn test_trait_implementation() {
        let path = [0u64, 1, 2, 3];
        let sig = MonodromySignature::compute(&path, 8);

        assert_eq!(sig.winding, 0);
    }
}
