//! Rust implementation standards validators.
//!
//! These validators check that the Rust source code meets the project's
//! conventions — items that clippy/rustfmt don't cover automatically.

pub mod all_features_build_check;
pub mod alloc_build_check;
pub mod api;
pub mod bridge_namespace_completion;
pub mod const_fn_frontier;
pub mod driver_shape;
pub mod escape_hatch_lint;
pub mod feature_flag_layout;
pub mod grounding_combinator_check;
pub mod multiplication_resolver;
pub mod no_std_build_check;
pub mod parametric_constraints;
pub mod phantom_tag;
pub mod public_api_snapshot;
pub mod style;
pub mod uor_foundation_verify_build;
pub mod uor_time_surface;
pub mod witt_tower_completeness;
