//! Emanation sequence — the k-th irreducible in datum-value order.
//!
//! The emanation map `E(k)` returns the k-th element of the irreducible
//! set ordered by datum value. Different types produce different emanation
//! sequences — this is a direct observable of type-relativity.
//!
//! # Examples
//!
//! ```
//! use uor::frame::{TypeDeclaration, Partition, emanation};
//!
//! let t2 = TypeDeclaration::integer_mul();
//! let p2 = Partition::compute(&t2);
//!
//! assert_eq!(emanation(&p2, 1), Some(2));  // E(1) = 2
//! assert_eq!(emanation(&p2, 2), Some(3));  // E(2) = 3
//! assert_eq!(emanation(&p2, 3), Some(5));  // E(3) = 5
//! ```

use super::Partition;

/// Return the k-th irreducible element in datum-value order (1-indexed).
///
/// Returns `None` if `k` is 0 or exceeds the number of irreducibles.
#[inline]
pub fn emanation(partition: &Partition, k: usize) -> Option<u8> {
    if k == 0 {
        return None;
    }
    partition.irr().iter().nth(k - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{Partition, TypeDeclaration};

    #[test]
    fn emanation_t2_first_10() {
        let t2 = TypeDeclaration::integer_mul();
        let p2 = Partition::compute(&t2);
        let expected = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(
                emanation(&p2, i + 1),
                Some(e),
                "T₂ E({}) = {:?}, expected {}",
                i + 1,
                emanation(&p2, i + 1),
                e,
            );
        }
    }

    #[test]
    fn emanation_poly_first_10() {
        let tp = TypeDeclaration::poly_gf2();
        let pp = Partition::compute(&tp);
        let expected = [2, 3, 7, 11, 13, 19, 25, 31, 37, 41];
        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(
                emanation(&pp, i + 1),
                Some(e),
                "T_poly(2) E({}) = {:?}, expected {}",
                i + 1,
                emanation(&pp, i + 1),
                e,
            );
        }
    }

    #[test]
    fn emanation_diverges_at_k3() {
        let t2 = TypeDeclaration::integer_mul();
        let tp = TypeDeclaration::poly_gf2();
        let p2 = Partition::compute(&t2);
        let pp = Partition::compute(&tp);

        // k=1,2 agree
        assert_eq!(emanation(&p2, 1), emanation(&pp, 1)); // 2
        assert_eq!(emanation(&p2, 2), emanation(&pp, 2)); // 3

        // k=3 diverges: T₂ → 5, T_poly(2) → 7
        assert_eq!(emanation(&p2, 3), Some(5));
        assert_eq!(emanation(&pp, 3), Some(7));
    }

    #[test]
    fn emanation_zero_returns_none() {
        let t2 = TypeDeclaration::integer_mul();
        let p2 = Partition::compute(&t2);
        assert_eq!(emanation(&p2, 0), None);
    }

    #[test]
    fn emanation_out_of_range() {
        let t2 = TypeDeclaration::integer_mul();
        let p2 = Partition::compute(&t2);
        assert_eq!(emanation(&p2, 55), None); // only 54 primes
    }
}
