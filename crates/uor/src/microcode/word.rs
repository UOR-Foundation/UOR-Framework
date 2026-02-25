//! Word types that support microcode operations.
//!
//! The `MicrocodeWord` trait defines the requirements for types that can be
//! used as operands in microcode primitives. This includes scalar types
//! (u8, u32, u64) and SIMD vector types.

/// A word type that supports microcode operations.
///
/// This trait captures the minimal requirements for a type to participate
/// in microcode computation:
///
/// - Bitwise operations: AND, OR, XOR, NOT
/// - Two's complement negation
/// - Copy semantics (no ownership transfer in operations)
/// - Constants for all-zeros and all-ones patterns
///
/// # Implementors
///
/// - Scalar: `u8`, `u32`, `u64`
/// - SIMD: `[u32; 4]` (WASM v128), `[u32; 8]` (AVX2 YMM)
///
/// For actual SIMD intrinsic types (like `__m256i`), implement this trait
/// in the backend crate where those types are available.
pub trait MicrocodeWord: Copy + Default + Eq + Sized {
    /// Bitwise NOT
    fn bit_not(self) -> Self;

    /// Bitwise XOR
    fn bit_xor(self, other: Self) -> Self;

    /// Bitwise AND
    fn bit_and(self, other: Self) -> Self;

    /// Bitwise OR
    fn bit_or(self, other: Self) -> Self;

    /// Two's complement negation: `-a = !a + 1`
    fn wrapping_neg(self) -> Self;

    /// All-ones constant (e.g., `0xFFFFFFFF` for u32).
    const ONES: Self;

    /// All-zeros constant.
    const ZEROS: Self;

    /// Number of bits in this word type.
    const BITS: u32;

    /// Number of bits per element (for SIMD arrays, this is the element bit width).
    ///
    /// For scalar types: `ELEMENT_BITS == BITS`
    /// For `[u32; N]`: `ELEMENT_BITS == 32`
    /// For `[u64; N]`: `ELEMENT_BITS == 64`
    ///
    /// This is used for per-element shift operations in comparison functions.
    const ELEMENT_BITS: u32 = Self::BITS;
}

// -----------------------------------------------------------------------------
// Scalar implementations
// -----------------------------------------------------------------------------

macro_rules! impl_microcode_word_scalar {
    ($ty:ty, $bits:expr) => {
        impl MicrocodeWord for $ty {
            #[inline(always)]
            fn bit_not(self) -> Self {
                !self
            }
            #[inline(always)]
            fn bit_xor(self, other: Self) -> Self {
                self ^ other
            }
            #[inline(always)]
            fn bit_and(self, other: Self) -> Self {
                self & other
            }
            #[inline(always)]
            fn bit_or(self, other: Self) -> Self {
                self | other
            }
            #[inline(always)]
            fn wrapping_neg(self) -> Self {
                <$ty>::wrapping_neg(self)
            }
            const ONES: Self = <$ty>::MAX;
            const ZEROS: Self = 0;
            const BITS: u32 = $bits;
        }
    };
}

impl_microcode_word_scalar!(u8, 8);
impl_microcode_word_scalar!(u16, 16);
impl_microcode_word_scalar!(u32, 32);
impl_microcode_word_scalar!(u64, 64);
impl_microcode_word_scalar!(u128, 128);

// -----------------------------------------------------------------------------
// Array implementations (for portable SIMD-like operations)
// -----------------------------------------------------------------------------

impl MicrocodeWord for [u32; 4] {
    #[inline(always)]
    fn bit_not(self) -> Self {
        [!self[0], !self[1], !self[2], !self[3]]
    }
    #[inline(always)]
    fn bit_xor(self, other: Self) -> Self {
        [
            self[0] ^ other[0],
            self[1] ^ other[1],
            self[2] ^ other[2],
            self[3] ^ other[3],
        ]
    }
    #[inline(always)]
    fn bit_and(self, other: Self) -> Self {
        [
            self[0] & other[0],
            self[1] & other[1],
            self[2] & other[2],
            self[3] & other[3],
        ]
    }
    #[inline(always)]
    fn bit_or(self, other: Self) -> Self {
        [
            self[0] | other[0],
            self[1] | other[1],
            self[2] | other[2],
            self[3] | other[3],
        ]
    }
    #[inline(always)]
    fn wrapping_neg(self) -> Self {
        [
            self[0].wrapping_neg(),
            self[1].wrapping_neg(),
            self[2].wrapping_neg(),
            self[3].wrapping_neg(),
        ]
    }
    const ONES: Self = [u32::MAX; 4];
    const ZEROS: Self = [0; 4];
    const BITS: u32 = 128;
    const ELEMENT_BITS: u32 = 32;
}

impl MicrocodeWord for [u32; 8] {
    #[inline(always)]
    fn bit_not(self) -> Self {
        [
            !self[0], !self[1], !self[2], !self[3], !self[4], !self[5], !self[6], !self[7],
        ]
    }
    #[inline(always)]
    fn bit_xor(self, other: Self) -> Self {
        [
            self[0] ^ other[0],
            self[1] ^ other[1],
            self[2] ^ other[2],
            self[3] ^ other[3],
            self[4] ^ other[4],
            self[5] ^ other[5],
            self[6] ^ other[6],
            self[7] ^ other[7],
        ]
    }
    #[inline(always)]
    fn bit_and(self, other: Self) -> Self {
        [
            self[0] & other[0],
            self[1] & other[1],
            self[2] & other[2],
            self[3] & other[3],
            self[4] & other[4],
            self[5] & other[5],
            self[6] & other[6],
            self[7] & other[7],
        ]
    }
    #[inline(always)]
    fn bit_or(self, other: Self) -> Self {
        [
            self[0] | other[0],
            self[1] | other[1],
            self[2] | other[2],
            self[3] | other[3],
            self[4] | other[4],
            self[5] | other[5],
            self[6] | other[6],
            self[7] | other[7],
        ]
    }
    #[inline(always)]
    fn wrapping_neg(self) -> Self {
        [
            self[0].wrapping_neg(),
            self[1].wrapping_neg(),
            self[2].wrapping_neg(),
            self[3].wrapping_neg(),
            self[4].wrapping_neg(),
            self[5].wrapping_neg(),
            self[6].wrapping_neg(),
            self[7].wrapping_neg(),
        ]
    }
    const ONES: Self = [u32::MAX; 8];
    const ZEROS: Self = [0; 8];
    const BITS: u32 = 256;
    const ELEMENT_BITS: u32 = 32;
}

impl MicrocodeWord for [u64; 4] {
    #[inline(always)]
    fn bit_not(self) -> Self {
        [!self[0], !self[1], !self[2], !self[3]]
    }
    #[inline(always)]
    fn bit_xor(self, other: Self) -> Self {
        [
            self[0] ^ other[0],
            self[1] ^ other[1],
            self[2] ^ other[2],
            self[3] ^ other[3],
        ]
    }
    #[inline(always)]
    fn bit_and(self, other: Self) -> Self {
        [
            self[0] & other[0],
            self[1] & other[1],
            self[2] & other[2],
            self[3] & other[3],
        ]
    }
    #[inline(always)]
    fn bit_or(self, other: Self) -> Self {
        [
            self[0] | other[0],
            self[1] | other[1],
            self[2] | other[2],
            self[3] | other[3],
        ]
    }
    #[inline(always)]
    fn wrapping_neg(self) -> Self {
        [
            self[0].wrapping_neg(),
            self[1].wrapping_neg(),
            self[2].wrapping_neg(),
            self[3].wrapping_neg(),
        ]
    }
    const ONES: Self = [u64::MAX; 4];
    const ZEROS: Self = [0; 4];
    const BITS: u32 = 256;
    const ELEMENT_BITS: u32 = 64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_word() {
        assert_eq!(u32::ONES, 0xFFFF_FFFF);
        assert_eq!(u32::ZEROS, 0);
        assert_eq!(42u32.wrapping_neg(), (!42u32).wrapping_add(1));
    }

    #[test]
    fn test_array_word() {
        let a: [u32; 4] = [1, 2, 3, 4];
        let neg_a = a.wrapping_neg();
        assert_eq!(neg_a[0], 1u32.wrapping_neg());
        assert_eq!(neg_a[1], 2u32.wrapping_neg());
        assert_eq!(neg_a[2], 3u32.wrapping_neg());
        assert_eq!(neg_a[3], 4u32.wrapping_neg());
    }

    #[test]
    fn test_bitwise_ops() {
        let a: [u32; 4] = [0xFF00_FF00, 0x00FF_00FF, 0xAAAA_AAAA, 0x5555_5555];
        let b: [u32; 4] = [0xF0F0_F0F0, 0x0F0F_0F0F, 0xCCCC_CCCC, 0x3333_3333];

        // XOR
        let xor = a.bit_xor(b);
        assert_eq!(xor[0], 0xFF00_FF00 ^ 0xF0F0_F0F0);

        // AND
        let and = a.bit_and(b);
        assert_eq!(and[0], 0xFF00_FF00 & 0xF0F0_F0F0);

        // OR
        let or = a.bit_or(b);
        assert_eq!(or[0], 0xFF00_FF00 | 0xF0F0_F0F0);

        // NOT
        let not_a = a.bit_not();
        assert_eq!(not_a[0], !0xFF00_FF00);
    }
}
