//! GF(2) polynomial arithmetic for the UOR Invariance Frame.
//!
//! Polynomials over GF(2) have coefficients in {0, 1}. Addition is XOR
//! (carryless), multiplication is shift-and-XOR. Each `u8` encodes a
//! polynomial where bit `i` is the coefficient of x^i.
//!
//! # Encoding
//!
//! ```text
//! 0b0000_0111 = 7  → x² + x + 1
//! 0b0000_1011 = 11 → x³ + x + 1
//! 0b0001_1001 = 25 → x⁴ + x³ + 1
//! ```
//!
//! # Irreducible Counts (Möbius formula)
//!
//! | Degree | Count | Cumulative |
//! |--------|-------|------------|
//! | 1      | 2     | 2          |
//! | 2      | 1     | 3          |
//! | 3      | 2     | 5          |
//! | 4      | 3     | 8          |
//! | 5      | 6     | 14         |
//! | 6      | 9     | 23         |
//! | 7      | 18    | 41         |
//!
//! # Example
//!
//! ```
//! use uor::frame::gf2;
//!
//! // x² + x + 1 (datum 7) is irreducible over GF(2)
//! assert!(gf2::is_irreducible(7));
//!
//! // x² + 1 (datum 5) factors as (x+1)² over GF(2)
//! assert!(!gf2::is_irreducible(5));
//!
//! // Carryless multiplication: (x+1)(x+1) = x²+1 in GF(2)
//! assert_eq!(gf2::mul(0b11, 0b11), 0b101);
//! ```

/// Degree of a polynomial (position of highest set bit).
///
/// Returns `None` for the zero polynomial.
///
/// # Examples
///
/// ```
/// use uor::frame::gf2;
///
/// assert_eq!(gf2::degree_u16(0b111), Some(2));  // x² + x + 1
/// assert_eq!(gf2::degree_u16(0b1000), Some(3)); // x³
/// assert_eq!(gf2::degree_u16(0), None);
/// ```
#[inline]
pub const fn degree_u16(p: u16) -> Option<u8> {
    if p == 0 {
        None
    } else {
        Some((15 - p.leading_zeros()) as u8)
    }
}

/// Degree of an 8-bit polynomial.
#[inline]
pub const fn degree(p: u8) -> Option<u8> {
    degree_u16(p as u16)
}

/// Carryless multiplication of two polynomials over GF(2).
///
/// Result can be up to degree 14, returned as `u16`.
///
/// # Examples
///
/// ```
/// use uor::frame::gf2;
///
/// // (x+1) * (x+1) = x² + 1  (note: 2x = 0 in GF(2))
/// assert_eq!(gf2::mul(0b11, 0b11), 0b101);
///
/// // x * (x² + x + 1) = x³ + x² + x
/// assert_eq!(gf2::mul(0b10, 0b111), 0b1110);
/// ```
#[inline]
pub const fn mul(a: u8, b: u8) -> u16 {
    let a16 = a as u16;
    let mut result: u16 = 0;
    let mut b_rem = b;
    let mut shift = 0u32;
    while b_rem != 0 {
        if b_rem & 1 != 0 {
            result ^= a16 << shift;
        }
        b_rem >>= 1;
        shift += 1;
    }
    result
}

/// Polynomial reduction: `a mod m` over GF(2).
///
/// Reduces `a` by repeatedly XORing with aligned modulus `m`
/// until `degree(a) < degree(m)`.
///
/// # Examples
///
/// ```
/// use uor::frame::gf2;
///
/// // (x³ + x + 1) mod (x² + x + 1) = x
/// assert_eq!(gf2::modulo(0b1011, 0b111), 0b10);
/// ```
#[inline]
pub const fn modulo(a: u16, m: u16) -> u8 {
    let deg_m = match degree_u16(m) {
        Some(d) => d,
        None => return a as u8,
    };
    let mut r = a;
    while let Some(deg_r) = degree_u16(r) {
        if deg_r < deg_m {
            break;
        }
        r ^= m << (deg_r - deg_m);
    }
    r as u8
}

/// Polynomial long division quotient: `a / b` over GF(2).
///
/// Returns the quotient; the remainder is `modulo(mul(a_original), b)`.
///
/// # Panics
///
/// Panics if `b` is zero.
///
/// # Examples
///
/// ```
/// use uor::frame::gf2;
///
/// // (x² + 1) / (x + 1) = (x + 1) with remainder 0
/// assert_eq!(gf2::div(0b101, 0b11), 0b11);
/// ```
#[inline]
pub const fn div(a: u8, b: u8) -> u8 {
    assert!(b != 0, "division by zero polynomial");
    let deg_b = match degree(b) {
        Some(d) => d,
        None => unreachable!(),
    };
    let b16 = b as u16;
    let mut rem = a as u16;
    let mut quot: u16 = 0;
    while let Some(deg_r) = degree_u16(rem) {
        if deg_r < deg_b {
            break;
        }
        let shift = deg_r - deg_b;
        quot ^= 1u16 << shift;
        rem ^= b16 << shift;
    }
    quot as u8
}

/// Check if `divisor` divides `value` over GF(2).
///
/// Returns `Some(quotient)` if divisible, `None` otherwise.
///
/// # Examples
///
/// ```
/// use uor::frame::gf2;
///
/// // x² + 1 = (x+1)², so (x+1) divides (x²+1)
/// assert_eq!(gf2::trial_divide(0b101, 0b11), Some(0b11));
///
/// // x² + x + 1 is irreducible, (x+1) does not divide it
/// assert_eq!(gf2::trial_divide(0b111, 0b11), None);
/// ```
#[inline]
pub const fn trial_divide(value: u8, divisor: u8) -> Option<u8> {
    if divisor == 0 {
        return None;
    }
    if modulo(value as u16, divisor as u16) == 0 {
        Some(div(value, divisor))
    } else {
        None
    }
}

/// Test if a polynomial is irreducible over GF(2).
///
/// Uses exhaustive trial division by all polynomials of degree
/// 1 through `degree(f)/2`.
///
/// # Examples
///
/// ```
/// use uor::frame::gf2;
///
/// assert!(gf2::is_irreducible(0b10));   // x (degree 1)
/// assert!(gf2::is_irreducible(0b11));   // x+1 (degree 1)
/// assert!(gf2::is_irreducible(0b111));  // x²+x+1 (degree 2)
/// assert!(!gf2::is_irreducible(0b101)); // x²+1 = (x+1)²
/// assert!(gf2::is_irreducible(0b1011)); // x³+x+1 (degree 3)
/// ```
pub const fn is_irreducible(f: u8) -> bool {
    let f16 = f as u16;
    let deg = match degree_u16(f16) {
        Some(d) => d,
        None => return false,
    };
    if deg == 0 {
        return false; // constants are units
    }
    if deg == 1 {
        return true; // degree-1 always irreducible
    }
    let max_div_deg = deg / 2;
    // Try all polynomials of degree 1..=max_div_deg
    let mut d: u16 = 2; // smallest degree-1: x = 0b10
    let limit = 1u16 << (max_div_deg + 1);
    while d < limit {
        // Only test polynomials that actually have degree >= 1
        if d >= 2 && modulo(f16, d) == 0 {
            return false;
        }
        d += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degree_examples() {
        assert_eq!(degree(0), None);
        assert_eq!(degree(1), Some(0)); // constant 1
        assert_eq!(degree(0b10), Some(1)); // x
        assert_eq!(degree(0b111), Some(2)); // x²+x+1
        assert_eq!(degree(0b1000_0000), Some(7)); // x⁷
    }

    #[test]
    fn mul_basic() {
        // 1 * anything = anything
        assert_eq!(mul(1, 0), 0);
        assert_eq!(mul(1, 7), 7);
        assert_eq!(mul(0, 255), 0);
        // (x+1)*(x+1) = x² + 2x + 1 = x² + 1 (2x=0 in GF(2))
        assert_eq!(mul(0b11, 0b11), 0b101);
        // x * x = x²
        assert_eq!(mul(0b10, 0b10), 0b100);
    }

    #[test]
    fn modulo_basic() {
        // 0 mod anything = 0
        assert_eq!(modulo(0, 0b111), 0);
        // x mod (x+1): degree(x) = degree(x+1) = 1, so x XOR (x+1) = 1
        assert_eq!(modulo(0b10, 0b11), 1);
    }

    #[test]
    fn div_basic() {
        // (x²+1) / (x+1) = (x+1), remainder 0
        assert_eq!(div(0b101, 0b11), 0b11);
        // Verify: (x+1)*(x+1) = x²+1
        assert_eq!(mul(0b11, 0b11), 0b101);
    }

    #[test]
    fn trial_divide_examples() {
        // x²+1 divisible by x+1
        assert_eq!(trial_divide(0b101, 0b11), Some(0b11));
        // x²+x+1 NOT divisible by x+1
        assert_eq!(trial_divide(0b111, 0b11), None);
        // x²+x+1 NOT divisible by x
        assert_eq!(trial_divide(0b111, 0b10), None);
    }

    #[test]
    fn irreducible_degree_1() {
        // Degree 1: x (0b10) and x+1 (0b11) — both irreducible
        assert!(is_irreducible(0b10));
        assert!(is_irreducible(0b11));
        // Count: 2
    }

    #[test]
    fn irreducible_degree_2() {
        // Degree 2: only x²+x+1 (0b111) is irreducible
        assert!(is_irreducible(0b111)); // x²+x+1
        assert!(!is_irreducible(0b101)); // x²+1 = (x+1)²
        assert!(!is_irreducible(0b110)); // x²+x = x(x+1)
        assert!(!is_irreducible(0b100)); // x² = x·x
                                         // Count: 1
    }

    #[test]
    fn irreducible_degree_3() {
        // Degree 3: x³+x+1 (0b1011) and x³+x²+1 (0b1101)
        assert!(is_irreducible(0b1011)); // x³+x+1
        assert!(is_irreducible(0b1101)); // x³+x²+1
        assert!(!is_irreducible(0b1001)); // x³+1 = (x+1)(x²+x+1)
        assert!(!is_irreducible(0b1111)); // x³+x²+x+1 = (x+1)(x²+1) = (x+1)³
                                          // Count: 2
    }

    #[test]
    fn irreducible_constants_not_irreducible() {
        assert!(!is_irreducible(0)); // zero
        assert!(!is_irreducible(1)); // unit
    }

    /// Verify counts match the Möbius formula for degrees 1-7.
    #[test]
    fn mobius_counts() {
        let expected = [2, 1, 2, 3, 6, 9, 18]; // degrees 1-7
        let mut counts = [0u32; 7];
        for v in 2u16..256 {
            if is_irreducible(v as u8) {
                let deg = degree(v as u8).unwrap() as usize;
                if (1..=7).contains(&deg) {
                    counts[deg - 1] += 1;
                }
            }
        }
        assert_eq!(counts, expected, "Möbius formula mismatch: got {counts:?}");
    }

    /// Total irreducible count at Q0 should be 41.
    #[test]
    fn total_irreducible_count() {
        let count = (2u16..256).filter(|&v| is_irreducible(v as u8)).count();
        assert_eq!(count, 41);
    }

    /// Verify the first 10 irreducible polynomials from the SIH spec.
    #[test]
    fn emanation_table() {
        let expected = [2, 3, 7, 11, 13, 19, 25, 31, 37, 41];
        let irreducibles: Vec<u8> = (2u16..256)
            .filter(|&v| is_irreducible(v as u8))
            .map(|v| v as u8)
            .collect();
        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(
                irreducibles[i],
                e,
                "Emanation E({}) = {}, expected {}",
                i + 1,
                irreducibles[i],
                e
            );
        }
    }

    /// Datum 5 encodes x²+1 which factors as (x+1)² over GF(2).
    #[test]
    fn datum_5_reducible() {
        assert!(!is_irreducible(5)); // 0b101 = x²+1 = (x+1)²
        assert_eq!(trial_divide(5, 3), Some(3)); // (x²+1)/(x+1) = (x+1)
    }

    /// Datum 25 encodes x⁴+x³+1 which is irreducible over GF(2).
    #[test]
    fn datum_25_irreducible() {
        assert!(is_irreducible(25)); // 0b11001 = x⁴+x³+1
    }
}
