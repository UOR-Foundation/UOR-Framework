//! Wavefront construction patterns and combinators.
//!
//! This module provides higher-level patterns for constructing wavefronts
//! that represent common computation patterns.
//!
//! # Pattern Library
//!
//! - **SHA-256 patterns**: Σ₀, Σ₁, σ₀, σ₁, Ch, Maj
//! - **AES patterns**: Round, InvRound, KeyExpand
//! - **Composition**: Sequential, parallel, conditional

extern crate alloc;

use super::isa::{PortAssignment, Wavefront, WavefrontOp};
use alloc::vec::Vec;

/// SHA-256 wavefront patterns.
///
/// These patterns implement the SHA-256 compression function primitives
/// using maximum port parallelism.
pub mod sha256 {
    use super::*;

    /// Σ₀(a) = ROTR²(a) ⊕ ROTR¹³(a) ⊕ ROTR²²(a)
    ///
    /// Requires 3 wavefronts for the 3 rotation amounts.
    pub fn big_sigma0() -> [Wavefront; 3] {
        [
            Wavefront::rotate_xor(2),
            Wavefront::rotate_xor(13),
            Wavefront::rotate_xor(22),
        ]
    }

    /// Σ₁(e) = ROTR⁶(e) ⊕ ROTR¹¹(e) ⊕ ROTR²⁵(e)
    ///
    /// Requires 3 wavefronts for the 3 rotation amounts.
    pub fn big_sigma1() -> [Wavefront; 3] {
        [
            Wavefront::rotate_xor(6),
            Wavefront::rotate_xor(11),
            Wavefront::rotate_xor(25),
        ]
    }

    /// σ₀(x) = ROTR⁷(x) ⊕ ROTR¹⁸(x) ⊕ SHR³(x)
    ///
    /// Message schedule small sigma 0.
    pub fn small_sigma0() -> [Wavefront; 3] {
        [
            Wavefront::rotate_xor(7),
            Wavefront::rotate_xor(18),
            Wavefront::new(PortAssignment::shift_and_xor(3)),
        ]
    }

    /// σ₁(x) = ROTR¹⁷(x) ⊕ ROTR¹⁹(x) ⊕ SHR¹⁰(x)
    ///
    /// Message schedule small sigma 1.
    pub fn small_sigma1() -> [Wavefront; 3] {
        [
            Wavefront::rotate_xor(17),
            Wavefront::rotate_xor(19),
            Wavefront::new(PortAssignment::shift_and_xor(10)),
        ]
    }

    /// Ch(e, f, g) = (e ∧ f) ⊕ (¬e ∧ g)
    ///
    /// Choice function pattern.
    pub fn ch() -> [Wavefront; 2] {
        [
            Wavefront::new(PortAssignment::all_and()),
            Wavefront::all_xor(),
        ]
    }

    /// Maj(a, b, c) = (a ∧ b) ⊕ (a ∧ c) ⊕ (b ∧ c)
    ///
    /// Majority function pattern.
    pub fn maj() -> [Wavefront; 3] {
        [
            Wavefront::new(PortAssignment::all_and()),
            Wavefront::new(PortAssignment::all_and()),
            Wavefront::all_xor(),
        ]
    }

    /// Complete SHA-256 round using SHA-NI.
    ///
    /// Single wavefront executes 2 SHA-256 rounds via hardware.
    pub fn round_sha_ni() -> Wavefront {
        Wavefront::sha256_round()
    }
}

/// AES wavefront patterns.
///
/// These patterns implement AES primitives using AES-NI.
pub mod aes {
    use super::*;

    /// Single AES encryption round.
    ///
    /// Executes SubBytes, ShiftRows, MixColumns, AddRoundKey.
    pub fn enc_round() -> Wavefront {
        Wavefront::aes_round()
    }

    /// Single AES decryption round.
    pub fn dec_round() -> Wavefront {
        Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::AesRoundDec,
            port5: WavefrontOp::AesRoundDec,
        })
    }

    /// AES-128 encryption (10 rounds).
    pub fn aes128_encrypt() -> [Wavefront; 10] {
        [enc_round(); 10]
    }

    /// AES-256 encryption (14 rounds).
    pub fn aes256_encrypt() -> [Wavefront; 14] {
        [enc_round(); 14]
    }

    /// AES-128 decryption (10 rounds).
    pub fn aes128_decrypt() -> [Wavefront; 10] {
        [dec_round(); 10]
    }

    /// AES-256 decryption (14 rounds).
    pub fn aes256_decrypt() -> [Wavefront; 14] {
        [dec_round(); 14]
    }
}

/// Bitwise operation patterns.
pub mod bitwise {
    use super::*;

    /// XOR two register sets.
    pub fn xor() -> Wavefront {
        Wavefront::all_xor()
    }

    /// AND two register sets.
    pub fn and() -> Wavefront {
        Wavefront::new(PortAssignment::all_and())
    }

    /// OR two register sets.
    pub fn or() -> Wavefront {
        Wavefront::new(PortAssignment::all_or())
    }

    /// NOT (complement) a register set.
    pub fn not() -> Wavefront {
        Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Not,
            port5: WavefrontOp::Not,
        })
    }
}

/// Arithmetic operation patterns.
pub mod arith {
    use super::*;

    /// Wrapping addition.
    pub fn add() -> Wavefront {
        Wavefront::new(PortAssignment::all_add())
    }

    /// Wrapping subtraction.
    pub fn sub() -> Wavefront {
        Wavefront::new(PortAssignment {
            port0: WavefrontOp::Nop,
            port1: WavefrontOp::Sub,
            port5: WavefrontOp::Sub,
        })
    }
}

/// Rotation patterns.
pub mod rotate {
    use super::*;

    /// Rotate right by n bits.
    pub fn right(n: u8) -> Wavefront {
        Wavefront::new(PortAssignment {
            port0: WavefrontOp::RotR(n),
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Nop,
        })
    }

    /// Rotate left by n bits.
    pub fn left(n: u8) -> Wavefront {
        Wavefront::new(PortAssignment {
            port0: WavefrontOp::RotL(n),
            port1: WavefrontOp::Nop,
            port5: WavefrontOp::Nop,
        })
    }
}

/// Program builder for wavefront sequences.
///
/// Provides a fluent API for constructing wavefront programs.
#[derive(Debug, Clone, Default)]
pub struct ProgramBuilder {
    wavefronts: Vec<Wavefront>,
}

impl ProgramBuilder {
    /// Create a new empty program.
    pub fn new() -> Self {
        Self {
            wavefronts: Vec::new(),
        }
    }

    /// Add a single wavefront.
    pub fn push(mut self, wf: Wavefront) -> Self {
        self.wavefronts.push(wf);
        self
    }

    /// Add multiple wavefronts.
    pub fn extend<I: IntoIterator<Item = Wavefront>>(mut self, wfs: I) -> Self {
        self.wavefronts.extend(wfs);
        self
    }

    /// Repeat a wavefront N times.
    pub fn repeat(mut self, wf: Wavefront, n: usize) -> Self {
        for _ in 0..n {
            self.wavefronts.push(wf);
        }
        self
    }

    /// Build the program.
    pub fn build(self) -> Vec<Wavefront> {
        self.wavefronts
    }

    /// Get the number of wavefronts in the program.
    pub fn len(&self) -> usize {
        self.wavefronts.len()
    }

    /// Check if the program is empty.
    pub fn is_empty(&self) -> bool {
        self.wavefronts.is_empty()
    }
}

/// Create a program that performs SHA-256 compression (64 rounds).
///
/// Uses SHA-NI for maximum throughput (2 rounds per wavefront).
pub fn sha256_compress_program() -> Vec<Wavefront> {
    // SHA-NI executes 2 rounds per sha256rnds2 instruction
    // 64 rounds / 2 = 32 wavefronts
    ProgramBuilder::new()
        .repeat(sha256::round_sha_ni(), 32)
        .build()
}

/// Create a program that performs AES-128 encryption.
pub fn aes128_encrypt_program() -> Vec<Wavefront> {
    ProgramBuilder::new().extend(aes::aes128_encrypt()).build()
}

/// Create a program that performs AES-256 encryption.
pub fn aes256_encrypt_program() -> Vec<Wavefront> {
    ProgramBuilder::new().extend(aes::aes256_encrypt()).build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_patterns() {
        let sigma0 = sha256::big_sigma0();
        assert_eq!(sigma0.len(), 3);

        let sigma1 = sha256::big_sigma1();
        assert_eq!(sigma1.len(), 3);

        let round = sha256::round_sha_ni();
        assert!(round.ports.is_valid());
    }

    #[test]
    fn test_aes_patterns() {
        let enc = aes::enc_round();
        assert!(enc.ports.is_valid());

        let full = aes::aes128_encrypt();
        assert_eq!(full.len(), 10);
    }

    #[test]
    fn test_program_builder() {
        let program = ProgramBuilder::new()
            .push(bitwise::xor())
            .push(bitwise::and())
            .repeat(rotate::right(7), 3)
            .build();

        assert_eq!(program.len(), 5);
    }

    #[test]
    fn test_sha256_compress_program() {
        let program = sha256_compress_program();
        assert_eq!(program.len(), 32); // 64 rounds / 2 per wavefront
    }
}
