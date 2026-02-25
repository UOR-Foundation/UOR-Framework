//! Content-addressable identifier as a Braille glyph string.

use super::Glyph;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

/// A content-addressable identifier represented as a Braille string.
///
/// An Address is a sequence of Glyphs, where each glyph encodes one byte.
/// This provides a bijective mapping from byte sequences to Unicode strings.
///
/// # Ontology
///
/// Corresponds to `https://uor.foundation/u/Address` with properties:
/// - `glyph`: The Braille string representation
/// - `length`: Number of glyphs/bytes
///
/// # Content Addressing
///
/// Addresses enable content-addressable identifiers:
/// - Byte sequence `[0, 1, 17, 255]` → Address `"⠀⠁⠑⣿"`
/// - The Braille string IS the canonical identifier
/// - Bijective: every byte sequence has exactly one address representation
///
/// # Example
///
/// ```
/// use uor::address::Address;
///
/// // Create from bytes
/// let addr = Address::from_bytes(&[0, 1, 17, 255]);
/// assert_eq!(addr.to_string(), "⠀⠁⠑⣿");
///
/// // Parse from Braille string
/// let parsed = Address::from_braille("⠀⠁⠑⣿").unwrap();
/// assert_eq!(parsed.to_bytes(), vec![0, 1, 17, 255]);
///
/// // Round-trip
/// assert_eq!(addr, parsed);
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct Address {
    glyphs: Vec<Glyph>,
}

/// Error when parsing an Address from a string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressParseError {
    /// Character is not a valid Braille glyph.
    InvalidCharacter {
        /// The invalid character.
        character: char,
        /// Position in the string.
        position: usize,
    },
    /// Invalid UTF-8 encoding (for codec integration).
    InvalidUtf8 {
        /// Byte position where the error occurred.
        position: usize,
    },
}

impl fmt::Display for AddressParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharacter {
                character,
                position,
            } => {
                write!(
                    f,
                    "invalid character '{}' (U+{:04X}) at position {}",
                    character, *character as u32, position
                )
            }
            Self::InvalidUtf8 { position } => {
                write!(f, "invalid UTF-8 encoding at byte position {}", position)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AddressParseError {}

impl Address {
    /// The empty address.
    pub const EMPTY: Self = Self { glyphs: Vec::new() };

    /// Creates a new empty address.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let addr = Address::new();
    /// assert!(addr.is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self { glyphs: Vec::new() }
    }

    /// Creates an address from a byte slice.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let addr = Address::from_bytes(&[1, 2, 3]);
    /// assert_eq!(addr.len(), 3);
    /// assert_eq!(addr.to_bytes(), vec![1, 2, 3]);
    /// ```
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            glyphs: bytes.iter().map(|&b| Glyph::new(b)).collect(),
        }
    }

    /// Creates an address from a vector of bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let addr = Address::from_vec(vec![1, 2, 3]);
    /// assert_eq!(addr.len(), 3);
    /// ```
    #[must_use]
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self {
            glyphs: bytes.into_iter().map(Glyph::new).collect(),
        }
    }

    /// Creates an address from a Braille string.
    ///
    /// Returns an error if any character is not in the Braille Patterns block.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let addr = Address::from_braille("⠀⠁⠑⣿").unwrap();
    /// assert_eq!(addr.to_bytes(), vec![0, 1, 17, 255]);
    ///
    /// // Non-Braille characters are rejected
    /// assert!(Address::from_braille("⠀A⠁").is_err());
    /// ```
    pub fn from_braille(s: &str) -> Result<Self, AddressParseError> {
        let mut glyphs = Vec::with_capacity(s.len());
        for (pos, c) in s.chars().enumerate() {
            match Glyph::from_char(c) {
                Some(g) => glyphs.push(g),
                None => {
                    return Err(AddressParseError::InvalidCharacter {
                        character: c,
                        position: pos,
                    })
                }
            }
        }
        Ok(Self { glyphs })
    }

    /// Creates an address from a hex string.
    ///
    /// Each pair of hex digits becomes one glyph.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let addr = Address::from_hex("00011fff").unwrap();
    /// assert_eq!(addr.to_bytes(), vec![0, 1, 31, 255]);
    /// ```
    pub fn from_hex(s: &str) -> Result<Self, AddressParseError> {
        let bytes: Result<Vec<u8>, _> = (0..s.len())
            .step_by(2)
            .enumerate()
            .map(|(i, pos)| {
                let end = (pos + 2).min(s.len());
                u8::from_str_radix(&s[pos..end], 16).map_err(|_| {
                    AddressParseError::InvalidCharacter {
                        character: s.chars().nth(pos).unwrap_or('?'),
                        position: i,
                    }
                })
            })
            .collect();
        Ok(Self::from_vec(bytes?))
    }

    /// Returns the glyphs as a vector of bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let addr = Address::from_bytes(&[1, 2, 3]);
    /// assert_eq!(addr.to_bytes(), vec![1, 2, 3]);
    /// ```
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.glyphs.iter().map(|g| g.byte_value()).collect()
    }

    /// Returns the Braille string representation.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let addr = Address::from_bytes(&[0, 1, 17, 255]);
    /// assert_eq!(addr.to_braille(), "⠀⠁⠑⣿");
    /// ```
    #[must_use]
    pub fn to_braille(&self) -> String {
        self.glyphs.iter().map(|g| g.character()).collect()
    }

    /// Returns the hex string representation.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let addr = Address::from_bytes(&[0, 1, 17, 255]);
    /// assert_eq!(addr.to_hex(), "000111ff");
    /// ```
    #[must_use]
    pub fn to_hex(&self) -> String {
        self.glyphs
            .iter()
            .map(|g| alloc::format!("{:02x}", g.byte_value()))
            .collect()
    }

    /// Returns the number of glyphs (bytes) in this address.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// assert_eq!(Address::new().len(), 0);
    /// assert_eq!(Address::from_bytes(&[1, 2, 3]).len(), 3);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    /// Returns true if this address is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// assert!(Address::new().is_empty());
    /// assert!(!Address::from_bytes(&[1]).is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }

    /// Returns the glyph at the given index.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let addr = Address::from_bytes(&[0, 17, 255]);
    /// assert_eq!(addr.glyph(1).unwrap().byte_value(), 17);
    /// assert!(addr.glyph(10).is_none());
    /// ```
    #[must_use]
    pub fn glyph(&self, index: usize) -> Option<Glyph> {
        self.glyphs.get(index).copied()
    }

    /// Returns an iterator over the glyphs.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let addr = Address::from_bytes(&[1, 2, 3]);
    /// let values: Vec<u8> = addr.glyphs().map(|g| g.byte_value()).collect();
    /// assert_eq!(values, vec![1, 2, 3]);
    /// ```
    pub fn glyphs(&self) -> impl Iterator<Item = Glyph> + '_ {
        self.glyphs.iter().copied()
    }

    /// Returns the IRI for this address in the UOR ontology.
    ///
    /// Format: `https://uor.foundation/u/{hex}`
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let addr = Address::from_bytes(&[1, 2, 3]);
    /// assert_eq!(addr.iri(), "https://uor.foundation/u/010203");
    /// ```
    #[must_use]
    pub fn iri(&self) -> String {
        alloc::format!("https://uor.foundation/u/{}", self.to_hex())
    }

    /// Appends a byte to this address.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let mut addr = Address::from_bytes(&[1, 2]);
    /// addr.push(3);
    /// assert_eq!(addr.to_bytes(), vec![1, 2, 3]);
    /// ```
    pub fn push(&mut self, byte: u8) {
        self.glyphs.push(Glyph::new(byte));
    }

    /// Appends a glyph to this address.
    pub fn push_glyph(&mut self, glyph: Glyph) {
        self.glyphs.push(glyph);
    }

    /// Concatenates two addresses.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let a = Address::from_bytes(&[1, 2]);
    /// let b = Address::from_bytes(&[3, 4]);
    /// let c = a.concat(&b);
    /// assert_eq!(c.to_bytes(), vec![1, 2, 3, 4]);
    /// ```
    #[must_use]
    pub fn concat(&self, other: &Self) -> Self {
        let mut glyphs = self.glyphs.clone();
        glyphs.extend(other.glyphs.iter().copied());
        Self { glyphs }
    }

    /// Returns a slice of this address.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let addr = Address::from_bytes(&[0, 1, 2, 3, 4]);
    /// let slice = addr.slice(1, 4);
    /// assert_eq!(slice.to_bytes(), vec![1, 2, 3]);
    /// ```
    #[must_use]
    pub fn slice(&self, start: usize, end: usize) -> Self {
        let end = end.min(self.glyphs.len());
        let start = start.min(end);
        Self {
            glyphs: self.glyphs[start..end].to_vec(),
        }
    }

    // ========================================================================
    // Codec Integration Methods
    // ========================================================================

    /// Creates an address from UTF-8 encoded Braille bytes.
    ///
    /// This is the format produced by Braille codecs. Each Braille character
    /// is encoded as 3 UTF-8 bytes (codepoints U+2800-U+28FF).
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// // UTF-8 encoding of "⠀⠁" (bytes 0, 1)
    /// let utf8_braille = "⠀⠁".as_bytes();
    /// let addr = Address::from_utf8_braille(utf8_braille).unwrap();
    /// assert_eq!(addr.to_bytes(), vec![0, 1]);
    /// ```
    pub fn from_utf8_braille(bytes: &[u8]) -> Result<Self, AddressParseError> {
        let s = core::str::from_utf8(bytes).map_err(|e| AddressParseError::InvalidUtf8 {
            position: e.valid_up_to(),
        })?;
        Self::from_braille(s)
    }

    /// Returns the address as UTF-8 encoded Braille bytes.
    ///
    /// This format is compatible with Braille codecs. Each glyph becomes
    /// a 3-byte UTF-8 sequence.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// let addr = Address::from_bytes(&[0, 1]);
    /// let utf8 = addr.to_utf8_braille();
    /// assert_eq!(utf8, "⠀⠁".as_bytes());
    /// ```
    #[must_use]
    pub fn to_utf8_braille(&self) -> Vec<u8> {
        self.to_braille().into_bytes()
    }

    /// Returns the quantum level for this address.
    ///
    /// Quantum levels (Q0-Q3) indicate the address bit-width:
    /// - Q0: 1 glyph = 8 bits
    /// - Q1: 2 glyphs = 16 bits
    /// - Q2: 3 glyphs = 24 bits
    /// - Q3: 4 glyphs = 32 bits
    ///
    /// For addresses longer than 4 glyphs, returns None.
    ///
    /// # Example
    ///
    /// ```
    /// use uor::address::Address;
    ///
    /// assert_eq!(Address::from_bytes(&[0]).quantum_level(), Some(0));
    /// assert_eq!(Address::from_bytes(&[0, 1]).quantum_level(), Some(1));
    /// assert_eq!(Address::from_bytes(&[0, 1, 2]).quantum_level(), Some(2));
    /// assert_eq!(Address::from_bytes(&[0, 1, 2, 3]).quantum_level(), Some(3));
    /// assert_eq!(Address::from_bytes(&[0, 1, 2, 3, 4]).quantum_level(), None);
    /// ```
    #[must_use]
    pub fn quantum_level(&self) -> Option<u8> {
        match self.len() {
            1 => Some(0),
            2 => Some(1),
            3 => Some(2),
            4 => Some(3),
            _ => None,
        }
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address(\"{}\")", self.to_braille())
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_braille())
    }
}

impl From<&[u8]> for Address {
    fn from(bytes: &[u8]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<Vec<u8>> for Address {
    fn from(bytes: Vec<u8>) -> Self {
        Self::from_vec(bytes)
    }
}

impl From<Address> for Vec<u8> {
    fn from(addr: Address) -> Self {
        addr.to_bytes()
    }
}

impl FromIterator<u8> for Address {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        Self {
            glyphs: iter.into_iter().map(Glyph::new).collect(),
        }
    }
}

impl FromIterator<Glyph> for Address {
    fn from_iter<I: IntoIterator<Item = Glyph>>(iter: I) -> Self {
        Self {
            glyphs: iter.into_iter().collect(),
        }
    }
}

impl core::ops::Index<usize> for Address {
    type Output = Glyph;

    fn index(&self, index: usize) -> &Self::Output {
        &self.glyphs[index]
    }
}
