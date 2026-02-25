# Architecture

## Workspace Layout

The UOR Framework is a Rust workspace with seven member crates:

| Crate | Type | Role |
|-------|------|------|
| `uor-ontology` | Library | Ontology as typed Rust data + serializers (internal, not published) |
| `uor-codegen` | Library | Code generator: ontology → Rust traits (internal) |
| `uor-foundation` | Library | **Generated** Rust trait crate (published to crates.io) |
| `uor-conformance` | Library | Workspace-wide conformance validators |
| `uor-docs` | Library | Documentation generator |
| `uor-website` | Library | Static site generator |
| `uor-clients` | Binaries | CLI tools: build, conformance, docs, website, crate |

## Dependency Order

```
uor-ontology (spec/)
  ↓ Ontology::full()
  ├── uor-build      → public/uor.foundation.{json,ttl,nt}   (RDF/OWL export)
  ├── uor-docs       → public/docs/                           (documentation export)
  ├── uor-website    → public/                                 (website export)
  ├── uor-crate      → foundation/src/                         (Rust export)
  └── uor-conformance → validates all of the above
```

## spec/ Library (uor-ontology)

`uor-ontology` encodes the complete ontology as typed Rust static data:

- **`model.rs`**: Core types — `Namespace`, `Class`, `Property`, `Individual`, `Ontology`
- **`namespaces/*.rs`**: 14 modules, one per namespace (all 12 amendments applied)
- **`serializer/jsonld.rs`**: Serializes to JSON-LD 1.1
- **`serializer/turtle.rs`**: Serializes to Turtle 1.1
- **`serializer/ntriples.rs`**: Serializes to N-Triples

The entry point `Ontology::full()` uses `OnceLock` for thread-safe lazy initialization.

This crate is the single source of truth. It is internal (`publish = false`).

## codegen/ Library (uor-codegen)

`uor-codegen` generates the published Rust trait crate from the ontology:

- **`mapping.rs`**: Namespace → module, XSD → Rust type, class IRI → path tables
- **`traits.rs`**: Class → trait, property → method generation
- **`enums.rs`**: Enum detection (PrimitiveOp, MetricAxis, Space, FiberState, GeometricCharacter)
- **`individuals.rs`**: Named individual → const module / PrimitiveOp impl generation
- **`emit.rs`**: Rust source code writer

## foundation/ Library (uor-foundation)

`uor-foundation` is the **generated** published crate. Every file in `foundation/src/`
is produced by `uor-crate` — never hand-edited.

- **98 traits** (one per OWL class, generic over `Primitives`)
- **164 methods** (one per property with a domain)
- **5 enums** (Space, PrimitiveOp, MetricAxis, FiberState, GeometricCharacter)
- **5 constant modules** (for singleton named individuals)
- **Zero mandatory dependencies** — pure traits

Module structure: `kernel/` (3 namespaces), `bridge/` (8 namespaces), `user/` (3 namespaces).

## conformance/ Library

`uor-conformance` is the workspace quality gate. It validates:

1. **Rust source**: API documentation, style conventions
2. **Ontology artifacts**: JSON-LD 1.1, OWL 2 DL constraints, RDF 1.1, Turtle 1.1, inventory counts
3. **SHACL instances**: 15 test graphs validated against 98 NodeShapes
4. **Generated crate**: Trait completeness, method completeness, individual completeness
5. **Documentation**: Diataxis structure, completeness, accuracy, links
6. **Website**: HTML5, WCAG 2.1 AA, CSS, coverage, links

The conformance suite is the **single gate** — all components must pass before a release.

## docs/ Library

`uor-docs` generates documentation with enforced accuracy:

- Namespace reference pages are **100% auto-generated** from `uor_ontology::Ontology::full()`
- Prose pages use `{@class}`, `{@prop}`, `{@ind}` DSL, validated at build time
- Every spec term must appear in at least one page (completeness check)

## website/ Library

`uor-website` generates the static site at `https://uor.foundation/`:

- Templates use the Tera template engine
- Namespace pages are auto-generated (no hand-written HTML for spec terms)
- Search index is generated from all 98 classes, 166 properties, 18 individuals
- No external dependencies (no CDN, no tracking, no third-party scripts)

## Build Pipeline

```
cargo run --bin uor-build       → public/uor.foundation.{json,ttl,nt}
cargo run --bin uor-crate       → foundation/src/ (generated Rust traits)
cargo run --bin uor-docs        → public/docs/ + README.md
cargo run --bin uor-website     → public/ (HTML, CSS, JS, search-index.json)
cargo run --bin uor-conformance → validates all of the above
```

## Amendment History

The spec crate implements all 12 amendments from the UOR Foundation completion plan:

| Amendment | Namespace | Key Additions |
|-----------|-----------|---------------|
| 1 | op/ | 10 named operation individuals |
| 2 | schema/ | Ring class, 6 properties, pi1/zero individuals |
| 3 | op/, proof/ | Identity class, criticalIdentity individual, provesIdentity property |
| 4 | op/ | Group/DihedralGroup classes, D2n individual |
| 5 | partition/ (NEW) | 6 classes, 9 properties |
| 6 | morphism/ (NEW) | 4 classes, 10 properties |
| 7 | state/ (NEW) | 4 classes, 16 properties |
| 8 | all | uor:space annotation property on all namespaces |
| 9 | partition/ | FiberCoordinate, FiberBudget, FiberPinning; 11 properties |
| 10 | type/ | Constraint hierarchy (6 classes), MetricAxis, 3 axis individuals |
| 11 | resolver/, derivation/ | ResolutionState, RefinementSuggestion, DerivationStep/RefinementStep |
| 12 | morphism/ | Composition, Identity, CompositionLaw; criticalComposition individual |
