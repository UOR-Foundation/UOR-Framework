//! `conformance/` namespace — Conformance shapes.
//!
//! The `conformance/` namespace defines SHACL-equivalent constraint shapes
//! specifying what a Prism implementation must provide at each extension
//! point. Machine-verifiable contracts.
//!
//! - **Amendment 82**: 11 classes, 9 properties, 0 individuals
//! - **Amendment 84**: 0 classes, 0 properties, 5 individuals
//!   (CompileUnitShape + 4 PropertyConstraint)
//!
//! **Space classification:** `bridge` — kernel-computed, user-consumed.

use crate::model::iris::*;
use crate::model::{
    Class, Individual, IndividualValue, Namespace, NamespaceModule, Property, PropertyKind, Space,
};

/// Returns the `conformance/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "conformance",
            iri: NS_CONFORMANCE,
            label: "UOR Conformance Shapes",
            comment: "SHACL-equivalent constraint shapes defining what a \
                      Prism implementation must provide at each extension \
                      point. Machine-verifiable contracts.",
            space: Space::Bridge,
            imports: &[
                NS_SCHEMA,
                NS_TYPE,
                NS_OP,
                NS_EFFECT,
                NS_PREDICATE,
                NS_PARALLEL,
                NS_STREAM,
                NS_LINEAR,
                NS_REGION,
                NS_FAILURE,
                NS_RECURSION,
                NS_BOUNDARY,
                NS_CASCADE,
                NS_CERT,
                NS_TRACE,
                NS_STATE,
                NS_MORPHISM,
            ],
        },
        classes: classes(),
        properties: properties(),
        individuals: individuals(),
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/conformance/Shape",
            label: "Shape",
            comment: "A constraint shape that a Prism-declared extension \
                      must satisfy. Analogous to sh:NodeShape in SHACL.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/PropertyConstraint",
            label: "PropertyConstraint",
            comment: "A single required property within a shape: the \
                      property URI, its expected range, minimum and maximum \
                      cardinality.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/QuantumLevelShape",
            label: "QuantumLevelShape",
            comment: "Shape for declaring a new QuantumLevel beyond Q3.",
            subclass_of: &["https://uor.foundation/conformance/Shape"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/EffectShape",
            label: "EffectShape",
            comment: "Shape for declaring an ExternalEffect.",
            subclass_of: &["https://uor.foundation/conformance/Shape"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/ParallelShape",
            label: "ParallelShape",
            comment: "Shape for declaring a ParallelProduct.",
            subclass_of: &["https://uor.foundation/conformance/Shape"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/StreamShape",
            label: "StreamShape",
            comment: "Shape for declaring a ProductiveStream (targets \
                      stream:Unfold, the coinductive constructor).",
            subclass_of: &["https://uor.foundation/conformance/Shape"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/DispatchShape",
            label: "DispatchShape",
            comment: "Shape for declaring a new DispatchRule in a \
                      DispatchTable.",
            subclass_of: &["https://uor.foundation/conformance/Shape"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/LeaseShape",
            label: "LeaseShape",
            comment: "Shape for declaring a Lease with LinearFiber \
                      allocation.",
            subclass_of: &["https://uor.foundation/conformance/Shape"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/GroundingShape",
            label: "GroundingShape",
            comment: "Shape for declaring a GroundingMap from surface data \
                      to the ring.",
            subclass_of: &["https://uor.foundation/conformance/Shape"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/ValidationResult",
            label: "ValidationResult",
            comment: "The result of validating an extension against a shape: \
                      conforms (boolean), and violation details if \
                      non-conformant.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/PredicateShape",
            label: "PredicateShape",
            comment: "Shape for user-declared predicates. Requires a \
                      bounded evaluator (termination witness) and input \
                      type declaration.",
            subclass_of: &["https://uor.foundation/conformance/Shape"],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        // Object properties
        Property {
            id: "https://uor.foundation/conformance/targetClass",
            label: "targetClass",
            comment: "The OWL class that instances of this shape must \
                      belong to.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/Shape"),
            range: OWL_CLASS,
        },
        Property {
            id: "https://uor.foundation/conformance/requiredProperty",
            label: "requiredProperty",
            comment: "A required property in this shape.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/conformance/Shape"),
            range: "https://uor.foundation/conformance/PropertyConstraint",
        },
        Property {
            id: "https://uor.foundation/conformance/constraintProperty",
            label: "constraintProperty",
            comment: "The property URI that must be present.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/PropertyConstraint"),
            range: OWL_THING,
        },
        Property {
            id: "https://uor.foundation/conformance/constraintRange",
            label: "constraintRange",
            comment: "The expected range of the required property.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/PropertyConstraint"),
            range: OWL_CLASS,
        },
        Property {
            id: "https://uor.foundation/conformance/validationShape",
            label: "validationShape",
            comment: "The shape that was validated against.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/ValidationResult"),
            range: "https://uor.foundation/conformance/Shape",
        },
        Property {
            id: "https://uor.foundation/conformance/validationTarget",
            label: "validationTarget",
            comment: "The instance that was validated.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/ValidationResult"),
            range: OWL_THING,
        },
        // Datatype properties
        Property {
            id: "https://uor.foundation/conformance/minCount",
            label: "minCount",
            comment: "Minimum cardinality of the required property.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/PropertyConstraint"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/conformance/maxCount",
            label: "maxCount",
            comment: "Maximum cardinality (0 = unbounded).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/PropertyConstraint"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/conformance/conforms",
            label: "conforms",
            comment: "True iff the target satisfies all constraints of the \
                      shape.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/ValidationResult"),
            range: XSD_BOOLEAN,
        },
    ]
}

fn individuals() -> Vec<Individual> {
    vec![
        // Amendment 84: CompileUnit admission shape
        Individual {
            id: "https://uor.foundation/conformance/CompileUnitShape",
            type_: "https://uor.foundation/conformance/Shape",
            label: "CompileUnitShape",
            comment: "Shape validating that a CompileUnit carries all required \
                      properties before cascade admission. The unitAddress \
                      property is NOT required \u{2014} it is computed by \
                      stage_initialization after shape validation passes.",
            properties: &[
                (
                    "https://uor.foundation/conformance/targetClass",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cascade/CompileUnit",
                    ),
                ),
                (
                    "https://uor.foundation/conformance/requiredProperty",
                    IndividualValue::IriRef(
                        "https://uor.foundation/conformance/compileUnit_rootTerm_constraint",
                    ),
                ),
                (
                    "https://uor.foundation/conformance/requiredProperty",
                    IndividualValue::IriRef(
                        "https://uor.foundation/conformance/compileUnit_unitQuantumLevel_constraint",
                    ),
                ),
                (
                    "https://uor.foundation/conformance/requiredProperty",
                    IndividualValue::IriRef(
                        "https://uor.foundation/conformance/compileUnit_thermodynamicBudget_constraint",
                    ),
                ),
                (
                    "https://uor.foundation/conformance/requiredProperty",
                    IndividualValue::IriRef(
                        "https://uor.foundation/conformance/compileUnit_targetDomains_constraint",
                    ),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/conformance/compileUnit_rootTerm_constraint",
            type_: "https://uor.foundation/conformance/PropertyConstraint",
            label: "compileUnit_rootTerm_constraint",
            comment: "Exactly one root term is required. Range is schema:Term.",
            properties: &[
                (
                    "https://uor.foundation/conformance/constraintProperty",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cascade/rootTerm",
                    ),
                ),
                (
                    "https://uor.foundation/conformance/constraintRange",
                    IndividualValue::IriRef(
                        "https://uor.foundation/schema/Term",
                    ),
                ),
                (
                    "https://uor.foundation/conformance/minCount",
                    IndividualValue::Int(1),
                ),
                (
                    "https://uor.foundation/conformance/maxCount",
                    IndividualValue::Int(1),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/conformance/compileUnit_unitQuantumLevel_constraint",
            type_: "https://uor.foundation/conformance/PropertyConstraint",
            label: "compileUnit_unitQuantumLevel_constraint",
            comment: "Exactly one quantum level is required. Range is \
                      schema:QuantumLevel.",
            properties: &[
                (
                    "https://uor.foundation/conformance/constraintProperty",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cascade/unitQuantumLevel",
                    ),
                ),
                (
                    "https://uor.foundation/conformance/constraintRange",
                    IndividualValue::IriRef(
                        "https://uor.foundation/schema/QuantumLevel",
                    ),
                ),
                (
                    "https://uor.foundation/conformance/minCount",
                    IndividualValue::Int(1),
                ),
                (
                    "https://uor.foundation/conformance/maxCount",
                    IndividualValue::Int(1),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/conformance/compileUnit_thermodynamicBudget_constraint",
            type_: "https://uor.foundation/conformance/PropertyConstraint",
            label: "compileUnit_thermodynamicBudget_constraint",
            comment: "Exactly one thermodynamic budget is required. Shape \
                      validates presence and type; the BudgetSolvencyCheck \
                      preflight validates the value against the Landauer bound.",
            properties: &[
                (
                    "https://uor.foundation/conformance/constraintProperty",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cascade/thermodynamicBudget",
                    ),
                ),
                (
                    "https://uor.foundation/conformance/constraintRange",
                    IndividualValue::IriRef(
                        "http://www.w3.org/2001/XMLSchema#decimal",
                    ),
                ),
                (
                    "https://uor.foundation/conformance/minCount",
                    IndividualValue::Int(1),
                ),
                (
                    "https://uor.foundation/conformance/maxCount",
                    IndividualValue::Int(1),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/conformance/compileUnit_targetDomains_constraint",
            type_: "https://uor.foundation/conformance/PropertyConstraint",
            label: "compileUnit_targetDomains_constraint",
            comment: "At least one target verification domain is required. \
                      maxCount 0 means unbounded.",
            properties: &[
                (
                    "https://uor.foundation/conformance/constraintProperty",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cascade/targetDomains",
                    ),
                ),
                (
                    "https://uor.foundation/conformance/constraintRange",
                    IndividualValue::IriRef(
                        "https://uor.foundation/op/VerificationDomain",
                    ),
                ),
                (
                    "https://uor.foundation/conformance/minCount",
                    IndividualValue::Int(1),
                ),
                (
                    "https://uor.foundation/conformance/maxCount",
                    IndividualValue::Int(0),
                ),
            ],
        },
    ]
}
