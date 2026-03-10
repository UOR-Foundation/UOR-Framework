# UOR Foundation

The Universal Object Reference (UOR) Framework is a formal mathematical
framework for content-addressed, algebraically-structured object spaces.
It specifies how every object in a ring substrate Z/(2^n)Z is uniquely
identified, decomposed, and verified through algebraic operations grounded
in the dihedral group D_{2^n}.

## How It Works

1. **Define** — Declare a type with constraints (residue classes, carry
   patterns, depth bounds) that pin fibers of the iterated Z/2Z fibration.
2. **Resolve** — The resolver pipeline factorizes the element under the
   dihedral group, classifies it into a four-component partition (irreducible,
   reducible, unit, exterior), and measures observables.
3. **Certify** — A certificate attests the resolution result with a
   verification hash and computation trace, enabling independent replay.

## Key Concepts

- **Ring substrate**: Z/(2^n)Z carries two involutions — ring negation and
  bitwise complement — whose interaction is governed by the critical identity
  `neg(bnot(x)) = succ(x)`.
- **Partition**: Every ring element is classified as irreducible, reducible,
  a unit, or exterior.
- **Fiber budget**: Constraints pin fibers; resolution is complete when all
  fibers are pinned.
- **Structural reasoning**: Homology and cohomology detect topological
  obstructions in constraint spaces — Betti numbers diagnose convergence,
  sheaf cohomology detects gluing failures.

## Formal Artifacts

- **OWL 2 DL ontology**: 16 namespaces, 213 classes, 436 properties, 758
  named individuals.
- **SHACL shapes**: 213 NodeShapes validating instance data.
- **Algebraic identities**: 336 machine-verified identities spanning 10
  verification domains.
- **Rust trait crate**: `uor-foundation` on crates.io — 193 traits generated
  directly from the ontology.

## Downloads

- [JSON-LD](https://github.com/UOR-Framework/UOR-Framework/releases) —
  `uor.foundation.jsonld`
- [Turtle](https://github.com/UOR-Framework/UOR-Framework/releases) —
  `uor.foundation.ttl`
- [N-Triples](https://github.com/UOR-Framework/UOR-Framework/releases) —
  `uor.foundation.nt`
- [crates.io](https://crates.io/crates/uor-foundation) — `uor-foundation`

[Browse the ontology namespaces](/namespaces/) or [read the documentation](/docs/overview.html).
