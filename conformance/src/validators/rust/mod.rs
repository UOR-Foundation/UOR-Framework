//! Rust implementation standards validators.
//!
//! These validators check that the Rust source code meets the project's
//! conventions — items that clippy/rustfmt don't cover automatically.

pub mod api;
pub mod bridge_namespace_completion;
pub mod multiplication_resolver;
pub mod parametric_constraints;
pub mod phantom_tag;
pub mod public_api_snapshot;
pub mod style;
pub mod uor_time_surface;
pub mod witt_tower_completeness;
