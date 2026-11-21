# Documentation Standards

## Framework: Diataxis

Documentation follows the [Diataxis framework](https://diataxis.fr/), organized into four quadrants:

| Section | Path | Purpose |
|---------|------|---------|
| Reference | `public/docs/namespaces/` | Precise technical reference for all 34 namespaces |
| Explanation | `public/docs/concepts/` | Understanding the UOR mathematical model |
| How-to | `public/docs/guides/` | Task-oriented guides for practitioners |
| Tutorial | (homepage overview) | Introductory learning paths |

## Completeness

Every ontology term must be documented:
- All 474 classes must appear in at least one documentation page.
- All 948 properties must appear in at least one documentation page.
- All 3601 named individuals must appear in at least one documentation page.

## Accuracy

Documentation accuracy is enforced at build time:
- Namespace reference pages are 100% auto-generated from `uor_ontology::Ontology::full()`.
- Hand-written prose uses `{@class <iri>}`, `{@prop <iri>}`, `{@ind <iri>}` DSL for ontology references.
- All DSL references are validated against the live spec — invalid IRIs cause build failure.

## Reference DSL

Prose content files use a structured syntax for ontology references:

```
{@class https://uor.foundation/schema/Ring}       ← verified class reference
{@prop  https://uor.foundation/op/composedOf}     ← verified property reference
{@ind   https://uor.foundation/op/neg}            ← verified individual reference
```

Rendered as: `[Ring](../namespaces/schema.html#Ring)` with the class label from the spec.

## Cross-Linking

- All `{@class}`, `{@prop}`, `{@ind}` references render as links to the appropriate namespace page.
- Internal cross-references between concept pages use relative paths.
- No broken links (validated by `conformance/validators/docs/links.rs`).

## Prose Guidelines

- Use plain language. Avoid jargon where ordinary terms suffice.
- Reference the mathematical basis (ring theory, group theory) when accurate.
- Technical claims about the ontology (domain, range, functional properties) must be verified
  via the `[[claims]]` TOML front-matter mechanism.
- Each concept page: definition, motivation, mathematical basis, ontology representation, examples.
- Concept pages should cross-reference related concepts to help readers navigate the connected narrative.

## Public README and generated reference

- The root `README.md` is a human-curated, public-facing introduction.
- It must explain **why** the project exists, **what** it provides, and **how**
  an unfamiliar reader can begin, in plain language suitable for around a
  tenth-grade reading level. Mathematics belongs in progressively deeper
  technical documentation, not the first paragraph.
- A banner, badges, working links, and clear navigation are expected, but
  structural compliance alone does not establish communication quality.
- `cargo run --bin uor-docs` generates `docs/DEVELOPER_REFERENCE.md` and
  HTML documentation. It must not overwrite the root README.
- The generator accepts `--reference <path>` and the old `--readme` option
  as a backwards-compatible alias; its default output is the developer reference.
- CI compares the generated reference against the committed
  `docs/DEVELOPER_REFERENCE.md` to detect drift. Generated references must
  preserve working paths relative to their location.
- A cheap classifier checks the entire PR/push file set. Only changes limited
  to `README.md` and `assets/readme-banner.svg` may bypass expensive
  compilation and conformance checks. Documentation generator, source,
  workflow, technical content, and unknown changes must run the full gate.
- The lightweight gate checks required public introduction sections and
  local links, including the existing committed banner. No proposed art
  counts as delivered without a committed asset.

## References

- [Diataxis Framework](https://diataxis.fr/)
- [Write the Docs](https://www.writethedocs.org/guide/)
