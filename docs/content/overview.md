# UOR Framework Overview

The Universal Object Reference (UOR) Framework is a formal ontology and mathematical
framework for content-addressed object spaces. It provides a unified model for
representing, resolving, and transforming any computable object using an algebraic
substrate based on ring theory and group symmetry.

## Core Ideas

**Content addressing** means an object is identified by *what it is*, not *where it is*.
The UOR framework formalizes this via {@class https://uor.foundation/u/Address}: every
object has a canonical address derived from its content, not from an external naming system.

**The ring substrate** {@class https://uor.foundation/schema/Ring} is the algebraic
foundation: Z/(2^n)Z — integers modulo 2^n. At quantum level n=8 this is the byte ring
(Z/256Z), familiar from computer arithmetic.

**Two involutions** generate the structure:
- {@ind https://uor.foundation/op/neg}: ring negation (reflection)
- {@ind https://uor.foundation/op/bnot}: bitwise complement (hypercube reflection)

These generate the dihedral group D_{2^n}, captured by {@class https://uor.foundation/op/DihedralGroup}.

**The critical identity** {@ind https://uor.foundation/op/criticalIdentity}:
`neg(bnot(x)) = succ(x) for all x ∈ R_n` — successor is the composition of the two involutions.
This is the foundational theorem proved by {@class https://uor.foundation/proof/CriticalIdentityProof}.

## Namespace Layers

The 14 namespaces are organized into three **space** classifications:

**Kernel** (mathematical core):
- {@class https://uor.foundation/u/Address} — universal addressing
- {@class https://uor.foundation/schema/Ring} — ring substrate
- {@class https://uor.foundation/op/Operation} — operations

**Bridge** (resolution infrastructure):
- {@class https://uor.foundation/query/Query} — queries
- {@class https://uor.foundation/resolver/Resolver} — resolvers
- {@class https://uor.foundation/partition/Partition} — partitions
- {@class https://uor.foundation/observable/Observable} — observables
- {@class https://uor.foundation/proof/Proof} — proofs
- {@class https://uor.foundation/derivation/Derivation} — derivations
- {@class https://uor.foundation/trace/ComputationTrace} — traces
- {@class https://uor.foundation/cert/Certificate} — certificates

**User** (application layer):
- {@class https://uor.foundation/type/TypeDefinition} — types
- {@class https://uor.foundation/morphism/Transform} — morphisms
- {@class https://uor.foundation/state/Context} — state

## How It Works

1. A value has a **type** ({@class https://uor.foundation/type/TypeDefinition})
2. A **query** ({@class https://uor.foundation/query/Query}) asks about the value
3. A **resolver** ({@class https://uor.foundation/resolver/Resolver}) factorizes it in the ring
4. The **partition** ({@class https://uor.foundation/partition/Partition}) decomposes the result
5. **Observables** ({@class https://uor.foundation/observable/Observable}) measure properties
6. A **certificate** ({@class https://uor.foundation/cert/Certificate}) attests correctness
7. A **trace** ({@class https://uor.foundation/trace/ComputationTrace}) records the computation
8. **State** ({@class https://uor.foundation/state/Context}) maintains evaluation context
