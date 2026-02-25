//! IRI generation for taxons.
//!
//! Each taxon has a unique IRI (Internationalized Resource Identifier)
//! in the UOR namespace. The format is:
//!
//! ```text
//! https://uor.foundation/u/UXXXX
//! ```
//!
//! Where XXXX is the 4-digit hex representation of the Unicode codepoint
//! (always in the range 2800-28FF).

use alloc::string::String;

use super::constants::{BRAILLE_BASE, BRAILLE_MAX};
use super::taxon::Taxon;

/// The base IRI for UOR references.
pub const BASE_IRI: &str = "https://uor.foundation/u/";

/// The length of the IRI suffix (e.g., "U2800").
pub const SUFFIX_LEN: usize = 5;

/// Formats the IRI suffix for a taxon.
///
/// Format: `UXXXX` where XXXX is the hex codepoint.
///
/// # Example
///
/// ```
/// use uor::{Taxon, iri::iri_suffix};
///
/// let suffix = iri_suffix(Taxon::new(0));
/// assert_eq!(&suffix, b"U2800");
///
/// let suffix = iri_suffix(Taxon::new(255));
/// assert_eq!(&suffix, b"U28FF");
///
/// let suffix = iri_suffix(Taxon::new(17));
/// assert_eq!(&suffix, b"U2811");
/// ```
#[inline]
#[must_use]
pub const fn iri_suffix(taxon: Taxon) -> [u8; SUFFIX_LEN] {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";

    let cp = taxon.codepoint();
    [
        b'U',
        HEX[((cp >> 12) & 0xF) as usize],
        HEX[((cp >> 8) & 0xF) as usize],
        HEX[((cp >> 4) & 0xF) as usize],
        HEX[(cp & 0xF) as usize],
    ]
}

/// Formats the IRI suffix as a string.
///
/// # Example
///
/// ```
/// use uor::{Taxon, iri::iri_suffix_str};
///
/// assert_eq!(iri_suffix_str(Taxon::new(0)), "U2800");
/// assert_eq!(iri_suffix_str(Taxon::new(17)), "U2811");
/// ```
#[inline]
#[must_use]
pub fn iri_suffix_str(taxon: Taxon) -> &'static str {
    // Pre-computed lookup table for all 256 suffixes
    static SUFFIXES: [[u8; 5]; 256] = {
        let mut table = [[0u8; 5]; 256];
        let mut i = 0;
        while i < 256 {
            table[i] = iri_suffix(Taxon::new(i as u8));
            i += 1;
        }
        table
    };

    // SAFETY: The table contains valid ASCII hex strings
    unsafe { core::str::from_utf8_unchecked(&SUFFIXES[taxon.value() as usize]) }
}

/// Parses a taxon from its IRI suffix (UXXXX format).
///
/// Returns `None` if:
/// - The suffix doesn't start with 'U'
/// - The hex digits are invalid
/// - The codepoint is outside the Braille range (2800-28FF)
///
/// # Example
///
/// ```
/// use uor::{Taxon, iri::parse_iri_suffix};
///
/// assert_eq!(parse_iri_suffix(b"U2800"), Some(Taxon::new(0)));
/// assert_eq!(parse_iri_suffix(b"U28FF"), Some(Taxon::new(255)));
/// assert_eq!(parse_iri_suffix(b"U2811"), Some(Taxon::new(17)));
///
/// // Invalid inputs
/// assert_eq!(parse_iri_suffix(b"X2800"), None);  // Wrong prefix
/// assert_eq!(parse_iri_suffix(b"U2700"), None);  // Outside Braille range
/// ```
#[inline]
#[must_use]
pub const fn parse_iri_suffix(suffix: &[u8; SUFFIX_LEN]) -> Option<Taxon> {
    if suffix[0] != b'U' {
        return None;
    }

    // Parse hex digits
    let mut codepoint: u32 = 0;

    let mut i = 1;
    while i < SUFFIX_LEN {
        let digit = match suffix[i] {
            b'0'..=b'9' => suffix[i] - b'0',
            b'A'..=b'F' => suffix[i] - b'A' + 10,
            b'a'..=b'f' => suffix[i] - b'a' + 10,
            _ => return None,
        };
        codepoint = codepoint * 16 + digit as u32;
        i += 1;
    }

    // Validate Braille range
    if codepoint < BRAILLE_BASE || codepoint > BRAILLE_MAX {
        return None;
    }

    Some(Taxon::new((codepoint - BRAILLE_BASE) as u8))
}

/// Parses a taxon from a full IRI.
///
/// # Example
///
/// ```
/// use uor::{Taxon, iri::parse_iri};
///
/// assert_eq!(parse_iri("https://uor.foundation/u/U2800"), Some(Taxon::new(0)));
/// assert_eq!(parse_iri("https://uor.foundation/u/U2811"), Some(Taxon::new(17)));
/// assert_eq!(parse_iri("invalid"), None);
/// ```
#[inline]
#[must_use]
pub fn parse_iri(iri: &str) -> Option<Taxon> {
    let suffix = iri.strip_prefix(BASE_IRI)?;
    if suffix.len() != SUFFIX_LEN {
        return None;
    }

    let bytes: [u8; SUFFIX_LEN] = suffix.as_bytes().try_into().ok()?;
    parse_iri_suffix(&bytes)
}

/// Formats the full IRI for a taxon.
///
/// # Example
///
/// ```
/// use uor::{Taxon, iri::full_iri};
///
/// assert_eq!(full_iri(Taxon::new(0)), "https://uor.foundation/u/U2800");
/// assert_eq!(full_iri(Taxon::new(17)), "https://uor.foundation/u/U2811");
/// ```
#[inline]
#[must_use]
pub fn full_iri(taxon: Taxon) -> String {
    let mut iri = String::with_capacity(BASE_IRI.len() + SUFFIX_LEN);
    iri.push_str(BASE_IRI);
    iri.push_str(iri_suffix_str(taxon));
    iri
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iri_suffix() {
        assert_eq!(&iri_suffix(Taxon::new(0)), b"U2800");
        assert_eq!(&iri_suffix(Taxon::new(1)), b"U2801");
        assert_eq!(&iri_suffix(Taxon::new(17)), b"U2811");
        assert_eq!(&iri_suffix(Taxon::new(96)), b"U2860");
        assert_eq!(&iri_suffix(Taxon::new(255)), b"U28FF");
    }

    #[test]
    fn test_iri_suffix_str() {
        assert_eq!(iri_suffix_str(Taxon::new(0)), "U2800");
        assert_eq!(iri_suffix_str(Taxon::new(255)), "U28FF");
    }

    #[test]
    fn test_parse_iri_suffix_roundtrip() {
        for i in 0..=255u8 {
            let t = Taxon::new(i);
            let suffix = iri_suffix(t);
            let parsed = parse_iri_suffix(&suffix);
            assert_eq!(parsed, Some(t));
        }
    }

    #[test]
    fn test_parse_iri_suffix_lowercase() {
        // Should accept lowercase hex
        assert_eq!(parse_iri_suffix(b"U28ff"), Some(Taxon::new(255)));
        assert_eq!(parse_iri_suffix(b"U2811"), Some(Taxon::new(17)));
    }

    #[test]
    fn test_parse_iri_suffix_invalid() {
        // Wrong prefix
        assert_eq!(parse_iri_suffix(b"X2800"), None);

        // Invalid hex
        assert_eq!(parse_iri_suffix(b"U28GG"), None);

        // Outside Braille range
        assert_eq!(parse_iri_suffix(b"U2700"), None);
        assert_eq!(parse_iri_suffix(b"U2900"), None);
    }

    #[test]
    fn test_full_iri() {
        assert_eq!(full_iri(Taxon::new(0)), "https://uor.foundation/u/U2800");
        assert_eq!(full_iri(Taxon::new(17)), "https://uor.foundation/u/U2811");
    }

    #[test]
    fn test_parse_iri() {
        assert_eq!(
            parse_iri("https://uor.foundation/u/U2800"),
            Some(Taxon::new(0))
        );
        assert_eq!(
            parse_iri("https://uor.foundation/u/U2811"),
            Some(Taxon::new(17))
        );

        // Invalid
        assert_eq!(parse_iri("invalid"), None);
        assert_eq!(parse_iri("https://other.com/U2800"), None);
    }
}
