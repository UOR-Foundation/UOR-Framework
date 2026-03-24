//! SHACL test 249: `cascade:PropertyBind` and `cascade:StageAdvance`.

/// Instance graph for Test 249: PropertyBind and StageAdvance.
pub const TEST249_PROPERTY_BIND_ADVANCE: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:ex_bind_249 a owl:NamedIndividual, cascade:PropertyBind ;
    cascade:bindTarget "fiber_3" ;
    cascade:bindValue "42" .

cascade:ex_advance_249 a owl:NamedIndividual, cascade:StageAdvance ;
    cascade:advanceFrom cascade:Declare ;
    cascade:advanceTo cascade:Factorize .
"#;
