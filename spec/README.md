# uor-ontology

Internal crate encoding the complete [UOR Foundation](https://uor.foundation/)
ontology as typed Rust data structures, with serializers for JSON-LD, Turtle,
and N-Triples.

**This crate is not published to crates.io** (`publish = false`). For the
published Rust trait library, see
[`uor-foundation`](https://crates.io/crates/uor-foundation).

## Contents

- 14 namespaces in dependency order
- 98 OWL classes
- 167 OWL properties (166 namespace-level + 1 global annotation)
- 18 named individuals
- Three serialization formats: JSON-LD 1.1, Turtle 1.1, N-Triples

## Usage

```rust
use uor_ontology::{Ontology, iris};

let ontology = Ontology::full();
assert_eq!(ontology.namespaces.len(), 14);
assert_eq!(ontology.class_count(), 98);

// Look up a class by IRI
let address = ontology.find_class("https://uor.foundation/u/Address");
assert!(address.is_some());

// Serialize to JSON-LD (requires `serializers` feature, enabled by default)
let json_ld = uor_ontology::serializer::jsonld::to_json_ld(ontology);

// Serialize to Turtle
let turtle = uor_ontology::serializer::turtle::to_turtle(ontology);
```

## Feature flags

| Feature | Default | Description |
|---------|---------|-------------|
| `serde` | yes | Adds `Serialize` derive to all model types |
| `serializers` | yes | JSON-LD, Turtle, and N-Triples serializers (pulls in `serde_json`) |

## License

Apache-2.0 — see [LICENSE](https://github.com/UOR-Foundation/UOR-Framework/blob/main/LICENSE).
