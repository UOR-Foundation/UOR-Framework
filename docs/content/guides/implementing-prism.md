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
    query:subject   <my:target-address> .
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

## Complete Example

See SHACL test `test7_end_to_end` in `conformance/src/tests/fixtures/test7_end_to_end.rs`
for a complete end-to-end instance graph.
