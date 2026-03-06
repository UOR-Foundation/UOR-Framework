/// SHACL test 31: Quantum level — QuantumLevel individuals Q0–Q3 with
/// quantumIndex/bitsWidth/cycleSize/nextLevel. Q3 is terminal (no nextLevel).
pub const TEST31_QUANTUM_LEVEL: &str = r#"
@prefix rdf:    <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:    <http://www.w3.org/2002/07/owl#> .
@prefix xsd:    <http://www.w3.org/2001/XMLSchema#> .
@prefix schema: <https://uor.foundation/schema/> .

schema:Q0 a owl:NamedIndividual, schema:QuantumLevel ;
    schema:quantumIndex "0"^^xsd:nonNegativeInteger ;
    schema:bitsWidth    "8"^^xsd:positiveInteger ;
    schema:cycleSize    "256"^^xsd:positiveInteger ;
    schema:nextLevel    schema:Q1 .

schema:Q1 a owl:NamedIndividual, schema:QuantumLevel ;
    schema:quantumIndex "1"^^xsd:nonNegativeInteger ;
    schema:bitsWidth    "16"^^xsd:positiveInteger ;
    schema:cycleSize    "65536"^^xsd:positiveInteger ;
    schema:nextLevel    schema:Q2 .

schema:Q2 a owl:NamedIndividual, schema:QuantumLevel ;
    schema:quantumIndex "2"^^xsd:nonNegativeInteger ;
    schema:bitsWidth    "24"^^xsd:positiveInteger ;
    schema:cycleSize    "16777216"^^xsd:positiveInteger ;
    schema:nextLevel    schema:Q3 .

schema:Q3 a owl:NamedIndividual, schema:QuantumLevel ;
    schema:quantumIndex "3"^^xsd:nonNegativeInteger ;
    schema:bitsWidth    "32"^^xsd:positiveInteger ;
    schema:cycleSize    "4294967296"^^xsd:positiveInteger .
"#;
