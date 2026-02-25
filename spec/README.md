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
use uor_spec::{Ontology, iris};

let ontology = Ontology::full();
assert_eq!(ontology.namespaces.len(), 14);
assert_eq!(ontology.class_count(), 98);

// Look up a class by IRI
let address = ontology.find_class("https://uor.foundation/u/Address");
assert!(address.is_some());

// Look up a namespace by prefix
let schema = ontology.find_namespace("schema");
assert_eq!(schema.map(|m| m.namespace.iri), Some(iris::NS_SCHEMA));

// Serialize to JSON-LD (requires `serializers` feature, enabled by default)
let json_ld = uor_spec::serializer::jsonld::to_json_ld(ontology);

// Serialize to Turtle
let turtle = uor_spec::serializer::turtle::to_turtle(ontology);
```

## Feature flags

| Feature | Default | Description |
|---------|---------|-------------|
| `serde` | yes | Adds `Serialize` derive to all model types |
| `serializers` | yes | JSON-LD, Turtle, and N-Triples serializers (pulls in `serde_json`) |

For types only (no extra dependencies):

```toml
uor-spec = { version = "1.1", default-features = false }
```

## License

Apache-2.0 — see [LICENSE](https://github.com/UOR-Foundation/UOR-Framework/blob/main/LICENSE).
