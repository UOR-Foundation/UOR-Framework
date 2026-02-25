//! Content-addressable identifiers using Braille bijection.
//!
//! This module implements the UOR content addressing model where byte sequences
//! are represented as Braille glyph strings. Each byte (0-255) maps bijectively
//! to a Unicode Braille character (U+2800-U+28FF).
//!
//! # Ontology Mapping
//!
//! These types correspond to the UOR Foundation ontology:
//! - `Address` → `https://uor.foundation/u/Address`
//! - `Glyph` → `https://uor.foundation/u/Glyph`
//!
//! # Example
//!
//! ```
//! use uor::address::{Address, Glyph};
//!
//! // Create an address from bytes
//! let addr = Address::from_bytes(&[0, 1, 17, 255]);
//! assert_eq!(addr.len(), 4);
//! assert_eq!(addr.to_string(), "⠀⠁⠑⣿");
//!
//! // Parse back from Braille string
//! let parsed = Address::from_braille("⠀⠁⠑⣿").unwrap();
//! assert_eq!(parsed.to_bytes(), vec![0, 1, 17, 255]);
//!
//! // Access individual glyphs
//! let glyph = addr.glyph(2).unwrap();
//! assert_eq!(glyph.byte_value(), 17);
//! assert_eq!(glyph.codepoint(), 0x2811);
//! ```

mod content;
mod glyph;

pub use content::{Address, AddressParseError};
pub use glyph::Glyph;

#[cfg(test)]
mod tests;
