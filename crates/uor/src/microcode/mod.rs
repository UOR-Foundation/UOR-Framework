//! Microcode primitives for universal backend implementation.
//!
//! This module defines the 5 fundamental operations from which all other
//! operations can be derived:
//!
//! - `bnot`: Bitwise NOT
//! - `neg`: Two's complement negation
//! - `xor`: Bitwise XOR
//! - `and`: Bitwise AND
//! - `or`: Bitwise OR
//!
//! # Mathematical Foundation
//!
//! From the UOR Q3 framework:
//!
//! - **Critical Identity**: `neg(bnot(x)) = succ(x)` proves that the two
//!   involutions (`neg` and `bnot`) generate the entire integer ring.
//! - **INC**: `neg(bnot(x)) = x + 1`
//! - **DEC**: `bnot(neg(x)) = x - 1`
//!
//! # Usage
//!
//! Backends implement `MicrocodePrimitives` with hardware-specific intrinsics,
//! and all derived operations are automatically available via `MicrocodeOps`.
//!
//! ```ignore
//! use uor::microcode::{MicrocodeWord, MicrocodePrimitives, MicrocodeOps};
//!
//! struct MyBackend;
//!
//! impl MicrocodePrimitives<u32> for MyBackend {
//!     fn bnot(&self, a: u32) -> u32 { !a }
//!     fn neg(&self, a: u32) -> u32 { a.wrapping_neg() }
//!     fn xor(&self, a: u32, b: u32) -> u32 { a ^ b }
//!     fn and(&self, a: u32, b: u32) -> u32 { a & b }
//!     fn or(&self, a: u32, b: u32) -> u32 { a | b }
//! }
//!
//! // Now MyBackend automatically gets inc, dec, add, sub via MicrocodeOps
//! ```

pub mod derivation;
pub mod executor;
pub mod kogge_stone;
pub mod ops;
pub mod primitives;
pub mod word;

// Re-export core traits and types
pub use derivation::{Derivation, DerivationId, MicrocodeStep};
pub use executor::{ScalarMicrocodeExecutor, REGISTER_COUNT};
pub use kogge_stone::KoggeStoneAdder;
pub use ops::MicrocodeOps;
pub use primitives::{MicrocodePrimitives, ScalarPrimitives};
pub use word::MicrocodeWord;

#[cfg(test)]
mod tests {
    use super::*;

    /// Reference scalar implementation for testing
    struct ScalarPrimitives;

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

    #[test]
    fn test_inc_identity() {
        // Critical identity: neg(bnot(x)) = x + 1
        let p = ScalarPrimitives;
        for x in [0u32, 1, 42, 255, 1000, u32::MAX - 1] {
            let inc_microcode = p.neg(p.bnot(x));
            let inc_native = x.wrapping_add(1);
            assert_eq!(inc_microcode, inc_native, "INC failed for x={}", x);
        }
    }

    #[test]
    fn test_dec_identity() {
        // bnot(neg(x)) = x - 1
        let p = ScalarPrimitives;
        for x in [1u32, 2, 42, 255, 1000, u32::MAX] {
            let dec_microcode = p.bnot(p.neg(x));
            let dec_native = x.wrapping_sub(1);
            assert_eq!(dec_microcode, dec_native, "DEC failed for x={}", x);
        }
    }

    #[test]
    fn test_involution_properties() {
        let p = ScalarPrimitives;
        for x in [0u32, 1, 42, 255, u32::MAX] {
            // bnot is self-inverse: bnot(bnot(x)) = x
            assert_eq!(p.bnot(p.bnot(x)), x);

            // neg is self-inverse: neg(neg(x)) = x
            assert_eq!(p.neg(p.neg(x)), x);
        }
    }
}
