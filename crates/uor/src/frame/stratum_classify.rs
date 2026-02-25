//! Algebraic stratification for type declarations.
//!
//! Classifies each [`BinaryOp`] by its position in the algebraic hierarchy:
//!
//! ```text
//! CommutativeRing ⊂ IntegralDomain ⊂ UFD ⊂ PID ⊂ EuclideanDomain
//! ```
//!
//! The stratum determines which algorithms are available for a given type.
//! A Euclidean domain supports GCD-based factoring (O(log n)); a ring with
//! zero divisors requires fundamentally different approaches.
//!
//! Also provides exhaustive verification functions that prove algebraic
//! properties directly from the concrete Q0 operation tables.
//!
//! # Examples
//!
//! ```
//! use uor::frame::{BinaryOp, AlgebraicStratum};
//!
//! let s = AlgebraicStratum::classify(BinaryOp::IntegerMul);
//! assert_eq!(s, AlgebraicStratum::EuclideanDomain);
//! assert!(s.has_euclidean_division());
//! assert!(s.has_unique_factorization());
//! assert!(!s.has_zero_divisors());
//! ```

use super::{BinaryOp, TypeDeclaration};

/// Algebraic depth classification for a type's underlying ring structure.
///
/// Ordered by increasing algebraic strength. Each level implies all
/// properties of the levels below it.
///
/// Size: 1 byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AlgebraicStratum {
    /// Commutative ring with zero divisors.
    CommutativeRing = 0,
    /// Integral domain: no zero divisors.
    IntegralDomain = 1,
    /// Unique factorization domain.
    UFD = 2,
    /// Principal ideal domain: every ideal is principal.
    PID = 3,
    /// Euclidean domain: has a Euclidean division function.
    EuclideanDomain = 4,
}

impl AlgebraicStratum {
    /// Classify a binary operation by its algebraic stratum.
    #[inline]
    pub const fn classify(op: BinaryOp) -> Self {
        match op {
            BinaryOp::IntegerMul => Self::EuclideanDomain,
            BinaryOp::PolyGf2Mul => Self::EuclideanDomain,
            BinaryOp::PolyGf3Mul => Self::EuclideanDomain,
            BinaryOp::PolyGf5Mul => Self::EuclideanDomain,
        }
    }

    /// Numeric depth (0 = CommutativeRing, 4 = EuclideanDomain).
    #[inline]
    pub const fn depth(self) -> u8 {
        self as u8
    }

    /// True if `self` is at least as strong as `other`.
    #[inline]
    pub const fn subsumes(self, other: Self) -> bool {
        self as u8 >= other as u8
    }

    /// True for UFD or above (unique factorization holds).
    #[inline]
    pub const fn has_unique_factorization(self) -> bool {
        self as u8 >= Self::UFD as u8
    }

    /// True for EuclideanDomain only.
    #[inline]
    pub const fn has_euclidean_division(self) -> bool {
        self as u8 >= Self::EuclideanDomain as u8
    }

    /// True only for CommutativeRing (zero divisors exist).
    #[inline]
    pub const fn has_zero_divisors(self) -> bool {
        self as u8 == Self::CommutativeRing as u8
    }
}

// ============================================================================
// Algebraic property verification — ground-truth from Q0 operation tables
// ============================================================================

/// Verify commutativity: a ⊗ b == b ⊗ a for all carrier pairs.
pub fn verify_commutative(decl: &TypeDeclaration) -> bool {
    let op = decl.op();
    for a in decl.carrier().iter() {
        for b in decl.carrier().iter() {
            if op.apply(a, b) != op.apply(b, a) {
                return false;
            }
        }
    }
    true
}

/// Count zero-divisor pairs: carrier elements whose product leaves the carrier
/// or maps to a value associated with the additive identity.
///
/// A zero-divisor pair is (a, b) where a⊗b == 0.
/// For non-modular types, a⊗b may exceed 255 (overflow) which is different.
/// This counts only pairs where the product equals 0 (the additive identity).
pub fn zero_divisor_count(decl: &TypeDeclaration) -> u32 {
    let op = decl.op();
    let mut count = 0u32;
    for a in decl.carrier().iter() {
        for b in decl.carrier().iter() {
            if op.apply(a, b) == 0 {
                count += 1;
            }
        }
    }
    count
}

/// Closure ratio: fraction of carrier pairs whose product stays in the carrier.
///
/// Returns a value in [0.0, 1.0].
pub fn closure_ratio(decl: &TypeDeclaration) -> f64 {
    let op = decl.op();
    let carrier = decl.carrier();
    let mut closed = 0u64;
    let mut total = 0u64;
    for a in carrier.iter() {
        for b in carrier.iter() {
            total += 1;
            let product = op.apply(a, b);
            if product <= 255 && carrier.contains(product as u8) {
                closed += 1;
            }
        }
    }
    if total == 0 {
        return 0.0;
    }
    closed as f64 / total as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::cross_field::TYPE_COUNT;

    #[test]
    fn integer_mul_is_euclidean() {
        assert_eq!(
            AlgebraicStratum::classify(BinaryOp::IntegerMul),
            AlgebraicStratum::EuclideanDomain,
        );
    }

    #[test]
    fn poly_types_are_euclidean() {
        for op in [
            BinaryOp::PolyGf2Mul,
            BinaryOp::PolyGf3Mul,
            BinaryOp::PolyGf5Mul,
        ] {
            assert_eq!(
                AlgebraicStratum::classify(op),
                AlgebraicStratum::EuclideanDomain
            );
        }
    }

    #[test]
    fn ordering() {
        assert!(AlgebraicStratum::CommutativeRing < AlgebraicStratum::IntegralDomain);
        assert!(AlgebraicStratum::IntegralDomain < AlgebraicStratum::UFD);
        assert!(AlgebraicStratum::UFD < AlgebraicStratum::PID);
        assert!(AlgebraicStratum::PID < AlgebraicStratum::EuclideanDomain);
    }

    #[test]
    fn depth_values() {
        assert_eq!(AlgebraicStratum::CommutativeRing.depth(), 0);
        assert_eq!(AlgebraicStratum::IntegralDomain.depth(), 1);
        assert_eq!(AlgebraicStratum::UFD.depth(), 2);
        assert_eq!(AlgebraicStratum::PID.depth(), 3);
        assert_eq!(AlgebraicStratum::EuclideanDomain.depth(), 4);
    }

    #[test]
    fn subsumes_reflexive() {
        for s in [
            AlgebraicStratum::CommutativeRing,
            AlgebraicStratum::IntegralDomain,
            AlgebraicStratum::UFD,
            AlgebraicStratum::PID,
            AlgebraicStratum::EuclideanDomain,
        ] {
            assert!(s.subsumes(s));
        }
    }

    #[test]
    fn euclidean_subsumes_all() {
        let e = AlgebraicStratum::EuclideanDomain;
        assert!(e.subsumes(AlgebraicStratum::CommutativeRing));
        assert!(e.subsumes(AlgebraicStratum::IntegralDomain));
        assert!(e.subsumes(AlgebraicStratum::UFD));
        assert!(e.subsumes(AlgebraicStratum::PID));
    }

    #[test]
    fn commutative_ring_does_not_subsume_domain() {
        assert!(!AlgebraicStratum::CommutativeRing.subsumes(AlgebraicStratum::IntegralDomain));
    }

    #[test]
    fn has_unique_factorization_properties() {
        assert!(!AlgebraicStratum::CommutativeRing.has_unique_factorization());
        assert!(!AlgebraicStratum::IntegralDomain.has_unique_factorization());
        assert!(AlgebraicStratum::UFD.has_unique_factorization());
        assert!(AlgebraicStratum::PID.has_unique_factorization());
        assert!(AlgebraicStratum::EuclideanDomain.has_unique_factorization());
    }

    #[test]
    fn has_euclidean_division_properties() {
        assert!(!AlgebraicStratum::CommutativeRing.has_euclidean_division());
        assert!(!AlgebraicStratum::PID.has_euclidean_division());
        assert!(AlgebraicStratum::EuclideanDomain.has_euclidean_division());
    }

    #[test]
    fn has_zero_divisors_properties() {
        assert!(AlgebraicStratum::CommutativeRing.has_zero_divisors());
        assert!(!AlgebraicStratum::IntegralDomain.has_zero_divisors());
        assert!(!AlgebraicStratum::EuclideanDomain.has_zero_divisors());
    }

    // -- Algebraic property verification --

    fn all_decls() -> [TypeDeclaration; TYPE_COUNT] {
        [
            TypeDeclaration::integer_mul(),
            TypeDeclaration::poly_gf2(),
            TypeDeclaration::poly_gf3(),
            TypeDeclaration::poly_gf5(),
        ]
    }

    #[test]
    fn all_types_commutative() {
        for (i, decl) in all_decls().iter().enumerate() {
            assert!(verify_commutative(decl), "type {i} is not commutative");
        }
    }

    #[test]
    fn euclidean_types_no_zero_divisors() {
        // T₂, T_poly(2/3/5) should have zero zero-divisor pairs
        for decl in &all_decls()[..4] {
            assert_eq!(
                zero_divisor_count(decl),
                0,
                "Euclidean type should have no zero-divisor pairs"
            );
        }
    }

    #[test]
    fn integer_mul_closure_low() {
        let t2 = TypeDeclaration::integer_mul();
        let ratio = closure_ratio(&t2);
        // Most products of carrier values {2..255} exceed 255
        assert!(ratio < 0.5, "integer mul closure should be low: {ratio:.4}");
    }

    #[test]
    fn closure_ratio_bounded() {
        for decl in &all_decls() {
            let ratio = closure_ratio(decl);
            assert!(
                (0.0..=1.0).contains(&ratio),
                "closure should be in [0,1]: {ratio}"
            );
        }
    }
}
