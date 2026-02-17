# Content Addressing

## Definition

**Content addressing** is the principle that an object's identity is determined by
its content, not by an external location or name. In the UOR framework, this is
formalized through the {@class https://uor.foundation/u/Address} class.

## Mathematical Basis

A content address is a canonical representation of an object derived from its bytes.
The UOR framework uses the ring structure (see [Ring](ring.html)) to define a
**canonical form** — a unique representative for each equivalence class of objects.

The {@class https://uor.foundation/resolver/CanonicalFormResolver} computes this
canonical form by factorizing an object's representation in the dihedral group
D_{2^n}.

## Ontology Representation

The addressing namespace `u/` provides two foundational classes:

| Class | Description |
|-------|-------------|
| {@class https://uor.foundation/u/Address} | Universal content address |
| {@class https://uor.foundation/u/Glyph} | Canonical glyph (minimal representative) |

Properties in the `u/` namespace:

| Property | Description |
|----------|-------------|
| {@prop https://uor.foundation/u/glyph} | The glyph of an address |
| {@prop https://uor.foundation/u/codepoint} | Unicode code point |
| {@prop https://uor.foundation/u/byteValue} | Byte value in R_n |
| {@prop https://uor.foundation/u/length} | Length in bytes |

## Resolution

Given a content address, the {@class https://uor.foundation/resolver/Resolver}
hierarchy performs resolution:

1. **DihedralFactorizationResolver** — factorizes in D_{2^n}
2. **CanonicalFormResolver** — computes the canonical form
3. **EvaluationResolver** — evaluates the canonical form

The result is a {@class https://uor.foundation/partition/Partition} decomposing
the address into irreducible, reducible, unit, and exterior components.

## Schema Integration

The {@class https://uor.foundation/schema/Datum} class represents raw byte content,
while {@class https://uor.foundation/schema/Term} represents symbolic content.
These two are `owl:disjointWith` — a datum and a term are fundamentally different
kinds of things.

A {@class https://uor.foundation/schema/Literal} (a subclass of `Term`) can
**denote** a `Datum` via the {@prop https://uor.foundation/schema/denotes} property,
bridging the symbolic and data layers without conflating them.
