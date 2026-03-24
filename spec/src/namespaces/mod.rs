//! UOR Foundation namespace modules.
//!
//! Each sub-module encodes one namespace of the UOR ontology as Rust static data.
//! Modules are listed in dependency order; see [`crate::Ontology::full`] for the
//! assembly sequence.

pub mod carry;
pub mod cascade;
pub mod cert;
pub mod cohomology;
pub mod convergence;
pub mod derivation;
pub mod division;
pub mod homology;
pub mod interaction;
pub mod monoidal;
pub mod morphism;
pub mod observable;
pub mod op;
pub mod operad;
pub mod partition;
pub mod proof;
pub mod query;
pub mod resolver;
pub mod schema;
pub mod state;
pub mod trace;
pub mod type_;
pub mod u;
