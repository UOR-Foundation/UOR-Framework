//! Core data types and constants.
//!
//! This module contains the fundamental types and constants that form the
//! mathematical foundation of UOR:
//!
//! - [`taxon::Taxon`]: Universal byte reference (0-255 ↔ U+2800-U+28FF)
//! - [`domain::Domain`]: Triadic domains (Theta/Psi/Delta)
//! - [`word::Word`], [`word::Word2`], [`word::Word4`], [`word::Word8`], [`word::Word32`]: Multi-taxon containers
//! - [`constants`]: `T`, `O`, `B` and derived values
//! - [`ring`]: Wrapping byte algebra helpers
//! - [`basis`]: Binary basis decomposition

pub mod basis;
pub mod constants;
pub mod domain;
pub mod iri;
pub mod ring;
pub mod taxon;
pub mod traits;
pub mod word;
