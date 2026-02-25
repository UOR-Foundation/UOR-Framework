//! Extension traits for domain-specific UOR usage.
//!
//! These traits allow domains (like SHA-256) to use UOR without UOR
//! knowing about them. This is dependency inversion: domains depend
//! on UOR, not vice versa.

use super::domain::Domain;
use super::taxon::Taxon;

/// Trait for types that can be addressed via UOR.
///
/// Implementors map their values to sequences of taxons,
/// providing universal addressing without UOR knowing about
/// the specific type.
///
/// # Example
///
/// ```ignore
/// // In a SHA-256 crate (not in UOR)
/// use uor::{Taxon, Addressable};
///
/// struct Sha256Hash([u8; 32]);
///
/// impl Addressable for Sha256Hash {
///     const TAXON_COUNT: usize = 32;
///
///     fn to_taxons(&self) -> impl Iterator<Item = Taxon> {
///         self.0.iter().map(|&b| Taxon::new(b))
///     }
///
///     fn from_taxons(taxons: impl Iterator<Item = Taxon>) -> Option<Self> {
///         let bytes: Vec<u8> = taxons.map(|t| t.value()).collect();
///         if bytes.len() == 32 {
///             let mut arr = [0u8; 32];
///             arr.copy_from_slice(&bytes);
///             Some(Sha256Hash(arr))
///         } else {
///             None
///         }
///     }
/// }
/// ```
pub trait Addressable {
    /// The number of taxons needed to address this type.
    const TAXON_COUNT: usize;

    /// Converts to a sequence of taxons.
    fn to_taxons(&self) -> impl Iterator<Item = Taxon>;

    /// Constructs from a sequence of taxons.
    ///
    /// Returns `None` if the taxon sequence is invalid for this type
    /// (e.g., wrong length, invalid values).
    fn from_taxons(taxons: impl Iterator<Item = Taxon>) -> Option<Self>
    where
        Self: Sized;
}

/// Trait for types that have triadic domain structure.
///
/// This allows types to participate in the triadic domain
/// classification without depending on specific implementations.
pub trait Triadic {
    /// Returns the primary domain of this value.
    fn primary_domain(&self) -> Domain;

    /// Returns true if this value is in the theta (structure) domain.
    #[inline]
    fn is_structural(&self) -> bool {
        self.primary_domain() == Domain::Theta
    }

    /// Returns true if this value is in the psi (unity) domain.
    #[inline]
    fn is_unitary(&self) -> bool {
        self.primary_domain() == Domain::Psi
    }

    /// Returns true if this value is in the delta (duality) domain.
    #[inline]
    fn is_dual(&self) -> bool {
        self.primary_domain() == Domain::Delta
    }
}

/// Trait for ring-like operations.
///
/// UOR taxons form a ring under modular arithmetic.
/// This trait allows other types to participate in ring operations.
pub trait Ring: Sized {
    /// The zero/identity element.
    const ZERO: Self;

    /// The multiplicative identity (one).
    const ONE: Self;

    /// Successor operation (add 1 in the ring).
    fn ring_succ(&self) -> Self;

    /// Predecessor operation (subtract 1 in the ring).
    fn ring_pred(&self) -> Self;

    /// Additive inverse.
    fn ring_neg(&self) -> Self;

    /// Addition in the ring.
    fn ring_add(&self, other: &Self) -> Self;

    /// Subtraction in the ring.
    fn ring_sub(&self, other: &Self) -> Self;
}

// Implement Triadic for Taxon
impl Triadic for Taxon {
    #[inline]
    fn primary_domain(&self) -> Domain {
        self.domain()
    }
}

// Implement Ring for Taxon
impl Ring for Taxon {
    const ZERO: Self = Taxon::MIN;
    const ONE: Self = Taxon::ONE;

    #[inline]
    fn ring_succ(&self) -> Self {
        self.succ()
    }

    #[inline]
    fn ring_pred(&self) -> Self {
        self.pred()
    }

    #[inline]
    fn ring_neg(&self) -> Self {
        // Additive inverse in Z/256Z: -x = 256 - x = (0 - x) mod 256
        Taxon::new(0u8.wrapping_sub(self.value()))
    }

    #[inline]
    fn ring_add(&self, other: &Self) -> Self {
        // Addition in Z/256Z
        Taxon::new(self.value().wrapping_add(other.value()))
    }

    #[inline]
    fn ring_sub(&self, other: &Self) -> Self {
        // Subtraction in Z/256Z
        Taxon::new(self.value().wrapping_sub(other.value()))
    }
}

// Implement Addressable for single byte
impl Addressable for u8 {
    const TAXON_COUNT: usize = 1;

    fn to_taxons(&self) -> impl Iterator<Item = Taxon> {
        core::iter::once(Taxon::new(*self))
    }

    fn from_taxons(mut taxons: impl Iterator<Item = Taxon>) -> Option<Self> {
        taxons.next().map(|t| t.value())
    }
}

// Implement Addressable for byte slices of fixed size
impl<const N: usize> Addressable for [u8; N] {
    const TAXON_COUNT: usize = N;

    fn to_taxons(&self) -> impl Iterator<Item = Taxon> {
        self.iter().map(|&b| Taxon::new(b))
    }

    fn from_taxons(taxons: impl Iterator<Item = Taxon>) -> Option<Self> {
        let mut arr = [0u8; N];
        let mut count = 0;

        for (i, t) in taxons.enumerate() {
            if i >= N {
                return None; // Too many taxons
            }
            arr[i] = t.value();
            count += 1;
        }

        if count == N {
            Some(arr)
        } else {
            None // Not enough taxons
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn test_triadic_for_taxon() {
        assert!(Taxon::new(0).is_structural()); // 0 % 3 == 0
        assert!(Taxon::new(1).is_unitary()); // 1 % 3 == 1
        assert!(Taxon::new(2).is_dual()); // 2 % 3 == 2
    }

    #[test]
    fn test_ring_for_taxon() {
        let t = Taxon::new(10);
        assert_eq!(t.ring_succ(), Taxon::new(11));
        assert_eq!(t.ring_pred(), Taxon::new(9));
        assert_eq!(t.ring_add(&Taxon::new(5)), Taxon::new(15));
        assert_eq!(t.ring_sub(&Taxon::new(5)), Taxon::new(5));
    }

    #[test]
    fn test_addressable_u8() {
        let b: u8 = 42;
        let taxons: Vec<_> = b.to_taxons().collect();
        assert_eq!(taxons.len(), 1);
        assert_eq!(taxons[0], Taxon::new(42));

        let reconstructed = u8::from_taxons(taxons.into_iter());
        assert_eq!(reconstructed, Some(42));
    }

    #[test]
    fn test_addressable_array() {
        let arr: [u8; 4] = [1, 2, 3, 4];
        let taxons: Vec<_> = arr.to_taxons().collect();
        assert_eq!(taxons.len(), 4);

        let reconstructed = <[u8; 4]>::from_taxons(taxons.into_iter());
        assert_eq!(reconstructed, Some([1, 2, 3, 4]));
    }

    #[test]
    fn test_addressable_array_wrong_length() {
        let taxons = vec![Taxon::new(1), Taxon::new(2)];
        let result = <[u8; 4]>::from_taxons(taxons.into_iter());
        assert_eq!(result, None); // Not enough taxons
    }
}
