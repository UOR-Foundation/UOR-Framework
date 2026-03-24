//! SHACL test 248: `cascade:EpochBoundary`.

/// Instance graph for Test 248: EpochBoundary.
pub const TEST248_EPOCH_BOUNDARY: &str = r#"
@prefix rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix cascade: <https://uor.foundation/cascade/> .

cascade:ex_boundary_248 a owl:NamedIndividual, cascade:EpochBoundary ;
    cascade:epochBoundaryType "SaturationPreservation" ;
    cascade:preservedSaturation "true"^^xsd:boolean .
"#;
