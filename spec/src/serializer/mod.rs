//! Serializers for the UOR Foundation ontology.
//!
//! Four serialization formats are supported:
//! - **EBNF** ([`ebnf`]) — the UOR Term Language grammar, output to `public/uor.term.ebnf`
//! - **JSON-LD** ([`jsonld`]) — the canonical format, output to `public/uor.foundation.json`
//! - **Turtle** ([`turtle`]) — for RDF tooling, output to `public/uor.foundation.ttl`
//! - **N-Triples** ([`ntriples`]) — for streaming/bulk processing, output to `public/uor.foundation.nt`

pub mod ebnf;
pub mod jsonld;
pub mod ntriples;
pub mod turtle;
