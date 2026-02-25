# Implementing PRISM

PRISM (Polymorphic Resolution and Isometric Symmetry Machine) is the reference
implementation pattern for a UOR-compliant resolver.

## Overview

A PRISM implementation must:
1. Accept a {@class https://uor.foundation/query/Query}
2. Use a {@class https://uor.foundation/resolver/Resolver} to factorize the input
3. Produce a {@class https://uor.foundation/partition/Partition}
4. Measure {@class https://uor.foundation/observable/Observable} properties
5. Issue a {@class https://uor.foundation/cert/Certificate}
6. Record a {@class https://uor.foundation/trace/ComputationTrace}

## Step 1: Define the Context

```rust
// Establish the evaluation context at quantum level 8
let context = Context {
    quantum: 8,
    capacity: 256,
};
```

The corresponding ontology individual:
```turtle
<my:ctx>
    a               state:Context ;
    state:quantum   "8"^^xsd:nonNegativeInteger ;
    state:capacity  "256"^^xsd:nonNegativeInteger .
```

## Step 2: Create a Query

Choose the appropriate query type:
- {@class https://uor.foundation/query/CoordinateQuery} for spatial resolution
- {@class https://uor.foundation/query/MetricQuery} for metric measurement
- {@class https://uor.foundation/query/RepresentationQuery} for canonical form

```turtle
<my:query>
    a               query:RepresentationQuery ;
    query:inputType <my:target-address> .
```

## Step 3: Apply the Resolver

The {@class https://uor.foundation/resolver/DihedralFactorizationResolver}
performs the core computation:

```turtle
<my:resolver>
    a                   resolver:DihedralFactorizationResolver ;
    resolver:inputType  <my:type-u8> ;
    resolver:strategy   "dihedral-factorization" .
```

## Step 4: Inspect the Partition

The resolver produces a {@class https://uor.foundation/partition/Partition}:

```turtle
<my:partition>
    a                   partition:Partition ;
    partition:irreducibles  <my:irred-set> ;
    partition:reducibles    <my:red-set> ;
    partition:units         <my:unit-set> ;
    partition:exterior      <my:ext-set> .
```

## Step 5: Issue a Certificate

```turtle
<my:cert>
    a               cert:Certificate ;
    cert:certifies  <my:partition> .
```

Certificate types available:
- {@class https://uor.foundation/cert/TransformCertificate}
- {@class https://uor.foundation/cert/IsometryCertificate}
- {@class https://uor.foundation/cert/InvolutionCertificate}

## Step 6: Record the Trace

```turtle
<my:trace>
    a                   trace:ComputationTrace ;
    trace:certifiedBy   <my:cert> .
```

## Iterative Resolution (Amendment 11)

The single-pass pipeline above works when the type is fully determined. For
partially-constrained types, PRISM supports an iterative resolution loop:

1. **Declare** — create a {@class https://uor.foundation/type/ConstrainedType} with initial constraints
2. **Resolve** — run the resolver to produce a partition with a {@class https://uor.foundation/partition/FiberBudget}
3. **Observe** — check {@prop https://uor.foundation/partition/isClosed} on the budget
4. **Refine** — if not closed, apply a {@class https://uor.foundation/resolver/RefinementSuggestion} to pin more fibers
5. **Iterate** — repeat until the budget closes or convergence stalls

The {@class https://uor.foundation/resolver/ResolutionState} tracks iteration count,
fiber deficit, and {@prop https://uor.foundation/resolver/convergenceRate}. Each
iteration produces a {@class https://uor.foundation/derivation/RefinementStep} recording
the applied constraint and fibers closed.

## Composition (Amendment 12)

Transforms compose categorically via {@class https://uor.foundation/morphism/Composition}.
The critical composition law {@ind https://uor.foundation/morphism/criticalComposition}
asserts that `neg ∘ bnot = succ`, connecting the two involutions to cyclic rotation.

Use {@class https://uor.foundation/morphism/Identity} for identity transforms and
{@prop https://uor.foundation/morphism/composesWith} to declare composability.

## Complete Example

See SHACL test `test7_end_to_end` in `conformance/src/tests/fixtures/test7_end_to_end.rs`
for a complete single-pass pipeline, and `test12_factorization` for a full PRISM pipeline
with fiber budget and certification using {@prop https://uor.foundation/cert/certifies}
and {@prop https://uor.foundation/trace/certifiedBy}.
