//! Binary basis decomposition for taxons.
//!
//! Each taxon can be uniquely decomposed into a sum of basis elements
//! (powers of 2). This corresponds to the binary representation of the
//! byte value.
//!
//! The 8 basis elements are derived from O=8 (octonion dimension):
//! - 2^0 = 1 (⠁)
//! - 2^1 = 2 (⠂)
//! - 2^2 = 4 (⠄)
//! - 2^3 = 8 (⠈)
//! - 2^4 = 16 (⠐)
//! - 2^5 = 32 (⠠)
//! - 2^6 = 64 (⡀)
//! - 2^7 = 128 (⢀)

use super::constants::O;
use super::taxon::Taxon;
use crate::lut::stratum_q0;

/// The 8 basis taxons (powers of 2).
///
/// These form the lattice atoms from which all taxons can be composed.
/// Derived from O=8 (octonion dimension).
///
/// # Braille Correspondence
///
/// Each basis element corresponds to a single Braille dot:
/// - Bit 0 (value 1) → Dot 1 (top-left)
/// - Bit 1 (value 2) → Dot 2 (middle-left)
/// - Bit 2 (value 4) → Dot 3 (bottom-left)
/// - Bit 3 (value 8) → Dot 4 (top-right/bottom-left in 8-dot)
/// - Bit 4 (value 16) → Dot 5
/// - Bit 5 (value 32) → Dot 6
/// - Bit 6 (value 64) → Dot 7
/// - Bit 7 (value 128) → Dot 8
pub const BASIS: [Taxon; O] = [
    Taxon::new(1),   // 2^0
    Taxon::new(2),   // 2^1
    Taxon::new(4),   // 2^2
    Taxon::new(8),   // 2^3
    Taxon::new(16),  // 2^4
    Taxon::new(32),  // 2^5
    Taxon::new(64),  // 2^6
    Taxon::new(128), // 2^7
];

/// Decomposes a taxon into its basis representation.
///
/// Returns an array of 8 booleans indicating which basis elements
/// are present (i.e., the binary representation).
///
/// # Example
///
/// ```
/// use uor::{Taxon, basis::decompose};
///
/// let bits = decompose(Taxon::new(17));
/// // 17 = 0b00010001 = 2^0 + 2^4 = 1 + 16
/// assert_eq!(bits, [true, false, false, false, true, false, false, false]);
/// ```
#[inline]
#[must_use]
pub const fn decompose(taxon: Taxon) -> [bool; O] {
    let v = taxon.value();
    [
        v & 1 != 0,
        v & 2 != 0,
        v & 4 != 0,
        v & 8 != 0,
        v & 16 != 0,
        v & 32 != 0,
        v & 64 != 0,
        v & 128 != 0,
    ]
}

/// Composes a taxon from basis components.
///
/// # Example
///
/// ```
/// use uor::{Taxon, basis::compose};
///
/// // 17 = 2^0 + 2^4
/// let bits = [true, false, false, false, true, false, false, false];
/// assert_eq!(compose(bits), Taxon::new(17));
/// ```
#[inline]
#[must_use]
pub const fn compose(bits: [bool; O]) -> Taxon {
    let mut value: u8 = 0;
    if bits[0] {
        value |= 1;
    }
    if bits[1] {
        value |= 2;
    }
    if bits[2] {
        value |= 4;
    }
    if bits[3] {
        value |= 8;
    }
    if bits[4] {
        value |= 16;
    }
    if bits[5] {
        value |= 32;
    }
    if bits[6] {
        value |= 64;
    }
    if bits[7] {
        value |= 128;
    }
    Taxon::new(value)
}

/// Returns the Hamming weight (number of set bits / basis elements).
///
/// This is equivalent to `taxon.weight()` but provided here for
/// convenience when working with basis operations.
///
/// # Example
///
/// ```
/// use uor::{Taxon, basis::weight};
///
/// assert_eq!(weight(Taxon::new(0)), 0);
/// assert_eq!(weight(Taxon::new(17)), 2);  // 1 + 16
/// assert_eq!(weight(Taxon::new(255)), 8);
/// ```
/// Uses O(1) LUT lookup via `stratum_q0`.
#[inline]
#[must_use]
pub const fn weight(taxon: Taxon) -> u8 {
    stratum_q0(taxon.value())
}

/// Returns the indices of set bits (present basis elements).
///
/// # Example
///
/// ```
/// use uor::{Taxon, basis::indices};
///
/// let idx = indices(Taxon::new(17)); // 17 = 1 + 16 = 2^0 + 2^4
/// assert_eq!(idx, [Some(0), Some(4), None, None, None, None, None, None]);
/// ```
#[inline]
#[must_use]
pub const fn indices(taxon: Taxon) -> [Option<u8>; O] {
    let v = taxon.value();
    let mut result = [None; O];
    let mut pos = 0;

    let mut i = 0;
    while i < O {
        if v & (1 << i) != 0 {
            result[pos] = Some(i as u8);
            pos += 1;
        }
        i += 1;
    }

    result
}

/// Returns the dot numbers (1-8) for the Braille representation.
///
/// Braille dots are numbered 1-8, not 0-7. This function returns
/// which dots are raised for the given taxon.
///
/// # Example
///
/// ```
/// use uor::{Taxon, basis::dots};
///
/// let d = dots(Taxon::new(17)); // 17 = bit 0 + bit 4
/// assert_eq!(d, [Some(1), Some(5), None, None, None, None, None, None]);
/// ```
#[inline]
#[must_use]
pub const fn dots(taxon: Taxon) -> [Option<u8>; O] {
    let idx = indices(taxon);
    let mut result = [None; O];

    let mut i = 0;
    while i < O {
        if let Some(bit) = idx[i] {
            result[i] = Some(bit + 1); // Dots are 1-indexed
        }
        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basis_values() {
        for (i, b) in BASIS.iter().enumerate() {
            assert_eq!(b.value(), 1 << i);
            assert!(b.is_basis());
        }
    }

    #[test]
    fn test_decompose_compose_roundtrip() {
        for i in 0..=255u8 {
            let t = Taxon::new(i);
            let bits = decompose(t);
            let reconstructed = compose(bits);
            assert_eq!(reconstructed, t);
        }
    }

    #[test]
    fn test_decompose_zero() {
        let bits = decompose(Taxon::new(0));
        assert_eq!(bits, [false; O]);
    }

    #[test]
    fn test_decompose_max() {
        let bits = decompose(Taxon::new(255));
        assert_eq!(bits, [true; O]);
    }

    #[test]
    fn test_decompose_17() {
        // 17 = 0b00010001 = 1 + 16
        let bits = decompose(Taxon::new(17));
        assert_eq!(bits, [true, false, false, false, true, false, false, false]);
    }

    #[test]
    fn test_weight() {
        assert_eq!(weight(Taxon::new(0)), 0);
        assert_eq!(weight(Taxon::new(1)), 1);
        assert_eq!(weight(Taxon::new(3)), 2);
        assert_eq!(weight(Taxon::new(17)), 2);
        assert_eq!(weight(Taxon::new(255)), 8);
    }

    #[test]
    fn test_indices() {
        let idx = indices(Taxon::new(17));
        // 17 = 2^0 + 2^4, so indices 0 and 4
        assert_eq!(idx[0], Some(0));
        assert_eq!(idx[1], Some(4));
        assert_eq!(idx[2], None);
    }

    #[test]
    fn test_dots() {
        let d = dots(Taxon::new(17));
        // Bits 0 and 4 → Dots 1 and 5
        assert_eq!(d[0], Some(1));
        assert_eq!(d[1], Some(5));
        assert_eq!(d[2], None);
    }

    #[test]
    fn test_basis_domains() {
        use crate::Domain;

        // Powers of 2 oscillate between Psi and Delta (never Theta)
        // 1 = Psi, 2 = Delta, 4 = Psi, 8 = Delta, ...
        for (i, b) in BASIS.iter().enumerate() {
            let expected = if i % 2 == 0 {
                Domain::Psi
            } else {
                Domain::Delta
            };
            assert_eq!(b.domain(), expected, "Basis {} has wrong domain", b.value());
        }
    }
}
