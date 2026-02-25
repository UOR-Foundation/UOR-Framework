//! Multi-modal classification — classify a datum under all four types.
//!
//! [`MultiModalResult`] resolves a single datum across all types.
//! [`MultiModalBatch`] classifies all 256 values exhaustively.
//!
//! # Examples
//!
//! ```
//! use uor::frame::{Frame, MultiModalResult, DatumClass};
//!
//! let frame = Frame::compute();
//! let r = MultiModalResult::classify(5, &frame);
//! assert_eq!(r.class(0), DatumClass::Irreducible); // T₂: 5 is prime
//! assert_eq!(r.class(1), DatumClass::Reducible);    // T_poly(2): (x+1)²
//! assert_eq!(r.irreducible_count(), 3);
//! ```

use super::cross_field::TYPE_COUNT;
use super::{DatumClass, Frame};

/// Multi-modal classification result for a single datum.
#[derive(Debug, Clone, Copy)]
pub struct MultiModalResult {
    value: u8,
    classes: [DatumClass; TYPE_COUNT],
}

impl MultiModalResult {
    /// Classify a datum under all four types via the frame.
    #[inline]
    pub fn classify(value: u8, frame: &Frame) -> Self {
        Self {
            value,
            classes: frame.resolve_all(value),
        }
    }

    /// The datum value.
    #[inline]
    pub const fn value(&self) -> u8 {
        self.value
    }

    /// Classification under the type at the given index.
    #[inline]
    pub fn class(&self, idx: usize) -> DatumClass {
        self.classes[idx]
    }

    /// True if irreducible under any type.
    #[inline]
    pub fn is_irreducible_any(&self) -> bool {
        self.classes.contains(&DatumClass::Irreducible)
    }

    /// True if irreducible under all types.
    #[inline]
    pub fn is_irreducible_all(&self) -> bool {
        self.classes.iter().all(|&c| c == DatumClass::Irreducible)
    }

    /// Number of types under which this datum is irreducible.
    #[inline]
    pub fn irreducible_count(&self) -> usize {
        self.classes
            .iter()
            .filter(|&&c| c == DatumClass::Irreducible)
            .count()
    }

    /// Bitmask of types under which this datum is irreducible.
    ///
    /// Bit `i` is set if `class(i) == Irreducible`.
    #[inline]
    pub fn irreducible_mask(&self) -> u8 {
        let mut mask = 0u8;
        for (i, &c) in self.classes.iter().enumerate() {
            if c == DatumClass::Irreducible {
                mask |= 1 << i;
            }
        }
        mask
    }
}

/// Batch multi-modal classification of all 256 datum values.
pub struct MultiModalBatch {
    results: [MultiModalResult; 256],
}

impl MultiModalBatch {
    /// Classify all 256 values under all four types.
    pub fn classify_all(frame: &Frame) -> Self {
        Self {
            results: core::array::from_fn(|i| MultiModalResult::classify(i as u8, frame)),
        }
    }

    /// Access the result for a specific value.
    #[inline]
    pub fn get(&self, value: u8) -> &MultiModalResult {
        &self.results[value as usize]
    }

    /// Count datums irreducible under exactly `k` types.
    pub fn count_by_k(&self, k: usize) -> usize {
        self.results
            .iter()
            .filter(|r| r.irreducible_count() == k)
            .count()
    }

    /// Full k-distribution: count_by_k for k = 0..=TYPE_COUNT.
    pub fn k_distribution(&self) -> [usize; TYPE_COUNT + 1] {
        let mut dist = [0usize; TYPE_COUNT + 1];
        for r in &self.results {
            dist[r.irreducible_count()] += 1;
        }
        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame() -> Frame {
        Frame::compute()
    }

    // -- Datum 5 type-relativity --

    #[test]
    fn datum_5_t2_irreducible() {
        let f = frame();
        let r = MultiModalResult::classify(5, &f);
        assert_eq!(r.class(0), DatumClass::Irreducible); // T₂: prime
    }

    #[test]
    fn datum_5_poly2_reducible() {
        let f = frame();
        let r = MultiModalResult::classify(5, &f);
        assert_eq!(r.class(1), DatumClass::Reducible); // T_poly(2): (x+1)²
    }

    // -- Datum 25 type-relativity --

    #[test]
    fn datum_25_t2_reducible() {
        let f = frame();
        let r = MultiModalResult::classify(25, &f);
        assert_eq!(r.class(0), DatumClass::Reducible); // T₂: 5×5
    }

    #[test]
    fn datum_25_poly2_irreducible() {
        let f = frame();
        let r = MultiModalResult::classify(25, &f);
        assert_eq!(r.class(1), DatumClass::Irreducible); // T_poly(2): x⁴+x³+1
    }

    // -- External values --

    #[test]
    fn datum_0_external_all() {
        let f = frame();
        let r = MultiModalResult::classify(0, &f);
        for i in 0..TYPE_COUNT {
            assert_eq!(r.class(i), DatumClass::External, "type {i}");
        }
    }

    #[test]
    fn datum_1_external_all() {
        let f = frame();
        let r = MultiModalResult::classify(1, &f);
        for i in 0..TYPE_COUNT {
            assert_eq!(r.class(i), DatumClass::External, "type {i}");
        }
    }

    // -- Irreducibility queries --

    #[test]
    fn datum_5_irreducible_any() {
        let f = frame();
        let r = MultiModalResult::classify(5, &f);
        assert!(r.is_irreducible_any());
    }

    #[test]
    fn datum_0_not_irreducible_any() {
        let f = frame();
        let r = MultiModalResult::classify(0, &f);
        assert!(!r.is_irreducible_any());
    }

    #[test]
    fn datum_5_count() {
        let f = frame();
        let r = MultiModalResult::classify(5, &f);
        // 5 is prime (T₂ irr), but (x+1)² in GF(2) (red), and also differs in GF(3)/GF(5)
        assert!(r.irreducible_count() >= 1);
        assert!(r.irreducible_count() <= TYPE_COUNT);
    }

    // -- Mask --

    #[test]
    fn datum_5_mask_includes_t2() {
        let f = frame();
        let r = MultiModalResult::classify(5, &f);
        assert!(r.irreducible_mask() & 1 != 0, "T₂ bit should be set");
    }

    #[test]
    fn datum_5_mask_excludes_poly2() {
        let f = frame();
        let r = MultiModalResult::classify(5, &f);
        assert!(
            r.irreducible_mask() & 2 == 0,
            "T_poly(2) bit should not be set"
        );
    }

    // -- Batch --

    #[test]
    fn batch_exhaustive() {
        let f = frame();
        let batch = MultiModalBatch::classify_all(&f);
        for v in 0..=255u8 {
            let r = batch.get(v);
            assert_eq!(r.value(), v);
        }
    }

    #[test]
    fn k_distribution_sums_to_256() {
        let f = frame();
        let batch = MultiModalBatch::classify_all(&f);
        let dist = batch.k_distribution();
        let total: usize = dist.iter().sum();
        assert_eq!(total, 256, "k-distribution should sum to 256, got {total}");
    }

    #[test]
    fn count_by_k_matches_distribution() {
        let f = frame();
        let batch = MultiModalBatch::classify_all(&f);
        let dist = batch.k_distribution();
        for (k, &expected) in dist.iter().enumerate() {
            assert_eq!(batch.count_by_k(k), expected, "mismatch at k={k}");
        }
    }

    #[test]
    fn batch_agrees_with_single() {
        let f = frame();
        let batch = MultiModalBatch::classify_all(&f);
        for v in 0..=255u8 {
            let single = MultiModalResult::classify(v, &f);
            let batched = batch.get(v);
            for i in 0..TYPE_COUNT {
                assert_eq!(
                    single.class(i),
                    batched.class(i),
                    "mismatch at v={v} type={i}"
                );
            }
        }
    }

    #[test]
    fn value_accessor() {
        let f = frame();
        let r = MultiModalResult::classify(42, &f);
        assert_eq!(r.value(), 42);
    }
}
