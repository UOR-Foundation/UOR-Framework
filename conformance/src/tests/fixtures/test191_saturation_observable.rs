//! SHACL test 191: `observable:SaturationObservable`.

/// Instance graph for Test 191: SaturationObservable with numerator and denominator.
pub const TEST191_SATURATION_OBSERVABLE: &str = r#"
@prefix rdf:        <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:        <http://www.w3.org/2002/07/owl#> .
@prefix xsd:        <http://www.w3.org/2001/XMLSchema#> .
@prefix observable: <https://uor.foundation/observable/> .

observable:ex_sat_191 a owl:NamedIndividual, observable:SaturationObservable ;
    observable:saturationNumerator "5"^^xsd:nonNegativeInteger ;
    observable:saturationDenominator "8"^^xsd:positiveInteger .
"#;
