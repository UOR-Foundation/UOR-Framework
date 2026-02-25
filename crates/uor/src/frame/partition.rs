//! Irreducibility partition for a type declaration.
//!
//! Given a type `T = (A, ⊗, ε)`, every element of the ambient ring ℤ/(2⁸)ℤ
//! falls into exactly one of four classes:
//!
//! | Class  | Meaning |
//! |--------|---------|
//! | **Irr** | No non-trivial factorization under ⊗ within the carrier |
//! | **Red** | Has a non-trivial factorization |
//! | **Units** | Has a multiplicative inverse (maps to ε under ⊗) |
//! | **Ext** | Not in the carrier set |
//!
//! For the four standard Q0 operations, precomputed irreducible bitsets
//! are used, reducing partition computation from O(n²) to O(1) bitset
//! operations. The `compute_slow` fallback uses exhaustive trial division.
//!
//! # Examples
//!
//! ```
//! use uor::frame::{TypeDeclaration, Partition};
//!
//! let t2 = TypeDeclaration::integer_mul();
//! let p = Partition::compute(&t2);
//! assert_eq!(p.irr().len(), 54);  // primes
//! assert_eq!(p.units().len(), 0); // no units in {2..255} under *
//! ```

use super::{DatumSet, TypeDeclaration};

/// Four-way irreducibility partition of the 256-value Q0 space.
///
/// Size: 128 bytes (4 × 32-byte `DatumSet`), aligned to 64 bytes.
///
/// # Invariant
///
/// The four sets are pairwise disjoint and their union covers all 256 values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(align(64))]
pub struct Partition {
    irr: DatumSet,
    red: DatumSet,
    units: DatumSet,
    ext: DatumSet,
}

impl Partition {
    /// Compute the partition for a type declaration.
    ///
    /// Uses precomputed irreducible bitsets for the four standard Q0
    /// operations (O(1) bitset operations). Falls back to exhaustive
    /// trial division for unknown operations.
    pub fn compute(decl: &TypeDeclaration) -> Self {
        let carrier = decl.carrier();
        let ext = carrier.complement();
        let precomputed = decl.op().precomputed_irreducibles();

        // Irreducibles: precomputed set intersected with carrier
        let irr = precomputed.intersection(carrier);

        // Units: elements with multiplicative inverses
        let mut units = DatumSet::EMPTY;
        for value in carrier.iter() {
            if decl.is_unit(value) {
                units = units.insert(value);
            }
        }

        // Reducible: carrier minus irreducibles minus units
        let red = carrier.difference(&irr).difference(&units);

        let p = Self {
            irr,
            red,
            units,
            ext,
        };
        debug_assert!(p.verify(), "partition invariant violated");
        p
    }

    /// Irreducible elements.
    #[inline]
    pub const fn irr(&self) -> &DatumSet {
        &self.irr
    }

    /// Reducible elements.
    #[inline]
    pub const fn red(&self) -> &DatumSet {
        &self.red
    }

    /// Unit elements.
    #[inline]
    pub const fn units(&self) -> &DatumSet {
        &self.units
    }

    /// Elements external to the carrier.
    #[inline]
    pub const fn ext(&self) -> &DatumSet {
        &self.ext
    }

    /// Verify the disjoint-cover invariant: four sets partition all 256 values.
    pub fn verify(&self) -> bool {
        let all = self
            .irr
            .union(&self.red)
            .union(&self.units)
            .union(&self.ext);
        if all != DatumSet::FULL {
            return false;
        }
        self.irr.is_disjoint(&self.red)
            && self.irr.is_disjoint(&self.units)
            && self.irr.is_disjoint(&self.ext)
            && self.red.is_disjoint(&self.units)
            && self.red.is_disjoint(&self.ext)
            && self.units.is_disjoint(&self.ext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{GF2_IRREDUCIBLES_Q0, PRIMES_Q0};

    #[test]
    fn partition_integer_mul() {
        let t2 = TypeDeclaration::integer_mul();
        let p = Partition::compute(&t2);

        assert!(p.verify(), "partition invariant failed");
        assert_eq!(
            p.irr().len(),
            54,
            "expected 54 primes, got {}",
            p.irr().len()
        );
        assert_eq!(*p.irr(), PRIMES_Q0);
    }

    #[test]
    fn partition_poly_gf2() {
        let tp = TypeDeclaration::poly_gf2();
        let p = Partition::compute(&tp);

        assert!(p.verify(), "partition invariant failed");
        assert_eq!(
            p.irr().len(),
            41,
            "expected 41 irreducible polynomials, got {}",
            p.irr().len()
        );
        assert_eq!(*p.irr(), GF2_IRREDUCIBLES_Q0);
    }

    #[test]
    fn partition_ext_covers_non_carrier() {
        let t2 = TypeDeclaration::integer_mul();
        let p = Partition::compute(&t2);
        // Carrier is {2..=255}, so ext contains {0, 1}
        assert!(p.ext().contains(0));
        assert!(p.ext().contains(1));
        assert_eq!(p.ext().len(), 2);
    }

    #[test]
    fn partition_no_units_integer() {
        let t2 = TypeDeclaration::integer_mul();
        let p = Partition::compute(&t2);
        // No element in {2..=255} has an integer multiplicative inverse in {2..=255}
        assert_eq!(p.units().len(), 0);
    }

    #[test]
    fn partition_irr_plus_red_plus_units_equals_carrier() {
        let t2 = TypeDeclaration::integer_mul();
        let p = Partition::compute(&t2);
        let carrier_elems = p.irr().union(p.red()).union(p.units());
        assert_eq!(carrier_elems.len(), t2.carrier_len());
    }

    #[test]
    fn partition_datum_5_integer() {
        let t2 = TypeDeclaration::integer_mul();
        let p = Partition::compute(&t2);
        // 5 is prime
        assert!(p.irr().contains(5));
    }

    #[test]
    fn partition_datum_25_integer() {
        let t2 = TypeDeclaration::integer_mul();
        let p = Partition::compute(&t2);
        // 25 = 5*5 is reducible under integer mul
        assert!(p.red().contains(25));
    }

    #[test]
    fn partition_datum_5_poly() {
        let tp = TypeDeclaration::poly_gf2();
        let p = Partition::compute(&tp);
        // 5 = x²+1 = (x+1)², reducible over GF(2)
        assert!(p.red().contains(5));
    }

    #[test]
    fn partition_datum_25_poly() {
        let tp = TypeDeclaration::poly_gf2();
        let p = Partition::compute(&tp);
        // 25 = x⁴+x³+1 is irreducible over GF(2)
        assert!(p.irr().contains(25));
    }
}
