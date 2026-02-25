//! Cross-modal alignment — quantifying how two types partition Q0 differently.
//!
//! The canonical case is T₂ (primes) vs T_poly(2) (irreducible polynomials
//! over GF(2)) — these are the prototype "modalities" viewing the same bytes
//! through fundamentally different algebraic lenses.
//!
//! # Examples
//!
//! ```
//! use uor::frame::{TypeRegistry, Alignment};
//!
//! let reg = TypeRegistry::compute();
//! let a = Alignment::compute(&reg, 0, 1); // T₂ vs T_poly(2)
//! assert_eq!(a.both, 27);  // Irr in both
//! assert_eq!(a.a_only, 27); // Irr only in T₂
//! assert_eq!(a.b_only, 14); // Irr only in T_poly(2)
//! ```

use super::cross_field::TYPE_COUNT;
use super::distance::{partition_distance, stratum_distance};
use super::embedding::{embed, embedding_distance};
use super::TypeRegistry;

/// Alignment metrics between two types as modalities.
#[derive(Debug, Clone, Copy)]
pub struct Alignment {
    /// Datums irreducible under both types.
    pub both: u32,
    /// Datums irreducible only under type A.
    pub a_only: u32,
    /// Datums irreducible only under type B.
    pub b_only: u32,
    /// Datums irreducible under neither type.
    pub neither: u32,
    /// Partition distance between the two types.
    pub partition_dist: f64,
    /// Stratum distance between the two types.
    pub stratum_dist: f64,
    /// Euclidean distance in R^17 embedding space.
    pub embedding_dist: f64,
}

impl Alignment {
    /// Compute alignment between two types in the registry.
    pub fn compute(registry: &TypeRegistry, idx_a: usize, idx_b: usize) -> Self {
        let (decl_a, part_a) = registry.get(idx_a);
        let (decl_b, part_b) = registry.get(idx_b);

        let irr_a = part_a.irr();
        let irr_b = part_b.irr();

        let both_set = irr_a.intersection(irr_b);
        let a_only_set = irr_a.difference(irr_b);
        let b_only_set = irr_b.difference(irr_a);

        // Neither: count values not in either irr set (across full Q0)
        let either = irr_a.union(irr_b);
        let neither = 256 - either.len();

        let carrier_max = decl_a.carrier_len().max(decl_b.carrier_len());
        let pd = partition_distance(part_a, part_b, carrier_max);
        let sd = stratum_distance(part_a, part_b);

        let ea = embed(part_a, decl_a);
        let eb = embed(part_b, decl_b);
        let ed = embedding_distance(&ea, &eb);

        Self {
            both: both_set.len(),
            a_only: a_only_set.len(),
            b_only: b_only_set.len(),
            neither,
            partition_dist: pd,
            stratum_dist: sd,
            embedding_dist: ed,
        }
    }

    /// Jaccard similarity: |both| / |A ∪ B|.
    #[inline]
    pub fn jaccard(&self) -> f64 {
        let union = self.both + self.a_only + self.b_only;
        if union == 0 {
            return 1.0;
        }
        self.both as f64 / union as f64
    }

    /// Overlap coefficient: |both| / min(|A|, |B|).
    #[inline]
    pub fn overlap(&self) -> f64 {
        let size_a = self.both + self.a_only;
        let size_b = self.both + self.b_only;
        let min_size = size_a.min(size_b);
        if min_size == 0 {
            return 1.0;
        }
        self.both as f64 / min_size as f64
    }

    /// Symmetric difference count: |A_only| + |B_only|.
    #[inline]
    pub fn symmetric_diff(&self) -> u32 {
        self.a_only + self.b_only
    }
}

/// Compute alignment for all 10 type pairs.
pub fn all_alignments(registry: &TypeRegistry) -> alloc::vec::Vec<(usize, usize, Alignment)> {
    let mut results = alloc::vec::Vec::with_capacity(TYPE_COUNT * (TYPE_COUNT - 1) / 2);
    for i in 0..TYPE_COUNT {
        for j in (i + 1)..TYPE_COUNT {
            results.push((i, j, Alignment::compute(registry, i, j)));
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry() -> TypeRegistry {
        TypeRegistry::compute()
    }

    // -- T₂ vs T_poly(2) canonical pair --

    #[test]
    fn t2_vs_poly2_counts() {
        let reg = registry();
        let a = Alignment::compute(&reg, 0, 1);
        assert_eq!(a.both, 27, "both={}", a.both);
        assert_eq!(a.a_only, 27, "a_only={}", a.a_only);
        assert_eq!(a.b_only, 14, "b_only={}", a.b_only);
    }

    #[test]
    fn t2_vs_poly2_partition_dist() {
        let reg = registry();
        let a = Alignment::compute(&reg, 0, 1);
        let expected = 41.0 / 254.0;
        assert!(
            (a.partition_dist - expected).abs() < 1e-6,
            "partition_dist={}, expected {expected}",
            a.partition_dist
        );
    }

    #[test]
    fn t2_vs_poly2_jaccard() {
        let reg = registry();
        let a = Alignment::compute(&reg, 0, 1);
        // Jaccard = 27 / (27 + 27 + 14) = 27/68 ≈ 0.397
        let expected = 27.0 / 68.0;
        assert!(
            (a.jaccard() - expected).abs() < 1e-6,
            "jaccard={}, expected {expected}",
            a.jaccard()
        );
    }

    #[test]
    fn t2_vs_poly2_overlap() {
        let reg = registry();
        let a = Alignment::compute(&reg, 0, 1);
        // overlap = 27 / min(54, 41) = 27/41
        let expected = 27.0 / 41.0;
        assert!(
            (a.overlap() - expected).abs() < 1e-6,
            "overlap={}, expected {expected}",
            a.overlap()
        );
    }

    #[test]
    fn t2_vs_poly2_symmetric_diff() {
        let reg = registry();
        let a = Alignment::compute(&reg, 0, 1);
        assert_eq!(a.symmetric_diff(), 41); // 27 + 14
    }

    // -- Self-alignment --

    #[test]
    fn self_alignment_both_equals_irr() {
        let reg = registry();
        for i in 0..TYPE_COUNT {
            let a = Alignment::compute(&reg, i, i);
            let (_, part) = reg.get(i);
            assert_eq!(
                a.both,
                part.irr().len(),
                "type {i}: both should equal irr count"
            );
            assert_eq!(a.a_only, 0, "type {i}: a_only should be 0");
            assert_eq!(a.b_only, 0, "type {i}: b_only should be 0");
        }
    }

    #[test]
    fn self_alignment_jaccard_one() {
        let reg = registry();
        for i in 0..TYPE_COUNT {
            let a = Alignment::compute(&reg, i, i);
            assert!(
                (a.jaccard() - 1.0).abs() < 1e-10,
                "type {i}: jaccard={}",
                a.jaccard()
            );
        }
    }

    // -- All pairs --

    #[test]
    fn all_pairs_positive_distance() {
        let reg = registry();
        let alignments = all_alignments(&reg);
        assert_eq!(alignments.len(), 6);
        for (i, j, a) in &alignments {
            assert!(
                a.partition_dist > 0.0,
                "[{i},{j}]: partition_dist should be > 0"
            );
            assert!(
                a.embedding_dist > 0.0,
                "[{i},{j}]: embedding_dist should be > 0"
            );
        }
    }

    #[test]
    fn alignment_symmetry() {
        let reg = registry();
        let a01 = Alignment::compute(&reg, 0, 1);
        let a10 = Alignment::compute(&reg, 1, 0);
        assert_eq!(a01.both, a10.both);
        assert_eq!(a01.a_only, a10.b_only);
        assert_eq!(a01.b_only, a10.a_only);
        assert!((a01.partition_dist - a10.partition_dist).abs() < 1e-10);
    }

    #[test]
    fn counts_sum_to_256() {
        let reg = registry();
        let a = Alignment::compute(&reg, 0, 1);
        // both + a_only + b_only + neither should cover all 256 values
        // Note: this isn't exactly 256 because "neither" means not irr in either,
        // but the element could be red/unit/ext.
        let irr_union = a.both + a.a_only + a.b_only;
        assert_eq!(irr_union + a.neither, 256);
    }

    #[test]
    fn embedding_dist_positive_all_pairs() {
        let reg = registry();
        for i in 0..TYPE_COUNT {
            for j in (i + 1)..TYPE_COUNT {
                let a = Alignment::compute(&reg, i, j);
                assert!(a.embedding_dist > 0.0);
            }
        }
    }
}
