//! Type-space distance metrics.
//!
//! Two metrics quantify how different two type declarations are:
//!
//! - **Partition distance** `dΠ`: fraction of carrier elements classified
//!   differently (symmetric difference of Irr sets / carrier size).
//! - **Stratum distance** `dₛ`: total variation between stratum (Hamming weight)
//!   histograms of the irreducible sets.
//!
//! # Examples
//!
//! ```
//! use uor::frame::{TypeDeclaration, Partition, partition_distance, stratum_distance};
//!
//! let t2 = TypeDeclaration::integer_mul();
//! let tp = TypeDeclaration::poly_gf2();
//! let p2 = Partition::compute(&t2);
//! let pp = Partition::compute(&tp);
//!
//! let d = partition_distance(&p2, &pp, 254);
//! assert!((d - 41.0 / 254.0).abs() < 1e-6);
//! ```

use super::{DatumSet, Partition};
use crate::lut::stratum_q0;

/// Partition distance: `|Irr_a △ Irr_b| / carrier_size`.
///
/// Measures the fraction of elements that change irreducibility status
/// between two type declarations. Range: [0.0, 1.0].
#[inline]
pub fn partition_distance(a: &Partition, b: &Partition, carrier_size: u32) -> f64 {
    if carrier_size == 0 {
        return 0.0;
    }
    let sym_diff = a.irr().symmetric_difference(b.irr());
    sym_diff.len() as f64 / carrier_size as f64
}

/// Stratum histogram: count of set members at each Hamming weight (0–8).
///
/// Uses the existing `lut::stratum_q0` table for O(1) per-element lookup.
pub fn stratum_histogram(set: &DatumSet) -> [u32; 9] {
    let mut hist = [0u32; 9];
    for v in set.iter() {
        let s = stratum_q0(v) as usize;
        hist[s] += 1;
    }
    hist
}

/// Stratum distance: total variation between stratum histograms.
///
/// `dₛ = Σ |h_a[k] - h_b[k]| / (2 * max(|a|, |b|))` where `h` is
/// the normalized stratum histogram.
///
/// Range: [0.0, 1.0].
pub fn stratum_distance(a: &Partition, b: &Partition) -> f64 {
    let ha = stratum_histogram(a.irr());
    let hb = stratum_histogram(b.irr());
    let total_a: u32 = ha.iter().sum();
    let total_b: u32 = hb.iter().sum();
    let denom = total_a.max(total_b);
    if denom == 0 {
        return 0.0;
    }
    let mut sum_diff = 0i64;
    for i in 0..9 {
        sum_diff += (ha[i] as i64 - hb[i] as i64).abs();
    }
    sum_diff as f64 / (2 * denom) as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::TypeDeclaration;

    #[test]
    fn partition_distance_t2_vs_poly() {
        let t2 = TypeDeclaration::integer_mul();
        let tp = TypeDeclaration::poly_gf2();
        let p2 = Partition::compute(&t2);
        let pp = Partition::compute(&tp);

        let d = partition_distance(&p2, &pp, 254);
        let expected = 41.0 / 254.0; // ≈ 0.161
        assert!((d - expected).abs() < 1e-6, "dΠ = {d}, expected {expected}");
    }

    #[test]
    fn partition_distance_self_is_zero() {
        let t2 = TypeDeclaration::integer_mul();
        let p2 = Partition::compute(&t2);
        assert_eq!(partition_distance(&p2, &p2, 254), 0.0);
    }

    #[test]
    fn stratum_histogram_primes() {
        let t2 = TypeDeclaration::integer_mul();
        let p2 = Partition::compute(&t2);
        let h = stratum_histogram(p2.irr());
        // Stratum 0: value 0 has weight 0, but 0 is not prime
        assert_eq!(h[0], 0);
        // Stratum 1: only value 2 (0b10) — it's prime with weight 1
        assert_eq!(h[1], 1);
        // Total should be 54
        let total: u32 = h.iter().sum();
        assert_eq!(total, 54);
    }

    #[test]
    fn stratum_4_primes_vs_polys() {
        let t2 = TypeDeclaration::integer_mul();
        let tp = TypeDeclaration::poly_gf2();
        let p2 = Partition::compute(&t2);
        let pp = Partition::compute(&tp);

        let h_primes = stratum_histogram(p2.irr());
        let h_polys = stratum_histogram(pp.irr());

        // Stratum 4: 13 primes, 0 irreducible polynomials (from SIH spec)
        assert_eq!(h_primes[4], 13, "stratum 4 primes: {}", h_primes[4]);
        assert_eq!(h_polys[4], 0, "stratum 4 irred polys: {}", h_polys[4]);
    }

    #[test]
    fn stratum_distance_self_is_zero() {
        let t2 = TypeDeclaration::integer_mul();
        let p2 = Partition::compute(&t2);
        assert_eq!(stratum_distance(&p2, &p2), 0.0);
    }

    #[test]
    fn stratum_distance_t2_vs_poly_positive() {
        let t2 = TypeDeclaration::integer_mul();
        let tp = TypeDeclaration::poly_gf2();
        let p2 = Partition::compute(&t2);
        let pp = Partition::compute(&tp);

        let d = stratum_distance(&p2, &pp);
        assert!(d > 0.0, "stratum distance should be positive");
        assert!(d < 1.0, "stratum distance should be < 1");
    }
}
