//! GF(5) polynomial arithmetic for the UOR Invariance Frame.
//!
//! Polynomials over GF(5) have coefficients in {0, 1, 2, 3, 4}. Each `u8`
//! encodes a polynomial using base-5 representation: byte value `v` has
//! coefficient `(v / 5^i) % 5` for term x^i.
//!
//! # Encoding
//!
//! ```text
//! 6  = 1·5 + 1     → 1 + x       (degree 1)
//! 30 = 1·25 + 1·5  → x + x²      (degree 2)
//! 125 = 1·125      → x³           (degree 3)
//! ```
//!
//! Since 5³ = 125 ≤ 255 < 625 = 5⁴, a `u8` fully represents degrees 0–2
//! and partially represents degree 3 (values 125–255).
//!
//! # Irreducible Counts (Möbius formula)
//!
//! Total irreducibles = (p-1) × monic count.
//!
//! | Degree | Monic | Total | Cumulative |
//! |--------|-------|-------|------------|
//! | 1      | 5     | 20    | 20         |
//! | 2      | 10    | 40    | 60         |
//! | 3 (partial) | ≤40 | ≤160 | — |
//!
//! # Example
//!
//! ```
//! use uor::frame::gf5;
//!
//! // x + 1 (value 6 = 1·5 + 1) is irreducible over GF(5)
//! assert!(gf5::is_irreducible(6));
//!
//! // x² (value 25 = 1·25) is reducible: x · x
//! assert!(!gf5::is_irreducible(25));
//! ```

/// Maximum number of base-5 digits in a u8 (5^3 = 125, 5^4 = 625 > 255).
const MAX_COEFFS_U8: usize = 4;

/// Maximum number of base-5 digits in a u16 (5^6 = 15625 ≤ 65535).
const MAX_COEFFS_U16: usize = 7;

/// Powers of 5 up to 5^6.
const POW5: [u16; 7] = [1, 5, 25, 125, 625, 3125, 15625];

/// Extract the coefficient of x^i from a base-5 encoded polynomial.
#[inline]
pub const fn coeff(p: u16, i: usize) -> u8 {
    if i >= MAX_COEFFS_U16 {
        return 0;
    }
    ((p / POW5[i]) % 5) as u8
}

/// Degree of a base-5 encoded polynomial.
///
/// Returns `None` for the zero polynomial.
///
/// # Examples
///
/// ```
/// use uor::frame::gf5;
///
/// assert_eq!(gf5::degree(6), Some(1));   // x + 1
/// assert_eq!(gf5::degree(25), Some(2));  // x²
/// assert_eq!(gf5::degree(0), None);      // zero polynomial
/// ```
pub const fn degree(p: u16) -> Option<u8> {
    let mut d = MAX_COEFFS_U16;
    loop {
        if d == 0 {
            if coeff(p, 0) != 0 {
                return Some(0);
            }
            return None;
        }
        d -= 1;
        if coeff(p, d) != 0 {
            return Some(d as u8);
        }
    }
}

/// Multiplicative inverse in GF(5).
///
/// | a | inv(a) |
/// |---|--------|
/// | 1 | 1      |
/// | 2 | 3      |
/// | 3 | 2      |
/// | 4 | 4      |
const fn inv5(a: u8) -> u8 {
    match a {
        1 => 1,
        2 => 3,
        3 => 2,
        4 => 4,
        _ => 0, // undefined for 0
    }
}

/// Polynomial multiplication over GF(5).
///
/// Coefficients are computed mod 5. Result returned as `u16`.
///
/// # Examples
///
/// ```
/// use uor::frame::gf5;
///
/// // (x + 1) * (x + 1) = x² + 2x + 1
/// // Encode: x+1 = 1·5 + 1 = 6
/// // Result: 1 + 2·5 + 1·25 = 1 + 10 + 25 = 36
/// assert_eq!(gf5::mul(6, 6), 36);
/// ```
pub const fn mul(a: u8, b: u8) -> u16 {
    let mut ca = [0u8; MAX_COEFFS_U8];
    let mut i = 0;
    while i < MAX_COEFFS_U8 {
        ca[i] = coeff(a as u16, i);
        i += 1;
    }
    let mut cb = [0u8; MAX_COEFFS_U8];
    i = 0;
    while i < MAX_COEFFS_U8 {
        cb[i] = coeff(b as u16, i);
        i += 1;
    }
    let mut result = [0u8; MAX_COEFFS_U16];
    i = 0;
    while i < MAX_COEFFS_U8 {
        if ca[i] != 0 {
            let mut j = 0;
            while j < MAX_COEFFS_U8 {
                if cb[j] != 0 {
                    result[i + j] = (result[i + j] + ca[i] * cb[j]) % 5;
                }
                j += 1;
            }
        }
        i += 1;
    }
    let mut encoded: u16 = 0;
    i = 0;
    while i < MAX_COEFFS_U16 {
        encoded += result[i] as u16 * POW5[i];
        i += 1;
    }
    encoded
}

/// Set the coefficient of x^i in a base-5 encoded polynomial.
const fn set_coeff(p: u16, i: usize, c: u8) -> u16 {
    let old_c = coeff(p, i);
    p - old_c as u16 * POW5[i] + c as u16 * POW5[i]
}

/// Polynomial reduction: `a mod m` over GF(5).
pub const fn modulo(a: u16, m: u16) -> u8 {
    let deg_m = match degree(m) {
        Some(d) => d,
        None => return a as u8,
    };
    let lead_m = coeff(m, deg_m as usize);
    let inv_lead = inv5(lead_m);

    let mut r = a;
    loop {
        let deg_r = match degree(r) {
            Some(d) => d,
            None => return 0,
        };
        if deg_r < deg_m {
            return r as u8;
        }
        let lead_r = coeff(r, deg_r as usize);
        let factor = (lead_r * inv_lead) % 5;
        let shift = (deg_r - deg_m) as usize;
        let mut i = 0;
        while i < MAX_COEFFS_U8 {
            let mc = coeff(m, i);
            if mc != 0 {
                let target = i + shift;
                if target < MAX_COEFFS_U16 {
                    let cur = coeff(r, target);
                    let sub = (factor * mc) % 5;
                    let new_c = (cur + 5 - sub) % 5;
                    r = set_coeff(r, target, new_c);
                }
            }
            i += 1;
        }
    }
}

/// Polynomial long division quotient: `a / b` over GF(5).
const fn div(a: u8, b: u8) -> u8 {
    let deg_b = match degree(b as u16) {
        Some(d) => d,
        None => return 0,
    };
    let lead_b = coeff(b as u16, deg_b as usize);
    let inv_lead = inv5(lead_b);

    let mut rem = a as u16;
    let mut quot: u16 = 0;
    while let Some(deg_r) = degree(rem) {
        if deg_r < deg_b {
            break;
        }
        let lead_r = coeff(rem, deg_r as usize);
        let factor = (lead_r * inv_lead) % 5;
        let shift = (deg_r - deg_b) as usize;
        quot = set_coeff(quot, shift, (coeff(quot, shift) + factor) % 5);
        let mut i = 0;
        while i < MAX_COEFFS_U8 {
            let bc = coeff(b as u16, i);
            if bc != 0 {
                let target = i + shift;
                if target < MAX_COEFFS_U16 {
                    let cur = coeff(rem, target);
                    let sub = (factor * bc) % 5;
                    rem = set_coeff(rem, target, (cur + 5 - sub) % 5);
                }
            }
            i += 1;
        }
    }
    quot as u8
}

/// Check if `divisor` divides `value` over GF(5).
///
/// Returns `Some(quotient)` if divisible, `None` otherwise.
pub const fn trial_divide(value: u8, divisor: u8) -> Option<u8> {
    if divisor < 2 {
        return None;
    }
    let r = modulo(value as u16, divisor as u16);
    if r == 0 {
        Some(div(value, divisor))
    } else {
        None
    }
}

/// Test if a polynomial (base-5 encoded as u8) is irreducible over GF(5).
///
/// # Examples
///
/// ```
/// use uor::frame::gf5;
///
/// assert!(gf5::is_irreducible(5));   // x (degree 1)
/// assert!(gf5::is_irreducible(6));   // x + 1 (degree 1)
/// assert!(!gf5::is_irreducible(25)); // x² = x · x (reducible)
/// ```
pub const fn is_irreducible(f: u8) -> bool {
    let f16 = f as u16;
    let deg = match degree(f16) {
        Some(d) => d,
        None => return false,
    };
    if deg == 0 {
        return false; // constants are units
    }
    if deg == 1 {
        return true;
    }
    let max_div_deg = deg / 2;
    let mut d = 1u8;
    while d <= max_div_deg {
        let lo = POW5[d as usize];
        let hi = POW5[(d + 1) as usize];
        let mut candidate = lo;
        while candidate < hi {
            if modulo(f16, candidate) == 0 {
                return false;
            }
            candidate += 1;
        }
        d += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coeff_extraction() {
        // 6 = 1·5 + 1 → coeffs [1, 1] → 1 + x
        assert_eq!(coeff(6, 0), 1);
        assert_eq!(coeff(6, 1), 1);
        assert_eq!(coeff(6, 2), 0);
    }

    #[test]
    fn degree_examples() {
        assert_eq!(degree(0), None);
        assert_eq!(degree(1), Some(0)); // constant 1
        assert_eq!(degree(5), Some(1)); // x
        assert_eq!(degree(6), Some(1)); // x + 1
        assert_eq!(degree(25), Some(2)); // x²
        assert_eq!(degree(125), Some(3)); // x³
    }

    #[test]
    fn inv5_table() {
        assert_eq!(inv5(1) % 5, 1);
        assert_eq!((2 * inv5(2)) % 5, 1);
        assert_eq!((3 * inv5(3)) % 5, 1);
        assert_eq!((4 * inv5(4)) % 5, 1);
    }

    #[test]
    fn mul_identity() {
        assert_eq!(mul(1, 6), 6); // 1 * (x+1) = x+1
        assert_eq!(mul(6, 1), 6);
    }

    #[test]
    fn mul_x_plus_1_squared() {
        // (x+1)² = x² + 2x + 1 → 1 + 2·5 + 1·25 = 36
        assert_eq!(mul(6, 6), 36);
    }

    #[test]
    fn mul_zero() {
        assert_eq!(mul(0, 6), 0);
        assert_eq!(mul(6, 0), 0);
    }

    #[test]
    fn trial_divide_basic() {
        // x² = x · x, so x divides x²
        assert_eq!(trial_divide(25, 5), Some(5)); // 25=x², 5=x → quotient x=5
    }

    #[test]
    fn irreducible_degree_1() {
        // Degree 1 polynomials: values 5..25 (5^1..5^2)
        // All 20 should be irreducible (5 monic * 4 leading coeffs)
        let count = (5u16..25).filter(|&v| is_irreducible(v as u8)).count();
        assert_eq!(
            count, 20,
            "expected 20 irreducible degree-1 polynomials over GF(5)"
        );
    }

    #[test]
    fn irreducible_degree_2_count() {
        // Degree 2: 10 monic × 4 = 40 total
        let count = (25u16..125).filter(|&v| is_irreducible(v as u8)).count();
        assert_eq!(
            count, 40,
            "expected 40 irreducible degree-2 polynomials over GF(5)"
        );
    }

    /// Verify counts for degrees 1–2 (fully representable in u8).
    #[test]
    fn mobius_counts() {
        let expected = [20, 40]; // degrees 1–2: (p-1) × monic
        let mut counts = [0u32; 2];
        for v in 2u16..256 {
            if is_irreducible(v as u8) {
                if let Some(d) = degree(v) {
                    let d = d as usize;
                    if (1..=2).contains(&d) {
                        counts[d - 1] += 1;
                    }
                }
            }
        }
        assert_eq!(counts, expected, "Möbius formula mismatch: got {counts:?}");
    }

    /// Total irreducible count for Q0 over GF(5).
    #[test]
    fn total_irreducible_count() {
        let count = (5u16..256).filter(|&v| is_irreducible(v as u8)).count();
        // Degrees 1-2: 20 + 40 = 60, plus partial degree-3 (125..255)
        assert!(
            count >= 60,
            "at least 60 irreducibles expected, got {count}"
        );
    }

    #[test]
    fn x_squared_reducible() {
        assert!(!is_irreducible(25)); // x² = x · x
    }

    #[test]
    fn mul_then_divide_roundtrip() {
        // (x+1) * x = x² + x → 0 + 1·5 + 1·25 = 30
        let product = mul(6, 5); // (x+1) * x
        assert_eq!(product, 30);
        assert_eq!(trial_divide(product as u8, 5), Some(6));
    }

    #[test]
    fn constants_not_irreducible() {
        for v in 0u8..5 {
            assert!(!is_irreducible(v), "constant {v} should not be irreducible");
        }
    }
}
