//! Multi-taxon words - data containers with NO operations.
//!
//! In the UOR Channel Model, `Word` types are **data containers only**.
//! All operations exist on `Channel<T, N>`, NOT on individual words.
//!
//! # Type Aliases
//!
//! - `Word4` - 32-bit word (transparent wrapper around `u32`)
//! - `Word8` - 64-bit word (transparent wrapper around `u64`)
//! - `Word2` - 16-bit word (transparent wrapper around `u16`)
//! - `Word32` - 256-bit word (32 taxons, for SHA-256 hash)
//!
//! # Example
//!
//! ```ignore
//! use uor::word::Word4;
//! use uor::channel::Word4x8;
//!
//! // Word4 is a data container - no operations
//! let a = Word4::new(0x12345678);
//! let b = Word4::new(0xDEADBEEF);
//!
//! // To perform operations, use Channels (SIMD)
//! let ch_a = Word4x8::broadcast(a);
//! let ch_b = Word4x8::broadcast(b);
//! let result = unsafe { ch_a.xor(ch_b) };
//! ```
//!
//! # Design Philosophy
//!
//! This is the **strictly enforced UOR Channel Model**:
//! - Individual elements (Word4, Taxon) have NO operations
//! - Operations exist ONLY on `Channel<T, N>`
//! - This forces optimal batching and SIMD utilization

use crate::Taxon;
use core::fmt;

// ============================================================================
// Word4 - 32-bit word (native u32)
// ============================================================================

/// A 32-bit word - data container with NO operations.
///
/// Operations exist only on `Channel<Word4, 8>` (AVX2) or `Channel<Word4, 4>` (NEON).
///
/// # Example
///
/// ```
/// use uor::word::Word4;
///
/// let w = Word4::new(0x12345678);
/// assert_eq!(w.value(), 0x12345678);
/// assert_eq!(w.taxon(0).value(), 0x12);  // Most significant byte
/// assert_eq!(w.taxon(3).value(), 0x78);  // Least significant byte
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Word4(u32);

impl Word4 {
    /// The zero word.
    pub const ZERO: Self = Self(0);

    /// The maximum word (all bits set).
    pub const MAX: Self = Self(u32::MAX);

    /// Create a new Word4 from a u32 value.
    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Get the underlying u32 value.
    #[inline]
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Get taxon at index (0 = most significant byte, 3 = least significant).
    #[inline]
    pub const fn taxon(&self, index: usize) -> Taxon {
        debug_assert!(index < 4, "index out of bounds");
        let shift = (3 - index) * 8;
        Taxon::new(((self.0 >> shift) & 0xFF) as u8)
    }

    /// Create from byte array (big-endian).
    #[inline]
    pub const fn from_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(bytes))
    }

    /// Convert to byte array (big-endian).
    #[inline]
    pub const fn to_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    /// Check if word is zero.
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl From<u32> for Word4 {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl From<Word4> for u32 {
    fn from(w: Word4) -> Self {
        w.value()
    }
}

impl fmt::Debug for Word4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Word4({:#010x})", self.0)
    }
}

impl fmt::Display for Word4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..4 {
            write!(f, "{}", self.taxon(i).braille())?;
        }
        Ok(())
    }
}

// ============================================================================
// Word8 - 64-bit word (native u64)
// ============================================================================

/// A 64-bit word - data container with NO operations.
///
/// Operations exist only on `Channel<Word8, 4>` (AVX2) or `Channel<Word8, 2>` (NEON).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Word8(u64);

impl Word8 {
    /// The zero word.
    pub const ZERO: Self = Self(0);

    /// The maximum word (all bits set).
    pub const MAX: Self = Self(u64::MAX);

    /// Create a new Word8 from a u64 value.
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Get the underlying u64 value.
    #[inline]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Get taxon at index (0 = most significant byte, 7 = least significant).
    #[inline]
    pub const fn taxon(&self, index: usize) -> Taxon {
        debug_assert!(index < 8, "index out of bounds");
        let shift = (7 - index) * 8;
        Taxon::new(((self.0 >> shift) & 0xFF) as u8)
    }

    /// Create from byte array (big-endian).
    #[inline]
    pub const fn from_bytes(bytes: [u8; 8]) -> Self {
        Self(u64::from_be_bytes(bytes))
    }

    /// Convert to byte array (big-endian).
    #[inline]
    pub const fn to_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    /// Check if word is zero.
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl From<u64> for Word8 {
    fn from(v: u64) -> Self {
        Self::new(v)
    }
}

impl From<Word8> for u64 {
    fn from(w: Word8) -> Self {
        w.value()
    }
}

impl fmt::Debug for Word8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Word8({:#018x})", self.0)
    }
}

impl fmt::Display for Word8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..8 {
            write!(f, "{}", self.taxon(i).braille())?;
        }
        Ok(())
    }
}

// ============================================================================
// Word2 - 16-bit word (native u16)
// ============================================================================

/// A 16-bit word - data container with NO operations.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Word2(u16);

impl Word2 {
    /// The zero word.
    pub const ZERO: Self = Self(0);

    /// The maximum word (all bits set).
    pub const MAX: Self = Self(u16::MAX);

    /// Create a new Word2 from a u16 value.
    #[inline]
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    /// Get the underlying u16 value.
    #[inline]
    pub const fn value(self) -> u16 {
        self.0
    }

    /// Get taxon at index (0 = most significant byte, 1 = least significant).
    #[inline]
    pub const fn taxon(&self, index: usize) -> Taxon {
        debug_assert!(index < 2, "index out of bounds");
        let shift = (1 - index) * 8;
        Taxon::new(((self.0 >> shift) & 0xFF) as u8)
    }

    /// Create from byte array (big-endian).
    #[inline]
    pub const fn from_bytes(bytes: [u8; 2]) -> Self {
        Self(u16::from_be_bytes(bytes))
    }

    /// Convert to byte array (big-endian).
    #[inline]
    pub const fn to_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
    }

    /// Check if word is zero.
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl From<u16> for Word2 {
    fn from(v: u16) -> Self {
        Self::new(v)
    }
}

impl From<Word2> for u16 {
    fn from(w: Word2) -> Self {
        w.value()
    }
}

impl fmt::Debug for Word2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Word2({:#06x})", self.0)
    }
}

// ============================================================================
// Word<N> - Generic N-byte word (for Word32, etc.)
// ============================================================================

/// A generic N-byte word - data container with NO operations.
///
/// Used for larger words like Word32 (SHA-256 hash).
/// Operations exist only on `Channel<Word<N>, M>`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Word<const N: usize>([Taxon; N]);

/// 256-bit word (32 taxons) - SHA-256 hash size
pub type Word32 = Word<32>;

impl<const N: usize> Word<N> {
    /// The zero word (all taxons are 0).
    pub const ZERO: Self = Self([Taxon::MIN; N]);

    /// The maximum word (all taxons are 255).
    pub const MAX: Self = Self([Taxon::MAX; N]);

    /// Create from taxon array (big-endian: index 0 is most significant).
    #[inline]
    pub const fn new(taxons: [Taxon; N]) -> Self {
        Self(taxons)
    }

    /// Create from byte array (big-endian).
    #[inline]
    pub const fn from_bytes(bytes: [u8; N]) -> Self {
        let mut taxons = [Taxon::MIN; N];
        let mut i = 0;
        while i < N {
            taxons[i] = Taxon::new(bytes[i]);
            i += 1;
        }
        Self(taxons)
    }

    /// Get taxon at index (0 = most significant).
    #[inline]
    pub const fn taxon(&self, index: usize) -> Taxon {
        self.0[index]
    }

    /// Get mutable reference to taxon at index.
    #[inline]
    pub fn taxon_mut(&mut self, index: usize) -> &mut Taxon {
        &mut self.0[index]
    }

    /// Set taxon at index.
    #[inline]
    pub fn set_taxon(&mut self, index: usize, value: Taxon) {
        self.0[index] = value;
    }

    /// Convert to byte array (big-endian).
    #[inline]
    pub const fn to_bytes(self) -> [u8; N] {
        let mut bytes = [0u8; N];
        let mut i = 0;
        while i < N {
            bytes[i] = self.0[i].value();
            i += 1;
        }
        bytes
    }

    /// Get the underlying taxon array.
    #[inline]
    pub const fn taxons(&self) -> &[Taxon; N] {
        &self.0
    }

    /// Check if word is zero.
    #[inline]
    pub const fn is_zero(self) -> bool {
        let mut i = 0;
        while i < N {
            if self.0[i].value() != 0 {
                return false;
            }
            i += 1;
        }
        true
    }
}

impl<const N: usize> Default for Word<N> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const N: usize> fmt::Debug for Word<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Word<{}>(", N)?;
        for (i, t) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{:02x}", t.value())?;
        }
        write!(f, ")")
    }
}

impl<const N: usize> fmt::Display for Word<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for t in &self.0 {
            write!(f, "{}", t.braille())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word4_new() {
        let w = Word4::new(0x12345678);
        assert_eq!(w.value(), 0x12345678);
    }

    #[test]
    fn test_word4_taxon() {
        let w = Word4::new(0x12345678);
        assert_eq!(w.taxon(0).value(), 0x12); // Most significant
        assert_eq!(w.taxon(1).value(), 0x34);
        assert_eq!(w.taxon(2).value(), 0x56);
        assert_eq!(w.taxon(3).value(), 0x78); // Least significant
    }

    #[test]
    fn test_word4_from_bytes() {
        let w = Word4::from_bytes([0x12, 0x34, 0x56, 0x78]);
        assert_eq!(w.value(), 0x12345678);
    }

    #[test]
    fn test_word4_to_bytes() {
        let w = Word4::new(0xDEADBEEF);
        assert_eq!(w.to_bytes(), [0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn test_word4_conversion() {
        let v: u32 = 0xCAFEBABE;
        let w = Word4::from(v);
        let v2: u32 = w.into();
        assert_eq!(v, v2);
    }

    #[test]
    fn test_word8_taxon() {
        let w = Word8::new(0x123456789ABCDEF0);
        assert_eq!(w.taxon(0).value(), 0x12); // Most significant
        assert_eq!(w.taxon(7).value(), 0xF0); // Least significant
    }

    #[test]
    fn test_word32_taxon() {
        let mut bytes = [0u8; 32];
        bytes[0] = 0xBA;
        bytes[31] = 0xAD;
        let w = Word32::from_bytes(bytes);
        assert_eq!(w.taxon(0).value(), 0xBA);
        assert_eq!(w.taxon(31).value(), 0xAD);
    }

    #[test]
    fn test_word4_is_zero() {
        assert!(Word4::ZERO.is_zero());
        assert!(!Word4::new(1).is_zero());
    }
}
