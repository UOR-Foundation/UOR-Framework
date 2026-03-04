/// SHACL test 29: Coordinate kind vocabulary — typed coordinate queries.
pub const TEST29_COORDINATE_KIND: &str = r#"
@prefix rdf:  <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix query: <https://uor.foundation/query/> .
@prefix type:  <https://uor.foundation/type/> .

# CoordinateKind vocabulary individuals
query:StratumCoordinate a query:CoordinateKind .
query:SpectrumCoordinate a query:CoordinateKind .
query:AddressCoordinate a query:CoordinateKind .

# CoordinateQuery instances with typed coordinate kinds
<https://uor.foundation/instance/stratum-query>
    a query:CoordinateQuery ;
    query:hasCoordinateKind query:StratumCoordinate .

<https://uor.foundation/instance/spectrum-query>
    a query:CoordinateQuery ;
    query:hasCoordinateKind query:SpectrumCoordinate .

<https://uor.foundation/instance/address-query>
    a query:CoordinateQuery ;
    query:hasCoordinateKind query:AddressCoordinate ;
    type:axisSignatureNote "V" .
"#;
