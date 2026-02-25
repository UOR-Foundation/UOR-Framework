# Fiber Budget

## Definition

The **fiber budget** formalizes the completeness criterion for type resolution
in the UOR framework. The ring R_n = Z/(2^n)Z admits an iterated Z/2Z fibration
with exactly n binary fibers. Each constraint applied during resolution **pins**
one or more of these fibers. When all n fibers are pinned, the type is fully
resolved and the partition is complete.

## Fiber Coordinates

A {@class https://uor.foundation/partition/FiberCoordinate} represents a single
binary degree of freedom in the ring's structure. Each fiber has a position
within the fibration and a state indicating whether it has been determined:

| Property | Range | Description |
|----------|-------|-------------|
| {@prop https://uor.foundation/partition/fiberPosition} | xsd:nonNegativeInteger | Zero-based position in the fibration (0 = LSB) |
| {@prop https://uor.foundation/partition/fiberState} | xsd:string | Either `"pinned"` or `"free"` |

## The Budget

A {@class https://uor.foundation/partition/FiberBudget} tracks how many fibers
have been pinned versus how many remain free:

| Property | Range | Description |
|----------|-------|-------------|
| {@prop https://uor.foundation/partition/totalFibers} | xsd:nonNegativeInteger | Total fibers (= quantum level n) |
| {@prop https://uor.foundation/partition/pinnedCount} | xsd:nonNegativeInteger | Fibers pinned by constraints |
| {@prop https://uor.foundation/partition/freeCount} | xsd:nonNegativeInteger | Fibers still free (= total - pinned) |
| {@prop https://uor.foundation/partition/isClosed} | xsd:boolean | True when all fibers are pinned |
| {@prop https://uor.foundation/partition/hasFiber} | FiberCoordinate | A fiber in this budget |
| {@prop https://uor.foundation/partition/hasPinning} | FiberPinning | A pinning record |

A {@class https://uor.foundation/partition/Partition} links to its budget via
{@prop https://uor.foundation/partition/fiberBudget}.

## Fiber Pinning

A {@class https://uor.foundation/partition/FiberPinning} records which
constraint determined a specific fiber:

| Property | Range | Description |
|----------|-------|-------------|
| {@prop https://uor.foundation/partition/pinnedBy} | Constraint | The constraint that pinned this fiber |
| {@prop https://uor.foundation/partition/pinsCoordinate} | FiberCoordinate | The fiber that was pinned |

## Example: R_4 Budget

For R_4 (n=4), there are 4 fibers. A residue constraint `x ≡ 1 (mod 2)` pins
fiber 0 (the parity bit), leaving 3 free:

```turtle
<https://uor.foundation/instance/budget-R4>
    a                       partition:FiberBudget ;
    partition:totalFibers   "4"^^xsd:nonNegativeInteger ;
    partition:pinnedCount   "1"^^xsd:nonNegativeInteger ;
    partition:freeCount     "3"^^xsd:nonNegativeInteger ;
    partition:isClosed      false .
```

## Completeness

When {@prop https://uor.foundation/partition/isClosed} is `true`, the fiber
budget is closed and the resolution is complete. This corresponds to the
resolver's {@prop https://uor.foundation/resolver/isComplete} flag and a
{@prop https://uor.foundation/resolver/fiberDeficit} of zero.
