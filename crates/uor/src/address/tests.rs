//! Tests for content addressing types.

use super::{Address, AddressParseError, Glyph};
use alloc::vec;
use alloc::vec::Vec;

// ============================================================================
// Glyph tests
// ============================================================================

#[test]
fn test_glyph_new() {
    for i in 0..=255u8 {
        let g = Glyph::new(i);
        assert_eq!(g.byte_value(), i);
    }
}

#[test]
fn test_glyph_codepoint() {
    assert_eq!(Glyph::new(0).codepoint(), 0x2800);
    assert_eq!(Glyph::new(1).codepoint(), 0x2801);
    assert_eq!(Glyph::new(17).codepoint(), 0x2811);
    assert_eq!(Glyph::new(255).codepoint(), 0x28FF);
}

#[test]
fn test_glyph_character() {
    assert_eq!(Glyph::new(0).character(), '⠀');
    assert_eq!(Glyph::new(1).character(), '⠁');
    assert_eq!(Glyph::new(17).character(), '⠑');
    assert_eq!(Glyph::new(255).character(), '⣿');
}

#[test]
fn test_glyph_from_char() {
    assert_eq!(Glyph::from_char('⠀'), Some(Glyph::new(0)));
    assert_eq!(Glyph::from_char('⠁'), Some(Glyph::new(1)));
    assert_eq!(Glyph::from_char('⠑'), Some(Glyph::new(17)));
    assert_eq!(Glyph::from_char('⣿'), Some(Glyph::new(255)));

    // Non-Braille characters should return None
    assert_eq!(Glyph::from_char('A'), None);
    assert_eq!(Glyph::from_char('0'), None);
    assert_eq!(Glyph::from_char(' '), None);
}

#[test]
fn test_glyph_from_codepoint() {
    assert_eq!(Glyph::from_codepoint(0x2800), Some(Glyph::new(0)));
    assert_eq!(Glyph::from_codepoint(0x28FF), Some(Glyph::new(255)));

    // Out of range
    assert_eq!(Glyph::from_codepoint(0x2799), None);
    assert_eq!(Glyph::from_codepoint(0x2900), None);
    assert_eq!(Glyph::from_codepoint(0x41), None); // 'A'
}

#[test]
fn test_glyph_roundtrip() {
    for i in 0..=255u8 {
        let g = Glyph::new(i);
        let c = g.character();
        let parsed = Glyph::from_char(c).unwrap();
        assert_eq!(g, parsed);
    }
}

#[test]
fn test_glyph_taxon_conversion() {
    use crate::Taxon;

    let g = Glyph::new(17);
    let t = g.taxon();
    assert_eq!(t, Taxon::new(17));

    let g2: Glyph = t.into();
    assert_eq!(g, g2);
}

#[test]
fn test_glyph_dot_count() {
    assert_eq!(Glyph::new(0).dot_count(), 0);
    assert_eq!(Glyph::new(1).dot_count(), 1);
    assert_eq!(Glyph::new(3).dot_count(), 2);
    assert_eq!(Glyph::new(7).dot_count(), 3);
    assert_eq!(Glyph::new(255).dot_count(), 8);
}

#[test]
fn test_glyph_iri() {
    assert_eq!(Glyph::new(0).iri(), "https://uor.foundation/u/glyph/00");
    assert_eq!(Glyph::new(17).iri(), "https://uor.foundation/u/glyph/11");
    assert_eq!(Glyph::new(255).iri(), "https://uor.foundation/u/glyph/ff");
}

#[test]
fn test_glyph_display() {
    assert_eq!(Glyph::new(17).to_string(), "⠑");
}

#[test]
fn test_glyph_constants() {
    assert_eq!(Glyph::BLANK.byte_value(), 0);
    assert_eq!(Glyph::FULL.byte_value(), 255);
}

// ============================================================================
// Address tests
// ============================================================================

#[test]
fn test_address_new() {
    let addr = Address::new();
    assert!(addr.is_empty());
    assert_eq!(addr.len(), 0);
}

#[test]
fn test_address_from_bytes() {
    let addr = Address::from_bytes(&[0, 1, 17, 255]);
    assert_eq!(addr.len(), 4);
    assert_eq!(addr.to_bytes(), vec![0, 1, 17, 255]);
}

#[test]
fn test_address_from_vec() {
    let addr = Address::from_vec(vec![1, 2, 3]);
    assert_eq!(addr.len(), 3);
    assert_eq!(addr.to_bytes(), vec![1, 2, 3]);
}

#[test]
fn test_address_from_braille() {
    let addr = Address::from_braille("⠀⠁⠑⣿").unwrap();
    assert_eq!(addr.to_bytes(), vec![0, 1, 17, 255]);
}

#[test]
fn test_address_from_braille_invalid() {
    let result = Address::from_braille("⠀A⠁");
    assert!(result.is_err());

    if let Err(AddressParseError::InvalidCharacter {
        character,
        position,
    }) = result
    {
        assert_eq!(character, 'A');
        assert_eq!(position, 1);
    }
}

#[test]
fn test_address_from_hex() {
    let addr = Address::from_hex("00011fff").unwrap();
    assert_eq!(addr.to_bytes(), vec![0, 1, 31, 255]);
}

#[test]
fn test_address_to_braille() {
    let addr = Address::from_bytes(&[0, 1, 17, 255]);
    assert_eq!(addr.to_braille(), "⠀⠁⠑⣿");
}

#[test]
fn test_address_to_hex() {
    let addr = Address::from_bytes(&[0, 1, 17, 255]);
    assert_eq!(addr.to_hex(), "000111ff");
}

#[test]
fn test_address_roundtrip_bytes() {
    let original = vec![0u8, 1, 17, 96, 127, 255];
    let addr = Address::from_bytes(&original);
    assert_eq!(addr.to_bytes(), original);
}

#[test]
fn test_address_roundtrip_braille() {
    let braille = "⠀⠁⠑⣿";
    let addr = Address::from_braille(braille).unwrap();
    assert_eq!(addr.to_braille(), braille);
}

#[test]
fn test_address_roundtrip_hex() {
    let hex = "00011fff";
    let addr = Address::from_hex(hex).unwrap();
    assert_eq!(addr.to_hex(), hex);
}

#[test]
fn test_address_glyph() {
    let addr = Address::from_bytes(&[0, 17, 255]);

    assert_eq!(addr.glyph(0).unwrap().byte_value(), 0);
    assert_eq!(addr.glyph(1).unwrap().byte_value(), 17);
    assert_eq!(addr.glyph(2).unwrap().byte_value(), 255);
    assert!(addr.glyph(3).is_none());
}

#[test]
fn test_address_glyphs_iter() {
    let addr = Address::from_bytes(&[1, 2, 3]);
    let values: Vec<u8> = addr.glyphs().map(|g| g.byte_value()).collect();
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn test_address_push() {
    let mut addr = Address::from_bytes(&[1, 2]);
    addr.push(3);
    assert_eq!(addr.to_bytes(), vec![1, 2, 3]);
}

#[test]
fn test_address_concat() {
    let a = Address::from_bytes(&[1, 2]);
    let b = Address::from_bytes(&[3, 4]);
    let c = a.concat(&b);
    assert_eq!(c.to_bytes(), vec![1, 2, 3, 4]);
}

#[test]
fn test_address_slice() {
    let addr = Address::from_bytes(&[0, 1, 2, 3, 4]);

    assert_eq!(addr.slice(1, 4).to_bytes(), vec![1, 2, 3]);
    assert_eq!(addr.slice(0, 2).to_bytes(), vec![0, 1]);
    assert_eq!(addr.slice(3, 10).to_bytes(), vec![3, 4]); // Clamps to end
    assert_eq!(addr.slice(10, 20).to_bytes(), Vec::<u8>::new()); // Empty
}

#[test]
fn test_address_iri() {
    let addr = Address::from_bytes(&[1, 2, 3]);
    assert_eq!(addr.iri(), "https://uor.foundation/u/010203");
}

#[test]
fn test_address_display() {
    let addr = Address::from_bytes(&[0, 17, 255]);
    assert_eq!(addr.to_string(), "⠀⠑⣿");
}

#[test]
fn test_address_index() {
    let addr = Address::from_bytes(&[1, 2, 3]);
    assert_eq!(addr[0].byte_value(), 1);
    assert_eq!(addr[1].byte_value(), 2);
    assert_eq!(addr[2].byte_value(), 3);
}

#[test]
fn test_address_from_iter_bytes() {
    let addr: Address = vec![1u8, 2, 3].into_iter().collect();
    assert_eq!(addr.to_bytes(), vec![1, 2, 3]);
}

#[test]
fn test_address_from_iter_glyphs() {
    let glyphs = vec![Glyph::new(1), Glyph::new(2), Glyph::new(3)];
    let addr: Address = glyphs.into_iter().collect();
    assert_eq!(addr.to_bytes(), vec![1, 2, 3]);
}

#[test]
fn test_address_equality() {
    let a = Address::from_bytes(&[1, 2, 3]);
    let b = Address::from_braille("⠁⠂⠃").unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_address_empty() {
    let addr = Address::from_bytes(&[]);
    assert!(addr.is_empty());
    assert_eq!(addr.len(), 0);
    assert_eq!(addr.to_braille(), "");
    assert_eq!(addr.to_hex(), "");
}

// ============================================================================
// Full bijection verification
// ============================================================================

#[test]
fn test_full_bijection() {
    // Verify all 256 values round-trip correctly
    for i in 0..=255u8 {
        let addr = Address::from_bytes(&[i]);
        let braille = addr.to_braille();
        let parsed = Address::from_braille(&braille).unwrap();
        assert_eq!(parsed.to_bytes(), vec![i], "Failed for value {}", i);
    }
}

#[test]
fn test_bijection_preserves_order() {
    // Verify lexicographic ordering is preserved
    let a = Address::from_bytes(&[0, 0, 1]);
    let b = Address::from_bytes(&[0, 0, 2]);

    assert!(a.to_braille() < b.to_braille());
}

// ============================================================================
// Codec integration tests
// ============================================================================

#[test]
fn test_from_utf8_braille() {
    // UTF-8 encoding of "⠀⠁⠑⣿" (bytes 0, 1, 17, 255)
    let utf8_braille = "⠀⠁⠑⣿".as_bytes();
    let addr = Address::from_utf8_braille(utf8_braille).unwrap();
    assert_eq!(addr.to_bytes(), vec![0, 1, 17, 255]);
}

#[test]
fn test_from_utf8_braille_invalid_utf8() {
    // Invalid UTF-8 sequence
    let invalid = &[0xFF, 0xFE];
    let result = Address::from_utf8_braille(invalid);
    assert!(matches!(result, Err(AddressParseError::InvalidUtf8 { .. })));
}

#[test]
fn test_from_utf8_braille_invalid_character() {
    // Valid UTF-8 but not Braille
    let ascii = "ABC".as_bytes();
    let result = Address::from_utf8_braille(ascii);
    assert!(matches!(
        result,
        Err(AddressParseError::InvalidCharacter { .. })
    ));
}

#[test]
fn test_to_utf8_braille() {
    let addr = Address::from_bytes(&[0, 1, 17, 255]);
    let utf8 = addr.to_utf8_braille();
    assert_eq!(utf8, "⠀⠁⠑⣿".as_bytes());
}

#[test]
fn test_utf8_braille_roundtrip() {
    for i in 0..=255u8 {
        let addr = Address::from_bytes(&[i]);
        let utf8 = addr.to_utf8_braille();
        let parsed = Address::from_utf8_braille(&utf8).unwrap();
        assert_eq!(parsed, addr, "Failed for value {}", i);
    }
}

#[test]
fn test_quantum_level() {
    assert_eq!(Address::from_bytes(&[]).quantum_level(), None);
    assert_eq!(Address::from_bytes(&[0]).quantum_level(), Some(0));
    assert_eq!(Address::from_bytes(&[0, 1]).quantum_level(), Some(1));
    assert_eq!(Address::from_bytes(&[0, 1, 2]).quantum_level(), Some(2));
    assert_eq!(Address::from_bytes(&[0, 1, 2, 3]).quantum_level(), Some(3));
    assert_eq!(Address::from_bytes(&[0, 1, 2, 3, 4]).quantum_level(), None);
}

#[test]
fn test_address_parse_error_display() {
    let err = AddressParseError::InvalidCharacter {
        character: 'A',
        position: 5,
    };
    assert_eq!(
        err.to_string(),
        "invalid character 'A' (U+0041) at position 5"
    );

    let err = AddressParseError::InvalidUtf8 { position: 10 };
    assert_eq!(
        err.to_string(),
        "invalid UTF-8 encoding at byte position 10"
    );
}
