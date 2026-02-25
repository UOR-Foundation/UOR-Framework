# uor-spec

The complete [UOR Foundation](https://uor.foundation/) ontology encoded as
typed Rust data structures, with serializers for JSON-LD, Turtle, and
N-Triples.

## Contents

- 14 namespaces in dependency order
- 98 OWL classes
- 167 OWL properties (166 namespace-level + 1 global annotation)
- 18 named individuals
- Three serialization formats: JSON-LD 1.1, Turtle 1.1, N-Triples

## Quick start

```rust
use uor_spec::Ontology;

let ontology = Ontology::full();
assert_eq!(ontology.namespaces.len(), 14);
assert_eq!(ontology.class_count(), 98);

// Serialize to JSON-LD
let json_ld = uor_spec::serializer::jsonld::to_json_ld(ontology);
println!("{}", serde_json::to_string_pretty(&json_ld).unwrap());

// Serialize to Turtle
let turtle = uor_spec::serializer::turtle::to_turtle(ontology);
println!("{turtle}");
```

## License

Apache-2.0 — see [LICENSE](https://github.com/UOR-Foundation/UOR-Framework/blob/main/LICENSE).
