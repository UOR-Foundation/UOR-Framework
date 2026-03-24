//! SHACL test 210: `cascade:GuardExpression` instance.

/// Instance graph for Test 210: GuardExpression with guardPredicates.
pub const TEST210_GUARD_EXPRESSION: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:guard_example a owl:NamedIndividual, cascade:GuardExpression ;
    cascade:guardPredicates cascade:true_predicate .
"#;
