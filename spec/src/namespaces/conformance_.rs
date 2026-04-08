//! `conformance/` namespace — Conformance shapes.
//!
//! The `conformance/` namespace defines SHACL-equivalent constraint shapes
//! specifying what a Prism implementation must provide at each extension
//! point. Machine-verifiable contracts.
//!
//! - **Amendment 82**: 11 classes, 9 properties, 0 individuals
//! - **Amendment 84**: 0 classes, 0 properties, 5 individuals
//!   (CompileUnitShape + 4 PropertyConstraint)
//! - **Amendment 95**: 19 classes, 40 properties, 5 individuals
//!   (Declarative enforcement: opaque witnesses, builders, violation kinds)
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
                NS_REDUCTION,
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
            id: "https://uor.foundation/conformance/WittLevelShape",
            label: "WittLevelShape",
            comment: "Shape for declaring a new WittLevel beyond Q3.",
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
            comment: "Shape for declaring a Lease with LinearSite \
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
        // ── Amendment 95: Declarative enforcement types ──
        Class {
            id: "https://uor.foundation/conformance/WitnessDatum",
            label: "WitnessDatum",
            comment: "Opaque ring element witness. Cannot be constructed \
                      outside the foundation crate \u{2014} only produced by \
                      reduction evaluation or the two-phase minting boundary.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/GroundedCoordinate",
            label: "GroundedCoordinate",
            comment: "Boundary crossing intermediate for a single grounded \
                      coordinate value. Not a WitnessDatum \u{2014} must be \
                      validated and minted by the foundation.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/GroundedTuple",
            label: "GroundedTuple",
            comment: "Boundary crossing intermediate for a fixed-size array \
                      of GroundedCoordinate values. Stack-resident, no heap \
                      allocation.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/GroundedValueMarker",
            label: "GroundedValueMarker",
            comment: "Sealed marker trait class. Implemented only for \
                      GroundedCoordinate and GroundedTuple. Prevents \
                      downstream crates from substituting arbitrary types.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/ValidatedWrapper",
            label: "ValidatedWrapper",
            comment: "Generic validation-proof wrapper. Proves that the \
                      inner value was produced by the conformance checker, \
                      not fabricated by Prism code.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/WitnessDerivation",
            label: "WitnessDerivation",
            comment: "Opaque derivation trace that can only be extended \
                      by the rewrite engine. Records rewrite step count \
                      and root term content address.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/WitnessSiteBudget",
            label: "WitnessSiteBudget",
            comment: "Opaque site budget that can only be decremented by \
                      PinningEffect and incremented by UnbindingEffect \
                      \u{2014} never by direct mutation.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/ShapeViolationReport",
            label: "ShapeViolationReport",
            comment: "Structured violation diagnostic carrying the shape \
                      IRI, constraint IRI, property IRI, expected range, \
                      cardinality bounds, and violation kind.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/ViolationKind",
            label: "ViolationKind",
            comment: "The kind of shape violation: Missing, TypeMismatch, \
                      CardinalityViolation, ValueCheck, or LevelMismatch.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/CompileUnitBuilder",
            label: "CompileUnitBuilder",
            comment: "Builder for CompileUnit admission. Collects rootTerm, \
                      quantumLevelCeiling, thermodynamicBudget, and \
                      targetDomains. Validates against CompileUnitShape.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/EffectDeclaration",
            label: "EffectDeclaration",
            comment: "Builder for EffectShape. Collects effect name, target \
                      sites, budget delta, and commutation flag.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/GroundingDeclaration",
            label: "GroundingDeclaration",
            comment: "Builder for GroundingShape. Collects source type, \
                      ring mapping, and invertibility contract.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/DispatchDeclaration",
            label: "DispatchDeclaration",
            comment: "Builder for DispatchShape. Collects predicate, target \
                      resolver, and dispatch priority.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/LeaseDeclaration",
            label: "LeaseDeclaration",
            comment: "Builder for LeaseShape. Collects linear site and \
                      lease scope.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/StreamDeclaration",
            label: "StreamDeclaration",
            comment: "Builder for StreamShape. Collects unfold seed, step \
                      term, and productivity witness.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/PredicateDeclaration",
            label: "PredicateDeclaration",
            comment: "Builder for PredicateShape. Collects input type, \
                      evaluator term, and termination witness.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/ParallelDeclaration",
            label: "ParallelDeclaration",
            comment: "Builder for ParallelShape. Collects site partition \
                      and disjointness witness.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/WittLevelDeclaration",
            label: "WittLevelDeclaration",
            comment: "Builder for WittLevelShape. Collects declared bit \
                      width, cycle size, and predecessor level.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/conformance/MintingSession",
            label: "MintingSession",
            comment: "Boundary session state tracker. Records crossing count \
                      and idempotency flag for the two-phase minting \
                      boundary.",
            subclass_of: &[OWL_THING],
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
        // ── Amendment 95: Witness type properties (11) ──
        Property {
            id: "https://uor.foundation/conformance/witnessLevel",
            label: "witnessLevel",
            comment: "The quantum level at which this witness datum was minted.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/WitnessDatum"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/conformance/witnessBytes",
            label: "witnessBytes",
            comment: "The raw byte representation of this witness datum.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/WitnessDatum"),
            range: XSD_HEX_BINARY,
        },
        Property {
            id: "https://uor.foundation/conformance/coordinateLevel",
            label: "coordinateLevel",
            comment: "The quantum level tag of this grounded coordinate.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/GroundedCoordinate"),
            range: "https://uor.foundation/schema/WittLevel",
        },
        Property {
            id: "https://uor.foundation/conformance/validatedInner",
            label: "validatedInner",
            comment: "The validated inner value wrapped by this proof.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/ValidatedWrapper"),
            range: OWL_THING,
        },
        Property {
            id: "https://uor.foundation/conformance/shapeIri",
            label: "shapeIri",
            comment: "IRI of the conformance:Shape that was validated against.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/ShapeViolationReport"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/conformance/constraintIri",
            label: "constraintIri",
            comment: "IRI of the specific PropertyConstraint that failed.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/ShapeViolationReport"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/conformance/propertyIri",
            label: "propertyIri",
            comment: "IRI of the property that was missing or invalid.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/ShapeViolationReport"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/conformance/expectedRange",
            label: "expectedRange",
            comment: "The expected range class IRI for the violated property.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/ShapeViolationReport"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/conformance/violationMinCount",
            label: "violationMinCount",
            comment: "The minimum cardinality from the violated constraint.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/ShapeViolationReport"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/conformance/violationMaxCount",
            label: "violationMaxCount",
            comment: "The maximum cardinality from the violated constraint \
                      (0 = unbounded).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/ShapeViolationReport"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/conformance/violationKind",
            label: "violationKind",
            comment: "The kind of violation that occurred.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/ShapeViolationReport"),
            range: "https://uor.foundation/conformance/ViolationKind",
        },
        // ── Amendment 95: Builder properties (27) ──
        // CompileUnitBuilder (4)
        Property {
            id: "https://uor.foundation/conformance/builderRootTerm",
            label: "builderRootTerm",
            comment: "The root term expression for the CompileUnit.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/CompileUnitBuilder"),
            range: "https://uor.foundation/schema/Term",
        },
        Property {
            id: "https://uor.foundation/conformance/builderWittLevelCeiling",
            label: "builderWittLevelCeiling",
            comment: "The widest quantum level the computation may reference.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/CompileUnitBuilder"),
            range: "https://uor.foundation/schema/WittLevel",
        },
        Property {
            id: "https://uor.foundation/conformance/builderThermodynamicBudget",
            label: "builderThermodynamicBudget",
            comment: "Landauer-bounded energy budget in kBT ln 2 units.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/CompileUnitBuilder"),
            range: XSD_DECIMAL,
        },
        Property {
            id: "https://uor.foundation/conformance/builderTargetDomains",
            label: "builderTargetDomains",
            comment: "Verification domains targeted by the CompileUnit.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/conformance/CompileUnitBuilder"),
            range: "https://uor.foundation/op/VerificationDomain",
        },
        // EffectDeclaration (4)
        Property {
            id: "https://uor.foundation/conformance/effectName",
            label: "effectName",
            comment: "The name of the declared effect.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/EffectDeclaration"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/conformance/targetSites",
            label: "targetSites",
            comment: "Site coordinates this effect reads or writes.",
            kind: PropertyKind::Datatype,
            functional: false,
            domain: Some("https://uor.foundation/conformance/EffectDeclaration"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/conformance/budgetDelta",
            label: "budgetDelta",
            comment: "The site budget delta (positive = increment, \
                      negative = decrement).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/EffectDeclaration"),
            range: XSD_INTEGER,
        },
        Property {
            id: "https://uor.foundation/conformance/commutationFlag",
            label: "commutationFlag",
            comment: "Whether this effect commutes with effects on \
                      disjoint sites.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/EffectDeclaration"),
            range: XSD_BOOLEAN,
        },
        // GroundingDeclaration (3)
        Property {
            id: "https://uor.foundation/conformance/groundingSourceType",
            label: "groundingSourceType",
            comment: "The source type of incoming external data.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/GroundingDeclaration"),
            range: "https://uor.foundation/type/TypeDefinition",
        },
        Property {
            id: "https://uor.foundation/conformance/ringMapping",
            label: "ringMapping",
            comment: "Description of the mapping from surface data to ring.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/GroundingDeclaration"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/conformance/invertibilityContract",
            label: "invertibilityContract",
            comment: "Whether the grounding map is invertible.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/GroundingDeclaration"),
            range: XSD_BOOLEAN,
        },
        // DispatchDeclaration (3)
        Property {
            id: "https://uor.foundation/conformance/dispatchPredicate",
            label: "dispatchPredicate",
            comment: "The predicate expression guarding this dispatch rule.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/DispatchDeclaration"),
            range: "https://uor.foundation/reduction/PredicateExpression",
        },
        Property {
            id: "https://uor.foundation/conformance/targetResolver",
            label: "targetResolver",
            comment: "The resolver to dispatch to when the predicate holds.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/DispatchDeclaration"),
            range: "https://uor.foundation/resolver/Resolver",
        },
        Property {
            id: "https://uor.foundation/conformance/dispatchPriority",
            label: "dispatchPriority",
            comment: "Priority ordering for this dispatch rule (lower = first).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/DispatchDeclaration"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        // LeaseDeclaration (2)
        Property {
            id: "https://uor.foundation/conformance/linearSite",
            label: "linearSite",
            comment: "The site coordinate allocated linearly by this lease.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/LeaseDeclaration"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/conformance/leaseScope",
            label: "leaseScope",
            comment: "The scope within which this lease is valid.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/LeaseDeclaration"),
            range: XSD_STRING,
        },
        // StreamDeclaration (3)
        Property {
            id: "https://uor.foundation/conformance/unfoldSeed",
            label: "unfoldSeed",
            comment: "The seed term for the stream unfold constructor.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/StreamDeclaration"),
            range: "https://uor.foundation/schema/Term",
        },
        Property {
            id: "https://uor.foundation/conformance/stepTerm",
            label: "stepTerm",
            comment: "The step function term for the stream unfold.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/StreamDeclaration"),
            range: "https://uor.foundation/schema/Term",
        },
        Property {
            id: "https://uor.foundation/conformance/productivityWitness",
            label: "productivityWitness",
            comment: "Evidence that the stream is productive (always \
                      produces a next element).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/StreamDeclaration"),
            range: XSD_STRING,
        },
        // PredicateDeclaration (3)
        Property {
            id: "https://uor.foundation/conformance/predicateInputType",
            label: "predicateInputType",
            comment: "The input type for the declared predicate.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/PredicateDeclaration"),
            range: "https://uor.foundation/type/TypeDefinition",
        },
        Property {
            id: "https://uor.foundation/conformance/evaluatorTerm",
            label: "evaluatorTerm",
            comment: "The evaluator term for the declared predicate.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/PredicateDeclaration"),
            range: "https://uor.foundation/schema/Term",
        },
        Property {
            id: "https://uor.foundation/conformance/terminationWitness",
            label: "terminationWitness",
            comment: "Evidence that the predicate evaluator terminates on \
                      all inputs.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/PredicateDeclaration"),
            range: XSD_STRING,
        },
        // ParallelDeclaration (2)
        Property {
            id: "https://uor.foundation/conformance/sitePartition",
            label: "sitePartition",
            comment: "The site partition for the parallel composition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/ParallelDeclaration"),
            range: "https://uor.foundation/partition/Partition",
        },
        Property {
            id: "https://uor.foundation/conformance/disjointnessWitness",
            label: "disjointnessWitness",
            comment: "Evidence that the site partition components are \
                      pairwise disjoint.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/ParallelDeclaration"),
            range: XSD_STRING,
        },
        // WittLevelDeclaration (3)
        Property {
            id: "https://uor.foundation/conformance/declaredBitWidth",
            label: "declaredBitWidth",
            comment: "The declared bit width for this quantum level.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/WittLevelDeclaration"),
            range: XSD_POSITIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/conformance/declaredCycleSize",
            label: "declaredCycleSize",
            comment: "The declared number of ring states at this level.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/WittLevelDeclaration"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/conformance/predecessorLevel",
            label: "predecessorLevel",
            comment: "The predecessor quantum level in the chain.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/conformance/WittLevelDeclaration"),
            range: "https://uor.foundation/schema/WittLevel",
        },
        // MintingSession (2)
        Property {
            id: "https://uor.foundation/conformance/sessionCrossingCount",
            label: "sessionCrossingCount",
            comment: "Total boundary crossings in this minting session.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/MintingSession"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/conformance/sessionIsIdempotent",
            label: "sessionIsIdempotent",
            comment: "Whether applying this session's boundary effect \
                      twice equals applying it once.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/conformance/MintingSession"),
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
                      properties before reduction admission. The unitAddress \
                      property is NOT required \u{2014} it is computed by \
                      stage_initialization after shape validation passes.",
            properties: &[
                (
                    "https://uor.foundation/conformance/targetClass",
                    IndividualValue::IriRef(
                        "https://uor.foundation/reduction/CompileUnit",
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
                        "https://uor.foundation/conformance/compileUnit_unitWittLevel_constraint",
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
                        "https://uor.foundation/reduction/rootTerm",
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
            id: "https://uor.foundation/conformance/compileUnit_unitWittLevel_constraint",
            type_: "https://uor.foundation/conformance/PropertyConstraint",
            label: "compileUnit_unitWittLevel_constraint",
            comment: "Exactly one quantum level is required. Range is \
                      schema:WittLevel.",
            properties: &[
                (
                    "https://uor.foundation/conformance/constraintProperty",
                    IndividualValue::IriRef(
                        "https://uor.foundation/reduction/unitWittLevel",
                    ),
                ),
                (
                    "https://uor.foundation/conformance/constraintRange",
                    IndividualValue::IriRef(
                        "https://uor.foundation/schema/WittLevel",
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
                        "https://uor.foundation/reduction/thermodynamicBudget",
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
                        "https://uor.foundation/reduction/targetDomains",
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
        // ── Amendment 95: ViolationKind individuals (5) ──
        Individual {
            id: "https://uor.foundation/conformance/Missing",
            type_: "https://uor.foundation/conformance/ViolationKind",
            label: "Missing",
            comment: "Required property was not set on the builder.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/conformance/TypeMismatch",
            type_: "https://uor.foundation/conformance/ViolationKind",
            label: "TypeMismatch",
            comment: "Property was set but its value is not an instance \
                      of the constraintRange.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/conformance/CardinalityViolation",
            type_: "https://uor.foundation/conformance/ViolationKind",
            label: "CardinalityViolation",
            comment: "Cardinality violated: too few or too many values \
                      provided.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/conformance/ValueCheck",
            type_: "https://uor.foundation/conformance/ViolationKind",
            label: "ValueCheck",
            comment: "Value-dependent check failed (Tier 2). For example, \
                      thermodynamic budget insufficient for Landauer bound.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/conformance/LevelMismatch",
            type_: "https://uor.foundation/conformance/ViolationKind",
            label: "LevelMismatch",
            comment: "A term's quantum level annotation exceeds the \
                      CompileUnit ceiling, or binary operation operands \
                      are at different levels without an intervening \
                      lift or project.",
            properties: &[],
        },
    ]
}
