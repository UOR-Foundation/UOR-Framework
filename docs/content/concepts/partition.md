# Partition

## Definition

A **partition** in the UOR framework is a decomposition of the ring R_n into
disjoint components. The class {@class https://uor.foundation/partition/Partition}
represents this decomposition.

## Four Components

Every partition of R_n has exactly four component sets:

| Class | Description |
|-------|-------------|
| {@class https://uor.foundation/partition/IrreducibleSet} | Elements with no non-trivial factorization |
| {@class https://uor.foundation/partition/ReducibleSet} | Elements that factor into smaller pieces |
| {@class https://uor.foundation/partition/UnitSet} | Invertible elements (units of the ring) |
| {@class https://uor.foundation/partition/ExteriorSet} | Elements outside the kernel |

These four sets are mutually `owl:disjointWith` and their cardinalities sum to 2^n.

## Ontology Properties

| Property | Domain | Range | Description |
|----------|--------|-------|-------------|
| {@prop https://uor.foundation/partition/irreducibles} | Partition | IrreducibleSet | Link to irreducible set |
| {@prop https://uor.foundation/partition/reducibles} | Partition | ReducibleSet | Link to reducible set |
| {@prop https://uor.foundation/partition/units} | Partition | UnitSet | Link to unit set |
| {@prop https://uor.foundation/partition/exterior} | Partition | ExteriorSet | Link to exterior set |
| {@prop https://uor.foundation/partition/cardinality} | Component | xsd:nonNegativeInteger | Element count |
| {@prop https://uor.foundation/partition/density} | Component | xsd:string | Density as fraction |
| {@prop https://uor.foundation/partition/member} | Component | partition:Component | Member element |
| {@prop https://uor.foundation/partition/sourceType} | Partition | type:TypeDefinition | Source type |
| {@prop https://uor.foundation/partition/quantum} | Partition | xsd:nonNegativeInteger | Ring quantum level |

## Example: R_4

For R_4 = Z/16Z (n=4, 16 elements):

```turtle
<https://uor.foundation/instance/partition-R4>
    a                   partition:Partition ;
    schema:ringQuantum  "4"^^xsd:nonNegativeInteger ;
    partition:irreducibles  <...irred-set-R4> ;
    partition:reducibles    <...red-set-R4> ;
    partition:units         <...unit-set-R4> ;
    partition:exterior      <...ext-set-R4> .
```

## Role in Resolution

The {@class https://uor.foundation/resolver/DihedralFactorizationResolver}
produces a `Partition` as its output. The partition is then used by:
- {@class https://uor.foundation/observable/Observable} to measure properties
- {@class https://uor.foundation/cert/Certificate} to certify correctness
- {@class https://uor.foundation/morphism/Transform} to apply transformations
