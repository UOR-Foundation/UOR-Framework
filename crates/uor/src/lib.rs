//! Universal Object Reference (UOR) - Cellular Automaton ISA
//!
//! UOR is a **pure cellular automaton** where the entire CPU register file
//! forms one combined state, and instructions are **wavefronts** that fire
//! across all execution ports simultaneously.
//!
//! # Execution Model
//!
//! ```text
//! State     = Entire register file (624 Taxons = 4992 bits)
//! Wavefront = All execution ports fire in one cycle
//! Step      = One wavefront transforms the state
//! ```
//!
//! # Hardware Target (AMD Zen 3)
//!
//! ```text
//! Port 0: Shift, Rotate, SHA-NI (sha256rnds2)
//! Port 1: ALU (XOR, AND, OR, ADD), AES-NI (aesenc)
//! Port 5: ALU, AES-NI, Shuffle/Permute
//! ```
//!
//! A wavefront utilizes **ALL THREE PORTS** in a single cycle.
//!
//! # Register Budget
//!
//! | Register Set | Count | Bits | Taxons |
//! |--------------|-------|------|--------|
//! | YMM0-15 | 16 | 4096 | 512 |
//! | GPRs | 14 | 896 | 112 |
//! | **Total** | 30 | 4992 | 624 |
//!
//! # Zero Spillage Guarantee
//!
//! **CRITICAL**: All wavefront execution uses `options(nomem, nostack)` on
//! inline assembly. No stack access, no heap allocation, no memory operations
//! outside the register file.
//!
//! # Taxon Addressing
//!
//! UOR provides bijective mapping between bytes (0-255) and Unicode Braille
//! characters (U+2800-U+28FF), establishing universal identity for every
//! byte value.
//!
//! | Value | Codepoint | Glyph | Domain | Rank |
//! |-------|-----------|-------|--------|------|
//! | 0 | U+2800 | ⠀ | θ | 0 |
//! | 1 | U+2801 | ⠁ | ψ | 0 |
//! | 17 | U+2811 | ⠑ | δ | 5 |
//! | 96 | U+2860 | ⡠ | θ | 32 |
//! | 255 | U+28FF | ⣿ | θ | 85 |
//!
//! # Axiom Derivation
//!
//! All constants derive from two foundational axioms:
//! - **T = 3**: Triality - the number of domains
//! - **O = 8**: Octonion dimension - the number of basis elements
//!
//! # Example: Wavefront Execution
//!
//! ```ignore
//! use uor::{UorState, Wavefront, UorStep};
//! use uor::arch::Zen3Executor;
//!
//! let executor = Zen3Executor::new();
//! let mut state = UorState::zero();
//! let wf = Wavefront::all_xor();
//!
//! // Execute one wavefront cycle (all ports fire)
//! unsafe { executor.step(&mut state, &wf); }
//! ```
//!
//! # Example: Taxon Identity
//!
//! ```
//! use uor::{Taxon, Domain};
//!
//! let t = Taxon::new(17);
//! assert_eq!(t.codepoint(), 0x2811);
//! assert_eq!(t.braille(), '⠑');
//! assert_eq!(t.domain(), Domain::Delta);
//! assert_eq!(t.rank(), 5);
//! ```
//!
//! # Design Principles
//!
//! - **Pure State Transformation**: No side effects, no memory access
//! - **Maximum Port Utilization**: All execution ports fire per wavefront
//! - **Arbitrary Applications**: SHA-256, AES, GF(2^8) are consumers of UOR
//! - **Zero Dependencies**: No heap allocation in core operations

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// Core data types (taxon, domain, word, constants, ring, basis, iri, traits)
mod core;

// UOR Cellular Automaton (isa, state, wavefront)
mod automaton;

// Toroidal boundary substrate (cell, substrate, mutation)
mod boundary;

// SIMD channels
pub mod channel;

// View system for function composition
pub mod view;

// Microcode primitives for backend unification
pub mod microcode;

// Conformance validation
pub mod conformance;

// Architecture-specific implementations
pub mod arch;

// Monodromy observables for computation paths
pub mod observable;

// Content-addressable identifiers using Braille bijection
pub mod address;

// Precomputed lookup tables for O(1) operations
pub mod lut;

// UOR Invariance Frame — type declarations and irreducibility partitions
pub mod frame;

// Re-export submodules for external access (backwards compatibility)
pub use automaton::isa;
pub use automaton::state;
pub use automaton::wavefront;
pub use boundary::cell;
pub use boundary::mutation;
pub use core::basis;
pub use core::constants;
pub use core::domain;
pub use core::iri;
pub use core::ring;
pub use core::taxon;
pub use core::traits;
pub use core::word;

// Re-export commonly used items from submodules
pub use core::basis::BASIS;
pub use core::iri::BASE_IRI;

// Re-export core types at crate root
pub use core::constants::{BRAILLE_BASE, BRAILLE_MAX, BYTE_CARDINALITY, O, T};
pub use core::domain::Domain;
pub use core::taxon::Taxon;
pub use core::traits::{Addressable, Triadic};
pub use core::word::{Word, Word2, Word32, Word4, Word8};

// Re-export UOR cellular automaton types
pub use automaton::isa::{
    PortAssignment, UorStep, UorStepBinary, UorStepFused, UorStepLossless, Wavefront, WavefrontOp,
};
pub use automaton::state::{UorState, GPR_COUNT, GPR_TAXONS, STATE_TAXONS, YMM_COUNT, YMM_TAXONS};
pub use view::ElementWiseView;

// Re-export architecture-specific executors
#[cfg(target_arch = "x86_64")]
pub use arch::x86_64::Zen3AsmExecutor;
#[cfg(target_arch = "x86_64")]
pub use arch::Zen3Executor;

#[cfg(target_arch = "aarch64")]
pub use arch::NeonExecutor;

// Re-export portable executor (available on all architectures)
pub use arch::ScalarExecutor;

// Re-export conformance validation
pub use conformance::{
    bits_per_cycle_from_ns, ns_to_cycles, validate_sequence_latency, validate_throughput,
    validate_wavefront_latency, ConformanceReport, ConformanceTier, ConformanceViolation,
    TARGET_BITS_PER_CYCLE, TARGET_SEQUENCE_64_CYCLES, TARGET_SINGLE_WAVEFRONT_CYCLES,
    UOR_STATE_BITS,
};

// Re-export toroidal boundary types
pub use boundary::cell::{Cell, CellRange, CellRangeIter};
pub use boundary::mutation::{BoundaryMutation, MutationBatch};
pub use boundary::substrate::ToroidalBoundary;
pub use core::constants::{B, BOUNDARY_SIZE};

// Re-export microcode types
pub use microcode::{
    Derivation, DerivationId, KoggeStoneAdder, MicrocodeOps, MicrocodePrimitives, MicrocodeStep,
    MicrocodeWord, ScalarPrimitives,
};

// Re-export content addressing types
pub use address::{Address, AddressParseError, Glyph};

/// Prelude module for convenient imports.
///
/// ```
/// use uor::prelude::*;
/// ```
pub mod prelude {
    // Taxon types
    pub use crate::core::basis::{compose, decompose, weight, BASIS};
    pub use crate::core::constants::{BRAILLE_BASE, BRAILLE_MAX, BYTE_CARDINALITY, O, T};
    pub use crate::core::domain::Domain;
    pub use crate::core::iri::{full_iri, iri_suffix_str, parse_iri, BASE_IRI};
    pub use crate::core::taxon::Taxon;
    pub use crate::core::traits::{Addressable, Triadic};
    pub use crate::core::word::{Word, Word2, Word32, Word4, Word8};

    // Content addressing types
    pub use crate::address::{Address, AddressParseError, Glyph};

    // UOR cellular automaton types
    pub use crate::automaton::isa::{
        PortAssignment, UorStep, UorStepBinary, UorStepFused, UorStepLossless, Wavefront,
        WavefrontOp,
    };
    pub use crate::automaton::state::{
        UorState, GPR_COUNT, GPR_TAXONS, STATE_TAXONS, YMM_COUNT, YMM_TAXONS,
    };

    // Wavefront patterns
    pub use crate::automaton::wavefront::{aes, arith, bitwise, rotate, sha256, ProgramBuilder};

    // View system
    pub use crate::view::ElementWiseView;

    // Architecture-specific executors
    #[cfg(target_arch = "x86_64")]
    pub use crate::arch::Zen3Executor;

    #[cfg(target_arch = "aarch64")]
    pub use crate::arch::NeonExecutor;

    // Portable executor (all architectures)
    pub use crate::arch::ScalarExecutor;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braille_bijection() {
        for i in 0..=255u8 {
            let t = Taxon::new(i);
            assert_eq!(t.value(), i);
            assert_eq!(t.codepoint(), BRAILLE_BASE + i as u32);
        }
    }

    #[test]
    fn test_domain_partition() {
        let mut counts = [0usize; 3];
        for i in 0..=255u8 {
            counts[Taxon::new(i).domain().residue() as usize] += 1;
        }
        assert_eq!(counts, [86, 85, 85]); // Theta, Psi, Delta
    }

    #[test]
    fn test_ring_closure() {
        for i in 0..=255u8 {
            let t = Taxon::new(i);
            assert_eq!(t.succ().pred(), t);
            assert_eq!(t.not().not(), t);
        }
    }

    #[test]
    fn test_basis_roundtrip() {
        for i in 0..=255u8 {
            let t = Taxon::new(i);
            assert_eq!(core::basis::compose(core::basis::decompose(t)), t);
        }
    }

    #[test]
    fn test_iri_roundtrip() {
        for i in 0..=255u8 {
            let t = Taxon::new(i);
            let suffix = core::iri::iri_suffix(t);
            let parsed = core::iri::parse_iri_suffix(&suffix);
            assert_eq!(parsed, Some(t));
        }
    }

    #[test]
    fn test_key_values() {
        // 0 = blank (Theta_0)
        let t0 = Taxon::new(0);
        assert_eq!(t0.braille(), '⠀');
        assert_eq!(t0.domain(), Domain::Theta);

        // 1 = Unity (Psi_0)
        let t1 = Taxon::new(1);
        assert_eq!(t1.braille(), '⠁');
        assert_eq!(t1.domain(), Domain::Psi);

        // 17 = Fermat prime (Delta_5)
        let t17 = Taxon::new(17);
        assert_eq!(t17.braille(), '⠑');
        assert_eq!(t17.domain(), Domain::Delta);
        assert_eq!(t17.rank(), 5);

        // 96 = representative theta taxon at rank 32
        let t96 = Taxon::new(96);
        assert_eq!(t96.braille(), '⡠');
        assert_eq!(t96.domain(), Domain::Theta);
        assert_eq!(t96.rank(), 32);

        // 255 = max (Theta_85)
        let t255 = Taxon::new(255);
        assert_eq!(t255.braille(), '⣿');
        assert_eq!(t255.domain(), Domain::Theta);
        assert_eq!(t255.rank(), 85);
    }

    #[test]
    fn test_16_17_twin() {
        // The 16/17 "Fermat twin" relationship
        let t16 = Taxon::new(16);
        let t17 = Taxon::new(17);

        // 16 XOR 17 = 1 (Unity) - expressed via raw bytes
        assert_eq!(t16.value() ^ t17.value(), 1);

        // 16 is Psi (2^4, power of 2 with even exponent)
        // 17 is Delta (2^4 + 1)
        assert_eq!(t16.domain(), Domain::Psi);
        assert_eq!(t17.domain(), Domain::Delta);
    }
}
