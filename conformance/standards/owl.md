# OWL 2 DL Standards

## Overview

The UOR Foundation ontology is expressed in OWL 2 DL. All OWL 2 DL restrictions apply.

## DL Restrictions Applied

- All classes, properties, and individuals must be named (anonymous nodes are disallowed except as blank nodes for restrictions).
- No use of `owl:allValuesFrom` or `owl:someValuesFrom` with non-DL constructs.
- All subClassOf targets must be known class IRIs.
- All domain/range targets must be known class IRIs or XSD datatypes.
- All disjointWith targets must be known class IRIs.
- Individual type assertions must reference known classes.
- No circular imports between namespace modules.
- Functional properties must have at most one value per individual.

## Class Hierarchy Constraints

- `op:UnaryOp` and `op:BinaryOp` are subclasses of `op:Operation`
- `op:Involution` is a subclass of `op:UnaryOp`
- `op:Identity`, `op:Group`, `op:DihedralGroup` are subclasses of `op:Operation`
- `schema:Literal`, `schema:Application`, `schema:Ring` are subclasses of `schema:Term`
- `schema:Datum` and `schema:Term` are `owl:disjointWith`
- `partition:IrreducibleSet`, `partition:ReducibleSet`, `partition:UnitSet`, `partition:ExteriorSet`
  are mutually `owl:disjointWith`
- `state:Context`, `state:Binding`, `state:Frame`, `state:Transition` are mutually `owl:disjointWith`

## Property Constraints

- `op:arity` is a functional DatatypeProperty with domain `op:Operation` and range `xsd:nonNegativeInteger`
- `schema:ringQuantum` is a functional DatatypeProperty on `schema:Ring`
- `proof:provesIdentity` is an ObjectProperty from `proof:Proof` to `op:Identity`
- `state:from` and `state:to` are functional ObjectProperties on `state:Transition`

## Named Individuals

All 14 named individuals must be typed with a known OWL class:
- 10 operation individuals: `op:neg`, `op:bnot`, `op:succ`, `op:pred`, `op:add`, `op:sub`, `op:mul`, `op:xor`, `op:and`, `op:or`
- 2 schema individuals: `schema:pi1`, `schema:zero`
- 1 identity individual: `op:criticalIdentity`
- 1 group individual: `op:D2n`

## References

- [OWL 2 Web Ontology Language — Primer](https://www.w3.org/TR/owl2-primer/)
- [OWL 2 DL Profile](https://www.w3.org/TR/owl2-profiles/#OWL_2_DL)
- [OWL 2 Structural Specification](https://www.w3.org/TR/owl2-syntax/)
