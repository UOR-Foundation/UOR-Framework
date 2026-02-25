//! GF(3) polynomial arithmetic for the UOR Invariance Frame.
//!
//! Polynomials over GF(3) have coefficients in {0, 1, 2}. Each `u8` encodes
//! a polynomial using base-3 representation: byte value `v` has coefficient
//! `(v / 3^i) % 3` for term x^i.
//!
//! # Encoding
//!
//! ```text
//! 5  = 1·3 + 2     → 2 + x       (degree 1)
//! 14 = 1·9 + 1·3 + 2 → 2 + x + x² (degree 2)
//! 81 = 1·81        → x⁴          (degree 4)
//! ```
//!
//! Since 3⁵ = 243 ≤ 255 < 729 = 3⁶, a `u8` represents polynomials up to
//! degree 5 (partially — only degree-5 monic polynomials with small lower
//! terms fit, values 243–255).
//!
//! # Irreducible Counts (Möbius formula)
//!
//! The Möbius formula gives monic irreducible counts. Over GF(3), each monic
//! irreducible has one non-monic associate (leading coeff 2), so total
//! irreducible count = 2 × monic count.
//!
//! | Degree | Monic | Total | Cumulative |
//! |--------|-------|-------|------------|
//! | 1      | 3     | 6     | 6          |
//! | 2      | 3     | 6     | 12         |
//! | 3      | 8     | 16    | 28         |
//! | 4      | 18    | 36    | 64         |
//!
//! # Example
//!
//! ```
//! use uor::frame::gf3;
//!
//! // x + 1 (value 4 = 1·3 + 1) is irreducible over GF(3)
//! assert!(gf3::is_irreducible(4));
//!
//! // x² (value 9 = 1·9) is reducible: x · x
//! assert!(!gf3::is_irreducible(9));
//! ```

/// Maximum number of base-3 digits in a u8 (3^5 = 243 ≤ 255).
const MAX_COEFFS_U8: usize = 6;

/// Maximum number of base-3 digits in a u16 (3^10 = 59049 ≤ 65535).
const MAX_COEFFS_U16: usize = 11;

/// Powers of 3 up to 3^10 for coefficient extraction.
const POW3: [u16; 11] = [1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049];

/// Extract the coefficient of x^i from a base-3 encoded polynomial.
#[inline]
pub const fn coeff(p: u16, i: usize) -> u8 {
    if i >= MAX_COEFFS_U16 {
        return 0;
    }
    ((p / POW3[i]) % 3) as u8
}

/// Degree of a base-3 encoded polynomial.
///
/// Returns `None` for the zero polynomial.
///
/// # Examples
///
/// ```
/// use uor::frame::gf3;
///
/// assert_eq!(gf3::degree(4), Some(1));  // x + 1
/// assert_eq!(gf3::degree(9), Some(2));  // x²
/// assert_eq!(gf3::degree(0), None);     // zero polynomial
/// ```
pub const fn degree(p: u16) -> Option<u8> {
    let mut d = MAX_COEFFS_U16;
    loop {
        if d == 0 {
            // Check degree 0
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

/// Polynomial multiplication over GF(3).
///
/// Coefficients are computed mod 3. Result returned as `u16`.
///
/// # Examples
///
/// ```
/// use uor::frame::gf3;
///
/// // (x + 1) * (x + 1) = x² + 2x + 1
/// // Encode: x+1 = 1·3 + 1 = 4
/// // Result: 1 + 2·3 + 1·9 = 1 + 6 + 9 = 16
/// assert_eq!(gf3::mul(4, 4), 16);
/// ```
pub const fn mul(a: u8, b: u8) -> u16 {
    // Extract coefficients of a
    let mut ca = [0u8; MAX_COEFFS_U8];
    let mut i = 0;
    while i < MAX_COEFFS_U8 {
        ca[i] = coeff(a as u16, i);
        i += 1;
    }
    // Extract coefficients of b
    let mut cb = [0u8; MAX_COEFFS_U8];
    i = 0;
    while i < MAX_COEFFS_U8 {
        cb[i] = coeff(b as u16, i);
        i += 1;
    }
    // Convolve mod 3
    let mut result = [0u8; MAX_COEFFS_U16];
    i = 0;
    while i < MAX_COEFFS_U8 {
        if ca[i] != 0 {
            let mut j = 0;
            while j < MAX_COEFFS_U8 {
                if cb[j] != 0 {
                    result[i + j] = (result[i + j] + ca[i] * cb[j]) % 3;
                }
                j += 1;
            }
        }
        i += 1;
    }
    // Encode result as base-3 u16
    let mut encoded: u16 = 0;
    i = 0;
    while i < MAX_COEFFS_U16 {
        encoded += result[i] as u16 * POW3[i];
        i += 1;
    }
    encoded
}

/// Polynomial reduction: `a mod m` over GF(3).
///
/// Reduces `a` by subtracting aligned multiples of `m` (mod 3)
/// until `degree(a) < degree(m)`.
///
/// # Examples
///
/// ```
/// use uor::frame::gf3;
///
/// // (x²) mod (x + 1): x² = (x-1)(x+1) + 1, so x² mod (x+1) = 1
/// // But in GF(3), -1 = 2, so x² mod (x+1) = 1
/// let r = gf3::modulo(9, 4); // 9 = x², 4 = x+1
/// assert!(r < 3); // result has degree < degree(x+1) = 1
/// ```
pub const fn modulo(a: u16, m: u16) -> u8 {
    let deg_m = match degree(m) {
        Some(d) => d,
        None => return a as u8,
    };
    let lead_m = coeff(m, deg_m as usize);
    // Multiplicative inverse of lead_m in GF(3): inv(1)=1, inv(2)=2
    let inv_lead = lead_m; // In GF(3): 1*1=1, 2*2=4≡1

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
        let factor = (lead_r * inv_lead) % 3;
        // Subtract factor * m * x^(deg_r - deg_m) from r
        let shift = (deg_r - deg_m) as usize;
        let mut new_r = r;
        let mut i = 0;
        while i < MAX_COEFFS_U8 {
            let mc = coeff(m, i);
            if mc != 0 {
                let target = i + shift;
                if target < MAX_COEFFS_U16 {
                    let cur = coeff(new_r, target);
                    let sub = (factor * mc) % 3;
                    let new_c = (cur + 3 - sub) % 3;
                    // Update coefficient at position target
                    new_r = set_coeff(new_r, target, new_c);
                }
            }
            i += 1;
        }
        r = new_r;
    }
}

/// Set the coefficient of x^i in a base-3 encoded polynomial.
const fn set_coeff(p: u16, i: usize, c: u8) -> u16 {
    let old_c = coeff(p, i);
    // Remove old contribution, add new
    let without = p - old_c as u16 * POW3[i];
    without + c as u16 * POW3[i]
}

/// Check if `divisor` divides `value` over GF(3).
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

/// Polynomial long division quotient: `a / b` over GF(3).
const fn div(a: u8, b: u8) -> u8 {
    let deg_b = match degree(b as u16) {
        Some(d) => d,
        None => return 0, // division by zero
    };
    let lead_b = coeff(b as u16, deg_b as usize);
    let inv_lead = lead_b; // inv(1)=1, inv(2)=2 in GF(3)

    let mut rem = a as u16;
    let mut quot: u16 = 0;
    while let Some(deg_r) = degree(rem) {
        if deg_r < deg_b {
            break;
        }
        let lead_r = coeff(rem, deg_r as usize);
        let factor = (lead_r * inv_lead) % 3;
        let shift = (deg_r - deg_b) as usize;
        quot = set_coeff(quot, shift, (coeff(quot, shift) + factor) % 3);
        let mut i = 0;
        while i < MAX_COEFFS_U8 {
            let bc = coeff(b as u16, i);
            if bc != 0 {
                let target = i + shift;
                if target < MAX_COEFFS_U16 {
                    let cur = coeff(rem, target);
                    let sub = (factor * bc) % 3;
                    rem = set_coeff(rem, target, (cur + 3 - sub) % 3);
                }
            }
            i += 1;
        }
    }
    quot as u8
}

/// Test if a polynomial (base-3 encoded as u8) is irreducible over GF(3).
///
/// A polynomial of degree d is irreducible if it has no factors of degree
/// 1 through d/2. Uses exhaustive trial division.
///
/// # Examples
///
/// ```
/// use uor::frame::gf3;
///
/// assert!(gf3::is_irreducible(3));   // x (degree 1)
/// assert!(gf3::is_irreducible(4));   // x + 1 (degree 1)
/// assert!(gf3::is_irreducible(5));   // x + 2 (degree 1)
/// assert!(!gf3::is_irreducible(9));  // x² = x · x (reducible)
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
        return true; // degree-1 always irreducible
    }
    let max_div_deg = deg / 2;
    // Try all polynomials of degree 1..=max_div_deg as divisors
    // Degree d polynomials have values in [3^d, 3^(d+1))
    let mut d = 1u8;
    while d <= max_div_deg {
        let lo = POW3[d as usize];
        let hi = POW3[(d + 1) as usize];
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
        // 4 = 1·3 + 1 → coeffs [1, 1] → x + 1
        assert_eq!(coeff(4, 0), 1);
        assert_eq!(coeff(4, 1), 1);
        assert_eq!(coeff(4, 2), 0);
    }

    #[test]
    fn degree_examples() {
        assert_eq!(degree(0), None);
        assert_eq!(degree(1), Some(0)); // constant 1
        assert_eq!(degree(2), Some(0)); // constant 2
        assert_eq!(degree(3), Some(1)); // x
        assert_eq!(degree(4), Some(1)); // x + 1
        assert_eq!(degree(9), Some(2)); // x²
        assert_eq!(degree(81), Some(4)); // x⁴
    }

    #[test]
    fn mul_identity() {
        // 1 * f = f for several values
        assert_eq!(mul(1, 4), 4); // 1 * (x+1) = x+1
        assert_eq!(mul(1, 9), 9); // 1 * x² = x²
        assert_eq!(mul(4, 1), 4); // commutative
    }

    #[test]
    fn mul_x_plus_1_squared() {
        // (x+1)² = x² + 2x + 1 → 1 + 2·3 + 1·9 = 16
        assert_eq!(mul(4, 4), 16);
    }

    #[test]
    fn mul_zero() {
        assert_eq!(mul(0, 4), 0);
        assert_eq!(mul(4, 0), 0);
    }

    #[test]
    fn modulo_zero_dividend() {
        assert_eq!(modulo(0, 4), 0);
    }

    #[test]
    fn trial_divide_basic() {
        // x² = x · x, so x divides x²
        assert_eq!(trial_divide(9, 3), Some(3)); // 9=x², 3=x → quotient x=3
    }

    #[test]
    fn irreducible_degree_1() {
        // Degree 1: x (3), x+1 (4), x+2 (5) — all irreducible
        assert!(is_irreducible(3));
        assert!(is_irreducible(4));
        assert!(is_irreducible(5));
        // Constants not irreducible
        assert!(!is_irreducible(0));
        assert!(!is_irreducible(1));
        assert!(!is_irreducible(2));
    }

    #[test]
    fn irreducible_degree_2_count() {
        // Degree 2: 3 monic × 2 (associates) = 6 total
        let count = (9u16..27).filter(|&v| is_irreducible(v as u8)).count();
        assert_eq!(
            count, 6,
            "expected 6 irreducible degree-2 polynomials over GF(3)"
        );
    }

    #[test]
    fn irreducible_degree_3_count() {
        // Degree 3: 8 monic × 2 = 16 total
        let count = (27u16..81).filter(|&v| is_irreducible(v as u8)).count();
        assert_eq!(
            count, 16,
            "expected 16 irreducible degree-3 polynomials over GF(3)"
        );
    }

    #[test]
    fn irreducible_degree_4_count() {
        // Degree 4: 18 monic × 2 = 36 total
        let count = (81u16..243).filter(|&v| is_irreducible(v as u8)).count();
        assert_eq!(
            count, 36,
            "expected 36 irreducible degree-4 polynomials over GF(3)"
        );
    }

    /// Verify counts: 2 × Möbius monic counts for degrees 1–4.
    #[test]
    fn mobius_counts() {
        // Total irreducibles = 2 × monic count (leading coeff 1 or 2)
        let expected = [6, 6, 16, 36]; // degrees 1–4
        let mut counts = [0u32; 4];
        for v in 2u16..256 {
            if is_irreducible(v as u8) {
                if let Some(d) = degree(v) {
                    let d = d as usize;
                    if (1..=4).contains(&d) {
                        counts[d - 1] += 1;
                    }
                }
            }
        }
        assert_eq!(counts, expected, "Möbius formula mismatch: got {counts:?}");
    }

    /// Total irreducible count for Q0 over GF(3).
    #[test]
    fn total_irreducible_count() {
        let count = (2u16..256).filter(|&v| is_irreducible(v as u8)).count();
        // Degrees 1-4: 6 + 6 + 16 + 36 = 64
        // Plus degree-5 irreducibles in 243..255 (13 values)
        assert!(
            count >= 64,
            "at least 64 irreducibles expected, got {count}"
        );
    }

    #[test]
    fn x_squared_reducible() {
        // x² = x · x, so reducible
        assert!(!is_irreducible(9)); // 9 = x²
    }

    #[test]
    fn mul_then_divide_roundtrip() {
        // (x+1) * x = x² + x → encode: 0 + 1·3 + 1·9 = 12
        let product = mul(4, 3); // (x+1) * x
        assert_eq!(product, 12);
        // product / x should give x+1
        assert_eq!(trial_divide(product as u8, 3), Some(4));
    }
}
