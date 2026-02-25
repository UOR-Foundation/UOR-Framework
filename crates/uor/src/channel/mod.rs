//! Channel - The primitive type for SIMD-parallel operations.
//!
//! A `Channel<T, N>` represents N elements of type T processed in parallel.
//! **This is THE primitive type** - individual elements (`Word4`, `Taxon`) have no
//! operations. All operations exist only on Channel types.
//!
//! # Fibration Structure
//!
//! UOR types form a fibration where:
//! - **Base**: The element type (Word4, Taxon)
//! - **Channels**: Parallel lanes within a SIMD register
//! - **Operations**: Lift to all channels simultaneously
//!
//! ```text
//! Channel<Word4, 8>  (AVX2: 8 × 32-bit lanes)
//!        │
//!        ├── lane[0]: Word4
//!        ├── lane[1]: Word4
//!        ├── ...
//!        └── lane[7]: Word4
//! ```
//!
//! # Architecture-Specific Channel Widths
//!
//! | Architecture | Register | Channel Width |
//! |--------------|----------|---------------|
//! | x86_64 AVX2  | __m256i  | 8 × Word4, 4 × Word8, 32 × Taxon |
//! | aarch64 NEON | uint8x16 | 4 × Word4, 2 × Word8, 16 × Taxon |
//!
//! # Example
//!
//! ```ignore
//! use uor::channel::{Word4x8, Channel};
//! use uor::Word4;
//!
//! // Create channels with 8 Word4 values each
//! let a = Word4x8::broadcast(Word4::new(0x12345678));
//! let b = Word4x8::broadcast(Word4::new(0xDEADBEEF));
//!
//! // XOR all 8 lanes in one SIMD instruction
//! let c = unsafe { a.xor(b) };
//!
//! // Extract a single result
//! let result = c.lane(0);
//! ```

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use crate::core::taxon::Taxon;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use crate::core::word::{Word4, Word8};

/// A channel bundle - N elements of type T processed in parallel.
///
/// This is THE primitive type. All operations are on channels, not individual elements.
///
/// # Safety
///
/// Channel operations require SIMD target features and are marked `unsafe`.
/// The caller must ensure the target CPU supports the required SIMD instructions.
#[derive(Clone, Copy)]
#[repr(C, align(32))] // AVX2 alignment
pub struct Channel<T, const N: usize>(pub(crate) [T; N]);

// ============================================================================
// Type Aliases for x86_64 AVX2 (256-bit registers)
// ============================================================================

/// 8 × Word4 = 256 bits (AVX2 register width)
#[cfg(target_arch = "x86_64")]
pub type Word4x8 = Channel<Word4, 8>;

/// 4 × Word8 = 256 bits (AVX2 register width)
#[cfg(target_arch = "x86_64")]
pub type Word8x4 = Channel<Word8, 4>;

/// 32 × Taxon = 256 bits (AVX2 register width)
#[cfg(target_arch = "x86_64")]
pub type Taxonx32 = Channel<Taxon, 32>;

// ============================================================================
// Type Aliases for aarch64 NEON (128-bit registers)
// ============================================================================

/// 4 × Word4 = 128 bits (NEON register width)
#[cfg(target_arch = "aarch64")]
pub type Word4x4 = Channel<Word4, 4>;

/// 2 × Word8 = 128 bits (NEON register width)
#[cfg(target_arch = "aarch64")]
pub type Word8x2 = Channel<Word8, 2>;

/// 16 × Taxon = 128 bits (NEON register width)
#[cfg(target_arch = "aarch64")]
pub type Taxonx16 = Channel<Taxon, 16>;

// ============================================================================
// Generic Channel Implementation
// ============================================================================

impl<T: Copy + Default, const N: usize> Channel<T, N> {
    /// The number of lanes in this channel.
    pub const LANES: usize = N;

    /// Create a channel with all lanes set to the same value.
    #[inline]
    pub fn broadcast(value: T) -> Self {
        Self([value; N])
    }

    /// Create a channel from an array of values.
    #[inline]
    pub const fn from_array(values: [T; N]) -> Self {
        Self(values)
    }

    /// Get the value at a specific lane.
    ///
    /// # Panics
    ///
    /// Panics if `lane >= N`.
    #[inline]
    pub const fn lane(&self, lane: usize) -> T {
        self.0[lane]
    }

    /// Set the value at a specific lane.
    ///
    /// # Panics
    ///
    /// Panics if `lane >= N`.
    #[inline]
    pub fn set_lane(&mut self, lane: usize, value: T) {
        self.0[lane] = value;
    }

    /// Get the underlying array.
    #[inline]
    pub const fn as_array(&self) -> &[T; N] {
        &self.0
    }

    /// Convert to the underlying array.
    #[inline]
    pub const fn into_array(self) -> [T; N] {
        self.0
    }

    /// Create a channel with all lanes set to the default value.
    #[inline]
    pub fn zeroed() -> Self {
        Self([T::default(); N])
    }
}

impl<T: Copy + Default, const N: usize> Default for Channel<T, N> {
    fn default() -> Self {
        Self::zeroed()
    }
}

impl<T: Copy, const N: usize> From<[T; N]> for Channel<T, N> {
    fn from(values: [T; N]) -> Self {
        Self(values)
    }
}

impl<T: Copy, const N: usize> From<Channel<T, N>> for [T; N] {
    fn from(channel: Channel<T, N>) -> Self {
        channel.0
    }
}

// ============================================================================
// Debug Implementation
// ============================================================================

impl<T: core::fmt::Debug + Copy, const N: usize> core::fmt::Debug for Channel<T, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Channel").field(&self.0).finish()
    }
}

// ============================================================================
// PartialEq Implementation
// ============================================================================

impl<T: PartialEq + Copy, const N: usize> PartialEq for Channel<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq + Copy, const N: usize> Eq for Channel<T, N> {}

// ============================================================================
// Compile-Time Architecture Check
// ============================================================================

// Ensure SIMD is available - NO SCALAR FALLBACK
// Note: Skip this check during doctest compilation (which doesn't inherit RUSTFLAGS)
#[cfg(all(target_arch = "x86_64", not(target_feature = "avx2"), not(doctest)))]
compile_error!(
    "UOR Channel Model requires AVX2 on x86_64. \
     Build with RUSTFLAGS=\"-C target-feature=+avx2\""
);

#[cfg(all(target_arch = "aarch64", not(target_feature = "neon"), not(doctest)))]
compile_error!(
    "UOR Channel Model requires NEON on aarch64. \
     This should be enabled by default on aarch64."
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_broadcast() {
        let ch: Channel<u8, 4> = Channel::broadcast(42);
        assert_eq!(ch.lane(0), 42);
        assert_eq!(ch.lane(1), 42);
        assert_eq!(ch.lane(2), 42);
        assert_eq!(ch.lane(3), 42);
    }

    #[test]
    fn test_channel_from_array() {
        let ch = Channel::from_array([1u8, 2, 3, 4]);
        assert_eq!(ch.lane(0), 1);
        assert_eq!(ch.lane(1), 2);
        assert_eq!(ch.lane(2), 3);
        assert_eq!(ch.lane(3), 4);
    }

    #[test]
    fn test_channel_set_lane() {
        let mut ch: Channel<u8, 4> = Channel::broadcast(0);
        ch.set_lane(2, 99);
        assert_eq!(ch.lane(0), 0);
        assert_eq!(ch.lane(2), 99);
    }

    #[test]
    fn test_channel_lanes_const() {
        assert_eq!(Channel::<u8, 8>::LANES, 8);
        assert_eq!(Channel::<u32, 4>::LANES, 4);
    }
}
