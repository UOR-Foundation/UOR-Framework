//! Type registry — indexed collection of all Q0 type declarations.
//!
//! Bundles the canonical type declarations with their pre-computed partitions,
//! algebraic strata, and observable signatures. Provides O(1) access by
//! index or [`BinaryOp`] reverse lookup.
//!
//! This is component 4 of the nine-component invariance frame.
//!
//! # Examples
//!
//! ```
//! use uor::frame::TypeRegistry;
//!
//! let reg = TypeRegistry::compute();
//! assert_eq!(reg.len(), 4);
//! let (decl, part) = reg.get(0);
//! assert_eq!(decl.carrier_len(), 254); // T₂
//! ```

use super::cross_field::TYPE_COUNT;
use super::{AlgebraicStratum, BinaryOp, ObservableSignature, Partition, TypeDeclaration};

/// Indexed collection of all Q0 type declarations with pre-computed data.
///
/// Stores declarations, partitions, strata, and observable signatures
/// for all four types in canonical order.
///
/// Size: ~900 bytes (4 × (40 + 128 + 1 + 56) = 900).
pub struct TypeRegistry {
    decls: [TypeDeclaration; TYPE_COUNT],
    partitions: [Partition; TYPE_COUNT],
    strata: [AlgebraicStratum; TYPE_COUNT],
    signatures: [ObservableSignature; TYPE_COUNT],
}

impl TypeRegistry {
    /// Build the registry from all four Q0 type constructors.
    pub fn compute() -> Self {
        let decls = canonical_decls();
        let partitions: [Partition; TYPE_COUNT] =
            core::array::from_fn(|i| Partition::compute(&decls[i]));
        let strata: [AlgebraicStratum; TYPE_COUNT] =
            core::array::from_fn(|i| AlgebraicStratum::classify(decls[i].op()));
        let signatures: [ObservableSignature; TYPE_COUNT] =
            core::array::from_fn(|i| ObservableSignature::compute(partitions[i].irr()));
        Self {
            decls,
            partitions,
            strata,
            signatures,
        }
    }

    /// Number of types in the registry (always 4).
    #[inline]
    pub const fn len(&self) -> usize {
        TYPE_COUNT
    }

    /// Always false — the registry always has exactly 4 types.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Access type declaration and partition by index.
    #[inline]
    pub fn get(&self, idx: usize) -> (&TypeDeclaration, &Partition) {
        (&self.decls[idx], &self.partitions[idx])
    }

    /// Access all four components for a type by index.
    #[inline]
    pub fn get_full(
        &self,
        idx: usize,
    ) -> (
        &TypeDeclaration,
        &Partition,
        AlgebraicStratum,
        &ObservableSignature,
    ) {
        (
            &self.decls[idx],
            &self.partitions[idx],
            self.strata[idx],
            &self.signatures[idx],
        )
    }

    /// Reverse lookup: find the index of a type by its binary operation.
    #[inline]
    pub fn index_of(&self, op: BinaryOp) -> Option<usize> {
        self.decls.iter().position(|d| d.op() == op)
    }

    /// Algebraic stratum for the type at the given index.
    #[inline]
    pub fn stratum(&self, idx: usize) -> AlgebraicStratum {
        self.strata[idx]
    }

    /// The declarations array.
    #[inline]
    pub fn decls(&self) -> &[TypeDeclaration; TYPE_COUNT] {
        &self.decls
    }

    /// The partitions array.
    #[inline]
    pub fn partitions(&self) -> &[Partition; TYPE_COUNT] {
        &self.partitions
    }

    /// Iterate over `(index, declaration, partition)` triples.
    pub fn iter(&self) -> RegistryIter<'_> {
        RegistryIter { reg: self, idx: 0 }
    }
}

/// Iterator over registry entries.
pub struct RegistryIter<'a> {
    reg: &'a TypeRegistry,
    idx: usize,
}

impl<'a> Iterator for RegistryIter<'a> {
    type Item = (usize, &'a TypeDeclaration, &'a Partition);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < TYPE_COUNT {
            let i = self.idx;
            self.idx += 1;
            Some((i, &self.reg.decls[i], &self.reg.partitions[i]))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = TYPE_COUNT - self.idx;
        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for RegistryIter<'a> {}

/// The four Q0 type declarations in canonical order.
pub(crate) fn canonical_decls() -> [TypeDeclaration; TYPE_COUNT] {
    [
        TypeDeclaration::integer_mul(),
        TypeDeclaration::poly_gf2(),
        TypeDeclaration::poly_gf3(),
        TypeDeclaration::poly_gf5(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction_succeeds() {
        let reg = TypeRegistry::compute();
        assert_eq!(reg.len(), 4);
    }

    #[test]
    fn canonical_order() {
        let reg = TypeRegistry::compute();
        assert_eq!(reg.get(0).0.op(), BinaryOp::IntegerMul);
        assert_eq!(reg.get(1).0.op(), BinaryOp::PolyGf2Mul);
        assert_eq!(reg.get(2).0.op(), BinaryOp::PolyGf3Mul);
        assert_eq!(reg.get(3).0.op(), BinaryOp::PolyGf5Mul);
        assert_eq!(reg.len(), 4);
    }

    #[test]
    fn indexed_access() {
        let reg = TypeRegistry::compute();
        let (decl, part) = reg.get(0);
        assert_eq!(decl.carrier_len(), 254);
        assert_eq!(part.irr().len(), 54);
    }

    #[test]
    fn get_full_returns_all_components() {
        let reg = TypeRegistry::compute();
        let (decl, part, stratum, sig) = reg.get_full(0);
        assert_eq!(decl.op(), BinaryOp::IntegerMul);
        assert_eq!(part.irr().len(), 54);
        assert_eq!(stratum, AlgebraicStratum::EuclideanDomain);
        assert!(sig.stratum > 0.0);
    }

    #[test]
    fn reverse_lookup() {
        let reg = TypeRegistry::compute();
        assert_eq!(reg.index_of(BinaryOp::IntegerMul), Some(0));
        assert_eq!(reg.index_of(BinaryOp::PolyGf2Mul), Some(1));
        assert_eq!(reg.index_of(BinaryOp::PolyGf5Mul), Some(3));
    }

    #[test]
    fn stratum_access() {
        let reg = TypeRegistry::compute();
        assert_eq!(reg.stratum(0), AlgebraicStratum::EuclideanDomain);
        assert_eq!(reg.stratum(3), AlgebraicStratum::EuclideanDomain);
    }

    #[test]
    fn iterator_yields_5_items() {
        let reg = TypeRegistry::compute();
        let items: alloc::vec::Vec<_> = reg.iter().collect();
        assert_eq!(items.len(), 4);
    }

    #[test]
    fn iterator_indices_sequential() {
        let reg = TypeRegistry::compute();
        for (i, (idx, _, _)) in reg.iter().enumerate() {
            assert_eq!(idx, i);
        }
    }

    #[test]
    fn all_partitions_verified() {
        let reg = TypeRegistry::compute();
        for (_, _, part) in reg.iter() {
            assert!(part.verify(), "partition invariant failed");
        }
    }

    #[test]
    fn all_types_are_euclidean_stratum() {
        let reg = TypeRegistry::compute();
        for i in 0..reg.len() {
            assert_eq!(reg.stratum(i), AlgebraicStratum::EuclideanDomain);
        }
    }
}
