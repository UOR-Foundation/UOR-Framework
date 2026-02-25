//! The 5 fundamental microcode primitives.
//!
//! These operations form a functionally complete Boolean basis from which
//! all other operations can be derived:
//!
//! - `bnot`: Bitwise NOT (unary)
//! - `neg`: Two's complement negation (unary)
//! - `xor`: Bitwise XOR (binary)
//! - `and`: Bitwise AND (binary)
//! - `or`: Bitwise OR (binary)
//!
//! # Mathematical Properties
//!
//! - `{NOT, AND}` is functionally complete
//! - `{NOT, OR}` is functionally complete
//! - XOR provides efficient parity/addition
//! - NEG provides two's complement for arithmetic
//!
//! # Critical Identities
//!
//! ```text
//! neg(bnot(x)) = x + 1  (INC)
//! bnot(neg(x)) = x - 1  (DEC)
//! neg(neg(x)) = x       (involution)
//! bnot(bnot(x)) = x     (involution)
//! ```

use super::word::MicrocodeWord;

/// The 5 fundamental operations that generate all others.
///
/// Every backend implements these 5 operations using hardware-specific
/// intrinsics. All other operations (add, sub, mul, etc.) are synthesized
/// from compositions of these primitives via `MicrocodeOps`.
///
/// # Type Parameter
///
/// `W` is the word type - can be scalar (u32) or SIMD ([u32; 8] for AVX2).
/// This allows the same trait to work across different backend word sizes.
///
/// # Example Implementation
///
/// ```
/// use uor::microcode::{MicrocodeWord, MicrocodePrimitives};
///
/// struct ScalarBackend;
///
/// impl MicrocodePrimitives<u32> for ScalarBackend {
///     fn bnot(&self, a: u32) -> u32 { !a }
///     fn neg(&self, a: u32) -> u32 { a.wrapping_neg() }
///     fn xor(&self, a: u32, b: u32) -> u32 { a ^ b }
///     fn and(&self, a: u32, b: u32) -> u32 { a & b }
///     fn or(&self, a: u32, b: u32) -> u32 { a | b }
/// }
/// ```
pub trait MicrocodePrimitives<W: MicrocodeWord> {
    /// Bitwise NOT: `!a`
    ///
    /// Flips all bits. Self-inverse: `bnot(bnot(x)) = x`.
    ///
    /// # Hardware Mapping
    ///
    /// - x86_64 AVX2: `vpxor` with all-ones mask
    /// - aarch64 NEON: `vmvnq_u32`
    /// - WASM SIMD: `v128.not`
    fn bnot(&self, a: W) -> W;

    /// Two's complement negation: `-a = !a + 1`
    ///
    /// Self-inverse: `neg(neg(x)) = x`.
    ///
    /// # Hardware Mapping
    ///
    /// - x86_64 AVX2: `vpsubd` from zero
    /// - aarch64 NEON: `vnegq_s32`
    /// - WASM SIMD: `i32x4.neg`
    fn neg(&self, a: W) -> W;

    /// Bitwise XOR: `a ^ b`
    ///
    /// Self-inverse with same operand: `xor(xor(x, y), y) = x`.
    /// Commutative and associative.
    ///
    /// # Hardware Mapping
    ///
    /// - x86_64 AVX2: `vpxor`
    /// - aarch64 NEON: `veorq_u32`
    /// - WASM SIMD: `v128.xor`
    fn xor(&self, a: W, b: W) -> W;

    /// Bitwise AND: `a & b`
    ///
    /// Commutative and associative.
    /// `and(a, ONES) = a`, `and(a, ZEROS) = ZEROS`.
    ///
    /// # Hardware Mapping
    ///
    /// - x86_64 AVX2: `vpand`
    /// - aarch64 NEON: `vandq_u32`
    /// - WASM SIMD: `v128.and`
    fn and(&self, a: W, b: W) -> W;

    /// Bitwise OR: `a | b`
    ///
    /// Commutative and associative.
    /// `or(a, ZEROS) = a`, `or(a, ONES) = ONES`.
    ///
    /// # Hardware Mapping
    ///
    /// - x86_64 AVX2: `vpor`
    /// - aarch64 NEON: `vorrq_u32`
    /// - WASM SIMD: `v128.or`
    fn or(&self, a: W, b: W) -> W;

    // -------------------------------------------------------------------------
    // Convenience methods with default implementations
    // -------------------------------------------------------------------------

    /// NAND: `!(a & b)` - functionally complete on its own
    #[inline(always)]
    fn nand(&self, a: W, b: W) -> W {
        self.bnot(self.and(a, b))
    }

    /// NOR: `!(a | b)` - functionally complete on its own
    #[inline(always)]
    fn nor(&self, a: W, b: W) -> W {
        self.bnot(self.or(a, b))
    }

    /// XNOR (equivalence): `!(a ^ b)`
    #[inline(always)]
    fn xnor(&self, a: W, b: W) -> W {
        self.bnot(self.xor(a, b))
    }

    /// AND-NOT (bit clear): `a & !b`
    #[inline(always)]
    fn andn(&self, a: W, b: W) -> W {
        self.and(a, self.bnot(b))
    }

    /// OR-NOT: `a | !b`
    #[inline(always)]
    fn orn(&self, a: W, b: W) -> W {
        self.or(a, self.bnot(b))
    }
}

/// Reference scalar implementation of microcode primitives.
///
/// This implementation uses pure Rust operations and serves as:
/// 1. A reference for testing other implementations
/// 2. A fallback when no SIMD is available
/// 3. A baseline for benchmarks
#[derive(Debug, Clone, Copy, Default)]
pub struct ScalarPrimitives;

impl MicrocodePrimitives<u8> for ScalarPrimitives {
    #[inline(always)]
    fn bnot(&self, a: u8) -> u8 {
        !a
    }
    #[inline(always)]
    fn neg(&self, a: u8) -> u8 {
        a.wrapping_neg()
    }
    #[inline(always)]
    fn xor(&self, a: u8, b: u8) -> u8 {
        a ^ b
    }
    #[inline(always)]
    fn and(&self, a: u8, b: u8) -> u8 {
        a & b
    }
    #[inline(always)]
    fn or(&self, a: u8, b: u8) -> u8 {
        a | b
    }
}

impl MicrocodePrimitives<u16> for ScalarPrimitives {
    #[inline(always)]
    fn bnot(&self, a: u16) -> u16 {
        !a
    }
    #[inline(always)]
    fn neg(&self, a: u16) -> u16 {
        a.wrapping_neg()
    }
    #[inline(always)]
    fn xor(&self, a: u16, b: u16) -> u16 {
        a ^ b
    }
    #[inline(always)]
    fn and(&self, a: u16, b: u16) -> u16 {
        a & b
    }
    #[inline(always)]
    fn or(&self, a: u16, b: u16) -> u16 {
        a | b
    }
}

impl MicrocodePrimitives<u32> for ScalarPrimitives {
    #[inline(always)]
    fn bnot(&self, a: u32) -> u32 {
        !a
    }
    #[inline(always)]
    fn neg(&self, a: u32) -> u32 {
        a.wrapping_neg()
    }
    #[inline(always)]
    fn xor(&self, a: u32, b: u32) -> u32 {
        a ^ b
    }
    #[inline(always)]
    fn and(&self, a: u32, b: u32) -> u32 {
        a & b
    }
    #[inline(always)]
    fn or(&self, a: u32, b: u32) -> u32 {
        a | b
    }
}

impl MicrocodePrimitives<u64> for ScalarPrimitives {
    #[inline(always)]
    fn bnot(&self, a: u64) -> u64 {
        !a
    }
    #[inline(always)]
    fn neg(&self, a: u64) -> u64 {
        a.wrapping_neg()
    }
    #[inline(always)]
    fn xor(&self, a: u64, b: u64) -> u64 {
        a ^ b
    }
    #[inline(always)]
    fn and(&self, a: u64, b: u64) -> u64 {
        a & b
    }
    #[inline(always)]
    fn or(&self, a: u64, b: u64) -> u64 {
        a | b
    }
}

impl MicrocodePrimitives<[u32; 4]> for ScalarPrimitives {
    #[inline(always)]
    fn bnot(&self, a: [u32; 4]) -> [u32; 4] {
        [!a[0], !a[1], !a[2], !a[3]]
    }

    #[inline(always)]
    fn neg(&self, a: [u32; 4]) -> [u32; 4] {
        [
            a[0].wrapping_neg(),
            a[1].wrapping_neg(),
            a[2].wrapping_neg(),
            a[3].wrapping_neg(),
        ]
    }

    #[inline(always)]
    fn xor(&self, a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
        [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
    }

    #[inline(always)]
    fn and(&self, a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
        [a[0] & b[0], a[1] & b[1], a[2] & b[2], a[3] & b[3]]
    }

    #[inline(always)]
    fn or(&self, a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
        [a[0] | b[0], a[1] | b[1], a[2] | b[2], a[3] | b[3]]
    }
}

impl MicrocodePrimitives<[u32; 8]> for ScalarPrimitives {
    #[inline(always)]
    fn bnot(&self, a: [u32; 8]) -> [u32; 8] {
        [!a[0], !a[1], !a[2], !a[3], !a[4], !a[5], !a[6], !a[7]]
    }

    #[inline(always)]
    fn neg(&self, a: [u32; 8]) -> [u32; 8] {
        [
            a[0].wrapping_neg(),
            a[1].wrapping_neg(),
            a[2].wrapping_neg(),
            a[3].wrapping_neg(),
            a[4].wrapping_neg(),
            a[5].wrapping_neg(),
            a[6].wrapping_neg(),
            a[7].wrapping_neg(),
        ]
    }

    #[inline(always)]
    fn xor(&self, a: [u32; 8], b: [u32; 8]) -> [u32; 8] {
        [
            a[0] ^ b[0],
            a[1] ^ b[1],
            a[2] ^ b[2],
            a[3] ^ b[3],
            a[4] ^ b[4],
            a[5] ^ b[5],
            a[6] ^ b[6],
            a[7] ^ b[7],
        ]
    }

    #[inline(always)]
    fn and(&self, a: [u32; 8], b: [u32; 8]) -> [u32; 8] {
        [
            a[0] & b[0],
            a[1] & b[1],
            a[2] & b[2],
            a[3] & b[3],
            a[4] & b[4],
            a[5] & b[5],
            a[6] & b[6],
            a[7] & b[7],
        ]
    }

    #[inline(always)]
    fn or(&self, a: [u32; 8], b: [u32; 8]) -> [u32; 8] {
        [
            a[0] | b[0],
            a[1] | b[1],
            a[2] | b[2],
            a[3] | b[3],
            a[4] | b[4],
            a[5] | b[5],
            a[6] | b[6],
            a[7] | b[7],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_primitives() {
        let p = ScalarPrimitives;

        // bnot
        assert_eq!(p.bnot(0u32), u32::MAX);
        assert_eq!(p.bnot(u32::MAX), 0);

        // neg
        assert_eq!(p.neg(0u32), 0);
        assert_eq!(p.neg(1u32), u32::MAX); // -1 in two's complement

        // xor
        assert_eq!(p.xor(0xFFFF_0000u32, 0x00FF_FF00), 0xFF00_FF00);

        // and
        assert_eq!(p.and(0xFFFF_0000u32, 0x00FF_FF00), 0x00FF_0000);

        // or
        assert_eq!(p.or(0xFFFF_0000u32, 0x00FF_FF00), 0xFFFF_FF00);
    }

    #[test]
    fn test_derived_gates() {
        let p = ScalarPrimitives;

        // NAND truth table
        assert_eq!(p.nand(0u32, 0), u32::MAX);
        assert_eq!(p.nand(u32::MAX, 0), u32::MAX);
        assert_eq!(p.nand(0, u32::MAX), u32::MAX);
        assert_eq!(p.nand(u32::MAX, u32::MAX), 0);

        // NOR truth table
        assert_eq!(p.nor(0u32, 0), u32::MAX);
        assert_eq!(p.nor(u32::MAX, 0), 0);
        assert_eq!(p.nor(0, u32::MAX), 0);
        assert_eq!(p.nor(u32::MAX, u32::MAX), 0);
    }

    #[test]
    fn test_array_primitives() {
        let p = ScalarPrimitives;
        let a: [u32; 4] = [1, 2, 3, 4];
        let b: [u32; 4] = [4, 3, 2, 1];

        let xor = p.xor(a, b);
        assert_eq!(xor, [1 ^ 4, 2 ^ 3, 3 ^ 2, 4 ^ 1]);

        let and = p.and(a, b);
        assert_eq!(and, [1 & 4, 2 & 3, 3 & 2, 4 & 1]);
    }
}
