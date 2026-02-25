//! Derived operations built from microcode primitives.
//!
//! The `MicrocodeOps` trait provides default implementations for common
//! operations using only the 5 fundamental primitives. Backends can override
//! any of these with native implementations for better performance.

use super::kogge_stone::KoggeStoneAdder;
use super::primitives::MicrocodePrimitives;
use super::word::MicrocodeWord;

/// Derived operations synthesized from the 5 primitives.
///
/// This trait provides default implementations that work with any backend
/// implementing `MicrocodePrimitives`. Backends SHOULD override operations
/// where native hardware support is available.
///
/// # Override Guidelines
///
/// | Operation | Override When |
/// |-----------|---------------|
/// | `add` | Native SIMD add available (vpaddd, vaddq_u32) |
/// | `sub` | Native SIMD sub available |
/// | `mul` | Native SIMD mul available |
/// | `shl`/`shr` | Native SIMD shift available |
///
/// # Example
///
/// ```ignore
/// impl<W: MicrocodeWord> MicrocodeOps<W> for Avx2Backend {
///     // Override add with native vpaddd
///     fn add(&self, a: W, b: W) -> W {
///         unsafe { _mm256_add_epi32(a, b) }
///     }
///
///     // Let other ops use default microcode implementations
/// }
/// ```
pub trait MicrocodeOps<W: MicrocodeWord>: MicrocodePrimitives<W> {
    // -------------------------------------------------------------------------
    // Fundamental derived operations
    // -------------------------------------------------------------------------

    /// Increment: `x + 1`
    ///
    /// Microcode: `neg(bnot(x))`
    ///
    /// This is the critical identity from the UOR Q3 framework:
    /// `neg(bnot(x)) = succ(x)`
    #[inline(always)]
    fn inc(&self, a: W) -> W {
        self.neg(self.bnot(a))
    }

    /// Decrement: `x - 1`
    ///
    /// Microcode: `bnot(neg(x))`
    #[inline(always)]
    fn dec(&self, a: W) -> W {
        self.bnot(self.neg(a))
    }

    /// Addition: `a + b`
    ///
    /// Default implementation uses Kogge-Stone parallel prefix adder.
    /// Override with native SIMD add for better performance.
    ///
    /// # Performance
    ///
    /// - Microcode: ~5-7 operations per bit-width log2
    /// - Native vpaddd: 1 cycle
    #[inline]
    fn add(&self, a: W, b: W) -> W
    where
        Self: Sized,
    {
        KoggeStoneAdder::add(self, a, b)
    }

    /// Subtraction: `a - b = a + neg(b)`
    ///
    /// Microcode: `add(a, neg(b))`
    #[inline]
    fn sub(&self, a: W, b: W) -> W
    where
        Self: Sized,
    {
        self.add(a, self.neg(b))
    }

    // -------------------------------------------------------------------------
    // Comparison operations
    // -------------------------------------------------------------------------

    /// Less than (signed): returns all-ones if a < b, else all-zeros
    ///
    /// Microcode: Extract sign bit of (a - b)
    #[inline]
    fn lt_signed(&self, a: W, b: W) -> W
    where
        Self: Sized,
    {
        // a < b iff (a - b) is negative
        // For signed comparison, check the sign bit
        let diff = self.sub(a, b);
        self.shr_logical(diff, W::BITS - 1)
    }

    /// Equal: returns all-ones if a == b, else all-zeros.
    ///
    /// Microcode: `not(is_nonzero(xor(a, b)))`
    ///
    /// When a == b, xor(a, b) == 0, so is_nonzero returns all-zeros,
    /// and not(all-zeros) == all-ones.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let p = ScalarPrimitives;
    /// assert_eq!(p.eq(42u32, 42), u32::MAX); // all-ones (true)
    /// assert_eq!(p.eq(42u32, 43), 0);        // all-zeros (false)
    /// ```
    #[inline]
    fn eq(&self, a: W, b: W) -> W {
        self.bnot(self.is_nonzero(self.xor(a, b)))
    }

    /// Not equal: returns all-ones if a != b, else all-zeros.
    ///
    /// Microcode: `is_nonzero(xor(a, b))`
    ///
    /// When a != b, xor(a, b) has at least one bit set.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let p = ScalarPrimitives;
    /// assert_eq!(p.ne(42u32, 43), u32::MAX); // all-ones (true)
    /// assert_eq!(p.ne(42u32, 42), 0);        // all-zeros (false)
    /// ```
    #[inline]
    fn ne(&self, a: W, b: W) -> W {
        self.is_nonzero(self.xor(a, b))
    }

    /// Less than (unsigned): alias for `lt_unsigned`.
    ///
    /// Returns all-ones if a < b, else all-zeros.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let p = ScalarPrimitives;
    /// assert_eq!(p.lt(10u32, 20), u32::MAX); // true
    /// assert_eq!(p.lt(20u32, 10), 0);        // false
    /// assert_eq!(p.lt(10u32, 10), 0);        // false
    /// ```
    #[inline]
    fn lt(&self, a: W, b: W) -> W {
        self.lt_unsigned(a, b)
    }

    /// Less than or equal (unsigned): returns all-ones if a <= b, else all-zeros.
    ///
    /// Microcode: `not(lt(b, a))` which is equivalent to `or(lt(a, b), eq(a, b))`
    ///
    /// # Example
    ///
    /// ```ignore
    /// let p = ScalarPrimitives;
    /// assert_eq!(p.le(10u32, 20), u32::MAX); // true
    /// assert_eq!(p.le(20u32, 10), 0);        // false
    /// assert_eq!(p.le(10u32, 10), u32::MAX); // true (equal)
    /// ```
    #[inline]
    fn le(&self, a: W, b: W) -> W {
        // a <= b iff !(b < a)
        self.bnot(self.lt_unsigned(b, a))
    }

    /// Greater than (unsigned): returns all-ones if a > b, else all-zeros.
    ///
    /// Microcode: `lt(b, a)` (swap operands)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let p = ScalarPrimitives;
    /// assert_eq!(p.gt(20u32, 10), u32::MAX); // true
    /// assert_eq!(p.gt(10u32, 20), 0);        // false
    /// assert_eq!(p.gt(10u32, 10), 0);        // false
    /// ```
    #[inline]
    fn gt(&self, a: W, b: W) -> W {
        self.lt_unsigned(b, a)
    }

    /// Greater than or equal (unsigned): returns all-ones if a >= b, else all-zeros.
    ///
    /// Microcode: `not(lt(a, b))`
    ///
    /// # Example
    ///
    /// ```ignore
    /// let p = ScalarPrimitives;
    /// assert_eq!(p.ge(20u32, 10), u32::MAX); // true
    /// assert_eq!(p.ge(10u32, 20), 0);        // false
    /// assert_eq!(p.ge(10u32, 10), u32::MAX); // true (equal)
    /// ```
    #[inline]
    fn ge(&self, a: W, b: W) -> W {
        // a >= b iff !(a < b)
        self.bnot(self.lt_unsigned(a, b))
    }

    // -------------------------------------------------------------------------
    // Shift operations (basic microcode implementations)
    // -------------------------------------------------------------------------

    /// Logical shift right by n bits.
    ///
    /// Basic implementation using repeated halving.
    /// Override with native SIMD shift for performance.
    fn shr_logical(&self, a: W, n: u32) -> W;

    /// Logical shift left by n bits.
    ///
    /// Basic implementation using repeated doubling.
    /// Override with native SIMD shift for performance.
    fn shl_logical(&self, a: W, n: u32) -> W;

    /// Shift left by n bits. Alias for `shl_logical`.
    ///
    /// Shifts each element/lane left by n bits, inserting zeros at the LSB.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let p = ScalarPrimitives;
    /// assert_eq!(p.shl(0x0000_00FFu32, 8), 0x0000_FF00);
    /// ```
    #[inline(always)]
    fn shl(&self, a: W, n: u32) -> W {
        self.shl_logical(a, n)
    }

    /// Shift right by n bits. Alias for `shr_logical`.
    ///
    /// Shifts each element/lane right by n bits, inserting zeros at the MSB.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let p = ScalarPrimitives;
    /// assert_eq!(p.shr(0xFF00_0000u32, 8), 0x00FF_0000);
    /// ```
    #[inline(always)]
    fn shr(&self, a: W, n: u32) -> W {
        self.shr_logical(a, n)
    }

    // -------------------------------------------------------------------------
    // Rotate operations
    // -------------------------------------------------------------------------

    /// Rotate left by n bits.
    ///
    /// Rotates each element/lane left by n bits. Bits shifted out of the MSB
    /// are inserted at the LSB.
    ///
    /// Microcode: `shl(a, n) | shr(a, ELEMENT_BITS - n)`
    ///
    /// For scalar types, ELEMENT_BITS equals W::BITS.
    /// For SIMD arrays, ELEMENT_BITS is the element bit width (e.g., 32 for [u32; 4]).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let p = ScalarPrimitives;
    /// assert_eq!(p.rotl(0x8000_0001u32, 1), 0x0000_0003);
    /// ```
    fn rotl(&self, a: W, n: u32) -> W;

    /// Rotate right by n bits.
    ///
    /// Rotates each element/lane right by n bits. Bits shifted out of the LSB
    /// are inserted at the MSB.
    ///
    /// Microcode: `shr(a, n) | shl(a, ELEMENT_BITS - n)`
    ///
    /// For scalar types, ELEMENT_BITS equals W::BITS.
    /// For SIMD arrays, ELEMENT_BITS is the element bit width (e.g., 32 for [u32; 4]).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let p = ScalarPrimitives;
    /// assert_eq!(p.rotr(0x0000_0003u32, 1), 0x8000_0001);
    /// ```
    fn rotr(&self, a: W, n: u32) -> W;

    // -------------------------------------------------------------------------
    // Conditional operations
    // -------------------------------------------------------------------------

    /// Select: `if mask then a else b`
    ///
    /// Microcode: `(a & mask) | (b & !mask)`
    ///
    /// This is a branchless select useful for constant-time algorithms.
    #[inline(always)]
    fn select(&self, mask: W, a: W, b: W) -> W {
        self.or(self.and(a, mask), self.andn(b, mask))
    }

    /// Zero extend comparison: returns W::ONES if a != 0, else W::ZEROS
    #[inline]
    fn is_nonzero(&self, a: W) -> W {
        // neg(a) | a has sign bit set iff a != 0
        // Then we arithmetic shift to broadcast the sign bit
        // Use ELEMENT_BITS for per-element shift (32 for [u32; N], 64 for u64, etc.)
        let has_bits = self.or(self.neg(a), a);
        self.sar(has_bits, W::ELEMENT_BITS - 1)
    }

    /// Arithmetic shift right (sign-extending).
    fn sar(&self, a: W, n: u32) -> W;

    // -------------------------------------------------------------------------
    // Min/Max operations
    // -------------------------------------------------------------------------

    /// Unsigned minimum
    #[inline]
    fn min_unsigned(&self, a: W, b: W) -> W {
        let a_lt_b = self.lt_unsigned(a, b);
        self.select(a_lt_b, a, b)
    }

    /// Unsigned maximum
    #[inline]
    fn max_unsigned(&self, a: W, b: W) -> W {
        let a_lt_b = self.lt_unsigned(a, b);
        self.select(a_lt_b, b, a)
    }

    /// Less than (unsigned): returns all-ones if a < b, else all-zeros
    fn lt_unsigned(&self, a: W, b: W) -> W;
}

// -----------------------------------------------------------------------------
// u32 implementation with shift support
// -----------------------------------------------------------------------------

impl<P: MicrocodePrimitives<u32>> MicrocodeOps<u32> for P {
    #[inline(always)]
    fn shr_logical(&self, a: u32, n: u32) -> u32 {
        a >> n
    }

    #[inline(always)]
    fn shl_logical(&self, a: u32, n: u32) -> u32 {
        a << n
    }

    #[inline(always)]
    fn sar(&self, a: u32, n: u32) -> u32 {
        // Arithmetic shift right for u32: treat as i32
        ((a as i32) >> n) as u32
    }

    #[inline]
    fn lt_unsigned(&self, a: u32, b: u32) -> u32 {
        if a < b {
            u32::MAX
        } else {
            0
        }
    }

    #[inline(always)]
    fn rotl(&self, a: u32, n: u32) -> u32 {
        a.rotate_left(n)
    }

    #[inline(always)]
    fn rotr(&self, a: u32, n: u32) -> u32 {
        a.rotate_right(n)
    }
}

impl<P: MicrocodePrimitives<u64>> MicrocodeOps<u64> for P {
    #[inline(always)]
    fn shr_logical(&self, a: u64, n: u32) -> u64 {
        a >> n
    }

    #[inline(always)]
    fn shl_logical(&self, a: u64, n: u32) -> u64 {
        a << n
    }

    #[inline(always)]
    fn sar(&self, a: u64, n: u32) -> u64 {
        ((a as i64) >> n) as u64
    }

    #[inline]
    fn lt_unsigned(&self, a: u64, b: u64) -> u64 {
        if a < b {
            u64::MAX
        } else {
            0
        }
    }

    #[inline(always)]
    fn rotl(&self, a: u64, n: u32) -> u64 {
        a.rotate_left(n)
    }

    #[inline(always)]
    fn rotr(&self, a: u64, n: u32) -> u64 {
        a.rotate_right(n)
    }
}

impl<P: MicrocodePrimitives<[u32; 4]>> MicrocodeOps<[u32; 4]> for P {
    #[inline(always)]
    fn shr_logical(&self, a: [u32; 4], n: u32) -> [u32; 4] {
        [a[0] >> n, a[1] >> n, a[2] >> n, a[3] >> n]
    }

    #[inline(always)]
    fn shl_logical(&self, a: [u32; 4], n: u32) -> [u32; 4] {
        [a[0] << n, a[1] << n, a[2] << n, a[3] << n]
    }

    #[inline(always)]
    fn sar(&self, a: [u32; 4], n: u32) -> [u32; 4] {
        [
            ((a[0] as i32) >> n) as u32,
            ((a[1] as i32) >> n) as u32,
            ((a[2] as i32) >> n) as u32,
            ((a[3] as i32) >> n) as u32,
        ]
    }

    #[inline]
    fn lt_unsigned(&self, a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
        [
            if a[0] < b[0] { u32::MAX } else { 0 },
            if a[1] < b[1] { u32::MAX } else { 0 },
            if a[2] < b[2] { u32::MAX } else { 0 },
            if a[3] < b[3] { u32::MAX } else { 0 },
        ]
    }

    #[inline(always)]
    fn rotl(&self, a: [u32; 4], n: u32) -> [u32; 4] {
        [
            a[0].rotate_left(n),
            a[1].rotate_left(n),
            a[2].rotate_left(n),
            a[3].rotate_left(n),
        ]
    }

    #[inline(always)]
    fn rotr(&self, a: [u32; 4], n: u32) -> [u32; 4] {
        [
            a[0].rotate_right(n),
            a[1].rotate_right(n),
            a[2].rotate_right(n),
            a[3].rotate_right(n),
        ]
    }
}

impl<P: MicrocodePrimitives<[u32; 8]>> MicrocodeOps<[u32; 8]> for P {
    #[inline(always)]
    fn shr_logical(&self, a: [u32; 8], n: u32) -> [u32; 8] {
        [
            a[0] >> n,
            a[1] >> n,
            a[2] >> n,
            a[3] >> n,
            a[4] >> n,
            a[5] >> n,
            a[6] >> n,
            a[7] >> n,
        ]
    }

    #[inline(always)]
    fn shl_logical(&self, a: [u32; 8], n: u32) -> [u32; 8] {
        [
            a[0] << n,
            a[1] << n,
            a[2] << n,
            a[3] << n,
            a[4] << n,
            a[5] << n,
            a[6] << n,
            a[7] << n,
        ]
    }

    #[inline(always)]
    fn sar(&self, a: [u32; 8], n: u32) -> [u32; 8] {
        [
            ((a[0] as i32) >> n) as u32,
            ((a[1] as i32) >> n) as u32,
            ((a[2] as i32) >> n) as u32,
            ((a[3] as i32) >> n) as u32,
            ((a[4] as i32) >> n) as u32,
            ((a[5] as i32) >> n) as u32,
            ((a[6] as i32) >> n) as u32,
            ((a[7] as i32) >> n) as u32,
        ]
    }

    #[inline]
    fn lt_unsigned(&self, a: [u32; 8], b: [u32; 8]) -> [u32; 8] {
        [
            if a[0] < b[0] { u32::MAX } else { 0 },
            if a[1] < b[1] { u32::MAX } else { 0 },
            if a[2] < b[2] { u32::MAX } else { 0 },
            if a[3] < b[3] { u32::MAX } else { 0 },
            if a[4] < b[4] { u32::MAX } else { 0 },
            if a[5] < b[5] { u32::MAX } else { 0 },
            if a[6] < b[6] { u32::MAX } else { 0 },
            if a[7] < b[7] { u32::MAX } else { 0 },
        ]
    }

    #[inline(always)]
    fn rotl(&self, a: [u32; 8], n: u32) -> [u32; 8] {
        [
            a[0].rotate_left(n),
            a[1].rotate_left(n),
            a[2].rotate_left(n),
            a[3].rotate_left(n),
            a[4].rotate_left(n),
            a[5].rotate_left(n),
            a[6].rotate_left(n),
            a[7].rotate_left(n),
        ]
    }

    #[inline(always)]
    fn rotr(&self, a: [u32; 8], n: u32) -> [u32; 8] {
        [
            a[0].rotate_right(n),
            a[1].rotate_right(n),
            a[2].rotate_right(n),
            a[3].rotate_right(n),
            a[4].rotate_right(n),
            a[5].rotate_right(n),
            a[6].rotate_right(n),
            a[7].rotate_right(n),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::microcode::primitives::ScalarPrimitives;

    #[test]
    fn test_inc_dec() {
        let p = ScalarPrimitives;

        // inc
        assert_eq!(p.inc(0u32), 1);
        assert_eq!(p.inc(41u32), 42);
        assert_eq!(p.inc(u32::MAX), 0); // Wrap around

        // dec
        assert_eq!(p.dec(1u32), 0);
        assert_eq!(p.dec(42u32), 41);
        assert_eq!(p.dec(0u32), u32::MAX); // Wrap around
    }

    #[test]
    fn test_add_sub() {
        let p = ScalarPrimitives;

        // add
        assert_eq!(p.add(10u32, 20), 30);
        assert_eq!(p.add(u32::MAX, 1), 0); // Overflow wraps

        // sub
        assert_eq!(p.sub(30u32, 10), 20);
        assert_eq!(p.sub(0u32, 1), u32::MAX); // Underflow wraps
    }

    #[test]
    fn test_algebraic_identities() {
        let p = ScalarPrimitives;

        for a in [0u32, 1, 42, 255, 1000, u32::MAX - 1, u32::MAX] {
            // add(a, neg(b)) == sub(a, b)
            for b in [0u32, 1, 42, 255, 1000] {
                assert_eq!(p.add(a, p.neg(b)), p.sub(a, b));
            }

            // inc is add(a, 1)
            assert_eq!(p.inc(a), p.add(a, 1));

            // dec is sub(a, 1)
            assert_eq!(p.dec(a), p.sub(a, 1));
        }
    }

    #[test]
    fn test_select() {
        let p = ScalarPrimitives;

        let a = 0xAAAA_AAAAu32;
        let b = 0x5555_5555u32;

        // All-ones mask selects a
        assert_eq!(p.select(u32::MAX, a, b), a);

        // All-zeros mask selects b
        assert_eq!(p.select(0, a, b), b);

        // Partial mask blends
        let mask = 0xFF00_FF00u32;
        let expected = (a & mask) | (b & !mask);
        assert_eq!(p.select(mask, a, b), expected);
    }

    #[test]
    fn test_array_ops() {
        let p = ScalarPrimitives;

        let a: [u32; 4] = [10, 20, 30, 40];
        let b: [u32; 4] = [1, 2, 3, 4];

        let sum = p.add(a, b);
        assert_eq!(sum, [11, 22, 33, 44]);

        let diff = p.sub(a, b);
        assert_eq!(diff, [9, 18, 27, 36]);
    }

    // =========================================================================
    // Shift/Rotate operation tests (TASK-171)
    // =========================================================================

    #[test]
    fn test_shl_u32() {
        let p = ScalarPrimitives;

        assert_eq!(p.shl(0x0000_00FFu32, 8), 0x0000_FF00);
        assert_eq!(p.shl(0x0000_0001u32, 31), 0x8000_0000);
        assert_eq!(p.shl(0xFFFF_FFFFu32, 16), 0xFFFF_0000);
        assert_eq!(p.shl(0x1234_5678u32, 0), 0x1234_5678);
    }

    #[test]
    fn test_shr_u32() {
        let p = ScalarPrimitives;

        assert_eq!(p.shr(0x0000_FF00u32, 8), 0x0000_00FF);
        assert_eq!(p.shr(0x8000_0000u32, 31), 0x0000_0001);
        assert_eq!(p.shr(0xFFFF_FFFFu32, 16), 0x0000_FFFF);
        assert_eq!(p.shr(0x1234_5678u32, 0), 0x1234_5678);
    }

    #[test]
    fn test_rotl_u32() {
        let p = ScalarPrimitives;

        // Basic rotation
        assert_eq!(p.rotl(0x8000_0001u32, 1), 0x0000_0003);
        assert_eq!(p.rotl(0x0000_00FFu32, 8), 0x0000_FF00);
        assert_eq!(p.rotl(0x8000_0000u32, 1), 0x0000_0001);

        // Full rotation
        assert_eq!(p.rotl(0x1234_5678u32, 32), 0x1234_5678);
        assert_eq!(p.rotl(0xABCD_EF01u32, 0), 0xABCD_EF01);

        // Rotation wraps bits around
        assert_eq!(p.rotl(0xFF00_0000u32, 8), 0x0000_00FF);
    }

    #[test]
    fn test_rotr_u32() {
        let p = ScalarPrimitives;

        // Basic rotation
        assert_eq!(p.rotr(0x0000_0003u32, 1), 0x8000_0001);
        assert_eq!(p.rotr(0x0000_FF00u32, 8), 0x0000_00FF);
        assert_eq!(p.rotr(0x0000_0001u32, 1), 0x8000_0000);

        // Full rotation
        assert_eq!(p.rotr(0x1234_5678u32, 32), 0x1234_5678);
        assert_eq!(p.rotr(0xABCD_EF01u32, 0), 0xABCD_EF01);

        // Rotation wraps bits around
        assert_eq!(p.rotr(0x0000_00FFu32, 8), 0xFF00_0000);
    }

    #[test]
    fn test_rotl_rotr_inverse() {
        let p = ScalarPrimitives;

        // rotl and rotr are inverses
        for val in [0u32, 1, 0xDEAD_BEEF, 0xCAFE_BABE, u32::MAX] {
            for n in [0, 1, 7, 8, 15, 16, 31] {
                let rotated = p.rotl(val, n);
                let restored = p.rotr(rotated, n);
                assert_eq!(
                    restored, val,
                    "rotl/rotr inverse failed for val={val:#x}, n={n}"
                );
            }
        }
    }

    #[test]
    fn test_shift_array_u32_4() {
        let p = ScalarPrimitives;

        let a: [u32; 4] = [0x0000_00FF, 0x0000_FF00, 0x00FF_0000, 0xFF00_0000];

        // Shift left
        let shl = p.shl(a, 8);
        assert_eq!(shl, [0x0000_FF00, 0x00FF_0000, 0xFF00_0000, 0x0000_0000]);

        // Shift right
        let shr = p.shr(a, 8);
        assert_eq!(shr, [0x0000_0000, 0x0000_00FF, 0x0000_FF00, 0x00FF_0000]);
    }

    #[test]
    fn test_rotate_array_u32_4() {
        let p = ScalarPrimitives;

        let a: [u32; 4] = [0x8000_0001, 0x0000_0003, 0xFF00_0000, 0x0000_00FF];

        // Rotate left
        let rotl = p.rotl(a, 1);
        assert_eq!(
            rotl,
            [
                0x0000_0003, // 0x8000_0001 rotl 1
                0x0000_0006, // 0x0000_0003 rotl 1
                0xFE00_0001, // 0xFF00_0000 rotl 1
                0x0000_01FE, // 0x0000_00FF rotl 1
            ]
        );

        // Rotate right
        let rotr = p.rotr(a, 1);
        assert_eq!(
            rotr,
            [
                0xC000_0000, // 0x8000_0001 rotr 1
                0x8000_0001, // 0x0000_0003 rotr 1
                0x7F80_0000, // 0xFF00_0000 rotr 1
                0x8000_007F, // 0x0000_00FF rotr 1
            ]
        );
    }

    #[test]
    fn test_rotate_array_u32_8() {
        let p = ScalarPrimitives;

        let a: [u32; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

        // Rotate left by 1
        let rotl = p.rotl(a, 1);
        assert_eq!(rotl, [2, 4, 8, 16, 32, 64, 128, 256]);

        // Rotate right by 1
        let rotr = p.rotr(a, 1);
        assert_eq!(rotr, [0x8000_0000, 1, 2, 4, 8, 16, 32, 64]);
    }

    #[test]
    fn test_shl_shr_consistency() {
        let p = ScalarPrimitives;

        // shl and shr are consistent with shl_logical and shr_logical
        for val in [0u32, 1, 0xDEAD_BEEF, u32::MAX] {
            for n in [0, 1, 8, 16, 31] {
                assert_eq!(p.shl(val, n), p.shl_logical(val, n));
                assert_eq!(p.shr(val, n), p.shr_logical(val, n));
            }
        }
    }

    #[test]
    fn test_shift_rotate_u64() {
        let p = ScalarPrimitives;

        // u64 shift
        assert_eq!(p.shl(0x0000_0000_0000_00FFu64, 32), 0x0000_00FF_0000_0000);
        assert_eq!(p.shr(0x0000_00FF_0000_0000u64, 32), 0x0000_0000_0000_00FF);

        // u64 rotate
        assert_eq!(p.rotl(0x8000_0000_0000_0001u64, 1), 0x0000_0000_0000_0003);
        assert_eq!(p.rotr(0x0000_0000_0000_0003u64, 1), 0x8000_0000_0000_0001);
    }

    // =========================================================================
    // Comparison operation tests (TASK-172)
    // =========================================================================

    #[test]
    fn test_eq_u32() {
        let p = ScalarPrimitives;

        // Equal values return all-ones
        assert_eq!(p.eq(42u32, 42), u32::MAX);
        assert_eq!(p.eq(0u32, 0), u32::MAX);
        assert_eq!(p.eq(u32::MAX, u32::MAX), u32::MAX);

        // Not equal values return all-zeros
        assert_eq!(p.eq(42u32, 43), 0);
        assert_eq!(p.eq(0u32, 1), 0);
        assert_eq!(p.eq(u32::MAX, 0), 0);
    }

    #[test]
    fn test_ne_u32() {
        let p = ScalarPrimitives;

        // Not equal values return all-ones
        assert_eq!(p.ne(42u32, 43), u32::MAX);
        assert_eq!(p.ne(0u32, 1), u32::MAX);
        assert_eq!(p.ne(u32::MAX, 0), u32::MAX);

        // Equal values return all-zeros
        assert_eq!(p.ne(42u32, 42), 0);
        assert_eq!(p.ne(0u32, 0), 0);
        assert_eq!(p.ne(u32::MAX, u32::MAX), 0);
    }

    #[test]
    fn test_lt_u32() {
        let p = ScalarPrimitives;

        // a < b returns all-ones
        assert_eq!(p.lt(10u32, 20), u32::MAX);
        assert_eq!(p.lt(0u32, 1), u32::MAX);
        assert_eq!(p.lt(0u32, u32::MAX), u32::MAX);

        // a >= b returns all-zeros
        assert_eq!(p.lt(20u32, 10), 0);
        assert_eq!(p.lt(10u32, 10), 0); // Equal
        assert_eq!(p.lt(u32::MAX, 0), 0);
    }

    #[test]
    fn test_le_u32() {
        let p = ScalarPrimitives;

        // a <= b returns all-ones
        assert_eq!(p.le(10u32, 20), u32::MAX);
        assert_eq!(p.le(10u32, 10), u32::MAX); // Equal
        assert_eq!(p.le(0u32, 0), u32::MAX);

        // a > b returns all-zeros
        assert_eq!(p.le(20u32, 10), 0);
        assert_eq!(p.le(u32::MAX, 0), 0);
    }

    #[test]
    fn test_gt_u32() {
        let p = ScalarPrimitives;

        // a > b returns all-ones
        assert_eq!(p.gt(20u32, 10), u32::MAX);
        assert_eq!(p.gt(1u32, 0), u32::MAX);
        assert_eq!(p.gt(u32::MAX, 0), u32::MAX);

        // a <= b returns all-zeros
        assert_eq!(p.gt(10u32, 20), 0);
        assert_eq!(p.gt(10u32, 10), 0); // Equal
        assert_eq!(p.gt(0u32, u32::MAX), 0);
    }

    #[test]
    fn test_ge_u32() {
        let p = ScalarPrimitives;

        // a >= b returns all-ones
        assert_eq!(p.ge(20u32, 10), u32::MAX);
        assert_eq!(p.ge(10u32, 10), u32::MAX); // Equal
        assert_eq!(p.ge(u32::MAX, u32::MAX), u32::MAX);

        // a < b returns all-zeros
        assert_eq!(p.ge(10u32, 20), 0);
        assert_eq!(p.ge(0u32, u32::MAX), 0);
    }

    #[test]
    fn test_comparison_consistency() {
        let p = ScalarPrimitives;

        // Test that comparison operations are logically consistent
        for a in [0u32, 1, 10, 100, 1000, u32::MAX - 1, u32::MAX] {
            for b in [0u32, 1, 10, 100, 1000, u32::MAX - 1, u32::MAX] {
                // eq and ne are complements
                assert_eq!(
                    p.eq(a, b),
                    p.bnot(p.ne(a, b)),
                    "eq/ne complement failed for a={a}, b={b}"
                );

                // lt and ge are complements
                assert_eq!(
                    p.lt(a, b),
                    p.bnot(p.ge(a, b)),
                    "lt/ge complement failed for a={a}, b={b}"
                );

                // gt and le are complements
                assert_eq!(
                    p.gt(a, b),
                    p.bnot(p.le(a, b)),
                    "gt/le complement failed for a={a}, b={b}"
                );

                // lt(a, b) == gt(b, a)
                assert_eq!(
                    p.lt(a, b),
                    p.gt(b, a),
                    "lt/gt symmetry failed for a={a}, b={b}"
                );

                // le(a, b) == ge(b, a)
                assert_eq!(
                    p.le(a, b),
                    p.ge(b, a),
                    "le/ge symmetry failed for a={a}, b={b}"
                );
            }
        }
    }

    #[test]
    fn test_comparison_array_u32_4() {
        let p = ScalarPrimitives;

        let a: [u32; 4] = [10, 20, 30, 40];
        let b: [u32; 4] = [15, 20, 25, 50];

        // Element-wise eq
        let eq_result = p.eq(a, b);
        assert_eq!(eq_result, [0, u32::MAX, 0, 0]);

        // Element-wise ne
        let ne_result = p.ne(a, b);
        assert_eq!(ne_result, [u32::MAX, 0, u32::MAX, u32::MAX]);

        // Element-wise lt
        let lt_result = p.lt(a, b);
        assert_eq!(lt_result, [u32::MAX, 0, 0, u32::MAX]);

        // Element-wise le
        let le_result = p.le(a, b);
        assert_eq!(le_result, [u32::MAX, u32::MAX, 0, u32::MAX]);

        // Element-wise gt
        let gt_result = p.gt(a, b);
        assert_eq!(gt_result, [0, 0, u32::MAX, 0]);

        // Element-wise ge
        let ge_result = p.ge(a, b);
        assert_eq!(ge_result, [0, u32::MAX, u32::MAX, 0]);
    }

    #[test]
    fn test_comparison_array_u32_8() {
        let p = ScalarPrimitives;

        let a: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let b: [u32; 8] = [1, 3, 2, 4, 6, 5, 7, 9];

        // Element-wise eq
        let eq_result = p.eq(a, b);
        assert_eq!(eq_result, [u32::MAX, 0, 0, u32::MAX, 0, 0, u32::MAX, 0]);

        // Element-wise lt
        let lt_result = p.lt(a, b);
        assert_eq!(lt_result, [0, u32::MAX, 0, 0, u32::MAX, 0, 0, u32::MAX]);
    }

    #[test]
    fn test_comparison_select_integration() {
        let p = ScalarPrimitives;

        let a = 10u32;
        let b = 20u32;

        // Use comparison result as select mask to compute min
        let mask = p.lt(a, b);
        let min_val = p.select(mask, a, b);
        assert_eq!(min_val, 10); // a is smaller

        // Verify against min_unsigned
        assert_eq!(min_val, p.min_unsigned(a, b));

        // Use comparison result as select mask to compute max
        let mask = p.gt(a, b);
        let max_val = p.select(mask, a, b);
        assert_eq!(max_val, 20); // b is larger

        // Verify against max_unsigned
        assert_eq!(max_val, p.max_unsigned(a, b));
    }
}
