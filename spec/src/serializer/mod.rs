//! Serializers for the UOR Foundation ontology.
//!
//! Three serialization formats are supported:
//! - **JSON-LD** ([`jsonld`]) — the canonical format, output to `public/uor.foundation.json`
//! - **Turtle** ([`turtle`]) — for RDF tooling, output to `public/uor.foundation.ttl`
//! - **N-Triples** ([`ntriples`]) — for streaming/bulk processing, output to `public/uor.foundation.nt`

pub mod jsonld;
pub mod ntriples;
pub mod turtle;
