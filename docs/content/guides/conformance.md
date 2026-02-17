# Conformance Guide

## Overview

The UOR conformance suite validates all workspace artifacts against professional
standards. Run it with:

```sh
cargo run --bin uor-conformance
```

## What Is Validated

### Ontology Conformance

| Artifact | Standard | Validator |
|----------|----------|-----------|
| `public/uor.foundation.json` | JSON-LD 1.1 | `validators/ontology/jsonld.rs` |
| `public/uor.foundation.json` | OWL 2 DL | `validators/ontology/owl.rs` |
| Inventory counts | 14/82/118/14 | `validators/ontology/inventory.rs` |
| `public/uor.foundation.ttl` | RDF 1.1 / Turtle 1.1 | `validators/ontology/rdf.rs` |
| 7 test instance graphs | SHACL | `validators/ontology/shacl.rs` |

### Documentation Conformance

| Check | Validator |
|-------|-----------|
| All 82 classes documented | `validators/docs/completeness.rs` |
| Namespace pages accurate | `validators/docs/accuracy.rs` |
| Diataxis structure present | `validators/docs/structure.rs` |
| No broken internal links | `validators/docs/links.rs` |

### Website Conformance

| Check | Standard | Validator |
|-------|----------|-----------|
| HTML5 structure | HTML5 | `validators/website/html.rs` |
| Accessibility | WCAG 2.1 AA | `validators/website/accessibility.rs` |
| Namespace page coverage | — | `validators/website/coverage.rs` |
| CSS validity | CSS | `validators/website/css.rs` |
| Internal links | — | `validators/website/links.rs` |

## Adding a New SHACL Test

1. Create `conformance/src/tests/fixtures/test<n>_<name>.rs`
2. Define a `pub const TEST<N>_<NAME>: &str = r#"..."#;` with Turtle source
3. Export it from `conformance/src/tests/fixtures/mod.rs`
4. Register it in `conformance/src/validators/ontology/shacl.rs`
5. Add a check function `validate_<name>(src: &str) -> Result<(), String>`

## Running Individual Validators

The conformance library is structured so each validator can be called independently:

```rust
use uor_conformance::validators::ontology::owl;

let report = owl::validate();
assert!(report.all_passed());
```

## CI Integration

The CI workflow runs full conformance as the last step:

```yaml
- run: cargo run --bin uor-conformance  # exits non-zero on failure
```
