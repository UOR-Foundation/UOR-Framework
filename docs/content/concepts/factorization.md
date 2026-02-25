# Factorization

## Definition

**Factorization** is the process of decomposing ring elements under the action
of the dihedral group D_{2^n}. The
{@class https://uor.foundation/resolver/DihedralFactorizationResolver}
implements this process, producing a
{@class https://uor.foundation/partition/Partition} that classifies every
element as irreducible, reducible, a unit, or exterior.

## The Resolver

The {@class https://uor.foundation/resolver/DihedralFactorizationResolver} is
a subclass of {@class https://uor.foundation/resolver/Resolver}. It accepts a
type declaration ({@prop https://uor.foundation/resolver/inputType}) and produces
a partition ({@prop https://uor.foundation/resolver/outputType}):

| Property | Description |
|----------|-------------|
| {@prop https://uor.foundation/resolver/strategy} | Dihedral orbit analysis |
| {@prop https://uor.foundation/resolver/complexity} | Computational complexity |

## Partition Output

The output is a four-component partition of R_n:

| Component | Class |
|-----------|-------|
| Irreducible | {@class https://uor.foundation/partition/IrreducibleSet} |
| Reducible | {@class https://uor.foundation/partition/ReducibleSet} |
| Units | {@class https://uor.foundation/partition/UnitSet} |
| Exterior | {@class https://uor.foundation/partition/ExteriorSet} |

The property {@prop https://uor.foundation/partition/density} records the
irreducible density: |Irr| / |Carrier|.

## Fiber Budget Integration

With iterative resolution, factorization now tracks its progress through the
{@class https://uor.foundation/partition/FiberBudget}. Each dihedral orbit
analysis pins fibers, and the resolver's
{@prop https://uor.foundation/resolver/resolutionState} records convergence.

## Example: Factorizing R_8

For R_8 (the byte ring, 256 elements), the factorization resolver identifies
dihedral orbits under the action of D_256:

```turtle
<https://uor.foundation/instance/resolver-factor-R8>
    a                       resolver:DihedralFactorizationResolver ;
    resolver:inputType      <https://uor.foundation/instance/type-u8> ;
    resolver:outputType     <https://uor.foundation/instance/partition-R8> ;
    resolver:strategy       "dihedral orbit analysis" ;
    resolver:complexity     "O(n)" .
```

## Computation Trace

Every factorization produces a {@class https://uor.foundation/trace/ComputationTrace}
recording the operations applied. The trace contains step-by-step records that
can be independently verified by a
{@class https://uor.foundation/cert/Certificate}.

## Role in the Pipeline

Factorization is the first stage of the three-resolver pipeline:
1. **DihedralFactorizationResolver** -- decomposes into orbits
2. {@class https://uor.foundation/resolver/CanonicalFormResolver} -- computes canonical forms
3. {@class https://uor.foundation/resolver/EvaluationResolver} -- evaluates results
