//! Core Taxon type - the universal byte reference.
//!
//! A Taxon represents a single byte (0-255) with a bijective mapping
//! to Unicode Braille U+2800-U+28FF. This provides every byte value
//! with a unique, stable IRI identity.
//!
//! # Identity Principle
//!
//! The identity of a Taxon is its Braille codepoint:
//! - Value 0 → U+2800 (BRAILLE PATTERN BLANK)
//! - Value 255 → U+28FF (BRAILLE PATTERN DOTS-12345678)
//!
//! Everything else (domain, rank, etc.) is a computed property, not identity.
//!
//! # Zero-Copy Contract
//!
//! Taxon is a transparent wrapper around u8. All operations are O(1)
//! and require no heap allocation.

use super::constants::BRAILLE_BASE;
use super::domain::Domain;
use crate::lut::{curvature_q0, stratum_q0};
use core::fmt;

/// A universal object reference for a byte value.
///
/// Maps bijectively to Unicode Braille U+2800-U+28FF.
/// The Braille codepoint IS the identity.
///
/// # Example
///
/// ```
/// use uor::Taxon;
///
/// let t = Taxon::new(17);
/// assert_eq!(t.value(), 17);
/// assert_eq!(t.codepoint(), 0x2811);
/// assert_eq!(t.braille(), '⠑');
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Taxon(u8);

impl Taxon {
    /// The minimum taxon (value 0, U+2800, ⠀).
    pub const MIN: Self = Self(0);

    /// The maximum taxon (value 255, U+28FF, ⣿).
    pub const MAX: Self = Self(255);

    /// The unity taxon (value 1, U+2801, ⠁).
    pub const ONE: Self = Self(1);

    /// Creates a new Taxon from a byte value.
    ///
    /// This is a total function - all byte values are valid taxons.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::Taxon;
    ///
    /// let t = Taxon::new(42);
    /// assert_eq!(t.value(), 42);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(value: u8) -> Self {
        Self(value)
    }

    /// Returns the raw byte value.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::Taxon;
    ///
    /// assert_eq!(Taxon::new(17).value(), 17);
    /// ```
    #[inline]
    #[must_use]
    pub const fn value(self) -> u8 {
        self.0
    }

    /// Returns the Unicode Braille codepoint (identity).
    ///
    /// The codepoint is always in the range U+2800 to U+28FF.
    /// This IS the identity of the taxon.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::Taxon;
    ///
    /// assert_eq!(Taxon::new(0).codepoint(), 0x2800);
    /// assert_eq!(Taxon::new(255).codepoint(), 0x28FF);
    /// ```
    #[inline]
    #[must_use]
    pub const fn codepoint(self) -> u32 {
        BRAILLE_BASE + self.0 as u32
    }

    /// Returns the Braille character.
    ///
    /// The glyph visually encodes the binary representation:
    /// each dot position corresponds to a bit.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::Taxon;
    ///
    /// assert_eq!(Taxon::new(0).braille(), '⠀');   // blank
    /// assert_eq!(Taxon::new(1).braille(), '⠁');   // dot 1
    /// assert_eq!(Taxon::new(255).braille(), '⣿'); // all dots
    /// ```
    #[inline]
    #[must_use]
    pub const fn braille(self) -> char {
        // SAFETY: BRAILLE_BASE + 0..255 is always a valid Unicode codepoint
        // in the Braille Patterns block.
        unsafe { char::from_u32_unchecked(self.codepoint()) }
    }

    /// Returns the domain (mod 3).
    ///
    /// This is a computed property, not part of the identity.
    ///
    /// - Domain θ (Theta): value % 3 == 0 (Structure)
    /// - Domain ψ (Psi): value % 3 == 1 (Unity)
    /// - Domain δ (Delta): value % 3 == 2 (Duality)
    ///
    /// # Example
    ///
    /// ```
    /// use uor::{Taxon, Domain};
    ///
    /// assert_eq!(Taxon::new(0).domain(), Domain::Theta);
    /// assert_eq!(Taxon::new(1).domain(), Domain::Psi);
    /// assert_eq!(Taxon::new(2).domain(), Domain::Delta);
    /// assert_eq!(Taxon::new(17).domain(), Domain::Delta); // 17 % 3 == 2
    /// ```
    #[inline]
    #[must_use]
    pub const fn domain(self) -> Domain {
        Domain::from_residue(self.0 % 3)
    }

    /// Returns the rank within the domain (value / 3).
    ///
    /// This is a computed property, not part of the identity.
    /// Range: 0..=85 (since 255 / 3 = 85).
    ///
    /// # Example
    ///
    /// ```
    /// use uor::Taxon;
    ///
    /// assert_eq!(Taxon::new(0).rank(), 0);
    /// assert_eq!(Taxon::new(3).rank(), 1);
    /// assert_eq!(Taxon::new(17).rank(), 5);  // 17 / 3 = 5
    /// assert_eq!(Taxon::new(96).rank(), 32); // 96 / 3 = 32
    /// ```
    #[inline]
    #[must_use]
    pub const fn rank(self) -> u8 {
        self.0 / 3
    }

    /// Successor in the ring (n + 1 mod 256).
    ///
    /// Wraps around: 255.succ() == 0.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::Taxon;
    ///
    /// assert_eq!(Taxon::new(0).succ(), Taxon::new(1));
    /// assert_eq!(Taxon::new(255).succ(), Taxon::new(0));
    /// ```
    #[inline]
    #[must_use]
    pub const fn succ(self) -> Self {
        Self(self.0.wrapping_add(1))
    }

    /// Predecessor in the ring (n - 1 mod 256).
    ///
    /// Wraps around: 0.pred() == 255.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::Taxon;
    ///
    /// assert_eq!(Taxon::new(1).pred(), Taxon::new(0));
    /// assert_eq!(Taxon::new(0).pred(), Taxon::new(255));
    /// ```
    #[inline]
    #[must_use]
    pub const fn pred(self) -> Self {
        Self(self.0.wrapping_sub(1))
    }

    /// Bitwise complement (n XOR 255).
    ///
    /// Flips all bits. Self-inverse: t.not().not() == t.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::Taxon;
    ///
    /// assert_eq!(Taxon::new(0).not(), Taxon::new(255));
    /// assert_eq!(Taxon::new(1).not(), Taxon::new(254));
    /// assert_eq!(Taxon::new(17).not(), Taxon::new(238));
    /// ```
    #[inline]
    #[must_use]
    pub const fn not(self) -> Self {
        Self(self.0 ^ 255)
    }

    /// Returns true if this is the zero taxon.
    #[inline]
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns true if this is a power of 2 (basis element).
    ///
    /// Powers of 2: 1, 2, 4, 8, 16, 32, 64, 128.
    /// Note: 0 is not considered a power of 2.
    #[inline]
    #[must_use]
    pub const fn is_basis(self) -> bool {
        self.0 != 0 && (self.0 & (self.0 - 1)) == 0
    }

    /// Returns the Hamming weight (number of set bits).
    ///
    /// Uses O(1) LUT lookup via `stratum_q0`.
    #[inline]
    #[must_use]
    pub const fn weight(self) -> u8 {
        stratum_q0(self.0)
    }

    /// Returns the curvature: Hamming distance to successor.
    ///
    /// Curvature measures how many bits flip when incrementing by one.
    /// If the value has L trailing 1-bits, then succ(x) flips L+1 bits:
    /// the L trailing ones clear to zero and the next zero sets to one.
    ///
    /// For x = 255 (all ones), all 8 bits flip (wraps to 0).
    ///
    /// Mean curvature across all 256 values is approximately 1.992,
    /// converging to 2.0 as quantum level increases.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::Taxon;
    ///
    /// assert_eq!(Taxon::new(0).curvature(), 1);   // 0 → 1: one bit flips
    /// assert_eq!(Taxon::new(1).curvature(), 2);   // 1 → 2: two bits flip (01 → 10)
    /// assert_eq!(Taxon::new(3).curvature(), 3);   // 3 → 4: three bits flip (011 → 100)
    /// assert_eq!(Taxon::new(7).curvature(), 4);   // 7 → 8: four bits flip
    /// assert_eq!(Taxon::new(255).curvature(), 8); // 255 → 0: all bits flip
    /// ```
    ///
    /// Uses O(1) LUT lookup via `curvature_q0`.
    #[inline]
    #[must_use]
    pub const fn curvature(self) -> u8 {
        curvature_q0(self.0)
    }
}

impl fmt::Debug for Taxon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Taxon({}, {}, {}:{})",
            self.0,
            self.braille(),
            self.domain().symbol(),
            self.rank()
        )
    }
}

impl fmt::Display for Taxon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.braille())
    }
}

impl From<u8> for Taxon {
    #[inline]
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<Taxon> for u8 {
    #[inline]
    fn from(taxon: Taxon) -> Self {
        taxon.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn test_braille_bijection() {
        for i in 0..=255u8 {
            let t = Taxon::new(i);
            assert_eq!(t.value(), i);
            assert_eq!(t.codepoint(), BRAILLE_BASE + i as u32);
        }
    }

    #[test]
    fn test_braille_characters() {
        assert_eq!(Taxon::new(0).braille(), '⠀');
        assert_eq!(Taxon::new(1).braille(), '⠁');
        assert_eq!(Taxon::new(255).braille(), '⣿');
    }

    #[test]
    fn test_domain_classification() {
        use crate::Domain;

        for i in 0..=255u8 {
            let t = Taxon::new(i);
            let expected = match i % 3 {
                0 => Domain::Theta,
                1 => Domain::Psi,
                _ => Domain::Delta,
            };
            assert_eq!(t.domain(), expected);
        }
    }

    #[test]
    fn test_rank() {
        assert_eq!(Taxon::new(0).rank(), 0);
        assert_eq!(Taxon::new(1).rank(), 0);
        assert_eq!(Taxon::new(2).rank(), 0);
        assert_eq!(Taxon::new(3).rank(), 1);
        assert_eq!(Taxon::new(96).rank(), 32);
        assert_eq!(Taxon::new(255).rank(), 85);
    }

    #[test]
    fn test_succ_pred_inverse() {
        for i in 0..=255u8 {
            let t = Taxon::new(i);
            assert_eq!(t.succ().pred(), t);
            assert_eq!(t.pred().succ(), t);
        }
    }

    #[test]
    fn test_not_involution() {
        for i in 0..=255u8 {
            let t = Taxon::new(i);
            assert_eq!(t.not().not(), t);
        }
    }

    #[test]
    fn test_basis_elements() {
        let basis: Vec<u8> = (0..=255u8).filter(|&i| Taxon::new(i).is_basis()).collect();
        assert_eq!(basis, vec![1, 2, 4, 8, 16, 32, 64, 128]);
    }

    #[test]
    fn test_weight() {
        assert_eq!(Taxon::new(0).weight(), 0);
        assert_eq!(Taxon::new(1).weight(), 1);
        assert_eq!(Taxon::new(3).weight(), 2);
        assert_eq!(Taxon::new(255).weight(), 8);
    }

    #[test]
    fn test_curvature() {
        // Curvature = trailing 1-bits + 1 = Hamming distance to successor
        assert_eq!(Taxon::new(0).curvature(), 1); // 0→1: one bit flips
        assert_eq!(Taxon::new(1).curvature(), 2); // 1→2: 01→10 (two bits)
        assert_eq!(Taxon::new(2).curvature(), 1); // 2→3: 10→11 (one bit)
        assert_eq!(Taxon::new(3).curvature(), 3); // 3→4: 011→100 (three bits)
        assert_eq!(Taxon::new(7).curvature(), 4); // 7→8: 0111→1000 (four bits)
        assert_eq!(Taxon::new(15).curvature(), 5); // 15→16: five bits
        assert_eq!(Taxon::new(31).curvature(), 6); // 31→32: six bits
        assert_eq!(Taxon::new(63).curvature(), 7); // 63→64: seven bits
        assert_eq!(Taxon::new(127).curvature(), 8); // 127→128: eight bits
        assert_eq!(Taxon::new(255).curvature(), 8); // 255→0: all bits flip (wrap)
    }

    #[test]
    fn test_curvature_mean() {
        // Mean curvature across all 256 values should be ≈ 1.992
        // Sum = Σ curvature(x) for x in 0..256
        let sum: u32 = (0..=255u8).map(|i| Taxon::new(i).curvature() as u32).sum();
        let mean = sum as f64 / 256.0;

        // Theoretical: 2 - 2^(1-n) for n-bit integers
        // For n=8: 2 - 2^(-7) = 2 - 0.0078125 = 1.9921875
        assert!((mean - 1.9921875).abs() < 0.0001);
    }

    #[test]
    fn test_curvature_equals_hamming_to_succ() {
        // Verify curvature equals Hamming distance to successor for all values
        for i in 0..=255u8 {
            let t = Taxon::new(i);
            let succ = t.succ();
            let hamming_dist = (t.value() ^ succ.value()).count_ones() as u8;
            assert_eq!(t.curvature(), hamming_dist);
        }
    }

    #[test]
    fn test_key_values() {
        // 0 = blank
        let t0 = Taxon::new(0);
        assert_eq!(t0.domain(), Domain::Theta);
        assert_eq!(t0.rank(), 0);

        // 1 = Unity
        let t1 = Taxon::new(1);
        assert_eq!(t1.domain(), Domain::Psi);
        assert_eq!(t1.rank(), 0);

        // 17 = Fermat prime
        let t17 = Taxon::new(17);
        assert_eq!(t17.domain(), Domain::Delta);
        assert_eq!(t17.rank(), 5);

        // 96 = representative theta taxon at rank 32
        let t96 = Taxon::new(96);
        assert_eq!(t96.domain(), Domain::Theta);
        assert_eq!(t96.rank(), 32);
    }
}
