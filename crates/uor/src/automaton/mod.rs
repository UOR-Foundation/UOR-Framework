//! UOR Cellular Automaton.
//!
//! This module contains the cellular automaton execution model:
//!
//! - [`state::UorState`]: Complete CPU register state (624 taxons = 4992 bits)
//! - [`isa::Wavefront`]: Instructions that fire across all execution ports simultaneously
//! - [`isa::WavefrontOp`]: Individual operation types (XOR, AND, SHA256, AES, etc.)
//! - [`isa::UorStep`], [`isa::UorStepLossless`], [`isa::UorStepFused`]: Execution traits
//! - [`wavefront`]: Wavefront construction patterns (sha256, aes, bitwise, etc.)

pub mod isa;
pub mod state;
pub mod wavefront;
