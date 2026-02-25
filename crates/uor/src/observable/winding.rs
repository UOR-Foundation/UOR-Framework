//! Winding number observable for computation paths.
//!
//! The winding number measures how many complete cycles around the ring Z/(2^n)Z
//! a path traverses. It is the discrete analogue of the topological winding number
//! from complex analysis.

use super::PathObservable;

/// Winding number observable.
///
/// For a path x₀ → x₁ → ... → xₘ, the winding number counts how many times
/// the path wraps around the ring. The sign indicates direction:
/// - Positive: net movement in successor (increasing) direction
/// - Negative: net movement in predecessor (decreasing) direction
///
/// # Definition
///
/// ```text
/// W = floor(Σᵢ δᵢ / 2ⁿ)
/// ```
///
/// where δᵢ is the signed shortest-path distance from xᵢ to xᵢ₊₁.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WindingNumber {
    /// The winding number value.
    pub value: i64,
}

impl PathObservable for WindingNumber {
    type Value = i64;

    fn compute(path: &[u64], quantum: u32) -> Self::Value {
        winding_number(path, quantum)
    }
}

/// Compute the winding number of a path in Z/(2^quantum)Z.
///
/// # Arguments
///
/// * `path` - Sequence of values representing the computation path
/// * `quantum` - Bit width of the ring
///
/// # Returns
///
/// The winding number (integer, possibly negative).
///
/// # Example
///
/// ```
/// use uor::observable::winding_number;
///
/// // Path that doesn't complete a full cycle
/// let short_path = [0u64, 1, 2, 3];
/// assert_eq!(winding_number(&short_path, 8), 0);
///
/// // Path that wraps around once (in 8-bit ring)
/// let wrap_path: Vec<u64> = (0..=256).collect();
/// assert_eq!(winding_number(&wrap_path, 8), 1);
/// ```
pub fn winding_number(path: &[u64], quantum: u32) -> i64 {
    if path.len() < 2 || quantum == 0 || quantum > 63 {
        return 0;
    }

    let cycle = 1u64 << quantum;
    let half = cycle / 2;
    let mut total_displacement: i64 = 0;

    for window in path.windows(2) {
        let from = window[0] & (cycle - 1); // Mask to quantum bits
        let to = window[1] & (cycle - 1);

        // Compute unsigned delta (always in range [0, cycle))
        let delta = to.wrapping_sub(from) & (cycle - 1);

        // Convert to signed: choose shorter path around the ring
        let signed_delta = if delta > half {
            delta as i64 - cycle as i64
        } else {
            delta as i64
        };

        total_displacement += signed_delta;
    }

    // Winding number is total displacement divided by cycle length
    total_displacement / (cycle as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winding_number_no_wrap() {
        // Short path that doesn't wrap
        let path = [0u64, 1, 2, 3, 4, 5];
        assert_eq!(winding_number(&path, 8), 0);

        // Going backwards, still no wrap
        let path_back = [5u64, 4, 3, 2, 1, 0];
        assert_eq!(winding_number(&path_back, 8), 0);
    }

    #[test]
    fn test_winding_number_one_wrap() {
        // Path that wraps once forward (0 to 256 in 8-bit ring)
        let path: Vec<u64> = (0..=256).collect();
        assert_eq!(winding_number(&path, 8), 1);

        // Path that wraps once backward
        let path_back: Vec<u64> = (0..=256).rev().collect();
        assert_eq!(winding_number(&path_back, 8), -1);
    }

    #[test]
    fn test_winding_number_multiple_wraps() {
        // Two complete forward cycles
        let path: Vec<u64> = (0..=512).collect();
        assert_eq!(winding_number(&path, 8), 2);
    }

    #[test]
    fn test_winding_number_partial() {
        // Half a cycle forward
        let path: Vec<u64> = (0..=128).collect();
        assert_eq!(winding_number(&path, 8), 0);

        // Just under one cycle
        let path_almost: Vec<u64> = (0..=255).collect();
        assert_eq!(winding_number(&path_almost, 8), 0);
    }

    #[test]
    fn test_winding_number_empty_and_single() {
        assert_eq!(winding_number(&[], 8), 0);
        assert_eq!(winding_number(&[42], 8), 0);
    }

    #[test]
    fn test_winding_number_q16() {
        // One full cycle in 16-bit ring
        let cycle_16 = 65536u64;
        let path: Vec<u64> = (0..=cycle_16).collect();
        assert_eq!(winding_number(&path, 16), 1);
    }

    #[test]
    fn test_winding_observable_trait() {
        let path: Vec<u64> = (0..=256).collect();
        let result = WindingNumber::compute(&path, 8);
        assert_eq!(result, 1);
    }
}
