//! Precomputed lookup tables for O(1) observable and activation operations.
//!
//! This module provides compile-time generated lookup tables that enable
//! true O(1) constant-time access to observable values and activation operations.
//! All tables are computed at compile time and stored in read-only memory.
//!
//! # Tables Provided
//!
//! - **Stratum (Q0)**: Hamming weight for all 256 byte values
//! - **Curvature (Q0)**: Cascade length (bits flipped on increment)
//! - **Sigmoid (256)**: Precomputed sigmoid approximation
//! - **Tanh (256)**: Precomputed tanh approximation
//!
//! # Performance
//!
//! All lookups are O(1) with single array index operations.
//! Tables are sized to fit in L1 cache for optimal latency.
//!
//! # Example
//!
//! ```
//! use uor::lut::{stratum_q0, curvature_q0};
//!
//! // O(1) stratum lookup
//! assert_eq!(stratum_q0(0b11111111), 8);
//! assert_eq!(stratum_q0(0b10101010), 4);
//!
//! // O(1) curvature lookup
//! assert_eq!(curvature_q0(0), 1);  // 0→1: one bit flips
//! assert_eq!(curvature_q0(7), 4);  // 7→8: four bits flip
//!
//! ```

// ============================================================================
// Q0 (8-bit) Observable Tables
// ============================================================================

/// Precomputed stratum (Hamming weight) table for Q0.
///
/// `STRATUM_Q0[x]` = popcount(x) for x in 0..256.
/// Size: 256 bytes (fits in L1 cache).
pub static STRATUM_Q0: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        table[i as usize] = (i as u8).count_ones() as u8;
        i += 1;
    }
    table
};

/// Precomputed curvature (cascade length) table for Q0.
///
/// `CURVATURE_Q0[x]` = Hamming distance from x to x+1 (mod 256).
/// This equals trailing_ones(x) + 1, except for x=255 which wraps to 0.
/// Size: 256 bytes (fits in L1 cache).
pub static CURVATURE_Q0: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        let x = i as u8;
        let next = x.wrapping_add(1);
        // Hamming distance = popcount(x XOR next)
        table[i as usize] = (x ^ next).count_ones() as u8;
        i += 1;
    }
    table
};

/// Precomputed domain (mod 3) table for Q0.
///
/// `DOMAIN_Q0[x]` = x % 3 for x in 0..256.
/// Values: 0=Theta, 1=Psi, 2=Delta.
/// Size: 256 bytes (fits in L1 cache).
pub static DOMAIN_Q0: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        table[i as usize] = (i % 3) as u8;
        i += 1;
    }
    table
};

/// Precomputed rank (div 3) table for Q0.
///
/// `RANK_Q0[x]` = x / 3 for x in 0..256.
/// Size: 256 bytes (fits in L1 cache).
pub static RANK_Q0: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        table[i as usize] = (i / 3) as u8;
        i += 1;
    }
    table
};

/// O(1) stratum lookup for Q0 (8-bit).
#[inline]
pub const fn stratum_q0(value: u8) -> u8 {
    STRATUM_Q0[value as usize]
}

/// O(1) curvature lookup for Q0 (8-bit).
#[inline]
pub const fn curvature_q0(value: u8) -> u8 {
    CURVATURE_Q0[value as usize]
}

/// O(1) domain lookup for Q0 (8-bit).
#[inline]
pub const fn domain_q0(value: u8) -> u8 {
    DOMAIN_Q0[value as usize]
}

/// O(1) rank lookup for Q0 (8-bit).
#[inline]
pub const fn rank_q0(value: u8) -> u8 {
    RANK_Q0[value as usize]
}

/// Wrapping byte-domain addition.
#[inline]
pub const fn byte_add(a: u8, b: u8) -> u8 {
    a.wrapping_add(b)
}

/// Wrapping byte-domain subtraction.
#[inline]
pub const fn byte_sub(a: u8, b: u8) -> u8 {
    a.wrapping_sub(b)
}

/// Wrapping byte-domain multiplication.
#[inline]
pub const fn byte_mul(a: u8, b: u8) -> u8 {
    a.wrapping_mul(b)
}

// ============================================================================
// Activation Function Tables
// ============================================================================

/// Precomputed sigmoid table for 8-bit inputs.
///
/// Maps input byte to sigmoid output (0-255 scaled).
/// Uses piecewise linear approximation (const-compatible).
/// Input is treated as signed (-128 to 127).
/// Size: 256 bytes (fits in L1 cache).
///
/// Approximation:
/// - x <= -64: sigmoid ≈ 0
/// - x >= 64: sigmoid ≈ 255
/// - otherwise: linear interpolation
pub static SIGMOID_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        // Treat as signed: i in 0..256 maps to -128..127
        let x = if i < 128 { i as i16 } else { i as i16 - 256 };
        // Piecewise linear sigmoid approximation
        let sigmoid = if x <= -64 {
            0
        } else if x >= 64 {
            255
        } else {
            // Linear in [-64, 64] → [0, 255]
            // sigmoid = (x + 64) * 255 / 128 = (x + 64) * 2 - 1
            ((x + 64) * 2) as u8
        };
        table[i as usize] = sigmoid;
        i += 1;
    }
    table
};

/// Precomputed tanh table for 8-bit inputs.
///
/// Maps input byte to tanh output (0-255 scaled, 128 = 0).
/// Uses piecewise linear approximation (const-compatible).
/// Input is treated as signed (-128 to 127).
/// Size: 256 bytes (fits in L1 cache).
///
/// Approximation:
/// - x <= -64: tanh ≈ 0 (meaning -1 in tanh space)
/// - x >= 64: tanh ≈ 255 (meaning +1 in tanh space)
/// - otherwise: linear interpolation centered at 128
pub static TANH_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        let x = if i < 128 { i as i16 } else { i as i16 - 256 };
        // Piecewise linear tanh approximation
        let tanh_val = if x <= -64 {
            0
        } else if x >= 64 {
            255
        } else {
            // Linear in [-64, 64] → [0, 255]
            ((x + 64) * 2) as u8
        };
        table[i as usize] = tanh_val;
        i += 1;
    }
    table
};

/// O(1) sigmoid lookup for 8-bit input.
///
/// Returns sigmoid output scaled to 0-255.
#[inline]
pub const fn sigmoid_lut(value: u8) -> u8 {
    SIGMOID_256[value as usize]
}

/// O(1) tanh lookup for 8-bit input.
///
/// Returns tanh output scaled to 0-255 (128 = 0).
#[inline]
pub const fn tanh_lut(value: u8) -> u8 {
    TANH_256[value as usize]
}

/// Precomputed exp table for 8-bit inputs.
///
/// Maps signed input (-128 to 127) to exp output (0-255 scaled).
/// Uses piecewise linear approximation for const-compatibility.
/// exp(x) for x in [-6, 6] mapped to [0, 255].
/// Size: 256 bytes (fits in L1 cache).
///
/// Approximation:
/// - x <= -6 (byte 122): exp ≈ 0
/// - x >= 6 (byte 134): exp ≈ 255
/// - otherwise: linear interpolation (crude but O(1))
pub static EXP_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        // Treat as signed: i in 0..256 maps to -128..127
        // Scale to roughly -6..6 range (divide by ~21)
        let x = if i < 128 { i as i16 } else { i as i16 - 256 };
        // Piecewise linear exp approximation
        // Map -128..127 to 0..255 with exponential-like curve
        let exp_val = if x <= -64 {
            // exp(-3) ≈ 0.05, so output ~13
            let t = (x + 128) as u16; // 0..64
            ((t * 13) / 64) as u8
        } else if x >= 64 {
            // exp(3) ≈ 20, saturate to 255
            255
        } else if x < 0 {
            // [-64, 0) → [13, 128)
            // Linear: output = 13 + (x + 64) * 115 / 64
            let t = (x + 64) as u16; // 0..64
            (13 + (t * 115) / 64) as u8
        } else {
            // [0, 64) → [128, 255)
            // Linear: output = 128 + x * 127 / 64
            (128 + (x as u16 * 127) / 64) as u8
        };
        table[i as usize] = exp_val;
        i += 1;
    }
    table
};

/// Precomputed log table for 8-bit inputs.
///
/// Maps unsigned input (1-255) to log output (0-255 scaled).
/// log(0) is undefined, returns 0.
/// log(1) = 0, log(255) ≈ 5.54 → scaled to 255.
/// Size: 256 bytes (fits in L1 cache).
///
/// Uses integer approximation: output = floor(log2(x) * 255 / 8)
pub static LOG_256: [u8; 256] = {
    let mut table = [0u8; 256];
    // log(0) undefined, map to 0
    table[0] = 0;
    // For x >= 1, compute log2(x) scaled
    let mut i = 1u16;
    while i < 256 {
        // Find highest set bit (log2 floor)
        let x = i as u8;
        let log2_floor = 7 - x.leading_zeros() as u8;
        // Fractional part via bit below
        let frac = if log2_floor > 0 {
            // Get next bit for .5 precision
            ((x >> (log2_floor - 1)) & 1) as u16
        } else {
            0
        };
        // Scale: log2 range is 0..8, map to 0..255
        // output = (log2_floor * 2 + frac) * 255 / 16
        let log_val = ((log2_floor as u16 * 2 + frac) * 255) / 16;
        table[i as usize] = if log_val > 255 { 255 } else { log_val as u8 };
        i += 1;
    }
    table
};

/// Precomputed ReLU table for 8-bit signed inputs.
///
/// Maps signed input (-128 to 127) to max(0, x).
/// Bytes 0-127 map to themselves (positive).
/// Bytes 128-255 (negative in signed) map to 0.
/// Size: 256 bytes (fits in L1 cache).
pub static RELU_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        // Signed interpretation: 0-127 positive, 128-255 negative
        table[i as usize] = if i < 128 { i as u8 } else { 0 };
        i += 1;
    }
    table
};

/// Precomputed sqrt table for 8-bit unsigned inputs.
///
/// Maps unsigned input (0-255) to sqrt output (0-15.97 → 0-255 scaled).
/// sqrt(0) = 0, sqrt(255) ≈ 15.97 → scaled to 255.
/// Size: 256 bytes (fits in L1 cache).
///
/// Uses integer sqrt approximation.
#[allow(clippy::manual_div_ceil)] // div_ceil is not const-compatible in static context
pub static SQRT_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        // Integer sqrt via binary search
        let x = i;
        let mut low = 0u16;
        let mut high = 16u16; // sqrt(255) < 16
        while low < high {
            // Note: (low + high + 1) / 2 rounds up, which is required for binary search
            let mid = (low + high + 1) / 2;
            if mid * mid <= x {
                low = mid;
            } else {
                high = mid - 1;
            }
        }
        // low is floor(sqrt(i))
        // Scale: sqrt range 0..16 → 0..255
        // For better precision, compute (sqrt * 255 / 16)
        // Using fixed-point: (low * 255) / 16
        // But we can do better with fractional approximation
        let sqrt_floor = low;
        let remainder = x - sqrt_floor * sqrt_floor;
        let next_diff = 2 * sqrt_floor + 1; // Difference to next perfect square
                                            // Linear interpolation within the interval
        let frac = if next_diff > 0 {
            (remainder * 16) / next_diff
        } else {
            0
        };
        let scaled = sqrt_floor * 16 + frac;
        table[i as usize] = if scaled > 255 { 255 } else { scaled as u8 };
        i += 1;
    }
    table
};

/// Precomputed abs table for 8-bit signed inputs.
///
/// Maps signed input (-128 to 127) to |x|.
/// Bytes 0-127 map to themselves.
/// Bytes 128-255 map to 256 - x (e.g., 255→1, 128→128).
/// Note: |−128| = 128 (clamped due to asymmetry).
/// Size: 256 bytes (fits in L1 cache).
pub static ABS_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        // Signed interpretation: 0-127 positive, 128-255 negative
        table[i as usize] = if i < 128 {
            i as u8
        } else {
            // Two's complement: -x = 256 - x for x in 128..256
            // |−1| = 1 (255 → 1), |−128| = 128 (128 → 128)
            (256 - i) as u8
        };
        i += 1;
    }
    table
};

/// Precomputed GELU table for 8-bit signed inputs.
///
/// GELU(x) = x × Φ(x) where Φ is the CDF of standard normal.
/// Approximation: 0.5 × x × (1 + tanh(√(2/π) × (x + 0.044715 × x³)))
/// Input: signed 8-bit (-128 to 127 via two's complement byte)
/// Output: 0-255 (scaled GELU)
/// Size: 256 bytes (fits in L1 cache).
///
/// Very common in transformers (BERT, GPT, ViT).
pub static GELU_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        // Signed interpretation: 0-127 positive, 128-255 negative
        let signed_val = if i < 128 { i as i16 } else { i as i16 - 256 };

        // Scale to approximate range: -4 to 4 (where GELU is most interesting)
        // signed_val in [-128, 127] → x in [-4, ~4]
        let x = signed_val as f64 / 32.0;

        // GELU approximation: 0.5 * x * (1 + tanh(sqrt(2/π) * (x + 0.044715 * x³)))
        // sqrt(2/π) ≈ 0.7978845608
        let inner = 0.7978845608 * (x + 0.044715 * x * x * x);

        // tanh approximation for const context
        let tanh_val = if inner > 4.0 {
            1.0
        } else if inner < -4.0 {
            -1.0
        } else {
            // Pade approximation: tanh(x) ≈ x(27 + x²) / (27 + 9x²)
            let x2 = inner * inner;
            inner * (27.0 + x2) / (27.0 + 9.0 * x2)
        };

        let gelu = 0.5 * x * (1.0 + tanh_val);

        // Scale output: GELU(-4) ≈ 0, GELU(0) = 0, GELU(4) ≈ 4
        // Map [-4, 4] → [0, 255]
        let scaled = (gelu + 4.0) * 31.875; // (gelu + 4) * 255 / 8
        table[i as usize] = if scaled < 0.0 {
            0
        } else if scaled > 255.0 {
            255
        } else {
            scaled as u8
        };
        i += 1;
    }
    table
};

/// Precomputed SiLU (Swish) table for 8-bit signed inputs.
///
/// SiLU(x) = x × sigmoid(x) = x / (1 + exp(-x))
/// Input: signed 8-bit (-128 to 127 via two's complement byte)
/// Output: 0-255 (scaled SiLU)
/// Size: 256 bytes (fits in L1 cache).
///
/// Used in EfficientNet, ConvNeXt, and other modern architectures.
pub static SILU_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        // Signed interpretation: 0-127 positive, 128-255 negative
        let signed_val = if i < 128 { i as i16 } else { i as i16 - 256 };

        // Scale to approximate range: -8 to 8
        let x = signed_val as f64 / 16.0;

        // sigmoid approximation for const context
        let sigmoid = if x > 6.0 {
            1.0
        } else if x < -6.0 {
            0.0
        } else {
            // Pade approximation: sigmoid(x) ≈ 0.5 + x(0.25 - x²/48) for small x
            // Better approximation: 1 / (1 + exp(-x))
            // Using lookup approximation with piecewise linear
            let exp_neg_x = if x >= 0.0 {
                // exp(-x) for x >= 0
                let t = 1.0 - x / 8.0;
                if t > 0.0 {
                    t * t * t * t * t * t * t * t
                } else {
                    0.0
                }
            } else {
                // exp(-x) = exp(|x|) for x < 0
                let t = 1.0 + (-x) / 8.0;
                t * t * t * t * t * t * t * t
            };
            1.0 / (1.0 + exp_neg_x)
        };

        let silu = x * sigmoid;

        // Scale output: SiLU range is roughly [-0.28, x] where x can be large
        // For x in [-8, 8], SiLU in roughly [-0.28, 7.2]
        // Map to [0, 255]
        let scaled = (silu + 1.0) * 28.333; // (silu + 1) * 255 / 9
        table[i as usize] = if scaled < 0.0 {
            0
        } else if scaled > 255.0 {
            255
        } else {
            scaled as u8
        };
        i += 1;
    }
    table
};

/// O(1) exp lookup for 8-bit signed input.
///
/// Returns exp output scaled to 0-255.
#[inline]
pub const fn exp_lut(value: u8) -> u8 {
    EXP_256[value as usize]
}

/// O(1) log lookup for 8-bit unsigned input.
///
/// Returns log output scaled to 0-255.
/// log(0) returns 0 (undefined case).
#[inline]
pub const fn log_lut(value: u8) -> u8 {
    LOG_256[value as usize]
}

/// O(1) ReLU lookup for 8-bit signed input.
///
/// Returns max(0, x) where x is signed interpretation.
#[inline]
pub const fn relu_lut(value: u8) -> u8 {
    RELU_256[value as usize]
}

/// O(1) sqrt lookup for 8-bit unsigned input.
///
/// Returns sqrt output scaled to 0-255.
#[inline]
pub const fn sqrt_lut(value: u8) -> u8 {
    SQRT_256[value as usize]
}

/// O(1) abs lookup for 8-bit signed input.
///
/// Returns |x| where x is signed interpretation.
#[inline]
pub const fn abs_lut(value: u8) -> u8 {
    ABS_256[value as usize]
}

/// O(1) GELU lookup for 8-bit signed input.
///
/// Returns GELU(x) output scaled to 0-255.
/// GELU is the primary activation in transformers (BERT, GPT, ViT).
#[inline]
pub const fn gelu_lut(value: u8) -> u8 {
    GELU_256[value as usize]
}

/// O(1) SiLU (Swish) lookup for 8-bit signed input.
///
/// Returns SiLU(x) = x × sigmoid(x) output scaled to 0-255.
/// SiLU is used in EfficientNet, ConvNeXt, and modern architectures.
#[inline]
pub const fn silu_lut(value: u8) -> u8 {
    SILU_256[value as usize]
}

// ============================================================================
// Scientific Function Tables
// ============================================================================

// --- Const-compatible math helpers ---

/// Reduce angle to [-pi, pi] range.
const fn const_reduce_to_pi(mut x: f64) -> f64 {
    const TAU: f64 = std::f64::consts::TAU;
    const PI: f64 = std::f64::consts::PI;
    x = x - ((x / TAU) as i64 as f64) * TAU;
    if x > PI {
        x -= TAU;
    } else if x < -PI {
        x += TAU;
    }
    x
}

/// 7th-order Taylor sin, accurate for x in [-pi/2, pi/2].
const fn sin_taylor(x: f64) -> f64 {
    let x2 = x * x;
    x * (1.0 - x2 / 6.0 * (1.0 - x2 / 20.0 * (1.0 - x2 / 42.0)))
}

/// Const sin with range reduction to [-pi/2, pi/2].
const fn const_sin(x: f64) -> f64 {
    const PI: f64 = std::f64::consts::PI;
    const FRAC_PI_2: f64 = std::f64::consts::FRAC_PI_2;
    let x = const_reduce_to_pi(x);
    // Reduce to [-pi/2, pi/2] where Taylor is accurate
    if x > FRAC_PI_2 {
        sin_taylor(PI - x)
    } else if x < -FRAC_PI_2 {
        sin_taylor(-PI - x)
    } else {
        sin_taylor(x)
    }
}

/// Const cos via sin(x + pi/2).
const fn const_cos(x: f64) -> f64 {
    const FRAC_PI_2: f64 = std::f64::consts::FRAC_PI_2;
    const_sin(x + FRAC_PI_2)
}

/// Const atan via 7th-order Padé-like approximation for x in [-1, 1].
/// For |x| > 1, uses atan(x) = pi/2 - atan(1/x).
const fn const_atan(x: f64) -> f64 {
    const FRAC_PI_2: f64 = std::f64::consts::FRAC_PI_2;
    if x > 1.0 {
        FRAC_PI_2 - const_atan(1.0 / x)
    } else if x < -1.0 {
        -FRAC_PI_2 - const_atan(1.0 / x)
    } else {
        let x2 = x * x;
        // Padé approximant: atan(x) ≈ x(15 + 4x²) / (15 + 9x²)
        x * (15.0 + 4.0 * x2) / (15.0 + 9.0 * x2)
    }
}

/// Const asin via atan(x / sqrt(1-x²)).
const fn const_asin(x: f64) -> f64 {
    const FRAC_PI_2: f64 = std::f64::consts::FRAC_PI_2;
    if x >= 1.0 {
        return FRAC_PI_2;
    }
    if x <= -1.0 {
        return -FRAC_PI_2;
    }
    let denom = const_sqrt_f64(1.0 - x * x);
    if denom < 1e-10 {
        return if x >= 0.0 { FRAC_PI_2 } else { -FRAC_PI_2 };
    }
    const_atan(x / denom)
}

/// Const sqrt via Newton's method.
const fn const_sqrt_f64(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut guess = x;
    if x > 1.0 {
        guess = x / 2.0;
    }
    let mut i = 0;
    while i < 20 {
        guess = (guess + x / guess) * 0.5;
        i += 1;
    }
    guess
}

/// Const log2 via integer part + Newton refinement.
const fn const_log2(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    // Decompose: x = m * 2^e where 1 <= m < 2
    let mut e = 0i32;
    let mut m = x;
    while m >= 2.0 {
        m *= 0.5;
        e += 1;
    }
    while m < 1.0 {
        m *= 2.0;
        e -= 1;
    }
    // log2(m) for m in [1,2) via Padé: log2(m) ≈ (m-1) * (2 + (m-1)/3) / (2 + 2(m-1)/3) / ln(2)
    // Simpler: use series log2(m) = (m-1)/ln2 - (m-1)^2/(2*ln2) + ...
    let t = m - 1.0;
    const LN2_INV: f64 = std::f64::consts::LOG2_E;
    let log2_m = LN2_INV * (t - t * t / 2.0 + t * t * t / 3.0 - t * t * t * t / 4.0);
    e as f64 + log2_m
}

/// Const 2^x via decomposition into integer and fractional parts.
const fn const_exp2(x: f64) -> f64 {
    if x <= -20.0 {
        return 0.0;
    }
    if x >= 20.0 {
        return 1048576.0; // cap
    }
    let int_part = x as i32;
    let frac = x - int_part as f64;
    // 2^frac for frac in [0,1) via Taylor of exp(frac * ln2)
    let t = frac * std::f64::consts::LN_2;
    let exp_frac = 1.0 + t * (1.0 + t / 2.0 * (1.0 + t / 3.0 * (1.0 + t / 4.0 * (1.0 + t / 5.0))));
    let mut result = exp_frac;
    if int_part >= 0 {
        let mut i = 0;
        while i < int_part {
            result *= 2.0;
            i += 1;
        }
    } else {
        let mut i = 0;
        while i < -int_part {
            result *= 0.5;
            i += 1;
        }
    }
    result
}

/// Precomputed sin table for 8-bit angle inputs.
///
/// Input: byte as angle, 0..255 maps to `[0, 2*pi)`.
/// Output: signed byte, 128 = 0.0, 0 = -1.0, 255 = +1.0.
/// Size: 256 bytes (fits in L1 cache).
pub static SIN_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    const TAU: f64 = std::f64::consts::TAU;
    while i < 256 {
        let angle = i as f64 * TAU / 256.0;
        let v = const_sin(angle);
        // signed_to_byte: v in [-1,1] -> byte [0,255] where 128=0
        let b = (v * 127.0 + 128.0) as u8;
        table[i as usize] = b;
        i += 1;
    }
    table
};

/// Precomputed cos table for 8-bit angle inputs.
///
/// Input: byte as angle, 0..255 maps to `[0, 2*pi)`.
/// Output: signed byte, 128 = 0.0, 0 = -1.0, 255 = +1.0.
/// Size: 256 bytes (fits in L1 cache).
pub static COS_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    const TAU: f64 = std::f64::consts::TAU;
    while i < 256 {
        let angle = i as f64 * TAU / 256.0;
        let v = const_cos(angle);
        let b = (v * 127.0 + 128.0) as u8;
        table[i as usize] = b;
        i += 1;
    }
    table
};

/// Precomputed tan table for 8-bit angle inputs.
///
/// Input: byte as angle, 0..255 maps to `[0, 2*pi)`.
/// Output: signed byte, clamped to [-1, 1] range.
/// Size: 256 bytes (fits in L1 cache).
pub static TAN_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    const TAU: f64 = std::f64::consts::TAU;
    while i < 256 {
        let angle = i as f64 * TAU / 256.0;
        let s = const_sin(angle);
        let c = const_cos(angle);
        let t = if c > 0.001 || c < -0.001 {
            s / c
        } else if s >= 0.0 {
            1.0
        } else {
            -1.0
        };
        let clamped = if t > 1.0 {
            1.0
        } else if t < -1.0 {
            -1.0
        } else {
            t
        };
        table[i as usize] = (clamped * 127.0 + 128.0) as u8;
        i += 1;
    }
    table
};

/// Precomputed asin table for 8-bit signed inputs.
///
/// Input: signed byte [-1, 1]. Output: angle byte scaled to [0, 255] over [-pi/2, pi/2].
/// Size: 256 bytes (fits in L1 cache).
pub static ASIN_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    const PI: f64 = std::f64::consts::PI;
    const FRAC_PI_2: f64 = std::f64::consts::FRAC_PI_2;
    while i < 256 {
        // signed byte: 128=0, 0=-1, 255=+1
        let v = (i as f64 - 128.0) / 127.0;
        let clamped = if v > 1.0 {
            1.0
        } else if v < -1.0 {
            -1.0
        } else {
            v
        };
        let angle = const_asin(clamped);
        // Map [-pi/2, pi/2] to [0, 255]
        table[i as usize] = (((angle + FRAC_PI_2) / PI) * 255.0) as u8;
        i += 1;
    }
    table
};

/// Precomputed acos table for 8-bit signed inputs.
///
/// Input: signed byte [-1, 1]. Output: angle byte scaled to [0, 255] over [0, pi].
/// Size: 256 bytes (fits in L1 cache).
pub static ACOS_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    const PI: f64 = std::f64::consts::PI;
    const FRAC_PI_2: f64 = std::f64::consts::FRAC_PI_2;
    while i < 256 {
        let v = (i as f64 - 128.0) / 127.0;
        let clamped = if v > 1.0 {
            1.0
        } else if v < -1.0 {
            -1.0
        } else {
            v
        };
        // acos(x) = pi/2 - asin(x)
        let angle = FRAC_PI_2 - const_asin(clamped);
        table[i as usize] = ((angle / PI) * 255.0) as u8;
        i += 1;
    }
    table
};

/// Precomputed atan table for 8-bit signed inputs.
///
/// Input: signed byte [-1, 1]. Output: angle byte scaled to [0, 255] over [-pi/2, pi/2].
/// Size: 256 bytes (fits in L1 cache).
pub static ATAN_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    const PI: f64 = std::f64::consts::PI;
    const FRAC_PI_2: f64 = std::f64::consts::FRAC_PI_2;
    while i < 256 {
        let v = (i as f64 - 128.0) / 127.0;
        let angle = const_atan(v);
        table[i as usize] = (((angle + FRAC_PI_2) / PI) * 255.0) as u8;
        i += 1;
    }
    table
};

/// Precomputed log2 table for 8-bit unsigned inputs.
///
/// Input: unsigned byte [0, 255]. Output: unsigned [0, 255] scaled.
/// log2(0) returns 0 (undefined case).
/// Size: 256 bytes (fits in L1 cache).
pub static LOG2_256: [u8; 256] = {
    let mut table = [0u8; 256];
    table[0] = 0;
    let mut i = 1u16;
    let max_log2 = const_log2(255.0);
    while i < 256 {
        let v = const_log2(i as f64) / max_log2;
        let scaled = v * 255.0;
        table[i as usize] = if scaled > 255.0 { 255 } else { scaled as u8 };
        i += 1;
    }
    table
};

/// Precomputed log10 table for 8-bit unsigned inputs.
///
/// Input: unsigned byte [0, 255]. Output: unsigned [0, 255] scaled.
/// log10(0) returns 0 (undefined case).
/// Size: 256 bytes (fits in L1 cache).
pub static LOG10_256: [u8; 256] = {
    let mut table = [0u8; 256];
    table[0] = 0;
    let mut i = 1u16;
    // log10(x) = log2(x) / log2(10)
    let log2_10 = const_log2(10.0);
    let max_log10 = const_log2(255.0) / log2_10;
    while i < 256 {
        let v = (const_log2(i as f64) / log2_10) / max_log10;
        let scaled = v * 255.0;
        table[i as usize] = if scaled > 255.0 { 255 } else { scaled as u8 };
        i += 1;
    }
    table
};

/// Precomputed exp2 table for 8-bit unsigned inputs.
///
/// Input: unsigned byte [0, 255] mapped to [0, 8].
/// Output: unsigned [0, 255] (2^x / 256, normalized).
/// Size: 256 bytes (fits in L1 cache).
pub static EXP2_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        let x = i as f64 / 255.0 * 8.0; // [0, 8]
        let v = const_exp2(x) / 256.0; // normalize so max ~ 1
        let scaled = v * 255.0;
        table[i as usize] = if scaled > 255.0 { 255 } else { scaled as u8 };
        i += 1;
    }
    table
};

/// Precomputed exp10 table for 8-bit unsigned inputs.
///
/// Input: unsigned byte [0, 255] mapped to [0, 2.4].
/// Output: unsigned [0, 255] (10^x / 255, normalized).
/// Size: 256 bytes (fits in L1 cache).
pub static EXP10_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    // 10^x = 2^(x * log2(10))
    let log2_10 = const_log2(10.0);
    while i < 256 {
        let x = i as f64 / 255.0 * 2.4; // [0, 2.4]
        let v = const_exp2(x * log2_10) / 255.0;
        let scaled = v * 255.0;
        table[i as usize] = if scaled > 255.0 { 255 } else { scaled as u8 };
        i += 1;
    }
    table
};

/// Precomputed square table for 8-bit unsigned inputs.
///
/// Input: unsigned [0, 1] (byte [0, 255]). Output: unsigned [0, 1].
/// Size: 256 bytes (fits in L1 cache).
pub static SQUARE_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        // x in [0,1], x^2 in [0,1]
        let x = i as f64 / 255.0;
        table[i as usize] = (x * x * 255.0) as u8;
        i += 1;
    }
    table
};

/// Precomputed cube table for 8-bit unsigned inputs.
///
/// Input: unsigned [0, 1] (byte [0, 255]). Output: unsigned [0, 1].
/// Size: 256 bytes (fits in L1 cache).
pub static CUBE_256: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        let x = i as f64 / 255.0;
        table[i as usize] = (x * x * x * 255.0) as u8;
        i += 1;
    }
    table
};

/// O(1) sin lookup for 8-bit angle input.
#[inline]
pub const fn sin_lut(value: u8) -> u8 {
    SIN_256[value as usize]
}

/// O(1) cos lookup for 8-bit angle input.
#[inline]
pub const fn cos_lut(value: u8) -> u8 {
    COS_256[value as usize]
}

/// O(1) tan lookup for 8-bit angle input.
#[inline]
pub const fn tan_lut(value: u8) -> u8 {
    TAN_256[value as usize]
}

/// O(1) asin lookup for 8-bit signed input.
#[inline]
pub const fn asin_lut(value: u8) -> u8 {
    ASIN_256[value as usize]
}

/// O(1) acos lookup for 8-bit signed input.
#[inline]
pub const fn acos_lut(value: u8) -> u8 {
    ACOS_256[value as usize]
}

/// O(1) atan lookup for 8-bit signed input.
#[inline]
pub const fn atan_lut(value: u8) -> u8 {
    ATAN_256[value as usize]
}

/// O(1) log2 lookup for 8-bit unsigned input.
#[inline]
pub const fn log2_lut(value: u8) -> u8 {
    LOG2_256[value as usize]
}

/// O(1) log10 lookup for 8-bit unsigned input.
#[inline]
pub const fn log10_lut(value: u8) -> u8 {
    LOG10_256[value as usize]
}

/// O(1) exp2 lookup for 8-bit unsigned input.
#[inline]
pub const fn exp2_lut(value: u8) -> u8 {
    EXP2_256[value as usize]
}

/// O(1) exp10 lookup for 8-bit unsigned input.
#[inline]
pub const fn exp10_lut(value: u8) -> u8 {
    EXP10_256[value as usize]
}

/// O(1) square lookup for 8-bit unsigned input.
#[inline]
pub const fn square_lut(value: u8) -> u8 {
    SQUARE_256[value as usize]
}

/// O(1) cube lookup for 8-bit unsigned input.
#[inline]
pub const fn cube_lut(value: u8) -> u8 {
    CUBE_256[value as usize]
}

// ============================================================================
// Torus Coordinate Tables
// ============================================================================

/// Precomputed torus page table for Q0.
///
/// `TORUS_PAGE_Q0[x]` = x / 8 for x in 0..256.
/// Maps byte to page index (0..31).
/// Size: 256 bytes (fits in L1 cache).
pub static TORUS_PAGE_Q0: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        table[i as usize] = (i / 8) as u8;
        i += 1;
    }
    table
};

/// Precomputed torus offset table for Q0.
///
/// `TORUS_OFFSET_Q0[x]` = x % 8 (position within page).
/// Size: 256 bytes (fits in L1 cache).
pub static TORUS_OFFSET_Q0: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        table[i as usize] = (i % 8) as u8;
        i += 1;
    }
    table
};

/// O(1) torus page lookup for Q0.
#[inline]
pub const fn torus_page_q0(value: u8) -> u8 {
    TORUS_PAGE_Q0[value as usize]
}

/// O(1) torus offset lookup for Q0.
#[inline]
pub const fn torus_offset_q0(value: u8) -> u8 {
    TORUS_OFFSET_Q0[value as usize]
}

// ============================================================================
// Orbit Classification Tables
// ============================================================================

/// Precomputed orbit class table for Q0.
///
/// `ORBIT_CLASS_Q0[x]` = x / 8 for x in 0..256.
/// Maps byte to one of 32 orbit classes.
/// Size: 256 bytes (fits in L1 cache).
pub static ORBIT_CLASS_Q0: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        table[i as usize] = (i / 8) as u8;
        i += 1;
    }
    table
};

/// O(1) orbit class lookup for Q0.
#[inline]
pub const fn orbit_class_q0(value: u8) -> u8 {
    ORBIT_CLASS_Q0[value as usize]
}

// ============================================================================
// Q1 (16-bit) Helpers
// ============================================================================

/// O(1) stratum for Q1 via two Q0 lookups.
///
/// Uses stratum(high_byte) + stratum(low_byte).
#[inline]
pub const fn stratum_q1(value: u16) -> u8 {
    let high = (value >> 8) as u8;
    let low = value as u8;
    stratum_q0(high) + stratum_q0(low)
}

/// O(1) curvature for Q1.
///
/// Computes Hamming distance to successor.
#[inline]
pub const fn curvature_q1(value: u16) -> u8 {
    let next = value.wrapping_add(1);
    let xor = value ^ next;
    // Split into bytes and sum
    let high = (xor >> 8) as u8;
    let low = xor as u8;
    stratum_q0(high) + stratum_q0(low)
}

// ============================================================================
// Table Size Summary
// ============================================================================

/// Total static memory usage for all LUT tables.
///
/// Q0 observables: 256 * 4 = 1,024 bytes
/// Byte ops: computed via wrapping intrinsics (no static table storage)
/// Activation (sigmoid, tanh): 256 * 2 = 512 bytes
/// Extended activation (exp, log, relu, sqrt, abs): 256 * 5 = 1,280 bytes
/// Modern activation (gelu, silu): 256 * 2 = 512 bytes
/// Scientific (sin, cos, tan, asin, acos, atan, log2, log10, exp2, exp10, square, cube): 256 * 12 = 3,072 bytes
/// Torus: 256 * 2 = 512 bytes
/// Orbit: 256 bytes
///
/// Total: ~7 KB (easily fits in L1 cache)
pub const LUT_TOTAL_SIZE: usize = 256 * 4 + 256 * 2 + 256 * 5 + 256 * 2 + 256 * 12 + 256 * 2 + 256;

// ============================================================================
// Compose tables — chain multiple [u8; 256] LUTs at compile time
// ============================================================================

/// Compose two LUT tables: `result[i] = b[a[i]]`.
pub const fn compose_tables(a: &[u8; 256], b: &[u8; 256]) -> [u8; 256] {
    let mut result = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        result[i] = b[a[i] as usize];
        i += 1;
    }
    result
}

/// Map an activation function name to its static table reference.
pub fn activation_table(name: &str) -> Option<&'static [u8; 256]> {
    match name {
        "sigmoid" => Some(&SIGMOID_256),
        "tanh" => Some(&TANH_256),
        "relu" => Some(&RELU_256),
        "exp" => Some(&EXP_256),
        "log" => Some(&LOG_256),
        "sqrt" => Some(&SQRT_256),
        "abs" => Some(&ABS_256),
        "gelu" => Some(&GELU_256),
        "silu" => Some(&SILU_256),
        "sin" => Some(&SIN_256),
        "cos" => Some(&COS_256),
        "tan" => Some(&TAN_256),
        "asin" => Some(&ASIN_256),
        "acos" => Some(&ACOS_256),
        "atan" => Some(&ATAN_256),
        "log2" => Some(&LOG2_256),
        "log10" => Some(&LOG10_256),
        "exp2" => Some(&EXP2_256),
        "exp10" => Some(&EXP10_256),
        "square" => Some(&SQUARE_256),
        "cube" => Some(&CUBE_256),
        _ => None,
    }
}

/// Compose a chain of activation tables at runtime. Returns `None` if any
/// name is unknown.
pub fn compose_chain(names: &[&str]) -> Option<[u8; 256]> {
    if names.is_empty() {
        return None;
    }
    let first = activation_table(names[0])?;
    let mut result = *first;
    for name in &names[1..] {
        let table = activation_table(name)?;
        result = compose_tables(&result, table);
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stratum_q0_matches_popcount() {
        for i in 0..=255u8 {
            assert_eq!(stratum_q0(i), i.count_ones() as u8);
        }
    }

    #[test]
    fn test_curvature_q0_matches_hamming() {
        for i in 0..=255u8 {
            let next = i.wrapping_add(1);
            let expected = (i ^ next).count_ones() as u8;
            assert_eq!(curvature_q0(i), expected);
        }
    }

    #[test]
    fn test_domain_q0() {
        for i in 0..=255u8 {
            assert_eq!(domain_q0(i), i % 3);
        }
    }

    #[test]
    fn test_rank_q0() {
        for i in 0..=255u8 {
            assert_eq!(rank_q0(i), i / 3);
        }
    }

    #[test]
    fn test_sigmoid_bounds() {
        // Byte-to-signed mapping: i < 128 → x = i, i >= 128 → x = i - 256
        // i=0 → x=0 (middle)
        // i=64 → x=64 (saturation point high)
        // i=127 → x=127 (saturated high)
        // i=128 → x=-128 (saturated low)
        // i=192 → x=-64 (saturation point low)
        // i=255 → x=-1 (just below middle)
        assert_eq!(sigmoid_lut(0), 128); // x=0 → middle
        assert_eq!(sigmoid_lut(64), 255); // x=64 → saturated high
        assert_eq!(sigmoid_lut(127), 255); // x=127 → saturated high
        assert_eq!(sigmoid_lut(128), 0); // x=-128 → saturated low
        assert_eq!(sigmoid_lut(192), 0); // x=-64 → saturation boundary
        assert_eq!(sigmoid_lut(255), 126); // x=-1 → just below middle
    }

    #[test]
    fn test_tanh_bounds() {
        // Same byte-to-signed mapping as sigmoid
        assert_eq!(tanh_lut(0), 128); // x=0 → middle
        assert_eq!(tanh_lut(64), 255); // x=64 → saturated high
        assert_eq!(tanh_lut(128), 0); // x=-128 → saturated low
        assert_eq!(tanh_lut(255), 126); // x=-1 → just below middle
    }

    #[test]
    fn test_torus_page() {
        assert_eq!(torus_page_q0(0), 0);
        assert_eq!(torus_page_q0(1), 0);
        assert_eq!(torus_page_q0(7), 0);
        assert_eq!(torus_page_q0(8), 1);
        assert_eq!(torus_page_q0(94), 11);
        assert_eq!(torus_page_q0(95), 11);
        assert_eq!(torus_page_q0(96), 12);
        assert_eq!(torus_page_q0(255), 31);
    }

    #[test]
    fn test_orbit_class() {
        for i in 0..=255u8 {
            assert_eq!(orbit_class_q0(i), i / 8);
        }
    }

    #[test]
    fn test_stratum_q1() {
        assert_eq!(stratum_q1(0), 0);
        assert_eq!(stratum_q1(0xFFFF), 16);
        assert_eq!(stratum_q1(0x00FF), 8);
        assert_eq!(stratum_q1(0xFF00), 8);
        assert_eq!(stratum_q1(0x5555), 8); // Alternating bits
    }

    #[test]
    fn test_curvature_q1() {
        assert_eq!(curvature_q1(0), 1); // 0 → 1: one bit
        assert_eq!(curvature_q1(1), 2); // 1 → 2: two bits
        assert_eq!(curvature_q1(0xFF), 9); // 0xFF → 0x100: 9 bits
        assert_eq!(curvature_q1(0xFFFF), 16); // All bits flip
    }

    #[test]
    fn test_lut_total_size() {
        // Verify our size calculation
        // Q0 obs + sigmoid/tanh + ext act + gelu/silu + scientific + torus + orbit
        assert_eq!(
            LUT_TOTAL_SIZE,
            256 * 4 + 256 * 2 + 256 * 5 + 256 * 2 + 256 * 12 + 256 * 2 + 256
        );
        // Compile-time check: tables must fit in L2 cache (<256KB)
        const { assert!(LUT_TOTAL_SIZE < 256 * 1024) }
    }

    #[test]
    fn test_exp_lut_bounds() {
        // exp(-128) should be near 0
        assert!(exp_lut(128) < 15); // -128 in signed
                                    // exp(0) should be around middle
        let mid = exp_lut(0);
        assert!((120..=136).contains(&mid)); // Around 128
                                             // exp(64) should be near max
        assert!(exp_lut(64) > 240);
        // exp(127) should be saturated
        assert_eq!(exp_lut(127), 255);
    }

    #[test]
    fn test_log_lut_bounds() {
        // log(0) undefined, returns 0
        assert_eq!(log_lut(0), 0);
        // log(1) = 0
        assert_eq!(log_lut(1), 0);
        // log(2) should be small
        assert!(log_lut(2) > 0 && log_lut(2) < 50);
        // log(255) should be near max
        assert!(log_lut(255) > 200);
        // Monotonic: log(x) <= log(x+1) for x >= 1
        for i in 1..255u8 {
            assert!(log_lut(i) <= log_lut(i + 1));
        }
    }

    #[test]
    fn test_relu_lut() {
        // Positive values unchanged
        for i in 0..128u8 {
            assert_eq!(relu_lut(i), i);
        }
        // Negative values (128-255 in unsigned) become 0
        for i in 128..=255u8 {
            assert_eq!(relu_lut(i), 0);
        }
    }

    #[test]
    fn test_sqrt_lut_bounds() {
        // sqrt(0) = 0
        assert_eq!(sqrt_lut(0), 0);
        // sqrt(1) should be small
        assert!(sqrt_lut(1) > 0 && sqrt_lut(1) < 32);
        // sqrt(4) ≈ 2, sqrt(9) ≈ 3, sqrt(16) ≈ 4
        // Scaled: sqrt(255) ≈ 16 → 255
        assert!(sqrt_lut(255) > 240);
        // Monotonic
        for i in 0..255u8 {
            assert!(sqrt_lut(i) <= sqrt_lut(i + 1));
        }
    }

    #[test]
    fn test_abs_lut() {
        // Positive values unchanged
        for i in 0..128u8 {
            assert_eq!(abs_lut(i), i);
        }
        // abs(-1) = 1 (255 in unsigned → 1)
        assert_eq!(abs_lut(255), 1);
        // abs(-128) = 128 (128 in unsigned → 128)
        assert_eq!(abs_lut(128), 128);
        // abs(-64) = 64 (192 in unsigned → 64)
        assert_eq!(abs_lut(192), 64);
    }

    #[test]
    fn test_mean_curvature_q0() {
        let sum: u32 = (0..=255u8).map(|i| curvature_q0(i) as u32).sum();
        let mean = sum as f64 / 256.0;
        // Theoretical: 2 - 2^(1-8) = 2 - 0.0078125 = 1.9921875
        assert!((mean - 1.9921875).abs() < 0.0001);
    }

    #[test]
    fn test_gelu_lut_bounds() {
        // GELU(-128) should be near 0 (strongly negative → ~0)
        assert!(gelu_lut(128) < 130); // -128 in signed, scaled near the lower half
                                      // GELU(0) should be around middle (GELU(0) = 0, scaled to ~128)
        let mid = gelu_lut(0);
        assert!((120..=136).contains(&mid));
        // GELU(127) should be high (GELU(4) ≈ 4, scaled high)
        assert!(gelu_lut(127) > 200);
        // Monotonic for positive values
        for i in 0..127u8 {
            assert!(gelu_lut(i) <= gelu_lut(i + 1));
        }
    }

    #[test]
    fn test_silu_lut_bounds() {
        // SiLU(-128) should be small (x × sigmoid(x) for very negative x → ~0)
        assert!(silu_lut(128) < 50); // -128 in signed
                                     // SiLU(0) = 0 × sigmoid(0) = 0, scaled to around 28 (due to offset)
        let zero_val = silu_lut(0);
        assert!(zero_val < 50);
        // SiLU(127) should be high (positive × ~1 = high)
        assert!(silu_lut(127) > 200);
        // Approximately monotonic for positive values
        let mut prev = silu_lut(0);
        for i in 1..128u8 {
            let curr = silu_lut(i);
            assert!(curr >= prev.saturating_sub(2)); // Allow small fluctuation
            prev = curr;
        }
    }

    #[test]
    fn compose_two_tables() {
        let composed = compose_tables(&SIGMOID_256, &TANH_256);
        for i in 0..256 {
            assert_eq!(composed[i], TANH_256[SIGMOID_256[i] as usize]);
        }
    }

    #[test]
    fn compose_three_tables() {
        let composed = compose_chain(&["sigmoid", "tanh", "relu"]).unwrap();
        for i in 0..256 {
            let step1 = SIGMOID_256[i];
            let step2 = TANH_256[step1 as usize];
            let step3 = RELU_256[step2 as usize];
            assert_eq!(composed[i], step3);
        }
    }

    #[test]
    fn compose_chain_unknown() {
        assert!(compose_chain(&["sigmoid", "unknown"]).is_none());
    }

    #[test]
    fn activation_table_lookup() {
        assert_eq!(activation_table("sigmoid").unwrap(), &SIGMOID_256);
        assert_eq!(activation_table("tanh").unwrap(), &TANH_256);
        assert_eq!(activation_table("sin").unwrap(), &SIN_256);
        assert_eq!(activation_table("cos").unwrap(), &COS_256);
        assert_eq!(activation_table("square").unwrap(), &SQUARE_256);
        assert!(activation_table("unknown").is_none());
    }

    #[test]
    fn test_sin_lut_bounds() {
        // sin(0) = 0.0 -> byte 128
        assert!((125..=131).contains(&sin_lut(0)));
        // sin(64) = sin(pi/2) = 1.0 -> byte 255
        assert!(sin_lut(64) > 250);
        // sin(128) = sin(pi) = 0.0 -> byte 128
        assert!((125..=131).contains(&sin_lut(128)));
        // sin(192) = sin(3pi/2) = -1.0 -> byte ~1
        assert!(sin_lut(192) < 5);
    }

    #[test]
    fn test_cos_lut_bounds() {
        // cos(0) = 1.0 -> byte 255
        assert!(cos_lut(0) > 250);
        // cos(64) = cos(pi/2) = 0.0 -> byte 128
        assert!((125..=131).contains(&cos_lut(64)));
        // cos(128) = cos(pi) = -1.0 -> byte ~1
        assert!(cos_lut(128) < 5);
    }

    #[test]
    fn test_tan_lut_bounds() {
        // tan(0) = 0.0 -> byte 128
        assert!((125..=131).contains(&tan_lut(0)));
        // tan near pi/4 (byte 32) ~ 1.0 -> byte 255
        assert!(tan_lut(32) > 240);
    }

    #[test]
    fn test_asin_lut_bounds() {
        // asin(0) = 0 -> maps to middle of output range
        assert!((120..=136).contains(&asin_lut(128)));
        // asin(1) = pi/2 -> maps to ~255
        assert!(asin_lut(255) > 240);
        // asin(-1) = -pi/2 -> maps to ~0
        assert!(asin_lut(1) < 15);
    }

    #[test]
    fn test_acos_lut_bounds() {
        // acos(0) = pi/2 -> middle of output range
        assert!((120..=136).contains(&acos_lut(128)));
        // acos(1) = 0 -> maps to ~0
        assert!(acos_lut(255) < 15);
        // acos(-1) = pi -> maps to ~255
        assert!(acos_lut(1) > 240);
    }

    #[test]
    fn test_atan_lut_bounds() {
        // atan(0) = 0 -> middle of output range
        assert!((120..=136).contains(&atan_lut(128)));
    }

    #[test]
    fn test_log2_lut_bounds() {
        assert_eq!(log2_lut(0), 0);
        assert_eq!(log2_lut(1), 0);
        assert!(log2_lut(255) > 240);
        // Monotonic
        for i in 1..255u8 {
            assert!(log2_lut(i) <= log2_lut(i + 1));
        }
    }

    #[test]
    fn test_log10_lut_bounds() {
        assert_eq!(log10_lut(0), 0);
        assert_eq!(log10_lut(1), 0);
        assert!(log10_lut(255) > 240);
        // Monotonic
        for i in 1..255u8 {
            assert!(log10_lut(i) <= log10_lut(i + 1));
        }
    }

    #[test]
    fn test_exp2_lut_bounds() {
        // exp2(0) = 2^0 / 256 ~ 0 -> small
        assert!(exp2_lut(0) < 5);
        // exp2(255) = 2^8 / 256 = 1.0 -> 255
        assert!(exp2_lut(255) > 240);
        // Monotonic
        for i in 0..255u8 {
            assert!(exp2_lut(i) <= exp2_lut(i + 1));
        }
    }

    #[test]
    fn test_exp10_lut_bounds() {
        // Monotonic
        for i in 0..255u8 {
            assert!(exp10_lut(i) <= exp10_lut(i + 1));
        }
    }

    #[test]
    fn test_square_lut() {
        assert_eq!(square_lut(0), 0);
        assert_eq!(square_lut(255), 255);
        // square(128) = (128/255)^2 * 255 ~ 64
        let mid = square_lut(128);
        assert!((60..=70).contains(&mid));
        // Monotonic
        for i in 0..255u8 {
            assert!(square_lut(i) <= square_lut(i + 1));
        }
    }

    #[test]
    fn test_cube_lut() {
        assert_eq!(cube_lut(0), 0);
        assert_eq!(cube_lut(255), 255);
        // Monotonic
        for i in 0..255u8 {
            assert!(cube_lut(i) <= cube_lut(i + 1));
        }
    }

    #[test]
    fn compose_chain_scientific() {
        let composed = compose_chain(&["sin", "square"]).unwrap();
        for i in 0..256 {
            let step1 = SIN_256[i];
            let step2 = SQUARE_256[step1 as usize];
            assert_eq!(composed[i], step2);
        }
    }
}
