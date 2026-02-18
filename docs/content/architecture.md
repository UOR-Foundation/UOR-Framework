# Architecture

## Workspace Layout

The UOR Framework is a Rust workspace with five member crates:

| Crate | Type | Role |
|-------|------|------|
| `uor-spec` | Library | Ontology as typed Rust data + serializers |
| `uor-conformance` | Library | Workspace-wide conformance validators |
| `uor-docs` | Library | Documentation generator |
| `uor-website` | Library | Static site generator |
| `uor-clients` | Binaries | CLI tools: build, conformance, docs, website |

## Dependency Order

```
uor-spec
  ↓
uor-conformance (validates spec outputs)
  ↓
uor-docs (generates docs from spec)
  ↓
uor-website (generates site from spec + docs)
  ↓
uor-clients (CLI for all of the above)
```

## spec/ Library

`uor-spec` encodes the complete ontology as typed Rust static data:

- **`model.rs`**: Core types — `Namespace`, `Class`, `Property`, `Individual`, `Ontology`
- **`namespaces/*.rs`**: 14 modules, one per namespace (all 8 amendments applied)
- **`serializer/jsonld.rs`**: Serializes to JSON-LD 1.1
- **`serializer/turtle.rs`**: Serializes to Turtle 1.1
- **`serializer/ntriples.rs`**: Serializes to N-Triples

The entry point `Ontology::full()` uses `OnceLock` for thread-safe lazy initialization.

## conformance/ Library

`uor-conformance` is the workspace quality gate. It validates:

1. **Rust source**: API documentation, style conventions
2. **Ontology artifacts**: JSON-LD 1.1, OWL 2 DL constraints, RDF 1.1, Turtle 1.1, inventory counts
3. **SHACL instances**: 7 test graphs validated against 82 NodeShapes
4. **Documentation**: Diataxis structure, completeness, accuracy, links
5. **Website**: HTML5, WCAG 2.1 AA, CSS, coverage, links

The conformance suite is the **single gate** — all components must pass before a release.

## docs/ Library

`uor-docs` generates documentation with enforced accuracy:

- Namespace reference pages are **100% auto-generated** from `uor_spec::Ontology::full()`
- Prose pages use `{@class}`, `{@prop}`, `{@ind}` DSL, validated at build time
- Every spec term must appear in at least one page (completeness check)

## website/ Library

`uor-website` generates the static site at `https://uor.foundation/`:

- Templates use the Tera template engine
- Namespace pages are auto-generated (no hand-written HTML for spec terms)
- Search index is generated from all 82 classes, 119 properties, 14 individuals
- No external dependencies (no CDN, no tracking, no third-party scripts)

## Build Pipeline

```
cargo run --bin uor-build       → public/uor.foundation.{json,ttl,nt}
cargo run --bin uor-docs        → public/docs/ + README.md
cargo run --bin uor-website     → public/ (HTML, CSS, JS, search-index.json)
cargo run --bin uor-conformance → validates all of the above
```

## Amendment History

The spec crate implements all 8 amendments from the UOR Foundation completion plan:

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
