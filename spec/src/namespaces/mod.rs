//! UOR Foundation namespace modules.
//!
//! Each sub-module encodes one namespace of the UOR ontology as Rust static data.
//! Modules are listed in dependency order; see [`crate::Ontology::full`] for the
//! assembly sequence.

pub mod u;
pub mod schema;
pub mod op;
pub mod query;
pub mod resolver;
pub mod type_;
pub mod partition;
pub mod observable;
pub mod proof;
pub mod derivation;
pub mod trace;
pub mod cert;
pub mod morphism;
pub mod state;
