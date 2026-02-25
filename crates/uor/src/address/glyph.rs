//! Single Braille glyph representing a byte value.

use crate::constants::{BRAILLE_BASE, BRAILLE_MAX};
use crate::Taxon;
use core::fmt;

/// A single Braille character representing a byte (0-255).
///
/// Glyph is semantically equivalent to Taxon but provides a content-addressing
/// focused API. Each glyph corresponds to a Unicode codepoint in the Braille
/// Patterns block (U+2800-U+28FF).
///
/// # Ontology
///
/// Corresponds to `https://uor.foundation/u/Glyph` with properties:
/// - `codepoint`: The Unicode codepoint (integer)
/// - `byteValue`: The byte value encoded (0-255)
///
/// # Visual Encoding
///
/// Each Braille dot corresponds to a bit position:
/// ```text
/// Dot Pattern:    Bit Mapping:
/// ┌───┬───┐       ┌───┬───┐
/// │ 1 │ 4 │       │ 0 │ 3 │
/// │ 2 │ 5 │       │ 1 │ 4 │
/// │ 3 │ 6 │       │ 2 │ 5 │
/// │ 7 │ 8 │       │ 6 │ 7 │
/// └───┴───┘       └───┴───┘
/// ```
///
/// # Example
///
/// ```
/// use uor::address::Glyph;
///
/// let g = Glyph::new(17);
/// assert_eq!(g.byte_value(), 17);
/// assert_eq!(g.codepoint(), 0x2811);
/// assert_eq!(g.character(), '⠑');
/// assert_eq!(g.to_string(), "⠑");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Glyph(Taxon);

impl Glyph {
    /// The blank glyph (value 0, U+2800).
    pub const BLANK: Self = Self(Taxon::MIN);

    /// The full glyph (value 255, U+28FF, all dots).
    pub const FULL: Self = Self(Taxon::MAX);

    /// Creates a new glyph from a byte value.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Glyph;
    ///
    /// let g = Glyph::new(42);
    /// assert_eq!(g.byte_value(), 42);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(value: u8) -> Self {
        Self(Taxon::new(value))
    }

    /// Creates a glyph from a Unicode character.
    ///
    /// Returns `None` if the character is not in the Braille Patterns block.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Glyph;
    ///
    /// let g = Glyph::from_char('⠑').unwrap();
    /// assert_eq!(g.byte_value(), 17);
    ///
    /// assert!(Glyph::from_char('A').is_none());
    /// ```
    #[must_use]
    pub const fn from_char(c: char) -> Option<Self> {
        let cp = c as u32;
        if cp >= BRAILLE_BASE && cp <= BRAILLE_MAX {
            Some(Self(Taxon::new((cp - BRAILLE_BASE) as u8)))
        } else {
            None
        }
    }

    /// Creates a glyph from a Unicode codepoint.
    ///
    /// Returns `None` if the codepoint is not in the Braille Patterns block.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Glyph;
    ///
    /// let g = Glyph::from_codepoint(0x2811).unwrap();
    /// assert_eq!(g.byte_value(), 17);
    ///
    /// assert!(Glyph::from_codepoint(0x41).is_none()); // 'A'
    /// ```
    #[must_use]
    pub const fn from_codepoint(cp: u32) -> Option<Self> {
        if cp >= BRAILLE_BASE && cp <= BRAILLE_MAX {
            Some(Self(Taxon::new((cp - BRAILLE_BASE) as u8)))
        } else {
            None
        }
    }

    /// Returns the byte value (0-255) encoded by this glyph.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Glyph;
    ///
    /// assert_eq!(Glyph::new(17).byte_value(), 17);
    /// ```
    #[inline]
    #[must_use]
    pub const fn byte_value(self) -> u8 {
        self.0.value()
    }

    /// Returns the Unicode codepoint (U+2800-U+28FF).
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Glyph;
    ///
    /// assert_eq!(Glyph::new(0).codepoint(), 0x2800);
    /// assert_eq!(Glyph::new(255).codepoint(), 0x28FF);
    /// ```
    #[inline]
    #[must_use]
    pub const fn codepoint(self) -> u32 {
        self.0.codepoint()
    }

    /// Returns the Braille character.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Glyph;
    ///
    /// assert_eq!(Glyph::new(0).character(), '⠀');
    /// assert_eq!(Glyph::new(17).character(), '⠑');
    /// assert_eq!(Glyph::new(255).character(), '⣿');
    /// ```
    #[inline]
    #[must_use]
    pub const fn character(self) -> char {
        self.0.braille()
    }

    /// Returns the underlying Taxon.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Glyph;
    /// use uor::Taxon;
    ///
    /// let g = Glyph::new(17);
    /// assert_eq!(g.taxon(), Taxon::new(17));
    /// ```
    #[inline]
    #[must_use]
    pub const fn taxon(self) -> Taxon {
        self.0
    }

    /// Returns the IRI for this glyph in the UOR ontology.
    ///
    /// Format: `https://uor.foundation/u/glyph/{hex}`
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Glyph;
    ///
    /// assert_eq!(Glyph::new(0).iri(), "https://uor.foundation/u/glyph/00");
    /// assert_eq!(Glyph::new(17).iri(), "https://uor.foundation/u/glyph/11");
    /// assert_eq!(Glyph::new(255).iri(), "https://uor.foundation/u/glyph/ff");
    /// ```
    #[must_use]
    pub fn iri(self) -> alloc::string::String {
        alloc::format!("https://uor.foundation/u/glyph/{:02x}", self.byte_value())
    }

    /// Returns the number of raised dots (Hamming weight).
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Glyph;
    ///
    /// assert_eq!(Glyph::new(0).dot_count(), 0);   // ⠀ - no dots
    /// assert_eq!(Glyph::new(1).dot_count(), 1);   // ⠁ - one dot
    /// assert_eq!(Glyph::new(255).dot_count(), 8); // ⣿ - all 8 dots
    /// ```
    #[inline]
    #[must_use]
    pub const fn dot_count(self) -> u8 {
        self.0.weight()
    }
}

impl From<u8> for Glyph {
    #[inline]
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<Glyph> for u8 {
    #[inline]
    fn from(glyph: Glyph) -> Self {
        glyph.byte_value()
    }
}

impl From<Taxon> for Glyph {
    #[inline]
    fn from(taxon: Taxon) -> Self {
        Self(taxon)
    }
}

impl From<Glyph> for Taxon {
    #[inline]
    fn from(glyph: Glyph) -> Self {
        glyph.0
    }
}

impl From<Glyph> for char {
    #[inline]
    fn from(glyph: Glyph) -> Self {
        glyph.character()
    }
}

impl fmt::Debug for Glyph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Glyph({}, U+{:04X}, {})",
            self.byte_value(),
            self.codepoint(),
            self.character()
        )
    }
}

impl fmt::Display for Glyph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.character())
    }
}
