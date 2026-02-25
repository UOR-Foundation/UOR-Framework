//! Wavefront operation implementations via inline assembly.
//!
//! Each module implements a specific operation type with guaranteed
//! zero memory access during execution.

// ALU Operations (Ports 1 and 5)
pub mod add;
pub mod and;
pub mod not;
pub mod or;
pub mod sub;
pub mod xor;

// Rotation/Shift Operations (Port 0)
pub mod rotate;
pub mod shift;

// Crypto Operations (SHA-NI on Port 0, AES-NI on Ports 1/5)
pub mod aes;
pub mod sha256;

// Permutation Operations (Port 5)
pub mod permute;
pub mod shuffle;
