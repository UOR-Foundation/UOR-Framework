# Constraint Algebra

## Definition

The **constraint algebra** provides composable predicates that refine types by
pinning fiber coordinates. A {@class https://uor.foundation/type/Constraint}
is a predicate that, when applied to a type, determines the value of one or
more fibers in the iterated Z/2Z fibration.

## Constraint Hierarchy

Four concrete constraint kinds are provided, mutually disjoint:

| Class | Description |
|-------|-------------|
| {@class https://uor.foundation/type/ResidueConstraint} | Membership in a residue class: x = r (mod m) |
| {@class https://uor.foundation/type/CarryConstraint} | Carry propagation pattern in ring arithmetic |
| {@class https://uor.foundation/type/DepthConstraint} | Bounds on factorization depth |
| {@class https://uor.foundation/type/CompositeConstraint} | Composition of two or more simpler constraints |

A {@class https://uor.foundation/type/ConstrainedType} links to its constraints via
{@prop https://uor.foundation/type/hasConstraint}.

## Constraint Properties

Each constraint kind has specialized parameters:

| Property | Domain | Range | Description |
|----------|--------|-------|-------------|
| {@prop https://uor.foundation/type/modulus} | ResidueConstraint | xsd:positiveInteger | The modulus m |
| {@prop https://uor.foundation/type/residue} | ResidueConstraint | xsd:nonNegativeInteger | The residue r |
| {@prop https://uor.foundation/type/carryPattern} | CarryConstraint | xsd:string | Binary carry pattern |
| {@prop https://uor.foundation/type/minDepth} | DepthConstraint | xsd:nonNegativeInteger | Minimum depth |
| {@prop https://uor.foundation/type/maxDepth} | DepthConstraint | xsd:nonNegativeInteger | Maximum depth |
| {@prop https://uor.foundation/type/composedFrom} | CompositeConstraint | Constraint | Component constraints |

## Metric Axes

Every constraint operates along a {@class https://uor.foundation/type/MetricAxis},
classified by its geometric effect. The three axes form the tri-metric coordinate
system of UOR:

| Individual | Description |
|------------|-------------|
| {@ind https://uor.foundation/type/verticalAxis} | Ring/additive: residue classes, divisibility |
| {@ind https://uor.foundation/type/horizontalAxis} | Hamming/bitwise: bit positions, carry patterns |
| {@ind https://uor.foundation/type/diagonalAxis} | Incompatibility: the gap between ring and Hamming |

The property {@prop https://uor.foundation/type/metricAxis} assigns each constraint
to its axis. The property {@prop https://uor.foundation/type/crossingCost} records
how many axis boundaries a constraint must traverse.

## Fiber Pinning

The property {@prop https://uor.foundation/type/pinsFibers} declares which
{@class https://uor.foundation/partition/FiberCoordinate} instances a constraint
pins when applied. A {@class https://uor.foundation/type/CompositeConstraint}
pins the union of fibers pinned by its components.

## Example: Residue + Depth

```turtle
<https://uor.foundation/instance/constraint-odd>
    a               type:ResidueConstraint ;
    type:modulus     "2"^^xsd:positiveInteger ;
    type:residue     "1"^^xsd:nonNegativeInteger ;
    type:metricAxis  type:verticalAxis .

<https://uor.foundation/instance/constraint-shallow>
    a               type:DepthConstraint ;
    type:minDepth   "0"^^xsd:nonNegativeInteger ;
    type:maxDepth   "2"^^xsd:nonNegativeInteger ;
    type:metricAxis  type:verticalAxis .
```
