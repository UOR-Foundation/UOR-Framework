//! Additional ring operations for taxons.
//!
//! Taxons form a ring under modular arithmetic (mod 256).
//! This module provides extended ring operations beyond the
//! basic ones defined on Taxon itself.

use super::taxon::Taxon;

/// Multiplication in the ring (mod 256).
///
/// Note: This is standard byte multiplication with overflow wrapping.
///
/// # Example
///
/// ```
/// use uor::{Taxon, ring::mul};
///
/// assert_eq!(mul(Taxon::new(16), Taxon::new(16)), Taxon::new(0)); // 256 mod 256
/// assert_eq!(mul(Taxon::new(3), Taxon::new(5)), Taxon::new(15));
/// ```
#[inline]
#[must_use]
pub const fn mul(a: Taxon, b: Taxon) -> Taxon {
    Taxon::new(a.value().wrapping_mul(b.value()))
}

/// Division in the ring when divisor is coprime to 256.
///
/// Returns `None` if `b` is not coprime to 256 (i.e., if `b` is even).
/// Only odd values have multiplicative inverses mod 256.
///
/// # Example
///
/// ```
/// use uor::{Taxon, ring::div};
///
/// // 3 × 171 ≡ 1 (mod 256), so 15 / 3 = 5
/// assert_eq!(div(Taxon::new(15), Taxon::new(3)), Some(Taxon::new(5)));
/// // Cannot divide by even numbers
/// assert_eq!(div(Taxon::new(10), Taxon::new(2)), None);
/// ```
#[inline]
#[must_use]
pub const fn div(a: Taxon, b: Taxon) -> Option<Taxon> {
    if let Some(inv) = mul_inverse(b) {
        Some(mul(a, inv))
    } else {
        None
    }
}

/// Multiplicative inverse mod 256 (if exists).
///
/// Only odd values (coprime to 256) have multiplicative inverses.
///
/// # Example
///
/// ```
/// use uor::{Taxon, ring::mul_inverse};
///
/// // 3 × 171 ≡ 1 (mod 256)
/// assert_eq!(mul_inverse(Taxon::new(3)), Some(Taxon::new(171)));
/// // Even numbers have no inverse
/// assert_eq!(mul_inverse(Taxon::new(2)), None);
/// ```
#[inline]
#[must_use]
pub const fn mul_inverse(a: Taxon) -> Option<Taxon> {
    let v = a.value();

    // Only odd numbers are coprime to 256
    if v.is_multiple_of(2) {
        return None;
    }

    // Extended Euclidean algorithm for mod 256
    // For odd n, n^(-1) mod 256 can be computed via:
    // inv = n * (2 - n*n) * (2 - n*n*n*n) ... (Newton's method)
    // Or use lookup table for small modulus

    // Newton's method: x_{k+1} = x_k * (2 - n * x_k) mod 256
    // Starting with x_0 = n (works because n is odd)
    let mut x = v;

    // 3 iterations sufficient for mod 256
    x = x.wrapping_mul(2u8.wrapping_sub(v.wrapping_mul(x)));
    x = x.wrapping_mul(2u8.wrapping_sub(v.wrapping_mul(x)));
    x = x.wrapping_mul(2u8.wrapping_sub(v.wrapping_mul(x)));

    Some(Taxon::new(x))
}

/// Power operation (a^n mod 256).
///
/// # Example
///
/// ```
/// use uor::{Taxon, ring::pow};
///
/// assert_eq!(pow(Taxon::new(2), 4), Taxon::new(16));
/// assert_eq!(pow(Taxon::new(2), 8), Taxon::new(0)); // 256 mod 256
/// assert_eq!(pow(Taxon::new(3), 0), Taxon::new(1));
/// ```
#[inline]
#[must_use]
pub const fn pow(base: Taxon, exp: u8) -> Taxon {
    if exp == 0 {
        return Taxon::ONE;
    }

    let mut result: u8 = 1;
    let mut b = base.value();
    let mut e = exp;

    while e > 0 {
        if e & 1 == 1 {
            result = result.wrapping_mul(b);
        }
        b = b.wrapping_mul(b);
        e >>= 1;
    }

    Taxon::new(result)
}

/// Left rotation (shift left with wrap-around).
///
/// # Example
///
/// ```
/// use uor::{Taxon, ring::rotate_left};
///
/// assert_eq!(rotate_left(Taxon::new(1), 4), Taxon::new(16));
/// assert_eq!(rotate_left(Taxon::new(128), 1), Taxon::new(1));
/// ```
#[inline]
#[must_use]
pub const fn rotate_left(a: Taxon, n: u32) -> Taxon {
    Taxon::new(a.value().rotate_left(n))
}

/// Right rotation (shift right with wrap-around).
///
/// # Example
///
/// ```
/// use uor::{Taxon, ring::rotate_right};
///
/// assert_eq!(rotate_right(Taxon::new(16), 4), Taxon::new(1));
/// assert_eq!(rotate_right(Taxon::new(1), 1), Taxon::new(128));
/// ```
#[inline]
#[must_use]
pub const fn rotate_right(a: Taxon, n: u32) -> Taxon {
    Taxon::new(a.value().rotate_right(n))
}

/// Shift left (with zero fill).
///
/// # Example
///
/// ```
/// use uor::{Taxon, ring::shl};
///
/// assert_eq!(shl(Taxon::new(1), 4), Taxon::new(16));
/// assert_eq!(shl(Taxon::new(16), 4), Taxon::new(0));
/// ```
#[inline]
#[must_use]
pub const fn shl(a: Taxon, n: u32) -> Taxon {
    if n >= 8 {
        Taxon::MIN
    } else {
        Taxon::new(a.value() << n)
    }
}

/// Shift right (with zero fill).
///
/// # Example
///
/// ```
/// use uor::{Taxon, ring::shr};
///
/// assert_eq!(shr(Taxon::new(16), 4), Taxon::new(1));
/// assert_eq!(shr(Taxon::new(1), 4), Taxon::new(0));
/// ```
#[inline]
#[must_use]
pub const fn shr(a: Taxon, n: u32) -> Taxon {
    if n >= 8 {
        Taxon::MIN
    } else {
        Taxon::new(a.value() >> n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        assert_eq!(mul(Taxon::new(3), Taxon::new(5)), Taxon::new(15));
        assert_eq!(mul(Taxon::new(16), Taxon::new(16)), Taxon::new(0));
        assert_eq!(mul(Taxon::new(0), Taxon::new(100)), Taxon::new(0));
    }

    #[test]
    fn test_mul_inverse() {
        // Test that odd numbers have inverses
        for i in (1..=255u8).step_by(2) {
            let t = Taxon::new(i);
            let inv = mul_inverse(t);
            assert!(inv.is_some(), "Odd number {} should have inverse", i);
            let inv = inv.unwrap();
            assert_eq!(
                mul(t, inv),
                Taxon::ONE,
                "{} * {} should be 1",
                i,
                inv.value()
            );
        }

        // Test that even numbers don't have inverses
        for i in (0..=254u8).step_by(2) {
            assert!(
                mul_inverse(Taxon::new(i)).is_none(),
                "Even number {} should not have inverse",
                i
            );
        }
    }

    #[test]
    fn test_div() {
        // 15 / 3 = 5 (since 3 × 5 = 15)
        assert_eq!(div(Taxon::new(15), Taxon::new(3)), Some(Taxon::new(5)));

        // Division by even numbers fails
        assert_eq!(div(Taxon::new(10), Taxon::new(2)), None);
    }

    #[test]
    fn test_pow() {
        assert_eq!(pow(Taxon::new(2), 0), Taxon::ONE);
        assert_eq!(pow(Taxon::new(2), 1), Taxon::new(2));
        assert_eq!(pow(Taxon::new(2), 4), Taxon::new(16));
        assert_eq!(pow(Taxon::new(2), 8), Taxon::new(0)); // 256 mod 256
        assert_eq!(pow(Taxon::new(3), 4), Taxon::new(81));
    }

    #[test]
    fn test_rotate() {
        assert_eq!(rotate_left(Taxon::new(1), 4), Taxon::new(16));
        assert_eq!(rotate_right(Taxon::new(16), 4), Taxon::new(1));
        assert_eq!(rotate_left(Taxon::new(128), 1), Taxon::new(1));
        assert_eq!(rotate_right(Taxon::new(1), 1), Taxon::new(128));
    }

    #[test]
    fn test_shift() {
        assert_eq!(shl(Taxon::new(1), 4), Taxon::new(16));
        assert_eq!(shr(Taxon::new(16), 4), Taxon::new(1));
        assert_eq!(shl(Taxon::new(1), 8), Taxon::new(0));
        assert_eq!(shr(Taxon::new(128), 8), Taxon::new(0));
    }
}
