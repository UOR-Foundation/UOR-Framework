//! Constants derived from axioms T=3, O=8.
//!
//! All constants in UOR are algebraically derived from the two foundational
//! axioms. No constant is arbitrary.

/// Triality axiom (T = 3).
///
/// The prime order of symmetry. Represents the three-fold phase structure
/// and the number of triadic domains (Theta, Psi, Delta).
pub const T: usize = 3;

/// Octonion dimension (O = 8).
///
/// The dimension of octonion algebra. Represents the number of bits in a byte
/// and the number of basis elements for binary decomposition.
pub const O: usize = 8;

/// Binary depth (B = 2^(O-T) = 32).
///
/// Derived from the gap between octonion dimension and triality.
/// Represents the number of powers of 2 in the byte-structured basis.
pub const B: usize = 1 << (O - T); // 32

/// Toroidal boundary size (12,288 cells).
///
/// The number of cells in the toroidal execution substrate.
/// Computed as 48 pages × 256 byte slots.
pub const BOUNDARY_SIZE: usize = 12_288;

/// Byte cardinality (2^O = 256).
///
/// The total number of distinct taxons (byte values).
pub const BYTE_CARDINALITY: usize = 1 << O; // 256

/// Unicode Braille base codepoint (U+2800).
///
/// The Braille Patterns block begins at this codepoint.
/// Value 0 maps to U+2800 (BRAILLE PATTERN BLANK).
pub const BRAILLE_BASE: u32 = 0x2800;

/// Unicode Braille maximum codepoint (U+28FF).
///
/// Value 255 maps to U+28FF (BRAILLE PATTERN DOTS-12345678).
pub const BRAILLE_MAX: u32 = 0x28FF;

/// Domain cardinalities.
///
/// The number of taxons in each domain:
/// - Theta (mod 3 = 0): 86 taxons (0, 3, 6, ..., 255)
/// - Psi (mod 3 = 1): 85 taxons (1, 4, 7, ..., 253)
/// - Delta (mod 3 = 2): 85 taxons (2, 5, 8, ..., 254)
///
/// Note: 86 + 85 + 85 = 256 = BYTE_CARDINALITY
pub const DOMAIN_CARDINALITIES: [usize; T] = [86, 85, 85];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axiom_derivation() {
        assert_eq!(T, 3);
        assert_eq!(O, 8);
        assert_eq!(BYTE_CARDINALITY, 256);
        assert_eq!(BYTE_CARDINALITY, 1 << O);
    }

    #[test]
    fn test_braille_range() {
        assert_eq!(BRAILLE_MAX - BRAILLE_BASE + 1, BYTE_CARDINALITY as u32);
    }

    #[test]
    fn test_domain_cardinalities_sum() {
        let sum: usize = DOMAIN_CARDINALITIES.iter().sum();
        assert_eq!(sum, BYTE_CARDINALITY);
    }

    #[test]
    fn test_domain_cardinalities_computed() {
        // Verify cardinalities match actual counts
        let mut counts = [0usize; T];
        for i in 0..BYTE_CARDINALITY {
            counts[i % T] += 1;
        }
        assert_eq!(counts, DOMAIN_CARDINALITIES);
    }
}
