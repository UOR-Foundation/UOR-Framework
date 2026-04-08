# CLAUDE.md — UOR-Framework

## Project overview

Rust workspace encoding the UOR Foundation ontology as typed data structures, a generated `#![no_std]` trait crate (`uor-foundation`), and validated serializations (JSON-LD, Turtle, N-Triples, OWL RDF/XML, JSON Schema, SHACL Shapes, EBNF). All source code, documentation, and web artifacts are machine-generated from the authoritative ontology defined in `spec/`.

## Workspace layout

| Crate | Path | Published | Purpose |
|---|---|---|---|
| `uor-ontology` | `spec/` | no | Ontology source of truth (classes, properties, individuals, serializers) |
| `uor-codegen` | `codegen/` | no | Ontology-to-Rust trait generator |
| `uor-foundation` | `foundation/` | **crates.io** | Generated `#![no_std]` trait library — never edit manually |
| `uor-conformance` | `conformance/` | no | Conformance suite (OWL, SHACL, RDF, Rust API, docs, website) — check count in `spec/src/counts.rs` |
| `uor-docs` | `docs/` | no | Documentation generator |
| `uor-website` | `website/` | no | Static site generator |
| `uor-clients` | `clients/` | no | CLI binaries: `uor-build`, `uor-crate`, `uor-docs`, `uor-website`, `uor-conformance` |

## Critical rules

- **Never hand-edit `foundation/src/`** — it is regenerated from `spec/` by `uor-crate`. CI enforces `git diff --exit-code foundation/src/`.
- **All clippy warnings are errors.** CI runs `cargo clippy --all-targets -- -D warnings`.
- **Every crate denies:** `clippy::unwrap_used`, `clippy::expect_used`, `clippy::panic`, `missing_docs`, `clippy::missing_errors_doc`.
- **Formatting is enforced.** CI runs `cargo fmt --check`.
- **The conformance suite must pass.** `cargo run --bin uor-conformance` — zero failures allowed (check count in `spec/src/counts.rs`).
- **No `unsafe` code.** The `uor-foundation` crate is `#![no_std]` with zero dependencies.
- **Bracket-escape doc comments.** Use `normalize_comment()` to prevent rustdoc intra-doc link warnings on `[text]` in comments.

## Build commands

```sh
cargo fmt --check                    # Format check
cargo clippy --all-targets -- -D warnings  # Lint
cargo test                           # Unit + integration tests
cargo run --bin uor-crate            # Regenerate foundation/src/ from spec/
cargo run --bin uor-build            # Emit JSON-LD, Turtle, N-Triples to public/
cargo run --bin uor-docs             # Generate documentation site
cargo run --bin uor-website          # Generate website
cargo run --bin uor-conformance      # Run full conformance suite
```

Docs/website/conformance binaries accept `PUBLIC_BASE_PATH` env var for URL prefixing.

## CI pipeline (in order)

`cargo fmt --check` → `cargo clippy` → `cargo test` → `cargo run --bin uor-crate` → `git diff --exit-code foundation/src/` → `cargo check -p uor-foundation --no-default-features` → `cargo publish --dry-run` → `uor-build` → `uor-docs` → `uor-website` → `uor-conformance` → deploy pages

## Ontology architecture

- **33 namespaces**, assembly order: `u → schema → op → query → resolver → type → partition → observable → carry → homology → cohomology → proof → derivation → trace → cert → morphism → state → reduction → convergence → division → interaction → monoidal → operad → effect → predicate → parallel → stream → failure → linear → recursion → region → boundary → conformance`
- **Space classification:** Kernel (17: `u`, `schema`, `op`, `carry`, `reduction`, `convergence`, `division`, `monoidal`, `operad`, `effect`, `predicate`, `parallel`, `stream`, `failure`, `linear`, `recursion`, `region`), Bridge (13: `query`, `resolver`, `partition`, `observable`, `homology`, `cohomology`, `proof`, `derivation`, `trace`, `cert`, `interaction`, `boundary`, `conformance`), User (`type`, `morphism`, `state`)
- **441 classes** → 423 traits + 18 enum classes (includes WittLevel newtype struct)
- **892 properties** → 856 trait methods (generic over `P: Primitives`)
- **3356 named individuals** → 1501 constant modules
- **18 enum classes:** `AchievabilityStatus`, `ComplexityClass`, `ExecutionPolicyKind`, `GeometricCharacter`, `GroundingPhase`, `MeasurementUnit`, `MetricAxis`, `PhaseBoundaryType`, `ProofStrategy`, `QuantifierKind`, `RewriteRule`, `SessionBoundaryType`, `TriadProjection`, `ValidityScopeKind`, `VarianceAnnotation`, `VerificationDomain`, `ViolationKind`, `WittLevel`

## Code generation patterns

- All traits are generic over `P: Primitives` (no hardcoded XSD types)
- Enum classes are detected by `detect_vocabulary_enum()` and skip trait generation; WittLevel is a struct (not enum) but also skips trait generation
- `object_property_enum_override()` maps ObjectProperties to enum/struct return types (delegates to `enum_class_names()`)
- Multi-value IriRef properties on individuals → `&[&str]` slices via `BTreeMap` grouping
- `RustFile::finish()` trims trailing whitespace to match `cargo fmt`
- Module declarations in `mod.rs` are sorted alphabetically
- Cross-namespace domain properties and enum-class domain properties are not generated

## Conformance categories

1. **Rust source** — formatting, line width, public API surface
2. **Ontology inventory** — exact namespace/class/property/individual counts
3. **JSON-LD 1.1** — `@context`, `@graph`, non-functional property arrays
4. **OWL 2 DL** — disjointness, functionality, domain/range constraints
5. **RDF / Turtle** — serialization format, prefixes, IRIs
6. **SHACL** — shapes (1:1 with classes), instance test graphs (counts in `spec/src/counts.rs`)
7. **Generated crate** — trait/method/enum/constant counts, `#![no_std]` build
8. **Documentation + Website** — completeness, accessibility, broken links

## Centralized counts

All inventory counts are in **`spec/src/counts.rs`** — the single file to update when ontology terms change. All crates import from `uor_ontology::counts`. Enum class names are centralized in `Ontology::enum_class_names()` in `spec/src/model.rs`. The version string is auto-derived from `Cargo.toml` via `env!("CARGO_PKG_VERSION")`.

## Editing workflow

1. Modify the ontology in `spec/src/namespaces/`
2. Update counts in `spec/src/counts.rs` (single file)
3. Run `cargo run --bin uor-crate` to regenerate `foundation/src/`
4. Run `cargo fmt`
5. Run `cargo clippy --all-targets -- -D warnings`
6. Run `cargo test`
7. Run `cargo run --bin uor-conformance` (full validation)

## Release process

See `RELEASING.md`. Summary: bump version in root `Cargo.toml`, regenerate, commit, tag `vX.Y.Z`, push. CI publishes to crates.io and GitHub Pages.

## Toolchain

- Rust stable (edition 2021, MSRV 1.70)
- Components: `rustfmt`, `clippy`
- `clippy.toml`: `too-many-lines-threshold = 100`, `avoid-breaking-exported-api = false`
- License: Apache-2.0
