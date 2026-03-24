//! SHACL test 232: `interaction:ThreeWayFiber` instance.

/// Instance graph for Test 232: ThreeWayFiber with grouping values.
pub const TEST232_THREE_WAY_FIBER: &str = r#"
@prefix rdf:         <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:         <http://www.w3.org/2002/07/owl#> .
@prefix xsd:         <http://www.w3.org/2001/XMLSchema#> .
@prefix interaction: <https://uor.foundation/interaction/> .

interaction:test_fiber a owl:NamedIndividual, interaction:ThreeWayFiber ;
    interaction:fiberPosition "3"^^xsd:nonNegativeInteger ;
    interaction:leftGroupingValue "(AB)C = 42" ;
    interaction:rightGroupingValue "A(BC) = 37" ;
    interaction:isPinned true ;
    interaction:pinningPair "alpha-beta" .
"#;
