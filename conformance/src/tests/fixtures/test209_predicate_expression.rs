//! SHACL test 209: `cascade:PredicateExpression` instance.

/// Instance graph for Test 209: PredicateExpression with predicateField.
pub const TEST209_PREDICATE_EXPRESSION: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:pred_example a owl:NamedIndividual, cascade:PredicateExpression ;
    cascade:predicateField "freeCount"^^xsd:string ;
    cascade:predicateOperator ">"^^xsd:string ;
    cascade:predicateValue "0"^^xsd:string .
"#;
