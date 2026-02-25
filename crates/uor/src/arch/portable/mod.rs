//! Portable scalar executor for UOR operations.
//!
//! This executor works on any architecture without requiring SIMD support.
//! It provides a pure Rust reference implementation of the UorStep traits.
//!
//! # Execution Model
//!
//! Operations follow the same pattern as SIMD executors:
//! - ALU operations: `ymm[i] op= ymm[i+8]` for i in 0..8
//! - Shift/rotate: Applied per 32-bit lane
//! - Crypto: Software implementations of SHA-256 and AES
//!
//! # Performance
//!
//! This executor is significantly slower than SIMD-accelerated executors
//! (Zen3, NEON) but provides a correctness baseline for testing.

use crate::isa::{UorStep, UorStepBinary, UorStepFused, UorStepLossless, Wavefront, WavefrontOp};
use crate::state::{UorState, GPR_COUNT, GPR_TAXONS, YMM_COUNT, YMM_TAXONS};

/// Portable scalar executor that works on any architecture.
///
/// This executor provides a reference implementation of the UorStep traits
/// without platform-specific optimizations. It's suitable for:
/// - Development and testing on any platform
/// - WASM targets
/// - Embedded systems without SIMD support
/// - Correctness baseline for conformance testing
#[derive(Debug, Clone, Copy, Default)]
pub struct ScalarExecutor;

impl ScalarExecutor {
    /// Create a new scalar executor.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

// =============================================================================
// Helper functions for 32-bit lane operations
// =============================================================================

/// Read a 32-bit lane from a taxon array.
#[inline(always)]
fn read_lane(taxons: &[crate::core::taxon::Taxon], lane: usize) -> u32 {
    let base = lane * 4;
    u32::from_le_bytes([
        taxons[base].value(),
        taxons[base + 1].value(),
        taxons[base + 2].value(),
        taxons[base + 3].value(),
    ])
}

/// Write a 32-bit lane to a taxon array.
#[inline(always)]
fn write_lane(taxons: &mut [crate::core::taxon::Taxon], lane: usize, value: u32) {
    let base = lane * 4;
    let bytes = value.to_le_bytes();
    taxons[base] = crate::core::taxon::Taxon::new(bytes[0]);
    taxons[base + 1] = crate::core::taxon::Taxon::new(bytes[1]);
    taxons[base + 2] = crate::core::taxon::Taxon::new(bytes[2]);
    taxons[base + 3] = crate::core::taxon::Taxon::new(bytes[3]);
}

/// Number of 32-bit lanes per YMM register (256 bits / 32 = 8 lanes).
const YMM_LANES: usize = YMM_TAXONS / 4; // 8

/// Number of 32-bit lanes per GPR (64 bits / 32 = 2 lanes).
const GPR_LANES: usize = GPR_TAXONS / 4; // 2

// =============================================================================
// ALU Operations (Ports 1/5)
// =============================================================================

/// Apply XOR operation: ymm[i] ^= ymm[i+8] for i in 0..8
#[inline]
fn apply_xor(state: &mut UorState, ymm_mask: u16, gpr_mask: u16) {
    // YMM registers: first 8 XOR with second 8
    for i in 0..8 {
        if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
            for lane in 0..YMM_LANES {
                let a = read_lane(&state.ymm[i], lane);
                let b = read_lane(&state.ymm[i + 8], lane);
                write_lane(&mut state.ymm[i], lane, a ^ b);
            }
        }
    }
    // GPR registers: first 7 XOR with second 7
    for i in 0..7 {
        if i + 7 < GPR_COUNT && (gpr_mask >> i) & 1 == 1 && (gpr_mask >> (i + 7)) & 1 == 1 {
            for lane in 0..GPR_LANES {
                let a = read_lane(&state.gpr[i], lane);
                let b = read_lane(&state.gpr[i + 7], lane);
                write_lane(&mut state.gpr[i], lane, a ^ b);
            }
        }
    }
}

/// Apply AND operation: ymm[i] &= ymm[i+8] for i in 0..8
#[inline]
fn apply_and(state: &mut UorState, ymm_mask: u16, gpr_mask: u16) {
    for i in 0..8 {
        if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
            for lane in 0..YMM_LANES {
                let a = read_lane(&state.ymm[i], lane);
                let b = read_lane(&state.ymm[i + 8], lane);
                write_lane(&mut state.ymm[i], lane, a & b);
            }
        }
    }
    for i in 0..7 {
        if i + 7 < GPR_COUNT && (gpr_mask >> i) & 1 == 1 && (gpr_mask >> (i + 7)) & 1 == 1 {
            for lane in 0..GPR_LANES {
                let a = read_lane(&state.gpr[i], lane);
                let b = read_lane(&state.gpr[i + 7], lane);
                write_lane(&mut state.gpr[i], lane, a & b);
            }
        }
    }
}

/// Apply OR operation: ymm[i] |= ymm[i+8] for i in 0..8
#[inline]
fn apply_or(state: &mut UorState, ymm_mask: u16, gpr_mask: u16) {
    for i in 0..8 {
        if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
            for lane in 0..YMM_LANES {
                let a = read_lane(&state.ymm[i], lane);
                let b = read_lane(&state.ymm[i + 8], lane);
                write_lane(&mut state.ymm[i], lane, a | b);
            }
        }
    }
    for i in 0..7 {
        if i + 7 < GPR_COUNT && (gpr_mask >> i) & 1 == 1 && (gpr_mask >> (i + 7)) & 1 == 1 {
            for lane in 0..GPR_LANES {
                let a = read_lane(&state.gpr[i], lane);
                let b = read_lane(&state.gpr[i + 7], lane);
                write_lane(&mut state.gpr[i], lane, a | b);
            }
        }
    }
}

/// Apply NOT operation: ymm[i] = !ymm[i] (bitwise complement)
#[inline]
fn apply_not(state: &mut UorState, ymm_mask: u16, gpr_mask: u16) {
    for i in 0..YMM_COUNT {
        if (ymm_mask >> i) & 1 == 1 {
            for lane in 0..YMM_LANES {
                let a = read_lane(&state.ymm[i], lane);
                write_lane(&mut state.ymm[i], lane, !a);
            }
        }
    }
    for i in 0..GPR_COUNT {
        if (gpr_mask >> i) & 1 == 1 {
            for lane in 0..GPR_LANES {
                let a = read_lane(&state.gpr[i], lane);
                write_lane(&mut state.gpr[i], lane, !a);
            }
        }
    }
}

/// Apply ADD operation: ymm[i] += ymm[i+8] (wrapping, per 32-bit lane)
#[inline]
fn apply_add(state: &mut UorState, ymm_mask: u16, gpr_mask: u16) {
    for i in 0..8 {
        if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
            for lane in 0..YMM_LANES {
                let a = read_lane(&state.ymm[i], lane);
                let b = read_lane(&state.ymm[i + 8], lane);
                write_lane(&mut state.ymm[i], lane, a.wrapping_add(b));
            }
        }
    }
    for i in 0..7 {
        if i + 7 < GPR_COUNT && (gpr_mask >> i) & 1 == 1 && (gpr_mask >> (i + 7)) & 1 == 1 {
            for lane in 0..GPR_LANES {
                let a = read_lane(&state.gpr[i], lane);
                let b = read_lane(&state.gpr[i + 7], lane);
                write_lane(&mut state.gpr[i], lane, a.wrapping_add(b));
            }
        }
    }
}

/// Apply SUB operation: ymm[i] -= ymm[i+8] (wrapping, per 32-bit lane)
#[inline]
fn apply_sub(state: &mut UorState, ymm_mask: u16, gpr_mask: u16) {
    for i in 0..8 {
        if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
            for lane in 0..YMM_LANES {
                let a = read_lane(&state.ymm[i], lane);
                let b = read_lane(&state.ymm[i + 8], lane);
                write_lane(&mut state.ymm[i], lane, a.wrapping_sub(b));
            }
        }
    }
    for i in 0..7 {
        if i + 7 < GPR_COUNT && (gpr_mask >> i) & 1 == 1 && (gpr_mask >> (i + 7)) & 1 == 1 {
            for lane in 0..GPR_LANES {
                let a = read_lane(&state.gpr[i], lane);
                let b = read_lane(&state.gpr[i + 7], lane);
                write_lane(&mut state.gpr[i], lane, a.wrapping_sub(b));
            }
        }
    }
}

// =============================================================================
// Shift/Rotate Operations (Port 0)
// =============================================================================

/// Apply rotate left: each 32-bit lane rotated left by n bits
#[inline]
fn apply_rotl(state: &mut UorState, n: u8, ymm_mask: u16, gpr_mask: u16) {
    let n = n as u32;
    for i in 0..YMM_COUNT {
        if (ymm_mask >> i) & 1 == 1 {
            for lane in 0..YMM_LANES {
                let a = read_lane(&state.ymm[i], lane);
                write_lane(&mut state.ymm[i], lane, a.rotate_left(n));
            }
        }
    }
    for i in 0..GPR_COUNT {
        if (gpr_mask >> i) & 1 == 1 {
            for lane in 0..GPR_LANES {
                let a = read_lane(&state.gpr[i], lane);
                write_lane(&mut state.gpr[i], lane, a.rotate_left(n));
            }
        }
    }
}

/// Apply rotate right: each 32-bit lane rotated right by n bits
#[inline]
fn apply_rotr(state: &mut UorState, n: u8, ymm_mask: u16, gpr_mask: u16) {
    let n = n as u32;
    for i in 0..YMM_COUNT {
        if (ymm_mask >> i) & 1 == 1 {
            for lane in 0..YMM_LANES {
                let a = read_lane(&state.ymm[i], lane);
                write_lane(&mut state.ymm[i], lane, a.rotate_right(n));
            }
        }
    }
    for i in 0..GPR_COUNT {
        if (gpr_mask >> i) & 1 == 1 {
            for lane in 0..GPR_LANES {
                let a = read_lane(&state.gpr[i], lane);
                write_lane(&mut state.gpr[i], lane, a.rotate_right(n));
            }
        }
    }
}

/// Apply shift left: each 32-bit lane shifted left by n bits
#[inline]
fn apply_shl(state: &mut UorState, n: u8, ymm_mask: u16, gpr_mask: u16) {
    let n = n as u32;
    for i in 0..YMM_COUNT {
        if (ymm_mask >> i) & 1 == 1 {
            for lane in 0..YMM_LANES {
                let a = read_lane(&state.ymm[i], lane);
                write_lane(&mut state.ymm[i], lane, a.wrapping_shl(n));
            }
        }
    }
    for i in 0..GPR_COUNT {
        if (gpr_mask >> i) & 1 == 1 {
            for lane in 0..GPR_LANES {
                let a = read_lane(&state.gpr[i], lane);
                write_lane(&mut state.gpr[i], lane, a.wrapping_shl(n));
            }
        }
    }
}

/// Apply shift right: each 32-bit lane shifted right by n bits
#[inline]
fn apply_shr(state: &mut UorState, n: u8, ymm_mask: u16, gpr_mask: u16) {
    let n = n as u32;
    for i in 0..YMM_COUNT {
        if (ymm_mask >> i) & 1 == 1 {
            for lane in 0..YMM_LANES {
                let a = read_lane(&state.ymm[i], lane);
                write_lane(&mut state.ymm[i], lane, a.wrapping_shr(n));
            }
        }
    }
    for i in 0..GPR_COUNT {
        if (gpr_mask >> i) & 1 == 1 {
            for lane in 0..GPR_LANES {
                let a = read_lane(&state.gpr[i], lane);
                write_lane(&mut state.gpr[i], lane, a.wrapping_shr(n));
            }
        }
    }
}

// =============================================================================
// SHA-256 Constants and Helpers
// =============================================================================

/// SHA-256 initial hash values.
#[rustfmt::skip]
const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// SHA-256 compression function helper: Choice (Ch).
#[inline(always)]
const fn sha256_ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

/// SHA-256 compression function helper: Majority (Maj).
#[inline(always)]
const fn sha256_maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

/// SHA-256 compression function helper: big sigma 0.
#[inline(always)]
const fn sha256_bsig0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

/// SHA-256 compression function helper: big sigma 1.
#[inline(always)]
const fn sha256_bsig1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

/// SHA-256 message schedule helper: small sigma 0.
#[inline(always)]
const fn sha256_ssig0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

/// SHA-256 message schedule helper: small sigma 1.
#[inline(always)]
const fn sha256_ssig1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

/// Perform one SHA-256 compression round.
/// Takes state [a,b,c,d,e,f,g,h] and returns new state.
#[inline]
fn sha256_round_step(state: [u32; 8], w: u32, k: u32) -> [u32; 8] {
    let [a, b, c, d, e, f, g, h] = state;

    let t1 = h
        .wrapping_add(sha256_bsig1(e))
        .wrapping_add(sha256_ch(e, f, g))
        .wrapping_add(k)
        .wrapping_add(w);
    let t2 = sha256_bsig0(a).wrapping_add(sha256_maj(a, b, c));

    [t1.wrapping_add(t2), a, b, c, d.wrapping_add(t1), e, f, g]
}

// =============================================================================
// AES Constants and Helpers
// =============================================================================

/// AES S-box for SubBytes transformation.
#[rustfmt::skip]
const AES_SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

/// AES inverse S-box for InvSubBytes transformation.
#[rustfmt::skip]
const AES_INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

/// GF(2^8) multiplication by 2 (xtime operation).
#[inline(always)]
const fn gf_mul2(x: u8) -> u8 {
    if x & 0x80 != 0 {
        (x << 1) ^ 0x1b
    } else {
        x << 1
    }
}

/// GF(2^8) multiplication by 3.
#[inline(always)]
const fn gf_mul3(x: u8) -> u8 {
    gf_mul2(x) ^ x
}

/// GF(2^8) multiplication by 9.
#[inline(always)]
const fn gf_mul9(x: u8) -> u8 {
    gf_mul2(gf_mul2(gf_mul2(x))) ^ x
}

/// GF(2^8) multiplication by 11 (0xB).
#[inline(always)]
const fn gf_mul11(x: u8) -> u8 {
    gf_mul2(gf_mul2(gf_mul2(x)) ^ x) ^ x
}

/// GF(2^8) multiplication by 13 (0xD).
#[inline(always)]
const fn gf_mul13(x: u8) -> u8 {
    gf_mul2(gf_mul2(gf_mul2(x) ^ x)) ^ x
}

/// GF(2^8) multiplication by 14 (0xE).
#[inline(always)]
const fn gf_mul14(x: u8) -> u8 {
    gf_mul2(gf_mul2(gf_mul2(x) ^ x) ^ x)
}

/// AES SubBytes transformation on a 16-byte state.
#[inline]
fn aes_sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = AES_SBOX[*byte as usize];
    }
}

/// AES InvSubBytes transformation on a 16-byte state.
#[inline]
fn aes_inv_sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = AES_INV_SBOX[*byte as usize];
    }
}

/// AES ShiftRows transformation on a 16-byte state.
#[inline]
fn aes_shift_rows(state: &mut [u8; 16]) {
    // Row 0: no shift
    // Row 1: shift left by 1
    let t = state[1];
    state[1] = state[5];
    state[5] = state[9];
    state[9] = state[13];
    state[13] = t;

    // Row 2: shift left by 2
    state.swap(2, 10);
    state.swap(6, 14);

    // Row 3: shift left by 3 (= shift right by 1)
    let t = state[15];
    state[15] = state[11];
    state[11] = state[7];
    state[7] = state[3];
    state[3] = t;
}

/// AES InvShiftRows transformation on a 16-byte state.
#[inline]
fn aes_inv_shift_rows(state: &mut [u8; 16]) {
    // Row 0: no shift
    // Row 1: shift right by 1
    let t = state[13];
    state[13] = state[9];
    state[9] = state[5];
    state[5] = state[1];
    state[1] = t;

    // Row 2: shift right by 2 (same as left by 2)
    state.swap(2, 10);
    state.swap(6, 14);

    // Row 3: shift right by 3 (= shift left by 1)
    let t = state[3];
    state[3] = state[7];
    state[7] = state[11];
    state[11] = state[15];
    state[15] = t;
}

/// AES MixColumns transformation on a 16-byte state.
#[inline]
fn aes_mix_columns(state: &mut [u8; 16]) {
    for col in 0..4 {
        let i = col * 4;
        let a = state[i];
        let b = state[i + 1];
        let c = state[i + 2];
        let d = state[i + 3];

        state[i] = gf_mul2(a) ^ gf_mul3(b) ^ c ^ d;
        state[i + 1] = a ^ gf_mul2(b) ^ gf_mul3(c) ^ d;
        state[i + 2] = a ^ b ^ gf_mul2(c) ^ gf_mul3(d);
        state[i + 3] = gf_mul3(a) ^ b ^ c ^ gf_mul2(d);
    }
}

/// AES InvMixColumns transformation on a 16-byte state.
#[inline]
fn aes_inv_mix_columns(state: &mut [u8; 16]) {
    for col in 0..4 {
        let i = col * 4;
        let a = state[i];
        let b = state[i + 1];
        let c = state[i + 2];
        let d = state[i + 3];

        state[i] = gf_mul14(a) ^ gf_mul11(b) ^ gf_mul13(c) ^ gf_mul9(d);
        state[i + 1] = gf_mul9(a) ^ gf_mul14(b) ^ gf_mul11(c) ^ gf_mul13(d);
        state[i + 2] = gf_mul13(a) ^ gf_mul9(b) ^ gf_mul14(c) ^ gf_mul11(d);
        state[i + 3] = gf_mul11(a) ^ gf_mul13(b) ^ gf_mul9(c) ^ gf_mul14(d);
    }
}

/// AES AddRoundKey: XOR state with round key.
#[inline]
fn aes_add_round_key(state: &mut [u8; 16], round_key: &[u8; 16]) {
    for (s, k) in state.iter_mut().zip(round_key.iter()) {
        *s ^= k;
    }
}

// =============================================================================
// Crypto Operations (TASK-133)
// =============================================================================

/// Read 128-bit block from YMM register (specified lane: 0 or 1).
#[inline]
fn read_128bit_lane(ymm: &[crate::core::taxon::Taxon; YMM_TAXONS], lane: usize) -> [u8; 16] {
    let base = lane * 16;
    let mut result = [0u8; 16];
    for (i, byte) in result.iter_mut().enumerate() {
        *byte = ymm[base + i].value();
    }
    result
}

/// Write 128-bit block to YMM register (specified lane: 0 or 1).
#[inline]
fn write_128bit_lane(
    ymm: &mut [crate::core::taxon::Taxon; YMM_TAXONS],
    lane: usize,
    data: &[u8; 16],
) {
    let base = lane * 16;
    for (i, byte) in data.iter().enumerate() {
        ymm[base + i] = crate::core::taxon::Taxon::new(*byte);
    }
}

/// SHA-256 round function: ymm[i] = SHA256RNDS2(ymm[i], ymm[i+8])
///
/// Performs two rounds of SHA-256 compression using the state in ymm[i]
/// and message words from ymm[i+8]. Uses rounds 0-1 constants.
#[inline]
fn apply_sha256_round(state: &mut UorState, ymm_mask: u16, _gpr_mask: u16) {
    for i in 0..8 {
        if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
            // Process each 128-bit lane (2 lanes per 256-bit YMM)
            for lane in 0..2 {
                // Read current state from ymm[i] as [a,b,c,d,e,f,g,h] packed into 128 bits
                // Intel format: state0=[d,c,b,a], state1=[h,g,f,e]
                let state_bytes = read_128bit_lane(&state.ymm[i], lane);
                let msg_bytes = read_128bit_lane(&state.ymm[i + 8], lane);

                // Extract 32-bit words (little-endian)
                let mut hash = [
                    u32::from_le_bytes([
                        state_bytes[12],
                        state_bytes[13],
                        state_bytes[14],
                        state_bytes[15],
                    ]), // a
                    u32::from_le_bytes([
                        state_bytes[8],
                        state_bytes[9],
                        state_bytes[10],
                        state_bytes[11],
                    ]), // b
                    u32::from_le_bytes([
                        state_bytes[4],
                        state_bytes[5],
                        state_bytes[6],
                        state_bytes[7],
                    ]), // c
                    u32::from_le_bytes([
                        state_bytes[0],
                        state_bytes[1],
                        state_bytes[2],
                        state_bytes[3],
                    ]), // d
                    u32::from_le_bytes([
                        msg_bytes[12],
                        msg_bytes[13],
                        msg_bytes[14],
                        msg_bytes[15],
                    ]), // e
                    u32::from_le_bytes([msg_bytes[8], msg_bytes[9], msg_bytes[10], msg_bytes[11]]), // f
                    u32::from_le_bytes([msg_bytes[4], msg_bytes[5], msg_bytes[6], msg_bytes[7]]), // g
                    u32::from_le_bytes([msg_bytes[0], msg_bytes[1], msg_bytes[2], msg_bytes[3]]), // h
                ];

                // Perform two SHA-256 rounds (using constants K[0] and K[1])
                hash = sha256_round_step(hash, hash[4], SHA256_K[0]);
                hash = sha256_round_step(hash, hash[5], SHA256_K[1]);

                // Pack result back
                let mut result = [0u8; 16];
                result[0..4].copy_from_slice(&hash[3].to_le_bytes()); // d
                result[4..8].copy_from_slice(&hash[2].to_le_bytes()); // c
                result[8..12].copy_from_slice(&hash[1].to_le_bytes()); // b
                result[12..16].copy_from_slice(&hash[0].to_le_bytes()); // a

                write_128bit_lane(&mut state.ymm[i], lane, &result);
            }
        }
    }
}

/// SHA-256 message schedule part 1: ymm[i] = SHA256MSG1(ymm[i], ymm[i+8])
///
/// Computes intermediate message schedule values using σ0 function.
#[inline]
fn apply_sha256_msg1(state: &mut UorState, ymm_mask: u16, _gpr_mask: u16) {
    for i in 0..8 {
        if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
            for lane in 0..2 {
                let w_bytes = read_128bit_lane(&state.ymm[i], lane);
                let next_bytes = read_128bit_lane(&state.ymm[i + 8], lane);

                // Extract message words
                let w0 = u32::from_le_bytes([w_bytes[0], w_bytes[1], w_bytes[2], w_bytes[3]]);
                let w1 = u32::from_le_bytes([w_bytes[4], w_bytes[5], w_bytes[6], w_bytes[7]]);
                let w2 = u32::from_le_bytes([w_bytes[8], w_bytes[9], w_bytes[10], w_bytes[11]]);
                let w3 = u32::from_le_bytes([w_bytes[12], w_bytes[13], w_bytes[14], w_bytes[15]]);
                let w4 = u32::from_le_bytes([
                    next_bytes[0],
                    next_bytes[1],
                    next_bytes[2],
                    next_bytes[3],
                ]);

                // Apply σ0 and add
                let r0 = w0.wrapping_add(sha256_ssig0(w1));
                let r1 = w1.wrapping_add(sha256_ssig0(w2));
                let r2 = w2.wrapping_add(sha256_ssig0(w3));
                let r3 = w3.wrapping_add(sha256_ssig0(w4));

                // Pack result
                let mut result = [0u8; 16];
                result[0..4].copy_from_slice(&r0.to_le_bytes());
                result[4..8].copy_from_slice(&r1.to_le_bytes());
                result[8..12].copy_from_slice(&r2.to_le_bytes());
                result[12..16].copy_from_slice(&r3.to_le_bytes());

                write_128bit_lane(&mut state.ymm[i], lane, &result);
            }
        }
    }
}

/// SHA-256 message schedule part 2: ymm[i] = SHA256MSG2(ymm[i], ymm[i+8])
///
/// Completes message schedule computation using σ1 function.
#[inline]
fn apply_sha256_msg2(state: &mut UorState, ymm_mask: u16, _gpr_mask: u16) {
    for i in 0..8 {
        if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
            for lane in 0..2 {
                let w_bytes = read_128bit_lane(&state.ymm[i], lane);
                let prev_bytes = read_128bit_lane(&state.ymm[i + 8], lane);

                // Extract message words
                let w0 = u32::from_le_bytes([w_bytes[0], w_bytes[1], w_bytes[2], w_bytes[3]]);
                let w1 = u32::from_le_bytes([w_bytes[4], w_bytes[5], w_bytes[6], w_bytes[7]]);
                let w2 = u32::from_le_bytes([w_bytes[8], w_bytes[9], w_bytes[10], w_bytes[11]]);
                let w3 = u32::from_le_bytes([w_bytes[12], w_bytes[13], w_bytes[14], w_bytes[15]]);
                let w14 = u32::from_le_bytes([
                    prev_bytes[8],
                    prev_bytes[9],
                    prev_bytes[10],
                    prev_bytes[11],
                ]);
                let w15 = u32::from_le_bytes([
                    prev_bytes[12],
                    prev_bytes[13],
                    prev_bytes[14],
                    prev_bytes[15],
                ]);

                // Apply σ1 and add
                let r0 = w0.wrapping_add(sha256_ssig1(w14));
                let r1 = w1.wrapping_add(sha256_ssig1(w15));
                let r2 = w2.wrapping_add(sha256_ssig1(r0));
                let r3 = w3.wrapping_add(sha256_ssig1(r1));

                // Pack result
                let mut result = [0u8; 16];
                result[0..4].copy_from_slice(&r0.to_le_bytes());
                result[4..8].copy_from_slice(&r1.to_le_bytes());
                result[8..12].copy_from_slice(&r2.to_le_bytes());
                result[12..16].copy_from_slice(&r3.to_le_bytes());

                write_128bit_lane(&mut state.ymm[i], lane, &result);
            }
        }
    }
}

/// AES encryption round: ymm[i] = AESENC(ymm[i], ymm[i+8])
///
/// Performs SubBytes, ShiftRows, MixColumns, AddRoundKey.
#[inline]
fn apply_aes_round(state: &mut UorState, ymm_mask: u16, _gpr_mask: u16) {
    for i in 0..8 {
        if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
            // Process each 128-bit lane (2 AES blocks per 256-bit YMM)
            for lane in 0..2 {
                let mut block = read_128bit_lane(&state.ymm[i], lane);
                let round_key = read_128bit_lane(&state.ymm[i + 8], lane);

                // AES round: SubBytes → ShiftRows → MixColumns → AddRoundKey
                aes_sub_bytes(&mut block);
                aes_shift_rows(&mut block);
                aes_mix_columns(&mut block);
                aes_add_round_key(&mut block, &round_key);

                write_128bit_lane(&mut state.ymm[i], lane, &block);
            }
        }
    }
}

/// AES decryption round: ymm[i] = AESDEC(ymm[i], ymm[i+8])
///
/// Performs InvShiftRows, InvSubBytes, InvMixColumns, AddRoundKey.
#[inline]
fn apply_aes_round_dec(state: &mut UorState, ymm_mask: u16, _gpr_mask: u16) {
    for i in 0..8 {
        if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
            // Process each 128-bit lane (2 AES blocks per 256-bit YMM)
            for lane in 0..2 {
                let mut block = read_128bit_lane(&state.ymm[i], lane);
                let round_key = read_128bit_lane(&state.ymm[i + 8], lane);

                // AES inverse round: InvShiftRows → InvSubBytes → InvMixColumns → AddRoundKey
                aes_inv_shift_rows(&mut block);
                aes_inv_sub_bytes(&mut block);
                aes_inv_mix_columns(&mut block);
                aes_add_round_key(&mut block, &round_key);

                write_128bit_lane(&mut state.ymm[i], lane, &block);
            }
        }
    }
}

// =============================================================================
// Permute Operations (TASK-134)
// =============================================================================

/// Shuffle: byte permute within 128-bit lanes (PSHUFB/VPSHUFB).
///
/// For each byte position in the output:
/// - If control byte has high bit set (>= 0x80): output is 0
/// - Otherwise: output is source[control & 0x0F] (within same 128-bit lane)
///
/// Control comes from ymm[i+8], applies to ymm[i].
/// Each 128-bit lane is shuffled independently.
#[inline]
fn apply_shuffle(state: &mut UorState, ymm_mask: u16, _gpr_mask: u16) {
    for i in 0..8 {
        if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
            // Process each 128-bit lane independently
            for lane in 0..2 {
                let src = read_128bit_lane(&state.ymm[i], lane);
                let ctrl = read_128bit_lane(&state.ymm[i + 8], lane);
                let mut result = [0u8; 16];

                for j in 0..16 {
                    let c = ctrl[j];
                    if c & 0x80 != 0 {
                        // High bit set: output zero
                        result[j] = 0;
                    } else {
                        // Select byte from source (within same lane)
                        result[j] = src[(c & 0x0F) as usize];
                    }
                }

                write_128bit_lane(&mut state.ymm[i], lane, &result);
            }
        }
    }
}

/// Permute: 32-bit lane permute across 256-bit register (VPERMD/VPERMPS).
///
/// For each 32-bit output position:
/// - Select which of the 8 source 32-bit lanes to use (control & 0x07)
///
/// Control indices come from ymm[i+8], applies to ymm[i].
/// Unlike shuffle, this can permute across 128-bit lane boundaries.
#[inline]
fn apply_permute(state: &mut UorState, ymm_mask: u16, _gpr_mask: u16) {
    for i in 0..8 {
        if (ymm_mask >> i) & 1 == 1 && (ymm_mask >> (i + 8)) & 1 == 1 {
            // Read source as 8 x 32-bit lanes
            let mut src_lanes = [0u32; 8];
            for (lane, src) in src_lanes.iter_mut().enumerate() {
                *src = read_lane(&state.ymm[i], lane);
            }

            // Read control indices as 8 x 32-bit values
            let mut ctrl_lanes = [0u32; 8];
            for (lane, ctrl) in ctrl_lanes.iter_mut().enumerate() {
                *ctrl = read_lane(&state.ymm[i + 8], lane);
            }

            // Permute: each output lane selects from any source lane
            for (lane, &ctrl) in ctrl_lanes.iter().enumerate() {
                let idx = (ctrl & 0x07) as usize;
                write_lane(&mut state.ymm[i], lane, src_lanes[idx]);
            }
        }
    }
}

// =============================================================================
// Port Operation Dispatch
// =============================================================================

/// Apply a single port operation.
///
/// This function is public to allow other executors (like NeonExecutor) to use it
/// as a fallback for operations they don't implement directly.
#[inline]
pub fn apply_op(state: &mut UorState, op: WavefrontOp, ymm_mask: u16, gpr_mask: u16) {
    match op {
        WavefrontOp::Nop => {}
        WavefrontOp::Xor => apply_xor(state, ymm_mask, gpr_mask),
        WavefrontOp::And => apply_and(state, ymm_mask, gpr_mask),
        WavefrontOp::Or => apply_or(state, ymm_mask, gpr_mask),
        WavefrontOp::Not => apply_not(state, ymm_mask, gpr_mask),
        WavefrontOp::Add => apply_add(state, ymm_mask, gpr_mask),
        WavefrontOp::Sub => apply_sub(state, ymm_mask, gpr_mask),
        WavefrontOp::RotL(n) => apply_rotl(state, n, ymm_mask, gpr_mask),
        WavefrontOp::RotR(n) => apply_rotr(state, n, ymm_mask, gpr_mask),
        WavefrontOp::ShL(n) => apply_shl(state, n, ymm_mask, gpr_mask),
        WavefrontOp::ShR(n) => apply_shr(state, n, ymm_mask, gpr_mask),
        WavefrontOp::Sha256Round => apply_sha256_round(state, ymm_mask, gpr_mask),
        WavefrontOp::Sha256Msg1 => apply_sha256_msg1(state, ymm_mask, gpr_mask),
        WavefrontOp::Sha256Msg2 => apply_sha256_msg2(state, ymm_mask, gpr_mask),
        WavefrontOp::AesRound => apply_aes_round(state, ymm_mask, gpr_mask),
        WavefrontOp::AesRoundDec => apply_aes_round_dec(state, ymm_mask, gpr_mask),
        WavefrontOp::Shuffle => apply_shuffle(state, ymm_mask, gpr_mask),
        WavefrontOp::Permute => apply_permute(state, ymm_mask, gpr_mask),
    }
}

// =============================================================================
// UorStep Implementation
// =============================================================================

impl UorStep for ScalarExecutor {
    unsafe fn step(&self, state: &mut UorState, wavefront: &Wavefront) {
        let ymm_mask = wavefront.ymm_mask;
        let gpr_mask = wavefront.gpr_mask;

        // Apply Port 0 operation (shift/rotate/SHA)
        apply_op(state, wavefront.ports.port0, ymm_mask, gpr_mask);

        // Apply Port 1 operation (ALU/AES)
        apply_op(state, wavefront.ports.port1, ymm_mask, gpr_mask);

        // Apply Port 5 operation ONLY if different from Port 1
        // In hardware, port1 and port5 execute the SAME operation in parallel
        // on different execution units. We don't apply it twice.
        if wavefront.ports.port5 != wavefront.ports.port1 {
            apply_op(state, wavefront.ports.port5, ymm_mask, gpr_mask);
        }
    }
}

impl UorStepLossless for ScalarExecutor {
    unsafe fn step_tracked(
        &self,
        state: &mut UorState,
        complement: &mut UorState,
        wavefront: &Wavefront,
    ) {
        let ymm_mask = wavefront.ymm_mask;
        let gpr_mask = wavefront.gpr_mask;

        // Save original state for registers that will be modified (first half)
        // For ALU ops: ymm[i] for i in 0..8, gpr[i] for i in 0..7
        // For unary ops (NOT, shifts): all masked registers
        for i in 0..8 {
            if (ymm_mask >> i) & 1 == 1 {
                complement.ymm[i] = state.ymm[i];
            }
        }
        for i in 0..7 {
            if (gpr_mask >> i) & 1 == 1 {
                complement.gpr[i] = state.gpr[i];
            }
        }

        // Execute the operation
        self.step(state, wavefront);
    }

    unsafe fn step_inverse(
        &self,
        state: &mut UorState,
        complement: &UorState,
        wavefront: &Wavefront,
    ) {
        let ymm_mask = wavefront.ymm_mask;
        let gpr_mask = wavefront.gpr_mask;

        // Apply inverse operations
        apply_inverse_op(state, complement, wavefront.ports.port0, ymm_mask, gpr_mask);
        apply_inverse_op(state, complement, wavefront.ports.port1, ymm_mask, gpr_mask);
        if wavefront.ports.port5 != wavefront.ports.port1 {
            apply_inverse_op(state, complement, wavefront.ports.port5, ymm_mask, gpr_mask);
        }
    }
}

/// Apply inverse operation using saved complement.
#[inline]
fn apply_inverse_op(
    state: &mut UorState,
    complement: &UorState,
    op: WavefrontOp,
    ymm_mask: u16,
    gpr_mask: u16,
) {
    match op {
        // Self-inverse operations
        WavefrontOp::Nop => {}
        WavefrontOp::Xor => apply_xor(state, ymm_mask, gpr_mask), // XOR is self-inverse
        WavefrontOp::Not => apply_not(state, ymm_mask, gpr_mask), // NOT is self-inverse

        // Invertible rotations (rotate opposite direction)
        WavefrontOp::RotL(n) => apply_rotr(state, n, ymm_mask, gpr_mask),
        WavefrontOp::RotR(n) => apply_rotl(state, n, ymm_mask, gpr_mask),

        // Operations that require complement restoration
        // For these, we restore original values from complement
        WavefrontOp::And
        | WavefrontOp::Or
        | WavefrontOp::Add
        | WavefrontOp::Sub
        | WavefrontOp::ShL(_)
        | WavefrontOp::ShR(_)
        | WavefrontOp::Sha256Round
        | WavefrontOp::Sha256Msg1
        | WavefrontOp::Sha256Msg2
        | WavefrontOp::AesRound
        | WavefrontOp::AesRoundDec
        | WavefrontOp::Shuffle
        | WavefrontOp::Permute => {
            // Restore original state from complement
            for i in 0..8 {
                if (ymm_mask >> i) & 1 == 1 {
                    state.ymm[i] = complement.ymm[i];
                }
            }
            for i in 0..7 {
                if (gpr_mask >> i) & 1 == 1 {
                    state.gpr[i] = complement.gpr[i];
                }
            }
        }
    }
}

impl UorStepFused for ScalarExecutor {
    unsafe fn run_fused(&self, state: &mut UorState, program: &[Wavefront]) {
        for wavefront in program {
            self.step(state, wavefront);
        }
    }

    unsafe fn step_n_fused(&self, state: &mut UorState, wavefront: &Wavefront, n: usize) {
        for _ in 0..n {
            self.step(state, wavefront);
        }
    }
}

impl UorStepBinary for ScalarExecutor {
    unsafe fn step_binary(
        &self,
        state_a: &mut UorState,
        state_b: &UorState,
        wavefront: &Wavefront,
    ) {
        let ymm_mask = wavefront.ymm_mask;
        let gpr_mask = wavefront.gpr_mask;

        // Apply operations using state_b as the second operand
        apply_binary_op(state_a, state_b, wavefront.ports.port0, ymm_mask, gpr_mask);
        apply_binary_op(state_a, state_b, wavefront.ports.port1, ymm_mask, gpr_mask);
        if wavefront.ports.port5 != wavefront.ports.port1 {
            apply_binary_op(state_a, state_b, wavefront.ports.port5, ymm_mask, gpr_mask);
        }
    }
}

/// Apply a binary operation: state_a[i] op= state_b[i]
///
/// Unlike the standard step which uses ymm[i+8] from the same state,
/// binary step uses corresponding registers from a second state.
#[inline]
fn apply_binary_op(
    state_a: &mut UorState,
    state_b: &UorState,
    op: WavefrontOp,
    ymm_mask: u16,
    gpr_mask: u16,
) {
    match op {
        WavefrontOp::Nop => {}

        WavefrontOp::Xor => {
            for i in 0..YMM_COUNT {
                if (ymm_mask >> i) & 1 == 1 {
                    for lane in 0..YMM_LANES {
                        let a = read_lane(&state_a.ymm[i], lane);
                        let b = read_lane(&state_b.ymm[i], lane);
                        write_lane(&mut state_a.ymm[i], lane, a ^ b);
                    }
                }
            }
            for i in 0..GPR_COUNT {
                if (gpr_mask >> i) & 1 == 1 {
                    for lane in 0..GPR_LANES {
                        let a = read_lane(&state_a.gpr[i], lane);
                        let b = read_lane(&state_b.gpr[i], lane);
                        write_lane(&mut state_a.gpr[i], lane, a ^ b);
                    }
                }
            }
        }

        WavefrontOp::And => {
            for i in 0..YMM_COUNT {
                if (ymm_mask >> i) & 1 == 1 {
                    for lane in 0..YMM_LANES {
                        let a = read_lane(&state_a.ymm[i], lane);
                        let b = read_lane(&state_b.ymm[i], lane);
                        write_lane(&mut state_a.ymm[i], lane, a & b);
                    }
                }
            }
            for i in 0..GPR_COUNT {
                if (gpr_mask >> i) & 1 == 1 {
                    for lane in 0..GPR_LANES {
                        let a = read_lane(&state_a.gpr[i], lane);
                        let b = read_lane(&state_b.gpr[i], lane);
                        write_lane(&mut state_a.gpr[i], lane, a & b);
                    }
                }
            }
        }

        WavefrontOp::Or => {
            for i in 0..YMM_COUNT {
                if (ymm_mask >> i) & 1 == 1 {
                    for lane in 0..YMM_LANES {
                        let a = read_lane(&state_a.ymm[i], lane);
                        let b = read_lane(&state_b.ymm[i], lane);
                        write_lane(&mut state_a.ymm[i], lane, a | b);
                    }
                }
            }
            for i in 0..GPR_COUNT {
                if (gpr_mask >> i) & 1 == 1 {
                    for lane in 0..GPR_LANES {
                        let a = read_lane(&state_a.gpr[i], lane);
                        let b = read_lane(&state_b.gpr[i], lane);
                        write_lane(&mut state_a.gpr[i], lane, a | b);
                    }
                }
            }
        }

        WavefrontOp::Add => {
            for i in 0..YMM_COUNT {
                if (ymm_mask >> i) & 1 == 1 {
                    for lane in 0..YMM_LANES {
                        let a = read_lane(&state_a.ymm[i], lane);
                        let b = read_lane(&state_b.ymm[i], lane);
                        write_lane(&mut state_a.ymm[i], lane, a.wrapping_add(b));
                    }
                }
            }
            for i in 0..GPR_COUNT {
                if (gpr_mask >> i) & 1 == 1 {
                    for lane in 0..GPR_LANES {
                        let a = read_lane(&state_a.gpr[i], lane);
                        let b = read_lane(&state_b.gpr[i], lane);
                        write_lane(&mut state_a.gpr[i], lane, a.wrapping_add(b));
                    }
                }
            }
        }

        WavefrontOp::Sub => {
            for i in 0..YMM_COUNT {
                if (ymm_mask >> i) & 1 == 1 {
                    for lane in 0..YMM_LANES {
                        let a = read_lane(&state_a.ymm[i], lane);
                        let b = read_lane(&state_b.ymm[i], lane);
                        write_lane(&mut state_a.ymm[i], lane, a.wrapping_sub(b));
                    }
                }
            }
            for i in 0..GPR_COUNT {
                if (gpr_mask >> i) & 1 == 1 {
                    for lane in 0..GPR_LANES {
                        let a = read_lane(&state_a.gpr[i], lane);
                        let b = read_lane(&state_b.gpr[i], lane);
                        write_lane(&mut state_a.gpr[i], lane, a.wrapping_sub(b));
                    }
                }
            }
        }

        // Unary operations that don't use the second operand
        WavefrontOp::Not => apply_not(state_a, ymm_mask, gpr_mask),
        WavefrontOp::RotL(n) => apply_rotl(state_a, n, ymm_mask, gpr_mask),
        WavefrontOp::RotR(n) => apply_rotr(state_a, n, ymm_mask, gpr_mask),
        WavefrontOp::ShL(n) => apply_shl(state_a, n, ymm_mask, gpr_mask),
        WavefrontOp::ShR(n) => apply_shr(state_a, n, ymm_mask, gpr_mask),

        // Crypto and permute operations use state_b as source for keys/control
        WavefrontOp::Sha256Round
        | WavefrontOp::Sha256Msg1
        | WavefrontOp::Sha256Msg2
        | WavefrontOp::AesRound
        | WavefrontOp::AesRoundDec
        | WavefrontOp::Shuffle
        | WavefrontOp::Permute => {
            apply_binary_crypto_permute(state_a, state_b, op, ymm_mask, gpr_mask);
        }
    }
}

/// Apply crypto/permute operation in binary mode.
///
/// These operations expect the second operand in ymm[i+8], so we temporarily
/// copy from state_b and apply the operation.
#[inline]
fn apply_binary_crypto_permute(
    state_a: &mut UorState,
    state_b: &UorState,
    op: WavefrontOp,
    ymm_mask: u16,
    gpr_mask: u16,
) {
    // Save state_a's ymm[8..16] and gpr[7..14]
    let saved_ymm: [_; 8] = core::array::from_fn(|i| state_a.ymm[i + 8]);
    let saved_gpr: [_; 7] = core::array::from_fn(|i| state_a.gpr[i + 7]);

    // Copy state_b's first half to state_a's second half for the operation
    for i in 0..8 {
        state_a.ymm[i + 8] = state_b.ymm[i];
    }
    for i in 0..7 {
        state_a.gpr[i + 7] = state_b.gpr[i];
    }

    // Apply the operation (it will use ymm[i+8] which now has state_b values)
    match op {
        WavefrontOp::Sha256Round => apply_sha256_round(state_a, ymm_mask, gpr_mask),
        WavefrontOp::Sha256Msg1 => apply_sha256_msg1(state_a, ymm_mask, gpr_mask),
        WavefrontOp::Sha256Msg2 => apply_sha256_msg2(state_a, ymm_mask, gpr_mask),
        WavefrontOp::AesRound => apply_aes_round(state_a, ymm_mask, gpr_mask),
        WavefrontOp::AesRoundDec => apply_aes_round_dec(state_a, ymm_mask, gpr_mask),
        WavefrontOp::Shuffle => apply_shuffle(state_a, ymm_mask, gpr_mask),
        WavefrontOp::Permute => apply_permute(state_a, ymm_mask, gpr_mask),
        _ => {}
    }

    // Restore state_a's second half
    for (i, saved) in saved_ymm.iter().enumerate() {
        state_a.ymm[i + 8] = *saved;
    }
    for (i, saved) in saved_gpr.iter().enumerate() {
        state_a.gpr[i + 7] = *saved;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isa::PortAssignment;

    #[test]
    fn test_scalar_executor_creation() {
        let executor = ScalarExecutor::new();
        let default = ScalarExecutor;
        assert_eq!(core::mem::size_of_val(&executor), 0);
        assert_eq!(core::mem::size_of_val(&default), 0);
    }

    #[test]
    fn test_scalar_xor_self_inverse() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set some initial values in ymm0 and ymm8
        write_lane(&mut state.ymm[0], 0, 0x12345678);
        write_lane(&mut state.ymm[8], 0, 0xAABBCCDD);

        let original_ymm0 = read_lane(&state.ymm[0], 0);
        let ymm8_val = read_lane(&state.ymm[8], 0);

        let wavefront = Wavefront::all_xor();

        // First XOR: ymm0 ^= ymm8
        unsafe { executor.step(&mut state, &wavefront) };
        let after_xor = read_lane(&state.ymm[0], 0);
        assert_eq!(after_xor, original_ymm0 ^ ymm8_val);

        // Second XOR: ymm0 ^= ymm8 (should restore original)
        unsafe { executor.step(&mut state, &wavefront) };
        let restored = read_lane(&state.ymm[0], 0);
        assert_eq!(restored, original_ymm0);
    }

    #[test]
    fn test_scalar_add_sub() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        write_lane(&mut state.ymm[0], 0, 100);
        write_lane(&mut state.ymm[8], 0, 50);

        let add_wf = Wavefront::all_add();
        unsafe { executor.step(&mut state, &add_wf) };
        assert_eq!(read_lane(&state.ymm[0], 0), 150);

        let sub_wf = Wavefront::all_sub();
        unsafe { executor.step(&mut state, &sub_wf) };
        assert_eq!(read_lane(&state.ymm[0], 0), 100);
    }

    #[test]
    fn test_scalar_rotate_left() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        write_lane(&mut state.ymm[0], 0, 0x80000001);

        let wf = Wavefront::new(PortAssignment::rotl_only(1));
        unsafe { executor.step(&mut state, &wf) };

        // 0x80000001 rotated left by 1 = 0x00000003
        assert_eq!(read_lane(&state.ymm[0], 0), 0x00000003);
    }

    #[test]
    fn test_scalar_rotate_right() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        write_lane(&mut state.ymm[0], 0, 0x80000001);

        let wf = Wavefront::new(PortAssignment::rotr_only(1));
        unsafe { executor.step(&mut state, &wf) };

        // 0x80000001 rotated right by 1 = 0xC0000000
        assert_eq!(read_lane(&state.ymm[0], 0), 0xC0000000);
    }

    #[test]
    fn test_scalar_shift_left() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        write_lane(&mut state.ymm[0], 0, 0x0000000F);

        let wf = Wavefront::new(PortAssignment::shl_only(4));
        unsafe { executor.step(&mut state, &wf) };

        assert_eq!(read_lane(&state.ymm[0], 0), 0x000000F0);
    }

    #[test]
    fn test_scalar_shift_right() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        write_lane(&mut state.ymm[0], 0, 0xF0000000);

        let wf = Wavefront::new(PortAssignment::shr_only(4));
        unsafe { executor.step(&mut state, &wf) };

        assert_eq!(read_lane(&state.ymm[0], 0), 0x0F000000);
    }

    #[test]
    fn test_scalar_and() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        write_lane(&mut state.ymm[0], 0, 0xFF00FF00);
        write_lane(&mut state.ymm[8], 0, 0x0F0F0F0F);

        let wf = Wavefront::all_and();
        unsafe { executor.step(&mut state, &wf) };

        assert_eq!(read_lane(&state.ymm[0], 0), 0x0F000F00);
    }

    #[test]
    fn test_scalar_or() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        write_lane(&mut state.ymm[0], 0, 0xFF00FF00);
        write_lane(&mut state.ymm[8], 0, 0x00FF00FF);

        let wf = Wavefront::all_or();
        unsafe { executor.step(&mut state, &wf) };

        assert_eq!(read_lane(&state.ymm[0], 0), 0xFFFFFFFF);
    }

    #[test]
    fn test_scalar_not() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        write_lane(&mut state.ymm[0], 0, 0x00000000);

        let wf = Wavefront::all_not();
        unsafe { executor.step(&mut state, &wf) };

        assert_eq!(read_lane(&state.ymm[0], 0), 0xFFFFFFFF);
    }

    #[test]
    fn test_scalar_wrapping_add() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        write_lane(&mut state.ymm[0], 0, 0xFFFFFFFF);
        write_lane(&mut state.ymm[8], 0, 1);

        let wf = Wavefront::all_add();
        unsafe { executor.step(&mut state, &wf) };

        // Should wrap to 0
        assert_eq!(read_lane(&state.ymm[0], 0), 0);
    }

    #[test]
    fn test_scalar_run_fused() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        write_lane(&mut state.ymm[0], 0, 0x12345678);
        write_lane(&mut state.ymm[8], 0, 0x12345678);

        let program = [Wavefront::all_xor(), Wavefront::all_xor()];
        unsafe { executor.run_fused(&mut state, &program) };

        // XOR twice with same value = original
        assert_eq!(read_lane(&state.ymm[0], 0), 0x12345678);
    }

    #[test]
    fn test_scalar_step_n_fused() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        write_lane(&mut state.ymm[0], 0, 100);
        write_lane(&mut state.ymm[8], 0, 10);

        let wf = Wavefront::all_add();
        unsafe { executor.step_n_fused(&mut state, &wf, 5) };

        // 100 + (10 * 5) = 150
        assert_eq!(read_lane(&state.ymm[0], 0), 150);
    }

    // =========================================================================
    // Crypto operation tests (TASK-133)
    // =========================================================================

    #[test]
    fn test_sha256_helpers() {
        // Test SHA-256 helper functions
        // Ch(x, y, z) = (x & y) ^ (!x & z)
        let ch = sha256_ch(0xFFFF0000, 0x12345678, 0xABCDEF01);
        // x=0xFFFF0000: upper 16 bits set, lower 16 bits clear
        // Ch should pick y for upper bits, z for lower bits
        assert_eq!(ch, (0xFFFF0000 & 0x12345678) ^ (!0xFFFF0000 & 0xABCDEF01));

        // Maj(x, y, z) = (x & y) ^ (x & z) ^ (y & z)
        let maj = sha256_maj(0xFF00FF00, 0xF0F0F0F0, 0x0F0F0F0F);
        // Note: (0xF0F0F0F0 & 0x0F0F0F0F) = 0, so simplified:
        assert_eq!(maj, (0xFF00FF00 & 0xF0F0F0F0) ^ (0xFF00FF00 & 0x0F0F0F0F));

        // BSIG0(x) = ROTR^2(x) XOR ROTR^13(x) XOR ROTR^22(x)
        let bsig0_val = sha256_bsig0(0x6a09e667);
        let expected = 0x6a09e667_u32.rotate_right(2)
            ^ 0x6a09e667_u32.rotate_right(13)
            ^ 0x6a09e667_u32.rotate_right(22);
        assert_eq!(bsig0_val, expected);

        // SSIG0(x) = ROTR^7(x) XOR ROTR^18(x) XOR SHR^3(x)
        let ssig0_val = sha256_ssig0(0x12345678);
        let expected = 0x12345678_u32.rotate_right(7)
            ^ 0x12345678_u32.rotate_right(18)
            ^ (0x12345678_u32 >> 3);
        assert_eq!(ssig0_val, expected);
    }

    #[test]
    fn test_sha256_round_modifies_state() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set up initial state in ymm0 and message in ymm8
        for i in 0..16 {
            state.ymm[0][i] = crate::core::taxon::Taxon::new((i + 1) as u8);
            state.ymm[8][i] = crate::core::taxon::Taxon::new((i + 17) as u8);
        }

        let original = state.ymm[0];

        // Create wavefront for SHA256 round
        let wf = Wavefront::new(PortAssignment::sha256_round());
        unsafe { executor.step(&mut state, &wf) };

        // State should be modified
        assert_ne!(state.ymm[0], original);
    }

    #[test]
    fn test_sha256_msg1_modifies_state() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set up message words
        for i in 0..16 {
            state.ymm[0][i] = crate::core::taxon::Taxon::new((i * 11) as u8);
            state.ymm[8][i] = crate::core::taxon::Taxon::new((i * 13) as u8);
        }

        let original = state.ymm[0];

        let wf = Wavefront::new(PortAssignment::sha256_msg1());
        unsafe { executor.step(&mut state, &wf) };

        // State should be modified
        assert_ne!(state.ymm[0], original);
    }

    #[test]
    fn test_sha256_msg2_modifies_state() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set up message words
        for i in 0..16 {
            state.ymm[0][i] = crate::core::taxon::Taxon::new((i * 7) as u8);
            state.ymm[8][i] = crate::core::taxon::Taxon::new((i * 17) as u8);
        }

        let original = state.ymm[0];

        let wf = Wavefront::new(PortAssignment::sha256_msg2());
        unsafe { executor.step(&mut state, &wf) };

        // State should be modified
        assert_ne!(state.ymm[0], original);
    }

    #[test]
    fn test_aes_sbox_known_values() {
        // Test AES S-box with known values from FIPS 197
        assert_eq!(AES_SBOX[0x00], 0x63);
        assert_eq!(AES_SBOX[0x01], 0x7c);
        assert_eq!(AES_SBOX[0x53], 0xed);
        assert_eq!(AES_SBOX[0xff], 0x16);

        // Test inverse S-box
        assert_eq!(AES_INV_SBOX[0x63], 0x00);
        assert_eq!(AES_INV_SBOX[0x7c], 0x01);
        assert_eq!(AES_INV_SBOX[0x16], 0xff);
    }

    #[test]
    fn test_aes_sbox_inverse_property() {
        // Verify S-box and inverse S-box are inverses
        for i in 0..=255u8 {
            let s = AES_SBOX[i as usize];
            let inv = AES_INV_SBOX[s as usize];
            assert_eq!(inv, i, "S-box inverse failed for {:#04x}", i);
        }
    }

    #[test]
    fn test_gf_multiplication() {
        // Test GF(2^8) multiplication
        assert_eq!(gf_mul2(0x57), 0xae);
        assert_eq!(gf_mul2(0xae), 0x47); // With reduction
        assert_eq!(gf_mul3(0x57), 0xf9);
    }

    #[test]
    fn test_aes_sub_bytes() {
        let mut state = [0u8; 16];
        for (i, byte) in state.iter_mut().enumerate() {
            *byte = i as u8;
        }

        let original = state;
        aes_sub_bytes(&mut state);

        // Verify each byte went through S-box
        for (i, &byte) in state.iter().enumerate() {
            assert_eq!(byte, AES_SBOX[original[i] as usize]);
        }
    }

    #[test]
    fn test_aes_shift_rows_inverse() {
        let original = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut state = original;

        aes_shift_rows(&mut state);
        aes_inv_shift_rows(&mut state);

        assert_eq!(state, original);
    }

    #[test]
    fn test_aes_mix_columns_inverse() {
        let original = [
            0x63, 0x53, 0xe0, 0x8c, 0x09, 0x60, 0xe1, 0x04, 0xcd, 0x70, 0xb7, 0x51, 0xba, 0xca,
            0xd0, 0xe7,
        ];
        let mut state = original;

        aes_mix_columns(&mut state);
        aes_inv_mix_columns(&mut state);

        assert_eq!(state, original);
    }

    #[test]
    fn test_aes_round_modifies_state() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set up state in ymm0 and round key in ymm8
        for i in 0..16 {
            state.ymm[0][i] = crate::core::taxon::Taxon::new((i + 0x32) as u8);
            state.ymm[8][i] = crate::core::taxon::Taxon::new((i * 3) as u8);
        }

        let original = state.ymm[0];

        let wf = Wavefront::new(PortAssignment::aes_round());
        unsafe { executor.step(&mut state, &wf) };

        // State should be modified
        assert_ne!(state.ymm[0], original);
    }

    #[test]
    fn test_aes_round_dec_modifies_state() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set up state in ymm0 and round key in ymm8
        for i in 0..16 {
            state.ymm[0][i] = crate::core::taxon::Taxon::new((i + 0x45) as u8);
            state.ymm[8][i] = crate::core::taxon::Taxon::new((i * 5) as u8);
        }

        let original = state.ymm[0];

        let wf = Wavefront::new(PortAssignment::aes_round_dec());
        unsafe { executor.step(&mut state, &wf) };

        // State should be modified
        assert_ne!(state.ymm[0], original);
    }

    #[test]
    fn test_aes_round_different_keys_different_output() {
        let executor = ScalarExecutor::new();

        // First encryption with key1
        let mut state1 = UorState::zero();
        for i in 0..16 {
            state1.ymm[0][i] = crate::core::taxon::Taxon::new(0x00);
            state1.ymm[8][i] = crate::core::taxon::Taxon::new(0x00);
        }
        let wf = Wavefront::new(PortAssignment::aes_round());
        unsafe { executor.step(&mut state1, &wf) };

        // Second encryption with key2
        let mut state2 = UorState::zero();
        for i in 0..16 {
            state2.ymm[0][i] = crate::core::taxon::Taxon::new(0x00);
            state2.ymm[8][i] = crate::core::taxon::Taxon::new(0x01); // Different key
        }
        unsafe { executor.step(&mut state2, &wf) };

        // Results should differ due to different keys
        assert_ne!(state1.ymm[0], state2.ymm[0]);
    }

    #[test]
    fn test_crypto_operations_use_both_lanes() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set different values in lane 0 and lane 1 of ymm0
        for i in 0..16 {
            state.ymm[0][i] = crate::core::taxon::Taxon::new(0x11);
            state.ymm[0][i + 16] = crate::core::taxon::Taxon::new(0x22); // Lane 1
            state.ymm[8][i] = crate::core::taxon::Taxon::new(0x33);
            state.ymm[8][i + 16] = crate::core::taxon::Taxon::new(0x44);
        }

        let wf = Wavefront::new(PortAssignment::aes_round());
        unsafe { executor.step(&mut state, &wf) };

        // Extract lane 0 and lane 1 results
        let lane0 = read_128bit_lane(&state.ymm[0], 0);
        let lane1 = read_128bit_lane(&state.ymm[0], 1);

        // Both lanes should be transformed differently
        assert_ne!(lane0, lane1);
    }

    // =========================================================================
    // Permute operation tests (TASK-134)
    // =========================================================================

    #[test]
    fn test_shuffle_identity() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set up source data in ymm0
        for i in 0..16 {
            state.ymm[0][i] = crate::core::taxon::Taxon::new(i as u8);
            state.ymm[0][i + 16] = crate::core::taxon::Taxon::new((i + 16) as u8);
        }

        // Set up identity shuffle in ymm8: each byte selects itself
        for i in 0..16 {
            state.ymm[8][i] = crate::core::taxon::Taxon::new(i as u8);
            state.ymm[8][i + 16] = crate::core::taxon::Taxon::new(i as u8);
        }

        let original = state.ymm[0];

        let wf = Wavefront::new(PortAssignment::shuffle());
        unsafe { executor.step(&mut state, &wf) };

        // Identity shuffle should preserve data
        assert_eq!(state.ymm[0], original);
    }

    #[test]
    fn test_shuffle_reverse() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set up source data in ymm0 (lane 0 only)
        for i in 0..16 {
            state.ymm[0][i] = crate::core::taxon::Taxon::new(i as u8);
        }

        // Set up reverse shuffle in ymm8: byte 0 gets byte 15, etc.
        for i in 0..16 {
            state.ymm[8][i] = crate::core::taxon::Taxon::new((15 - i) as u8);
        }

        let wf = Wavefront::new(PortAssignment::shuffle());
        unsafe { executor.step(&mut state, &wf) };

        // Verify reversed order
        for i in 0..16 {
            assert_eq!(state.ymm[0][i].value(), (15 - i) as u8);
        }
    }

    #[test]
    fn test_shuffle_zero_mask() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set up source data in ymm0
        for i in 0..32 {
            state.ymm[0][i] = crate::core::taxon::Taxon::new(0xFF);
        }

        // Set up zero shuffle mask (high bit set)
        for i in 0..32 {
            state.ymm[8][i] = crate::core::taxon::Taxon::new(0x80);
        }

        let wf = Wavefront::new(PortAssignment::shuffle());
        unsafe { executor.step(&mut state, &wf) };

        // All bytes should be zeroed
        for i in 0..32 {
            assert_eq!(state.ymm[0][i].value(), 0);
        }
    }

    #[test]
    fn test_shuffle_broadcast() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set up source data in ymm0: only byte 5 has value 0x42
        for i in 0..16 {
            state.ymm[0][i] = crate::core::taxon::Taxon::new(0x00);
        }
        state.ymm[0][5] = crate::core::taxon::Taxon::new(0x42);

        // Set up broadcast shuffle: all bytes select byte 5
        for i in 0..16 {
            state.ymm[8][i] = crate::core::taxon::Taxon::new(5);
        }

        let wf = Wavefront::new(PortAssignment::shuffle());
        unsafe { executor.step(&mut state, &wf) };

        // All bytes should be 0x42
        for i in 0..16 {
            assert_eq!(state.ymm[0][i].value(), 0x42);
        }
    }

    #[test]
    fn test_permute_identity() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set up source data in ymm0: 8 distinct 32-bit lanes
        for lane in 0..8 {
            write_lane(&mut state.ymm[0], lane, (lane * 0x11111111) as u32);
        }

        // Set up identity permute: each lane selects itself
        for lane in 0..8 {
            write_lane(&mut state.ymm[8], lane, lane as u32);
        }

        let original = state.ymm[0];

        let wf = Wavefront::new(PortAssignment::permute());
        unsafe { executor.step(&mut state, &wf) };

        // Identity permute should preserve data
        assert_eq!(state.ymm[0], original);
    }

    #[test]
    fn test_permute_reverse() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set up source data in ymm0: 8 distinct 32-bit lanes
        for lane in 0..8 {
            write_lane(&mut state.ymm[0], lane, lane as u32);
        }

        // Set up reverse permute: lane 0 gets lane 7, etc.
        for lane in 0..8 {
            write_lane(&mut state.ymm[8], lane, (7 - lane) as u32);
        }

        let wf = Wavefront::new(PortAssignment::permute());
        unsafe { executor.step(&mut state, &wf) };

        // Verify reversed order
        for lane in 0..8 {
            assert_eq!(read_lane(&state.ymm[0], lane), (7 - lane) as u32);
        }
    }

    #[test]
    fn test_permute_broadcast() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set up source data in ymm0: lane 3 has value 0xDEADBEEF
        for lane in 0..8 {
            write_lane(&mut state.ymm[0], lane, 0);
        }
        write_lane(&mut state.ymm[0], 3, 0xDEADBEEF);

        // Set up broadcast permute: all lanes select lane 3
        for lane in 0..8 {
            write_lane(&mut state.ymm[8], lane, 3);
        }

        let wf = Wavefront::new(PortAssignment::permute());
        unsafe { executor.step(&mut state, &wf) };

        // All lanes should be 0xDEADBEEF
        for lane in 0..8 {
            assert_eq!(read_lane(&state.ymm[0], lane), 0xDEADBEEF);
        }
    }

    #[test]
    fn test_permute_cross_lane() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();

        // Set up source data: lanes 0-3 in low 128 bits, lanes 4-7 in high 128 bits
        for lane in 0..8 {
            write_lane(&mut state.ymm[0], lane, (lane + 1) as u32);
        }

        // Set up permute that moves high lanes to low positions
        // Lane 0 <- Lane 4, Lane 1 <- Lane 5, Lane 2 <- Lane 6, Lane 3 <- Lane 7
        write_lane(&mut state.ymm[8], 0, 4);
        write_lane(&mut state.ymm[8], 1, 5);
        write_lane(&mut state.ymm[8], 2, 6);
        write_lane(&mut state.ymm[8], 3, 7);
        write_lane(&mut state.ymm[8], 4, 0);
        write_lane(&mut state.ymm[8], 5, 1);
        write_lane(&mut state.ymm[8], 6, 2);
        write_lane(&mut state.ymm[8], 7, 3);

        let wf = Wavefront::new(PortAssignment::permute());
        unsafe { executor.step(&mut state, &wf) };

        // Verify cross-lane swap
        assert_eq!(read_lane(&state.ymm[0], 0), 5); // Was lane 4
        assert_eq!(read_lane(&state.ymm[0], 1), 6); // Was lane 5
        assert_eq!(read_lane(&state.ymm[0], 2), 7); // Was lane 6
        assert_eq!(read_lane(&state.ymm[0], 3), 8); // Was lane 7
        assert_eq!(read_lane(&state.ymm[0], 4), 1); // Was lane 0
        assert_eq!(read_lane(&state.ymm[0], 5), 2); // Was lane 1
        assert_eq!(read_lane(&state.ymm[0], 6), 3); // Was lane 2
        assert_eq!(read_lane(&state.ymm[0], 7), 4); // Was lane 3
    }

    // =========================================================================
    // Lossless trait tests (TASK-135)
    // =========================================================================

    #[test]
    fn test_lossless_xor_self_inverse() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();
        let mut complement = UorState::zero();

        // Set up initial values
        write_lane(&mut state.ymm[0], 0, 0x12345678);
        write_lane(&mut state.ymm[8], 0, 0xAABBCCDD);

        let original = state.ymm[0];
        let wf = Wavefront::all_xor();

        // XOR with tracking
        unsafe { executor.step_tracked(&mut state, &mut complement, &wf) };

        // State should be changed
        assert_ne!(state.ymm[0], original);

        // Inverse should restore original
        unsafe { executor.step_inverse(&mut state, &complement, &wf) };
        assert_eq!(state.ymm[0], original);
    }

    #[test]
    fn test_lossless_and_restore() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();
        let mut complement = UorState::zero();

        // Set up initial values
        write_lane(&mut state.ymm[0], 0, 0xFF00FF00);
        write_lane(&mut state.ymm[8], 0, 0x0F0F0F0F);

        let original = state.ymm[0];
        let wf = Wavefront::all_and();

        // AND with tracking (AND destroys information)
        unsafe { executor.step_tracked(&mut state, &mut complement, &wf) };

        // Result of AND: 0xFF00FF00 & 0x0F0F0F0F = 0x0F000F00
        assert_eq!(read_lane(&state.ymm[0], 0), 0x0F000F00);

        // Inverse should restore original from complement
        unsafe { executor.step_inverse(&mut state, &complement, &wf) };
        assert_eq!(state.ymm[0], original);
    }

    #[test]
    fn test_lossless_rotate_inverse() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();
        let mut complement = UorState::zero();

        // Set up initial value
        write_lane(&mut state.ymm[0], 0, 0x12345678);

        let original = state.ymm[0];
        let wf = Wavefront::new(PortAssignment::rotl_only(7));

        // Rotate left with tracking
        unsafe { executor.step_tracked(&mut state, &mut complement, &wf) };

        // State should be changed
        assert_ne!(state.ymm[0], original);

        // Inverse (rotate right) should restore original
        unsafe { executor.step_inverse(&mut state, &complement, &wf) };
        assert_eq!(state.ymm[0], original);
    }

    #[test]
    fn test_lossless_shift_restore() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();
        let mut complement = UorState::zero();

        // Set up initial value
        write_lane(&mut state.ymm[0], 0, 0x12345678);

        let original = state.ymm[0];
        let wf = Wavefront::new(PortAssignment::shl_only(4));

        // Shift left with tracking (destroys high bits)
        unsafe { executor.step_tracked(&mut state, &mut complement, &wf) };

        // Result of shift left: 0x12345678 << 4 = 0x23456780
        assert_eq!(read_lane(&state.ymm[0], 0), 0x23456780);

        // Inverse should restore from complement
        unsafe { executor.step_inverse(&mut state, &complement, &wf) };
        assert_eq!(state.ymm[0], original);
    }

    #[test]
    fn test_lossless_add_restore() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();
        let mut complement = UorState::zero();

        // Set up initial values
        write_lane(&mut state.ymm[0], 0, 100);
        write_lane(&mut state.ymm[8], 0, 50);

        let original = state.ymm[0];
        let wf = Wavefront::all_add();

        // ADD with tracking
        unsafe { executor.step_tracked(&mut state, &mut complement, &wf) };

        // Result: 100 + 50 = 150
        assert_eq!(read_lane(&state.ymm[0], 0), 150);

        // Inverse should restore from complement
        unsafe { executor.step_inverse(&mut state, &complement, &wf) };
        assert_eq!(state.ymm[0], original);
    }

    #[test]
    fn test_lossless_not_self_inverse() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();
        let mut complement = UorState::zero();

        // Set up initial value
        write_lane(&mut state.ymm[0], 0, 0x12345678);

        let original = state.ymm[0];
        let wf = Wavefront::all_not();

        // NOT with tracking
        unsafe { executor.step_tracked(&mut state, &mut complement, &wf) };

        // State should be inverted
        assert_eq!(read_lane(&state.ymm[0], 0), !0x12345678u32);

        // NOT is self-inverse
        unsafe { executor.step_inverse(&mut state, &complement, &wf) };
        assert_eq!(state.ymm[0], original);
    }

    #[test]
    fn test_lossless_multiple_registers() {
        let executor = ScalarExecutor::new();
        let mut state = UorState::zero();
        let mut complement = UorState::zero();

        // Set up values in multiple YMM registers (non-zero values)
        for i in 0..8 {
            write_lane(&mut state.ymm[i], 0, ((i + 1) * 100) as u32);
            write_lane(&mut state.ymm[i + 8], 0, ((i + 1) * 10) as u32);
        }

        // Save originals
        let original_ymm: Vec<_> = (0..8).map(|i| state.ymm[i]).collect();

        let wf = Wavefront::all_add();

        // ADD with tracking
        unsafe { executor.step_tracked(&mut state, &mut complement, &wf) };

        // Verify values changed (ymm[i] += ymm[i+8])
        for i in 0..8 {
            let expected = ((i + 1) * 100 + (i + 1) * 10) as u32;
            assert_eq!(read_lane(&state.ymm[i], 0), expected);
        }

        // Inverse should restore all
        unsafe { executor.step_inverse(&mut state, &complement, &wf) };
        for (i, original) in original_ymm.iter().enumerate().take(8) {
            assert_eq!(state.ymm[i], *original);
        }
    }

    // =========================================================================
    // Binary trait tests (TASK-136)
    // =========================================================================

    #[test]
    fn test_binary_xor_two_states() {
        let executor = ScalarExecutor::new();
        let mut state_a = UorState::zero();
        let state_b = {
            let mut s = UorState::zero();
            write_lane(&mut s.ymm[0], 0, 0xAAAAAAAA);
            s
        };

        write_lane(&mut state_a.ymm[0], 0, 0x12345678);

        // Create a wavefront that XORs all registers
        let wf = Wavefront {
            ports: PortAssignment::all_xor(),
            ymm_mask: 0xFFFF,
            gpr_mask: 0x3FFF,
        };

        unsafe { executor.step_binary(&mut state_a, &state_b, &wf) };

        // state_a.ymm[0] ^= state_b.ymm[0]
        assert_eq!(read_lane(&state_a.ymm[0], 0), 0x12345678 ^ 0xAAAAAAAA);
    }

    #[test]
    fn test_binary_add_two_states() {
        let executor = ScalarExecutor::new();
        let mut state_a = UorState::zero();
        let state_b = {
            let mut s = UorState::zero();
            write_lane(&mut s.ymm[0], 0, 50);
            s
        };

        write_lane(&mut state_a.ymm[0], 0, 100);

        let wf = Wavefront {
            ports: PortAssignment::all_add(),
            ymm_mask: 0xFFFF,
            gpr_mask: 0x3FFF,
        };

        unsafe { executor.step_binary(&mut state_a, &state_b, &wf) };

        // state_a.ymm[0] += state_b.ymm[0]
        assert_eq!(read_lane(&state_a.ymm[0], 0), 150);
    }

    #[test]
    fn test_binary_and_two_states() {
        let executor = ScalarExecutor::new();
        let mut state_a = UorState::zero();
        let state_b = {
            let mut s = UorState::zero();
            write_lane(&mut s.ymm[0], 0, 0x0F0F0F0F);
            s
        };

        write_lane(&mut state_a.ymm[0], 0, 0xFF00FF00);

        let wf = Wavefront {
            ports: PortAssignment::all_and(),
            ymm_mask: 0xFFFF,
            gpr_mask: 0x3FFF,
        };

        unsafe { executor.step_binary(&mut state_a, &state_b, &wf) };

        // state_a.ymm[0] &= state_b.ymm[0]
        assert_eq!(read_lane(&state_a.ymm[0], 0), 0x0F000F00);
    }

    #[test]
    fn test_binary_or_two_states() {
        let executor = ScalarExecutor::new();
        let mut state_a = UorState::zero();
        let state_b = {
            let mut s = UorState::zero();
            write_lane(&mut s.ymm[0], 0, 0x00FF00FF);
            s
        };

        write_lane(&mut state_a.ymm[0], 0, 0xFF00FF00);

        let wf = Wavefront {
            ports: PortAssignment::all_or(),
            ymm_mask: 0xFFFF,
            gpr_mask: 0x3FFF,
        };

        unsafe { executor.step_binary(&mut state_a, &state_b, &wf) };

        // state_a.ymm[0] |= state_b.ymm[0]
        assert_eq!(read_lane(&state_a.ymm[0], 0), 0xFFFFFFFF);
    }

    #[test]
    fn test_binary_sub_two_states() {
        let executor = ScalarExecutor::new();
        let mut state_a = UorState::zero();
        let state_b = {
            let mut s = UorState::zero();
            write_lane(&mut s.ymm[0], 0, 30);
            s
        };

        write_lane(&mut state_a.ymm[0], 0, 100);

        let wf = Wavefront {
            ports: PortAssignment::all_sub(),
            ymm_mask: 0xFFFF,
            gpr_mask: 0x3FFF,
        };

        unsafe { executor.step_binary(&mut state_a, &state_b, &wf) };

        // state_a.ymm[0] -= state_b.ymm[0]
        assert_eq!(read_lane(&state_a.ymm[0], 0), 70);
    }

    #[test]
    fn test_binary_unary_operations() {
        let executor = ScalarExecutor::new();
        let mut state_a = UorState::zero();
        let state_b = UorState::zero(); // Not used for unary ops

        write_lane(&mut state_a.ymm[0], 0, 0x12345678);

        // NOT is unary - state_b should be ignored
        let wf = Wavefront {
            ports: PortAssignment::all_not(),
            ymm_mask: 0xFFFF,
            gpr_mask: 0x3FFF,
        };

        unsafe { executor.step_binary(&mut state_a, &state_b, &wf) };

        // state_a.ymm[0] = !state_a.ymm[0]
        assert_eq!(read_lane(&state_a.ymm[0], 0), !0x12345678u32);
    }

    #[test]
    fn test_binary_rotate() {
        let executor = ScalarExecutor::new();
        let mut state_a = UorState::zero();
        let state_b = UorState::zero(); // Not used for rotations

        write_lane(&mut state_a.ymm[0], 0, 0x80000001);

        let wf = Wavefront {
            ports: PortAssignment::rotl_only(1),
            ymm_mask: 0xFFFF,
            gpr_mask: 0x3FFF,
        };

        unsafe { executor.step_binary(&mut state_a, &state_b, &wf) };

        // 0x80000001 rotated left by 1 = 0x00000003
        assert_eq!(read_lane(&state_a.ymm[0], 0), 0x00000003);
    }

    #[test]
    fn test_binary_multiple_registers() {
        let executor = ScalarExecutor::new();
        let mut state_a = UorState::zero();
        let mut state_b = UorState::zero();

        // Set up values in multiple registers
        for i in 0..8 {
            write_lane(&mut state_a.ymm[i], 0, (i * 100) as u32);
            write_lane(&mut state_b.ymm[i], 0, (i * 10) as u32);
        }

        let wf = Wavefront {
            ports: PortAssignment::all_add(),
            ymm_mask: 0xFFFF,
            gpr_mask: 0x3FFF,
        };

        unsafe { executor.step_binary(&mut state_a, &state_b, &wf) };

        // Verify each register
        for i in 0..8 {
            let expected = (i * 100 + i * 10) as u32;
            assert_eq!(read_lane(&state_a.ymm[i], 0), expected);
        }
    }
}
