/// SHACL test 49: Reversible resolution — FiberBudget + FiberCoordinate +
/// ancillaFiber + reversibleStrategy (Amendment 31, RC_1–RC_4).
pub const TEST49_REVERSIBLE_RESOLUTION: &str = r#"
@prefix rdf:        <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:        <http://www.w3.org/2002/07/owl#> .
@prefix xsd:        <http://www.w3.org/2001/XMLSchema#> .
@prefix partition:  <https://uor.foundation/partition/> .

# 1. FiberCoordinate with ancilla pairing
partition:ex_fc_49 a owl:NamedIndividual, partition:FiberCoordinate ;
    partition:ancillaFiber partition:ex_ancilla_49 .

# 2. Ancilla fiber coordinate
partition:ex_ancilla_49 a owl:NamedIndividual, partition:FiberCoordinate .

# 3. FiberBudget with reversible strategy
partition:ex_fb_49 a owl:NamedIndividual, partition:FiberBudget ;
    partition:fiberCount "4"^^xsd:nonNegativeInteger ;
    partition:reversibleStrategy "true"^^xsd:boolean .
"#;
