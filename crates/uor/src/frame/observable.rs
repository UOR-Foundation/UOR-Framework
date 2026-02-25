//! Observable families — six type-dependent signatures over Q0.
//!
//! Each observable family maps a `DatumSet` (typically the irreducible set
//! of a partition) to a scalar signature. Different type declarations
//! produce different signatures on the same ambient ring, confirming
//! the SIH prediction that observables are type-relative.
//!
//! # The Six Families
//!
//! | Family | What it measures | LUT used |
//! |--------|-----------------|----------|
//! | Stratum | Mean Hamming weight | `stratum_q0` |
//! | Hamming metric | Mean pairwise Hamming distance | `stratum_q0` (on XOR) |
//! | Cascade | Mean curvature (bits flipped on +1) | `curvature_q0` |
//! | Catastrophe | Fraction with high curvature (≥4) | `curvature_q0` |
//! | Curvature | Curvature variance | `curvature_q0` |
//! | Holonomy | Torus page concentration | `torus_page_q0` |
//!
//! # Examples
//!
//! ```
//! use uor::frame::{TypeDeclaration, Partition, ObservableSignature};
//!
//! let t2 = TypeDeclaration::integer_mul();
//! let p = Partition::compute(&t2);
//! let sig = ObservableSignature::compute(p.irr());
//! assert!(sig.stratum > 4.0); // Mean Hamming weight of primes > 4
//! ```

use super::DatumSet;
use crate::lut::{curvature_q0, stratum_q0, torus_page_q0};

/// The six observable families defined in the UOR ontology.
///
/// Each family provides a different lens on the type-dependence of
/// structure (SIH §6.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObservableFamily {
    /// Hamming weight distribution.
    Stratum,
    /// Pairwise Hamming distance between elements.
    HammingMetric,
    /// Carry-cascade structure (curvature on increment).
    Cascade,
    /// Singularities in the cascade (high-curvature points).
    Catastrophe,
    /// Curvature dispersion (variance of cascade values).
    Curvature,
    /// Rotational structure on the torus.
    Holonomy,
}

impl ObservableFamily {
    /// All six families in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Stratum,
        Self::HammingMetric,
        Self::Cascade,
        Self::Catastrophe,
        Self::Curvature,
        Self::Holonomy,
    ];

    /// Compute this family's scalar signature for a datum set.
    #[inline]
    pub fn signature(self, set: &DatumSet) -> f64 {
        match self {
            Self::Stratum => stratum_signature(set),
            Self::HammingMetric => hamming_metric_signature(set),
            Self::Cascade => cascade_signature(set),
            Self::Catastrophe => catastrophe_signature(set),
            Self::Curvature => curvature_signature(set),
            Self::Holonomy => holonomy_signature(set),
        }
    }
}

/// All six observable signatures for a datum set.
///
/// Size: 48 bytes (6 × f64). Copy type.
#[derive(Debug, Clone, Copy)]
pub struct ObservableSignature {
    /// Mean Hamming weight.
    pub stratum: f64,
    /// Mean pairwise Hamming distance.
    pub hamming_metric: f64,
    /// Mean curvature (cascade).
    pub cascade: f64,
    /// Fraction of high-curvature elements.
    pub catastrophe: f64,
    /// Curvature variance.
    pub curvature: f64,
    /// Torus page concentration.
    pub holonomy: f64,
}

impl ObservableSignature {
    /// Compute all six signatures for a datum set.
    pub fn compute(set: &DatumSet) -> Self {
        Self {
            stratum: stratum_signature(set),
            hamming_metric: hamming_metric_signature(set),
            cascade: cascade_signature(set),
            catastrophe: catastrophe_signature(set),
            curvature: curvature_signature(set),
            holonomy: holonomy_signature(set),
        }
    }

    /// Return signatures as a 6-element array (canonical order).
    pub fn as_array(&self) -> [f64; 6] {
        [
            self.stratum,
            self.hamming_metric,
            self.cascade,
            self.catastrophe,
            self.curvature,
            self.holonomy,
        ]
    }
}

/// Catastrophe threshold: curvature ≥ this value is a "catastrophe point".
///
/// At Q0 (8-bit), curvature ranges 1–8. Threshold 4 = half-maximum.
const CATASTROPHE_THRESHOLD: u8 = 4;

/// Number of torus pages at Q0.
const TORUS_PAGES: usize = 48;

// ============================================================================
// Individual family signatures
// ============================================================================

/// Stratum: mean Hamming weight of elements.
///
/// Uses O(1) `stratum_q0` lookup per element. O(n) total.
pub fn stratum_signature(set: &DatumSet) -> f64 {
    let n = set.len();
    if n == 0 {
        return 0.0;
    }
    let sum: u32 = set.iter().map(|v| stratum_q0(v) as u32).sum();
    sum as f64 / n as f64
}

/// Hamming metric: mean pairwise Hamming distance.
///
/// For each pair (a, b), computes `popcount(a XOR b)` via `stratum_q0`.
/// O(n²) but n ≤ 54 at Q0, so ≤ 1431 pairs. Zero heap allocation.
pub fn hamming_metric_signature(set: &DatumSet) -> f64 {
    let n = set.len() as usize;
    if n < 2 {
        return 0.0;
    }
    let mut elements = [0u8; 256];
    for (i, v) in set.iter().enumerate() {
        elements[i] = v;
    }
    let mut sum = 0u64;
    let mut pairs = 0u64;
    for i in 0..n {
        for j in (i + 1)..n {
            sum += stratum_q0(elements[i] ^ elements[j]) as u64;
            pairs += 1;
        }
    }
    sum as f64 / pairs as f64
}

/// Cascade: mean curvature (bits flipped on increment by 1).
///
/// Uses O(1) `curvature_q0` lookup per element. O(n) total.
pub fn cascade_signature(set: &DatumSet) -> f64 {
    let n = set.len();
    if n == 0 {
        return 0.0;
    }
    let sum: u32 = set.iter().map(|v| curvature_q0(v) as u32).sum();
    sum as f64 / n as f64
}

/// Catastrophe: fraction of elements with curvature ≥ threshold.
///
/// Catastrophe points are where the cascade structure is singular —
/// many bits flip simultaneously on a unit increment.
pub fn catastrophe_signature(set: &DatumSet) -> f64 {
    let n = set.len();
    if n == 0 {
        return 0.0;
    }
    let count = set
        .iter()
        .filter(|&v| curvature_q0(v) >= CATASTROPHE_THRESHOLD)
        .count() as u32;
    count as f64 / n as f64
}

/// Curvature: variance of curvature values across elements.
///
/// Two-pass: mean then variance. Zero heap allocation.
/// Low variance = uniform cascade; high variance = diverse carry regimes.
pub fn curvature_signature(set: &DatumSet) -> f64 {
    let n = set.len();
    if n == 0 {
        return 0.0;
    }
    // Pass 1: mean
    let sum: u32 = set.iter().map(|v| curvature_q0(v) as u32).sum();
    let mean = sum as f64 / n as f64;
    // Pass 2: variance
    let var_sum: f64 = set
        .iter()
        .map(|v| {
            let d = curvature_q0(v) as f64 - mean;
            d * d
        })
        .sum();
    var_sum / n as f64
}

/// Holonomy: torus page concentration (max page fraction).
///
/// Maps each element to its torus page (0–47) via `torus_page_q0`,
/// returns `max_count / total`. Range: \[1/n, 1.0\].
///
/// Higher = irreducibles cluster on fewer pages (less rotational spread).
pub fn holonomy_signature(set: &DatumSet) -> f64 {
    let n = set.len();
    if n == 0 {
        return 0.0;
    }
    let mut pages = [0u32; TORUS_PAGES];
    for v in set.iter() {
        let page = torus_page_q0(v) as usize;
        if page < TORUS_PAGES {
            pages[page] += 1;
        }
    }
    let max_count = pages.iter().copied().max().unwrap_or(0);
    max_count as f64 / n as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{Partition, TypeDeclaration};

    fn t2_irr() -> DatumSet {
        let t2 = TypeDeclaration::integer_mul();
        *Partition::compute(&t2).irr()
    }

    fn poly_irr() -> DatumSet {
        let tp = TypeDeclaration::poly_gf2();
        *Partition::compute(&tp).irr()
    }

    // -- Family count --

    #[test]
    fn observable_family_count() {
        assert_eq!(ObservableFamily::ALL.len(), 6);
    }

    // -- Stratum (SIH §6.1 exact data) --

    #[test]
    fn stratum_t2_mean() {
        // SIH §6.1 table: 1×1 + 2×3 + 3×12 + 4×13 + 5×20 + 7×5 = 230
        let sig = stratum_signature(&t2_irr());
        assert!((sig - 230.0 / 54.0).abs() < 1e-10);
    }

    #[test]
    fn stratum_poly_mean() {
        // SIH §6.1 table: 1×1 + 2×1 + 3×14 + 5×21 + 7×4 = 178
        let sig = stratum_signature(&poly_irr());
        assert!((sig - 178.0 / 41.0).abs() < 1e-10);
    }

    #[test]
    fn stratum_differs_between_types() {
        let s_t2 = stratum_signature(&t2_irr());
        let s_poly = stratum_signature(&poly_irr());
        assert!((s_t2 - s_poly).abs() > 0.01);
    }

    // -- Hamming metric --

    #[test]
    fn hamming_metric_in_range() {
        let sig = hamming_metric_signature(&t2_irr());
        assert!(sig > 0.0);
        assert!(sig <= 8.0);
    }

    #[test]
    fn hamming_metric_differs() {
        let h_t2 = hamming_metric_signature(&t2_irr());
        let h_poly = hamming_metric_signature(&poly_irr());
        assert!((h_t2 - h_poly).abs() > 0.01);
    }

    // -- Cascade --

    #[test]
    fn cascade_in_range() {
        let sig = cascade_signature(&t2_irr());
        assert!(sig > 0.0);
        assert!(sig <= 8.0);
    }

    #[test]
    fn cascade_differs() {
        let c_t2 = cascade_signature(&t2_irr());
        let c_poly = cascade_signature(&poly_irr());
        assert!((c_t2 - c_poly).abs() > 0.001);
    }

    // -- Catastrophe --

    #[test]
    fn catastrophe_in_range() {
        let sig = catastrophe_signature(&t2_irr());
        assert!(sig >= 0.0);
        assert!(sig <= 1.0);
    }

    #[test]
    fn catastrophe_differs() {
        let c_t2 = catastrophe_signature(&t2_irr());
        let c_poly = catastrophe_signature(&poly_irr());
        // Both types should have some catastrophe points, but different fractions
        assert!(c_t2 > 0.0);
        assert!(c_poly > 0.0);
        assert!((c_t2 - c_poly).abs() > 0.001);
    }

    // -- Curvature variance --

    #[test]
    fn curvature_variance_positive() {
        let sig = curvature_signature(&t2_irr());
        assert!(sig > 0.0);
    }

    #[test]
    fn curvature_differs() {
        let v_t2 = curvature_signature(&t2_irr());
        let v_poly = curvature_signature(&poly_irr());
        assert!((v_t2 - v_poly).abs() > 0.001);
    }

    // -- Holonomy --

    #[test]
    fn holonomy_in_range() {
        let sig = holonomy_signature(&t2_irr());
        assert!(sig > 0.0);
        assert!(sig <= 1.0);
    }

    #[test]
    fn holonomy_differs() {
        let h_t2 = holonomy_signature(&t2_irr());
        let h_poly = holonomy_signature(&poly_irr());
        assert!((0.0..=1.0).contains(&h_t2));
        assert!((0.0..=1.0).contains(&h_poly));
    }

    // -- Composite tests --

    #[test]
    fn observable_signature_compute() {
        let sig = ObservableSignature::compute(&t2_irr());
        let arr = sig.as_array();
        assert_eq!(arr.len(), 6);
        for &v in &arr {
            assert!(v >= 0.0, "all signatures must be non-negative");
        }
    }

    #[test]
    fn observable_family_dispatch_matches_direct() {
        let set = t2_irr();
        assert_eq!(
            ObservableFamily::Stratum.signature(&set),
            stratum_signature(&set)
        );
        assert_eq!(
            ObservableFamily::Cascade.signature(&set),
            cascade_signature(&set)
        );
        assert_eq!(
            ObservableFamily::Holonomy.signature(&set),
            holonomy_signature(&set)
        );
    }

    #[test]
    fn empty_set_all_zero() {
        let empty = DatumSet::EMPTY;
        for family in ObservableFamily::ALL {
            assert_eq!(family.signature(&empty), 0.0);
        }
    }

    #[test]
    fn singleton_hamming_metric_zero() {
        assert_eq!(hamming_metric_signature(&DatumSet::singleton(42)), 0.0);
    }

    #[test]
    fn all_families_produce_distinct_signatures() {
        let sig = ObservableSignature::compute(&t2_irr());
        let arr = sig.as_array();
        let mut distinct = 0;
        for i in 0..6 {
            for j in (i + 1)..6 {
                if (arr[i] - arr[j]).abs() > 1e-10 {
                    distinct += 1;
                }
            }
        }
        // 6 families -> 15 pairs; most should differ
        assert!(
            distinct > 7,
            "families should produce mostly distinct values"
        );
    }

    #[test]
    fn t2_vs_poly_signatures_all_differ() {
        let sig_t2 = ObservableSignature::compute(&t2_irr());
        let sig_poly = ObservableSignature::compute(&poly_irr());
        let a = sig_t2.as_array();
        let b = sig_poly.as_array();
        // Every family should produce a different value for T₂ vs T_poly(2)
        for (i, family) in ObservableFamily::ALL.iter().enumerate() {
            assert!(
                (a[i] - b[i]).abs() > 1e-6,
                "{family:?} should differ between T₂ and T_poly(2): {} vs {}",
                a[i],
                b[i]
            );
        }
    }
}
