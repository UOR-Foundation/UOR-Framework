# CLAUDE.md — UOR-Framework

## Project overview

Rust workspace encoding the UOR Foundation ontology as typed data structures, a generated `#![no_std]` trait crate (`uor-foundation`), and validated serializations (JSON-LD 1.1, Turtle 1.1, N-Triples). All source code, documentation, and web artifacts are machine-generated from the authoritative ontology defined in `spec/`.

## Workspace layout

| Crate | Path | Published | Purpose |
|---|---|---|---|
| `uor-ontology` | `spec/` | no | Ontology source of truth (classes, properties, individuals, serializers) |
| `uor-codegen` | `codegen/` | no | Ontology-to-Rust trait generator |
| `uor-foundation` | `foundation/` | **crates.io** | Generated `#![no_std]` trait library — never edit manually |
| `uor-conformance` | `conformance/` | no | 160-check conformance suite (OWL, SHACL, RDF, Rust API, docs, website) |
| `uor-docs` | `docs/` | no | Documentation generator |
| `uor-website` | `website/` | no | Static site generator |
| `uor-clients` | `clients/` | no | CLI binaries: `uor-build`, `uor-crate`, `uor-docs`, `uor-website`, `uor-conformance` |

## Critical rules

- **Never hand-edit `foundation/src/`** — it is regenerated from `spec/` by `uor-crate`. CI enforces `git diff --exit-code foundation/src/`.
- **All clippy warnings are errors.** CI runs `cargo clippy --all-targets -- -D warnings`.
- **Every crate denies:** `clippy::unwrap_used`, `clippy::expect_used`, `clippy::panic`, `missing_docs`, `clippy::missing_errors_doc`.
- **Formatting is enforced.** CI runs `cargo fmt --check`.
- **The conformance suite must pass.** `cargo run --bin uor-conformance` — 160 checks, zero failures allowed.
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

- **16 namespaces**, assembly order: `u → schema → op → query → resolver → type → partition → observable → homology → cohomology → proof → derivation → trace → cert → morphism → state`
- **Space classification:** Kernel (`u`, `schema`, `op`), Bridge (10 namespaces), User (`type`, `morphism`, `state`)
- **205 classes** → 193 traits + 11 enum classes + 1 struct (QuantumLevel)
- **408 properties** → trait methods (generic over `P: Primitives`)
- **740 named individuals** → constant modules
- **11 enum classes:** `MetricAxis`, `GeometricCharacter`, `VerificationDomain`, `ComplexityClass`, `RewriteRule`, `MeasurementUnit`, `CoordinateKind`, `SessionBoundaryType`, `PhaseBoundaryType`, `SaturationPhase`, `AchievabilityStatus`

## Code generation patterns

- All traits are generic over `P: Primitives` (no hardcoded XSD types)
- Enum classes are detected by `detect_vocabulary_enum()` and skip trait generation; QuantumLevel is a struct (not enum) but also skips trait generation
- `object_property_enum_override()` maps 12 ObjectProperties to enum/struct return types
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
6. **SHACL** — 200 shapes (1:1 with classes), 74 instance test graphs
7. **Generated crate** — trait/method/enum/constant counts, `#![no_std]` build
8. **Documentation + Website** — completeness, accessibility, broken links

## Hardcoded count assertions

When adding classes, properties, or individuals, update counts in **all** of these:
- `spec/src/lib.rs`
- `conformance/src/lib.rs`
- `docs/src/lib.rs`
- `website/src/lib.rs`

## Editing workflow

1. Modify the ontology in `spec/src/namespaces/`
2. Update counts in the four files listed above
3. Run `cargo run --bin uor-crate` to regenerate `foundation/src/`
4. Run `cargo fmt`
5. Run `cargo clippy --all-targets -- -D warnings`
6. Run `cargo test`
7. Run `cargo run --bin uor-conformance` (full validation, 160 checks)

## Release process

See `RELEASING.md`. Summary: bump version in root `Cargo.toml`, regenerate, commit, tag `vX.Y.Z`, push. CI publishes to crates.io and GitHub Pages.

## Toolchain

- Rust stable (edition 2021, MSRV 1.70)
- Components: `rustfmt`, `clippy`
- `clippy.toml`: `too-many-lines-threshold = 100`, `avoid-breaking-exported-api = false`
- License: Apache-2.0
