//! SHACL test 217: `cascade:BackPressureSignal` instance.

/// Instance graph for Test 217: BackPressureSignal with pressure level.
pub const TEST217_BACK_PRESSURE: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:bp_example a owl:NamedIndividual, cascade:BackPressureSignal ;
    cascade:pressureLevel "High"^^xsd:string ;
    cascade:pressureThreshold "0.9"^^xsd:string .
"#;
