//! Test 8: Fiber budget accounting (Amendment 9).
//!
//! Validates: `partition:FiberBudget` + `partition:FiberCoordinate` with
//! `isClosed`, `pinnedCount`, `freeCount`, `hasFiber`, `hasPinning`.

/// Instance graph for Test 8: Fiber budget.
pub const TEST8_FIBER_BUDGET: &str = r#"
@prefix rdf:        <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:        <http://www.w3.org/2002/07/owl#> .
@prefix xsd:        <http://www.w3.org/2001/XMLSchema#> .
@prefix partition:  <https://uor.foundation/partition/> .
@prefix type:       <https://uor.foundation/type/> .

# A partition with fiber budget for R_4
<https://uor.foundation/instance/partition-R4-fb>
    a                       owl:NamedIndividual, partition:Partition ;
    partition:quantum       "4"^^xsd:positiveInteger ;
    partition:fiberBudget   <https://uor.foundation/instance/budget-R4> .

# The fiber budget: 4 total fibers, 2 pinned, 2 free
<https://uor.foundation/instance/budget-R4>
    a                       owl:NamedIndividual, partition:FiberBudget ;
    partition:totalFibers   "4"^^xsd:nonNegativeInteger ;
    partition:pinnedCount   "2"^^xsd:nonNegativeInteger ;
    partition:freeCount     "2"^^xsd:nonNegativeInteger ;
    partition:isClosed      false ;
    partition:hasFiber      <https://uor.foundation/instance/fiber-0> ;
    partition:hasFiber      <https://uor.foundation/instance/fiber-1> ;
    partition:hasFiber      <https://uor.foundation/instance/fiber-2> ;
    partition:hasFiber      <https://uor.foundation/instance/fiber-3> ;
    partition:hasPinning    <https://uor.foundation/instance/pinning-0> ;
    partition:hasPinning    <https://uor.foundation/instance/pinning-1> .

# Individual fiber coordinates
<https://uor.foundation/instance/fiber-0>
    a                       owl:NamedIndividual, partition:FiberCoordinate ;
    partition:fiberPosition "0"^^xsd:nonNegativeInteger ;
    partition:fiberState    "pinned" .

<https://uor.foundation/instance/fiber-1>
    a                       owl:NamedIndividual, partition:FiberCoordinate ;
    partition:fiberPosition "1"^^xsd:nonNegativeInteger ;
    partition:fiberState    "pinned" .

<https://uor.foundation/instance/fiber-2>
    a                       owl:NamedIndividual, partition:FiberCoordinate ;
    partition:fiberPosition "2"^^xsd:nonNegativeInteger ;
    partition:fiberState    "free" .

<https://uor.foundation/instance/fiber-3>
    a                       owl:NamedIndividual, partition:FiberCoordinate ;
    partition:fiberPosition "3"^^xsd:nonNegativeInteger ;
    partition:fiberState    "free" .

# Fiber pinnings — record which constraint pinned each fiber
<https://uor.foundation/instance/pinning-0>
    a                       owl:NamedIndividual, partition:FiberPinning ;
    partition:pinsCoordinate <https://uor.foundation/instance/fiber-0> ;
    partition:pinnedBy      <https://uor.foundation/instance/constraint-residue> .

<https://uor.foundation/instance/pinning-1>
    a                       owl:NamedIndividual, partition:FiberPinning ;
    partition:pinsCoordinate <https://uor.foundation/instance/fiber-1> ;
    partition:pinnedBy      <https://uor.foundation/instance/constraint-residue> .

# The constraint that does the pinning (declared as type:ResidueConstraint)
<https://uor.foundation/instance/constraint-residue>
    a                       owl:NamedIndividual, type:ResidueConstraint .
"#;
