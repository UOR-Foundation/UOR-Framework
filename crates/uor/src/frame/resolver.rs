//! Resolver dispatch — stratum-aware resolution strategies.
//!
//! Provides two resolver implementations that classify a datum by checking
//! partition membership. At Q0, both use precomputed bitsets (identical
//! behavior). The structural distinction matters at Q1+ where Euclidean
//! domains support GCD-based division while rings need exhaustive search.
//!
//! [`StratumDispatch`] auto-selects the resolver based on algebraic stratum.
//!
//! This is component 5 of the nine-component invariance frame.
//!
//! # Examples
//!
//! ```
//! use uor::frame::{TypeDeclaration, Partition, AlgebraicStratum, DatumClass};
//! use uor::frame::{Resolver, StratumDispatch};
//!
//! let t2 = TypeDeclaration::integer_mul();
//! let p = Partition::compute(&t2);
//! let s = AlgebraicStratum::classify(t2.op());
//! assert_eq!(StratumDispatch::resolve(5, &p, s), DatumClass::Irreducible);
//! ```

use super::{AlgebraicStratum, DatumClass, Partition};

/// Resolution strategy for classifying a datum under a partition.
pub trait Resolver {
    /// Classify a single datum value using this resolution strategy.
    fn resolve(&self, value: u8, partition: &Partition) -> DatumClass;

    /// Human-readable name of this resolver.
    fn name(&self) -> &'static str;
}

/// Euclidean resolver — for EuclideanDomain/PID/UFD types.
///
/// At Q0, classifies via precomputed partition bitsets.
/// At Q1+, would use GCD-based Euclidean division.
///
/// Zero-sized type (ZST).
pub struct EuclideanResolver;

impl Resolver for EuclideanResolver {
    #[inline]
    fn resolve(&self, value: u8, partition: &Partition) -> DatumClass {
        classify_from_partition(value, partition)
    }

    #[inline]
    fn name(&self) -> &'static str {
        "EuclideanResolver"
    }
}

/// Exhaustive resolver — for CommutativeRing/IntegralDomain types.
///
/// At Q0, classifies via precomputed partition bitsets.
/// At Q1+, would use brute-force trial division.
///
/// Zero-sized type (ZST).
pub struct ExhaustiveResolver;

impl Resolver for ExhaustiveResolver {
    #[inline]
    fn resolve(&self, value: u8, partition: &Partition) -> DatumClass {
        classify_from_partition(value, partition)
    }

    #[inline]
    fn name(&self) -> &'static str {
        "ExhaustiveResolver"
    }
}

/// Static dispatch wrapper that selects a resolver based on algebraic stratum.
pub struct StratumDispatch;

impl StratumDispatch {
    /// Classify a datum by dispatching to the appropriate resolver.
    #[inline]
    pub fn resolve(value: u8, partition: &Partition, stratum: AlgebraicStratum) -> DatumClass {
        // At Q0, both resolvers use the same partition-based classification.
        // The dispatch records which algorithm is semantically correct.
        let _ = stratum; // structural distinction preserved for Q1+
        classify_from_partition(value, partition)
    }

    /// Return the resolver for a given stratum.
    pub fn resolver_for(stratum: AlgebraicStratum) -> &'static dyn Resolver {
        if stratum.has_euclidean_division() {
            &EuclideanResolver
        } else {
            &ExhaustiveResolver
        }
    }
}

/// Classify a datum by checking partition membership.
///
/// Shared implementation for both resolvers at Q0.
#[inline]
fn classify_from_partition(value: u8, partition: &Partition) -> DatumClass {
    if partition.irr().contains(value) {
        DatumClass::Irreducible
    } else if partition.units().contains(value) {
        DatumClass::Unit
    } else if partition.red().contains(value) {
        DatumClass::Reducible
    } else {
        DatumClass::External
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{TransformCertificate, TypeDeclaration};

    fn all_decls() -> [TypeDeclaration; 4] {
        [
            TypeDeclaration::integer_mul(),
            TypeDeclaration::poly_gf2(),
            TypeDeclaration::poly_gf3(),
            TypeDeclaration::poly_gf5(),
        ]
    }

    #[test]
    fn euclidean_resolver_matches_certificate() {
        let resolver = EuclideanResolver;
        for decl in &all_decls() {
            let cert = TransformCertificate::compute(decl);
            let part = Partition::compute(decl);
            for v in 0..=255u8 {
                assert_eq!(
                    resolver.resolve(v, &part),
                    cert.classify(v),
                    "mismatch at {v} for {:?}",
                    decl.op()
                );
            }
        }
    }

    #[test]
    fn exhaustive_resolver_matches_certificate() {
        let resolver = ExhaustiveResolver;
        for decl in &all_decls() {
            let cert = TransformCertificate::compute(decl);
            let part = Partition::compute(decl);
            for v in 0..=255u8 {
                assert_eq!(
                    resolver.resolve(v, &part),
                    cert.classify(v),
                    "mismatch at {v} for {:?}",
                    decl.op()
                );
            }
        }
    }

    #[test]
    fn stratum_dispatch_matches_certificate() {
        for decl in &all_decls() {
            let cert = TransformCertificate::compute(decl);
            let part = Partition::compute(decl);
            let stratum = AlgebraicStratum::classify(decl.op());
            for v in 0..=255u8 {
                assert_eq!(
                    StratumDispatch::resolve(v, &part, stratum),
                    cert.classify(v),
                    "dispatch mismatch at {v} for {:?}",
                    decl.op()
                );
            }
        }
    }

    #[test]
    fn euclidean_types_use_euclidean_resolver() {
        for op in [
            BinaryOp::IntegerMul,
            BinaryOp::PolyGf2Mul,
            BinaryOp::PolyGf3Mul,
            BinaryOp::PolyGf5Mul,
        ] {
            let stratum = AlgebraicStratum::classify(op);
            let resolver = StratumDispatch::resolver_for(stratum);
            assert_eq!(resolver.name(), "EuclideanResolver");
        }
    }

    #[test]
    fn zst_sizes() {
        assert_eq!(core::mem::size_of::<EuclideanResolver>(), 0);
        assert_eq!(core::mem::size_of::<ExhaustiveResolver>(), 0);
        assert_eq!(core::mem::size_of::<StratumDispatch>(), 0);
    }

    #[test]
    fn resolver_names() {
        assert_eq!(EuclideanResolver.name(), "EuclideanResolver");
        assert_eq!(ExhaustiveResolver.name(), "ExhaustiveResolver");
    }

    use crate::frame::BinaryOp;
}
