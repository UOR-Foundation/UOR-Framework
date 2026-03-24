//! `cascade/` namespace — Euler cascade sequential composition.
//!
//! The `cascade/` namespace formalizes the sequential composition of
//! \u{03c8}-maps into a parameterized cascade \u{03c8} = \u{03c8}_9 \u{2218} \u{2026} \u{2218} \u{03c8}_1,
//! parameterized by the phase angle \u{03a9} = e^{i\u{03c0}/6}. It defines
//! the six-stage pipeline, phase gate attestation, complex conjugate
//! rollback, and epoch-based temporal segmentation.
//!
//! - **Amendment 63**: 10 classes, 25 properties, cascade core formalization
//! - **Amendment 64**: 10 classes, 20 properties, cascade expansion
//!   (predicates, guards, effects, service windows, transactions,
//!   preflight checks, pipeline termination)
//! - **Amendment 65**: 6 classes, 11 properties, cascade completion
//!   (feasibility results, lease lifecycle, back-pressure, deferred queries)
//!
//! **Space classification:** `kernel` — immutable algebra.

use crate::model::iris::*;
use crate::model::{
    Class, Individual, IndividualValue, Namespace, NamespaceModule, Property, PropertyKind, Space,
};

/// Returns the `cascade/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "cascade",
            iri: NS_CASCADE,
            label: "UOR Euler Cascade",
            comment: "Sequential composition of \u{03c8}-maps into a parameterized \
                      cascade \u{03c8} = \u{03c8}_9 \u{2218} \u{2026} \u{2218} \u{03c8}_1. \
                      Defines stages, phase gates, rollback, and epochs.",
            space: Space::Kernel,
            imports: &[
                NS_OP,
                NS_STATE,
                NS_PARTITION,
                NS_RESOLVER,
                NS_MORPHISM,
                NS_OBSERVABLE,
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
            id: "https://uor.foundation/cascade/EulerCascade",
            label: "EulerCascade",
            comment: "The composite endofunctor \u{03c8} = \u{03c8}_9 \u{2218} \u{2026} \
                      \u{2218} \u{03c8}_1, parameterized by \u{03a9} = e^{i\u{03c0}/6}.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/PhaseRotationScheduler",
            label: "PhaseRotationScheduler",
            comment: "Schedule \u{03a9}\u{2070}, \u{03a9}\u{00b9}, \u{2026}, \u{03a9}\u{2075} \
                      assigning a phase angle to each stage of the cascade.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/TargetConvergenceAngle",
            label: "TargetConvergenceAngle",
            comment: "The angle at which the cascade terminates \
                      (default: \u{03c0}).",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/PhaseGateAttestation",
            label: "PhaseGateAttestation",
            comment: "Validation at each stage boundary checking that the \
                      accumulated phase angle matches the expected \u{03a9}^k.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/ComplexConjugateRollback",
            label: "ComplexConjugateRollback",
            comment: "Recovery operation when a phase gate fails: z \u{2192} z\u{0304}. \
                      Involutory: applying twice yields the original value.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/CascadeStage",
            label: "CascadeStage",
            comment: "A named stage of the cascade. The standard cascade has \
                      six stages (Initialization through Convergence).",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/CascadeState",
            label: "CascadeState",
            comment: "State of cascade execution at a specific point, including \
                      the current stage, phase angle, and pinned mask.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/CascadeTransitionRule",
            label: "CascadeTransitionRule",
            comment: "Guard-effect pair governing stage transitions in the \
                      cascade. The guard must be satisfied before the effect \
                      is applied.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/Epoch",
            label: "Epoch",
            comment: "Temporal segment of cascade execution. Each epoch \
                      represents one complete pass through the cascade stages.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/EpochBoundary",
            label: "EpochBoundary",
            comment: "Transition between epochs. Carries metadata about \
                      the epoch boundary crossing.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 64: Cascade Expansion (10 classes)
        Class {
            id: "https://uor.foundation/cascade/PredicateExpression",
            label: "PredicateExpression",
            comment: "A Boolean expression over the cascade state. \
                      The atomic guard unit.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/GuardExpression",
            label: "GuardExpression",
            comment: "A conjunction of PredicateExpressions that must hold \
                      for a transition to fire.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/TransitionEffect",
            label: "TransitionEffect",
            comment: "State changes applied when a transition fires. \
                      Contains PropertyBind steps.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/PropertyBind",
            label: "PropertyBind",
            comment: "A single fiber pinning: target fiber + value.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/StageAdvance",
            label: "StageAdvance",
            comment: "Advancement from one CascadeStage to the next.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/ServiceWindow",
            label: "ServiceWindow",
            comment: "A sliding window over recent epochs providing \
                      BaseContext.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/CascadeTransaction",
            label: "CascadeTransaction",
            comment: "An atomic group of state changes within the cascade.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/PipelineSuccess",
            label: "PipelineSuccess",
            comment: "Successful termination (FullSaturation).",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/PipelineFailureReason",
            label: "PipelineFailureReason",
            comment: "Typed failure: DispatchMiss, GroundingFailure, \
                      ConvergenceStall, etc.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/PreflightCheck",
            label: "PreflightCheck",
            comment: "A pre-execution validation: feasibility, dispatch \
                      coverage, coherence.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 65: Cascade Completion (6 classes)
        Class {
            id: "https://uor.foundation/cascade/FeasibilityResult",
            label: "FeasibilityResult",
            comment: "Result of a preflight check: feasibility witness or \
                      infeasibility witness.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/LeaseState",
            label: "LeaseState",
            comment: "Lifecycle of a partitioned context lease: Pending \
                      \u{2192} Active \u{2192} Released/Expired/Suspended.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/ManagedLease",
            label: "ManagedLease",
            comment: "A context lease with lifecycle tracking.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/LeaseCheckpoint",
            label: "LeaseCheckpoint",
            comment: "Snapshot of lease state at a point in time.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/BackPressureSignal",
            label: "BackPressureSignal",
            comment: "Flow control when cascade produces faster than \
                      downstream can consume.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/DeferredQuerySet",
            label: "DeferredQuerySet",
            comment: "Queries postponed to a future epoch.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 71: SubleaseTransfer
        Class {
            id: "https://uor.foundation/cascade/SubleaseTransfer",
            label: "SubleaseTransfer",
            comment: "Transfer of a lease from one computation to another.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 71: Predicate subclasses (10)
        Class {
            id: "https://uor.foundation/cascade/ComparisonPredicate",
            label: "ComparisonPredicate",
            comment: "Predicate comparing a state field against a value.",
            subclass_of: &["https://uor.foundation/cascade/PredicateExpression"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/ConjunctionPredicate",
            label: "ConjunctionPredicate",
            comment: "Conjunction (AND) of multiple predicates.",
            subclass_of: &["https://uor.foundation/cascade/PredicateExpression"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/DisjunctionPredicate",
            label: "DisjunctionPredicate",
            comment: "Disjunction (OR) of multiple predicates.",
            subclass_of: &["https://uor.foundation/cascade/PredicateExpression"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/NegationPredicate",
            label: "NegationPredicate",
            comment: "Negation (NOT) of a single predicate.",
            subclass_of: &["https://uor.foundation/cascade/PredicateExpression"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/MembershipPredicate",
            label: "MembershipPredicate",
            comment: "Predicate testing membership of an element in a set.",
            subclass_of: &["https://uor.foundation/cascade/PredicateExpression"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/SaturationPredicate",
            label: "SaturationPredicate",
            comment: "Predicate testing whether saturation exceeds a threshold.",
            subclass_of: &["https://uor.foundation/cascade/PredicateExpression"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/FiberCoveragePredicate",
            label: "FiberCoveragePredicate",
            comment: "Predicate testing whether a fiber coverage target is met.",
            subclass_of: &["https://uor.foundation/cascade/PredicateExpression"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/EqualsPredicate",
            label: "EqualsPredicate",
            comment: "Predicate testing equality of two expressions.",
            subclass_of: &["https://uor.foundation/cascade/PredicateExpression"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/NonNullPredicate",
            label: "NonNullPredicate",
            comment: "Predicate testing that a field is non-null.",
            subclass_of: &["https://uor.foundation/cascade/PredicateExpression"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/cascade/QuerySubtypePredicate",
            label: "QuerySubtypePredicate",
            comment: "Predicate testing whether a query is a subtype of a given type.",
            subclass_of: &["https://uor.foundation/cascade/PredicateExpression"],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        // EulerCascade properties
        Property {
            id: "https://uor.foundation/cascade/phaseParameter",
            label: "phaseParameter",
            comment: "The base phase parameter \u{03a9} for this cascade \
                      (e.g., e^{i\u{03c0}/6}).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/EulerCascade"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/stageCount",
            label: "stageCount",
            comment: "The number of stages in this cascade.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/EulerCascade"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/cascade/convergenceAngle",
            label: "convergenceAngle",
            comment: "The cumulative phase angle at which the cascade converges.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/EulerCascade"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/composedOfMaps",
            label: "composedOfMaps",
            comment: "The ordered list of \u{03c8}-maps that compose this cascade.",
            kind: PropertyKind::Annotation,
            functional: false,
            domain: Some("https://uor.foundation/cascade/EulerCascade"),
            range: XSD_STRING,
        },
        // PhaseRotationScheduler properties
        Property {
            id: "https://uor.foundation/cascade/rotationSchedule",
            label: "rotationSchedule",
            comment: "String representation of the rotation schedule \
                      \u{03a9}\u{2070}, \u{03a9}\u{00b9}, \u{2026}, \u{03a9}\u{2075}.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PhaseRotationScheduler"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/baseAngle",
            label: "baseAngle",
            comment: "The base angle \u{03c0}/6 from which the schedule is derived.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PhaseRotationScheduler"),
            range: XSD_STRING,
        },
        // TargetConvergenceAngle properties
        Property {
            id: "https://uor.foundation/cascade/targetAngle",
            label: "targetAngle",
            comment: "The target convergence angle (default: \u{03c0}).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/TargetConvergenceAngle"),
            range: XSD_STRING,
        },
        // PhaseGateAttestation properties
        Property {
            id: "https://uor.foundation/cascade/gateStage",
            label: "gateStage",
            comment: "The cascade stage at which this gate is applied.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PhaseGateAttestation"),
            range: "https://uor.foundation/cascade/CascadeStage",
        },
        Property {
            id: "https://uor.foundation/cascade/gateExpectedPhase",
            label: "gateExpectedPhase",
            comment: "The expected phase angle \u{03a9}^k at this gate.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PhaseGateAttestation"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/gateResult",
            label: "gateResult",
            comment: "Whether the phase gate check passed or failed.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PhaseGateAttestation"),
            range: XSD_BOOLEAN,
        },
        // ComplexConjugateRollback properties
        Property {
            id: "https://uor.foundation/cascade/rollbackTarget",
            label: "rollbackTarget",
            comment: "The cascade stage to which execution rolls back on gate failure.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/cascade/ComplexConjugateRollback"),
            range: "https://uor.foundation/cascade/CascadeStage",
        },
        // CascadeStage properties
        Property {
            id: "https://uor.foundation/cascade/stageIndex",
            label: "stageIndex",
            comment: "Zero-based index of this stage in the cascade.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeStage"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/cascade/stageName",
            label: "stageName",
            comment: "Human-readable name of this cascade stage.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeStage"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/expectedPhase",
            label: "expectedPhase",
            comment: "The expected phase angle \u{03a9}^k at this stage.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeStage"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/entryCondition",
            label: "entryCondition",
            comment: "The condition that must hold to enter this stage.",
            kind: PropertyKind::Annotation,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeStage"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/exitCondition",
            label: "exitCondition",
            comment: "The condition that must hold to exit this stage.",
            kind: PropertyKind::Annotation,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeStage"),
            range: XSD_STRING,
        },
        // CascadeState properties
        Property {
            id: "https://uor.foundation/cascade/currentStage",
            label: "currentStage",
            comment: "The cascade stage at which execution is currently positioned.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeState"),
            range: "https://uor.foundation/cascade/CascadeStage",
        },
        Property {
            id: "https://uor.foundation/cascade/phaseAngle",
            label: "phaseAngle",
            comment: "The accumulated phase angle at the current point.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeState"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/pinnedMask",
            label: "pinnedMask",
            comment: "Bit mask of fibers that are pinned (resolved) at this point.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeState"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/freeCount",
            label: "freeCount",
            comment: "The number of free (unresolved) fibers at this point.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeState"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        // CascadeTransitionRule properties
        Property {
            id: "https://uor.foundation/cascade/transitionGuard",
            label: "transitionGuard",
            comment: "The predicate that must be satisfied for this transition.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeTransitionRule"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/transitionEffect",
            label: "transitionEffect",
            comment: "The effect applied when this transition fires.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeTransitionRule"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/transitionAdvance",
            label: "transitionAdvance",
            comment: "Whether this transition advances to the next stage.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeTransitionRule"),
            range: XSD_BOOLEAN,
        },
        // Epoch properties
        Property {
            id: "https://uor.foundation/cascade/epochIndex",
            label: "epochIndex",
            comment: "Zero-based index of this epoch in the cascade execution.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/Epoch"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/cascade/epochDatum",
            label: "epochDatum",
            comment: "Metadata or summary datum for this epoch.",
            kind: PropertyKind::Annotation,
            functional: true,
            domain: Some("https://uor.foundation/cascade/Epoch"),
            range: XSD_STRING,
        },
        // EpochBoundary properties
        Property {
            id: "https://uor.foundation/cascade/epochBoundaryType",
            label: "epochBoundaryType",
            comment: "The type of epoch boundary crossing (e.g., normal, forced, timeout).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/EpochBoundary"),
            range: XSD_STRING,
        },
        // Amendment 64: PredicateExpression properties
        Property {
            id: "https://uor.foundation/cascade/predicateField",
            label: "predicateField",
            comment: "The state field this predicate tests.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PredicateExpression"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/predicateOperator",
            label: "predicateOperator",
            comment: "The comparison operator (e.g., '=', '<', '>=').",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PredicateExpression"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/predicateValue",
            label: "predicateValue",
            comment: "The value against which the field is compared.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PredicateExpression"),
            range: XSD_STRING,
        },
        // GuardExpression properties
        Property {
            id: "https://uor.foundation/cascade/guardPredicates",
            label: "guardPredicates",
            comment: "The predicate expressions that compose this guard.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/cascade/GuardExpression"),
            range: "https://uor.foundation/cascade/PredicateExpression",
        },
        // TransitionEffect properties
        Property {
            id: "https://uor.foundation/cascade/effectBindings",
            label: "effectBindings",
            comment: "The property bind steps applied by this effect.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/cascade/TransitionEffect"),
            range: "https://uor.foundation/cascade/PropertyBind",
        },
        // PropertyBind properties
        Property {
            id: "https://uor.foundation/cascade/bindTarget",
            label: "bindTarget",
            comment: "The target fiber identifier for this binding.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PropertyBind"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/bindValue",
            label: "bindValue",
            comment: "The value to pin the target fiber to.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PropertyBind"),
            range: XSD_STRING,
        },
        // StageAdvance properties
        Property {
            id: "https://uor.foundation/cascade/advanceFrom",
            label: "advanceFrom",
            comment: "The source stage of the advancement.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/cascade/StageAdvance"),
            range: "https://uor.foundation/cascade/CascadeStage",
        },
        Property {
            id: "https://uor.foundation/cascade/advanceTo",
            label: "advanceTo",
            comment: "The target stage of the advancement.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/cascade/StageAdvance"),
            range: "https://uor.foundation/cascade/CascadeStage",
        },
        // ServiceWindow properties
        Property {
            id: "https://uor.foundation/cascade/windowSize",
            label: "windowSize",
            comment: "The number of recent epochs in this service window.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/ServiceWindow"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/cascade/windowOffset",
            label: "windowOffset",
            comment: "The starting epoch offset of this service window.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/ServiceWindow"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        // CascadeTransaction properties
        Property {
            id: "https://uor.foundation/cascade/transactionPolicy",
            label: "transactionPolicy",
            comment: "The execution policy for this transaction (e.g., AllOrNothing, BestEffort).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeTransaction"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/transactionOutcome",
            label: "transactionOutcome",
            comment: "The outcome of this transaction (e.g., committed, rolled back).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeTransaction"),
            range: XSD_STRING,
        },
        // PipelineFailureReason properties
        Property {
            id: "https://uor.foundation/cascade/failureKind",
            label: "failureKind",
            comment: "The kind of pipeline failure (e.g., DispatchMiss, ConvergenceStall).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PipelineFailureReason"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/failureDetail",
            label: "failureDetail",
            comment: "Detailed description of the pipeline failure.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PipelineFailureReason"),
            range: XSD_STRING,
        },
        // PreflightCheck properties
        Property {
            id: "https://uor.foundation/cascade/preflightKind",
            label: "preflightKind",
            comment: "The kind of preflight check (e.g., feasibility, dispatch coverage).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PreflightCheck"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/preflightResult",
            label: "preflightResult",
            comment: "The result of the preflight check (e.g., pass, fail).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PreflightCheck"),
            range: XSD_STRING,
        },
        // PipelineSuccess properties
        Property {
            id: "https://uor.foundation/cascade/successOutcome",
            label: "successOutcome",
            comment: "Description of the successful pipeline termination.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PipelineSuccess"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/saturationReached",
            label: "saturationReached",
            comment: "Whether full saturation was achieved.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PipelineSuccess"),
            range: XSD_BOOLEAN,
        },
        // Amendment 65: Cascade Completion (11 properties)
        // FeasibilityResult properties
        Property {
            id: "https://uor.foundation/cascade/feasibilityKind",
            label: "feasibilityKind",
            comment: "The kind of feasibility result (e.g., Feasible, Infeasible).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/FeasibilityResult"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/feasibilityWitness",
            label: "feasibilityWitness",
            comment: "The witness justifying the feasibility or infeasibility result.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/FeasibilityResult"),
            range: XSD_STRING,
        },
        // LeaseState properties
        Property {
            id: "https://uor.foundation/cascade/leasePhase",
            label: "leasePhase",
            comment: "The lifecycle phase of a lease (e.g., Pending, Active, Released).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/LeaseState"),
            range: XSD_STRING,
        },
        // ManagedLease properties
        Property {
            id: "https://uor.foundation/cascade/managedLeaseId",
            label: "managedLeaseId",
            comment: "Unique identifier for this managed lease.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/ManagedLease"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/leaseLifecycle",
            label: "leaseLifecycle",
            comment: "The current lifecycle state of this managed lease.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/cascade/ManagedLease"),
            range: "https://uor.foundation/cascade/LeaseState",
        },
        // LeaseCheckpoint properties
        Property {
            id: "https://uor.foundation/cascade/checkpointEpoch",
            label: "checkpointEpoch",
            comment: "The epoch index at which this checkpoint was taken.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/LeaseCheckpoint"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/cascade/checkpointState",
            label: "checkpointState",
            comment: "The cascade state captured at this checkpoint.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/cascade/LeaseCheckpoint"),
            range: "https://uor.foundation/cascade/CascadeState",
        },
        // BackPressureSignal properties
        Property {
            id: "https://uor.foundation/cascade/pressureLevel",
            label: "pressureLevel",
            comment: "The current back-pressure level (e.g., Low, Medium, High).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/BackPressureSignal"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/pressureThreshold",
            label: "pressureThreshold",
            comment: "The threshold at which back-pressure activates.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/BackPressureSignal"),
            range: XSD_STRING,
        },
        // DeferredQuerySet properties
        Property {
            id: "https://uor.foundation/cascade/deferredCount",
            label: "deferredCount",
            comment: "The number of queries in this deferred set.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/DeferredQuerySet"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/cascade/deferralEpoch",
            label: "deferralEpoch",
            comment: "The epoch in which these queries were deferred.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/DeferredQuerySet"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        // Amendment 71: SubleaseTransfer properties (4)
        Property {
            id: "https://uor.foundation/cascade/sourceLeaseRef",
            label: "sourceLeaseRef",
            comment: "The lease being transferred from.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/SubleaseTransfer"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/targetLeaseRef",
            label: "targetLeaseRef",
            comment: "The lease being transferred to.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/SubleaseTransfer"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/transferredBudget",
            label: "transferredBudget",
            comment: "The fiber budget transferred between leases.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/SubleaseTransfer"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/cascade/transferCompleted",
            label: "transferCompleted",
            comment: "Whether the sublease transfer has been completed.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/SubleaseTransfer"),
            range: XSD_BOOLEAN,
        },
        // Amendment 71: Predicate subclass properties (15)
        Property {
            id: "https://uor.foundation/cascade/comparisonField",
            label: "comparisonField",
            comment: "The state field tested by this comparison predicate.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/ComparisonPredicate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/comparisonOperator",
            label: "comparisonOperator",
            comment: "The comparison operator (e.g., '=', '<', '>=').",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/ComparisonPredicate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/comparisonValue",
            label: "comparisonValue",
            comment: "The value against which the comparison is made.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/ComparisonPredicate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/conjuncts",
            label: "conjuncts",
            comment: "A conjunct predicate in a conjunction.",
            kind: PropertyKind::Datatype,
            functional: false,
            domain: Some("https://uor.foundation/cascade/ConjunctionPredicate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/disjuncts",
            label: "disjuncts",
            comment: "A disjunct predicate in a disjunction.",
            kind: PropertyKind::Datatype,
            functional: false,
            domain: Some("https://uor.foundation/cascade/DisjunctionPredicate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/negatedPredicate",
            label: "negatedPredicate",
            comment: "The predicate being negated.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/NegationPredicate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/membershipSet",
            label: "membershipSet",
            comment: "The set against which membership is tested.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/MembershipPredicate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/membershipElement",
            label: "membershipElement",
            comment: "The element being tested for set membership.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/MembershipPredicate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/saturationThreshold",
            label: "saturationThreshold",
            comment: "The saturation threshold above which the predicate holds.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/SaturationPredicate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/coverageTarget",
            label: "coverageTarget",
            comment: "The fiber coverage target expression.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/FiberCoveragePredicate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/equalityLeft",
            label: "equalityLeft",
            comment: "The left-hand side of an equality test.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/EqualsPredicate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/equalityRight",
            label: "equalityRight",
            comment: "The right-hand side of an equality test.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/EqualsPredicate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/nonNullField",
            label: "nonNullField",
            comment: "The field that must be non-null.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/NonNullPredicate"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/queryTypeRef",
            label: "queryTypeRef",
            comment: "The query type reference for subtype testing.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/QuerySubtypePredicate"),
            range: XSD_STRING,
        },
        // Amendment 71: Missing cascade properties (15)
        Property {
            id: "https://uor.foundation/cascade/fiberState",
            label: "fiberState",
            comment: "The fiber state descriptor within a cascade state.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeState"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/transactionScope",
            label: "transactionScope",
            comment: "The scope of fibers affected by this transaction.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeTransaction"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/transactionStatus",
            label: "transactionStatus",
            comment: "Current status of this transaction (e.g., pending, committed).",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/CascadeTransaction"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/baseContextRef",
            label: "baseContextRef",
            comment: "Reference to the base context provided by this service window.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/ServiceWindow"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/leaseRemainingBudget",
            label: "leaseRemainingBudget",
            comment: "The remaining fiber budget at this checkpoint.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/LeaseCheckpoint"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/cascade/expiryEpoch",
            label: "expiryEpoch",
            comment: "The epoch at which this managed lease expires.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/ManagedLease"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/cascade/leaseBudget",
            label: "leaseBudget",
            comment: "The total fiber budget allocated to this managed lease.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/ManagedLease"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/cascade/sourceStage",
            label: "sourceStage",
            comment: "The source stage emitting back-pressure.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/BackPressureSignal"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/targetStage",
            label: "targetStage",
            comment: "The target stage receiving back-pressure.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/BackPressureSignal"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/deferralReason",
            label: "deferralReason",
            comment: "The reason for deferring these queries.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/DeferredQuerySet"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/infeasibilityKind",
            label: "infeasibilityKind",
            comment: "The kind of infeasibility detected.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/FeasibilityResult"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/infeasibilityDetail",
            label: "infeasibilityDetail",
            comment: "Detailed description of why infeasibility was detected.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/FeasibilityResult"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/failureStage",
            label: "failureStage",
            comment: "The cascade stage at which the pipeline failure occurred.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PipelineFailureReason"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/finalSaturation",
            label: "finalSaturation",
            comment: "The final saturation level achieved on pipeline success.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/PipelineSuccess"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/cascade/preservedSaturation",
            label: "preservedSaturation",
            comment: "Whether saturation was preserved across the epoch boundary.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/cascade/EpochBoundary"),
            range: XSD_BOOLEAN,
        },
    ]
}

fn individuals() -> Vec<Individual> {
    vec![
        // 7 CascadeStage individuals
        Individual {
            id: "https://uor.foundation/cascade/stage_initialization",
            type_: "https://uor.foundation/cascade/CascadeStage",
            label: "Initialization",
            comment: "Stage 0: initialize state vector to identity.",
            properties: &[
                (
                    "https://uor.foundation/cascade/stageIndex",
                    IndividualValue::Int(0),
                ),
                (
                    "https://uor.foundation/cascade/stageName",
                    IndividualValue::Str("Initialization"),
                ),
                (
                    "https://uor.foundation/cascade/expectedPhase",
                    IndividualValue::Str("\u{03a9}\u{2070}"),
                ),
                (
                    "https://uor.foundation/cascade/entryCondition",
                    IndividualValue::Str("true (initial stage)"),
                ),
                (
                    "https://uor.foundation/cascade/exitCondition",
                    IndividualValue::Str("state vector is 1"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/stage_declare",
            type_: "https://uor.foundation/cascade/CascadeStage",
            label: "Declare",
            comment: "Stage 1: dispatch resolver (\u{03b4} selects).",
            properties: &[
                (
                    "https://uor.foundation/cascade/stageIndex",
                    IndividualValue::Int(1),
                ),
                (
                    "https://uor.foundation/cascade/stageName",
                    IndividualValue::Str("Declare"),
                ),
                (
                    "https://uor.foundation/cascade/expectedPhase",
                    IndividualValue::Str("\u{03a9}\u{00b9}"),
                ),
                (
                    "https://uor.foundation/cascade/entryCondition",
                    IndividualValue::Str("state vector initialized"),
                ),
                (
                    "https://uor.foundation/cascade/exitCondition",
                    IndividualValue::Str("resolver dispatched"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/stage_factorize",
            type_: "https://uor.foundation/cascade/CascadeStage",
            label: "Factorize",
            comment: "Stage 2: produce valid ring address (G grounds).",
            properties: &[
                (
                    "https://uor.foundation/cascade/stageIndex",
                    IndividualValue::Int(2),
                ),
                (
                    "https://uor.foundation/cascade/stageName",
                    IndividualValue::Str("Factorize"),
                ),
                (
                    "https://uor.foundation/cascade/expectedPhase",
                    IndividualValue::Str("\u{03a9}\u{00b2}"),
                ),
                (
                    "https://uor.foundation/cascade/entryCondition",
                    IndividualValue::Str("resolver dispatched"),
                ),
                (
                    "https://uor.foundation/cascade/exitCondition",
                    IndividualValue::Str("ring address valid"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/stage_resolve",
            type_: "https://uor.foundation/cascade/CascadeStage",
            label: "Resolve",
            comment: "Stage 3: resolve constraints (\u{03a0} terminates).",
            properties: &[
                (
                    "https://uor.foundation/cascade/stageIndex",
                    IndividualValue::Int(3),
                ),
                (
                    "https://uor.foundation/cascade/stageName",
                    IndividualValue::Str("Resolve"),
                ),
                (
                    "https://uor.foundation/cascade/expectedPhase",
                    IndividualValue::Str("\u{03a9}\u{00b3}"),
                ),
                (
                    "https://uor.foundation/cascade/entryCondition",
                    IndividualValue::Str("ring address valid"),
                ),
                (
                    "https://uor.foundation/cascade/exitCondition",
                    IndividualValue::Str("constraints resolved"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/stage_attest",
            type_: "https://uor.foundation/cascade/CascadeStage",
            label: "Attest",
            comment: "Stage 4: accumulate without contradiction (\u{03b1} consistent).",
            properties: &[
                (
                    "https://uor.foundation/cascade/stageIndex",
                    IndividualValue::Int(4),
                ),
                (
                    "https://uor.foundation/cascade/stageName",
                    IndividualValue::Str("Attest"),
                ),
                (
                    "https://uor.foundation/cascade/expectedPhase",
                    IndividualValue::Str("\u{03a9}\u{2074}"),
                ),
                (
                    "https://uor.foundation/cascade/entryCondition",
                    IndividualValue::Str("constraints resolved"),
                ),
                (
                    "https://uor.foundation/cascade/exitCondition",
                    IndividualValue::Str("accumulation consistent"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/stage_extract",
            type_: "https://uor.foundation/cascade/CascadeStage",
            label: "Extract",
            comment: "Stage 5: extract coherent output (P projects).",
            properties: &[
                (
                    "https://uor.foundation/cascade/stageIndex",
                    IndividualValue::Int(5),
                ),
                (
                    "https://uor.foundation/cascade/stageName",
                    IndividualValue::Str("Extract"),
                ),
                (
                    "https://uor.foundation/cascade/expectedPhase",
                    IndividualValue::Str("\u{03a9}\u{2075}"),
                ),
                (
                    "https://uor.foundation/cascade/entryCondition",
                    IndividualValue::Str("accumulation consistent"),
                ),
                (
                    "https://uor.foundation/cascade/exitCondition",
                    IndividualValue::Str("output projected"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/stage_convergence",
            type_: "https://uor.foundation/cascade/CascadeStage",
            label: "Convergence",
            comment: "Terminal stage: cascade has reached the convergence angle \u{03c0}.",
            properties: &[
                (
                    "https://uor.foundation/cascade/stageIndex",
                    IndividualValue::Int(6),
                ),
                (
                    "https://uor.foundation/cascade/stageName",
                    IndividualValue::Str("Convergence"),
                ),
                (
                    "https://uor.foundation/cascade/expectedPhase",
                    IndividualValue::Str("\u{03c0}"),
                ),
                (
                    "https://uor.foundation/cascade/entryCondition",
                    IndividualValue::Str("output projected"),
                ),
                (
                    "https://uor.foundation/cascade/exitCondition",
                    IndividualValue::Str("convergence achieved"),
                ),
            ],
        },
        // 6 PhaseGateAttestation individuals
        Individual {
            id: "https://uor.foundation/cascade/gate_initialization",
            type_: "https://uor.foundation/cascade/PhaseGateAttestation",
            label: "gate_initialization",
            comment: "Phase gate at stage 0 boundary: checks \u{03a9}\u{2070} = 1.",
            properties: &[
                (
                    "https://uor.foundation/cascade/gateStage",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cascade/stage_initialization",
                    ),
                ),
                (
                    "https://uor.foundation/cascade/gateExpectedPhase",
                    IndividualValue::Str("\u{03a9}\u{2070}"),
                ),
                (
                    "https://uor.foundation/cascade/gateResult",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/gate_declare",
            type_: "https://uor.foundation/cascade/PhaseGateAttestation",
            label: "gate_declare",
            comment: "Phase gate at stage 1 boundary: checks \u{03a9}\u{00b9}.",
            properties: &[
                (
                    "https://uor.foundation/cascade/gateStage",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cascade/stage_declare",
                    ),
                ),
                (
                    "https://uor.foundation/cascade/gateExpectedPhase",
                    IndividualValue::Str("\u{03a9}\u{00b9}"),
                ),
                (
                    "https://uor.foundation/cascade/gateResult",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/gate_factorize",
            type_: "https://uor.foundation/cascade/PhaseGateAttestation",
            label: "gate_factorize",
            comment: "Phase gate at stage 2 boundary: checks \u{03a9}\u{00b2}.",
            properties: &[
                (
                    "https://uor.foundation/cascade/gateStage",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cascade/stage_factorize",
                    ),
                ),
                (
                    "https://uor.foundation/cascade/gateExpectedPhase",
                    IndividualValue::Str("\u{03a9}\u{00b2}"),
                ),
                (
                    "https://uor.foundation/cascade/gateResult",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/gate_resolve",
            type_: "https://uor.foundation/cascade/PhaseGateAttestation",
            label: "gate_resolve",
            comment: "Phase gate at stage 3 boundary: checks \u{03a9}\u{00b3}.",
            properties: &[
                (
                    "https://uor.foundation/cascade/gateStage",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cascade/stage_resolve",
                    ),
                ),
                (
                    "https://uor.foundation/cascade/gateExpectedPhase",
                    IndividualValue::Str("\u{03a9}\u{00b3}"),
                ),
                (
                    "https://uor.foundation/cascade/gateResult",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/gate_attest",
            type_: "https://uor.foundation/cascade/PhaseGateAttestation",
            label: "gate_attest",
            comment: "Phase gate at stage 4 boundary: checks \u{03a9}\u{2074}.",
            properties: &[
                (
                    "https://uor.foundation/cascade/gateStage",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cascade/stage_attest",
                    ),
                ),
                (
                    "https://uor.foundation/cascade/gateExpectedPhase",
                    IndividualValue::Str("\u{03a9}\u{2074}"),
                ),
                (
                    "https://uor.foundation/cascade/gateResult",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/gate_extract",
            type_: "https://uor.foundation/cascade/PhaseGateAttestation",
            label: "gate_extract",
            comment: "Phase gate at stage 5 boundary: checks \u{03a9}\u{2075}.",
            properties: &[
                (
                    "https://uor.foundation/cascade/gateStage",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cascade/stage_extract",
                    ),
                ),
                (
                    "https://uor.foundation/cascade/gateExpectedPhase",
                    IndividualValue::Str("\u{03a9}\u{2075}"),
                ),
                (
                    "https://uor.foundation/cascade/gateResult",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // Cascade-level individuals
        Individual {
            id: "https://uor.foundation/cascade/euler_cascade_instance",
            type_: "https://uor.foundation/cascade/EulerCascade",
            label: "euler_cascade_instance",
            comment: "The canonical Euler cascade instance with \u{03a9} = e^{i\u{03c0}/6} \
                      and 6 stages.",
            properties: &[
                (
                    "https://uor.foundation/cascade/phaseParameter",
                    IndividualValue::Str("e^{i\u{03c0}/6}"),
                ),
                (
                    "https://uor.foundation/cascade/stageCount",
                    IndividualValue::Int(6),
                ),
                (
                    "https://uor.foundation/cascade/convergenceAngle",
                    IndividualValue::Str("\u{03c0}"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/phase_schedule",
            type_: "https://uor.foundation/cascade/PhaseRotationScheduler",
            label: "phase_schedule",
            comment: "The canonical phase rotation schedule for the 6-stage cascade.",
            properties: &[
                (
                    "https://uor.foundation/cascade/rotationSchedule",
                    IndividualValue::Str("\u{03a9}\u{2070}, \u{03a9}\u{00b9}, \u{03a9}\u{00b2}, \u{03a9}\u{00b3}, \u{03a9}\u{2074}, \u{03a9}\u{2075}"),
                ),
                (
                    "https://uor.foundation/cascade/baseAngle",
                    IndividualValue::Str("\u{03c0}/6"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/convergence_target",
            type_: "https://uor.foundation/cascade/TargetConvergenceAngle",
            label: "convergence_target",
            comment: "The default convergence target angle \u{03c0}.",
            properties: &[
                (
                    "https://uor.foundation/cascade/targetAngle",
                    IndividualValue::Str("\u{03c0}"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/conjugate_rollback",
            type_: "https://uor.foundation/cascade/ComplexConjugateRollback",
            label: "conjugate_rollback",
            comment: "The canonical complex conjugate rollback operation: z \u{2192} z\u{0304}.",
            properties: &[
                (
                    "https://uor.foundation/cascade/rollbackTarget",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cascade/stage_initialization",
                    ),
                ),
            ],
        },
        // Amendment 64: PipelineFailureReason individuals (5)
        Individual {
            id: "https://uor.foundation/cascade/DispatchMiss",
            type_: "https://uor.foundation/cascade/PipelineFailureReason",
            label: "DispatchMiss",
            comment: "Failure: no resolver matched the dispatch query.",
            properties: &[
                (
                    "https://uor.foundation/cascade/failureKind",
                    IndividualValue::Str("DispatchMiss"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/GroundingFailure",
            type_: "https://uor.foundation/cascade/PipelineFailureReason",
            label: "GroundingFailure",
            comment: "Failure: grounding to a valid ring address failed.",
            properties: &[
                (
                    "https://uor.foundation/cascade/failureKind",
                    IndividualValue::Str("GroundingFailure"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/ConvergenceStall",
            type_: "https://uor.foundation/cascade/PipelineFailureReason",
            label: "ConvergenceStall",
            comment: "Failure: cascade stalled before reaching convergence angle.",
            properties: &[
                (
                    "https://uor.foundation/cascade/failureKind",
                    IndividualValue::Str("ConvergenceStall"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/ContradictionDetected",
            type_: "https://uor.foundation/cascade/PipelineFailureReason",
            label: "ContradictionDetected",
            comment: "Failure: accumulation detected a logical contradiction.",
            properties: &[
                (
                    "https://uor.foundation/cascade/failureKind",
                    IndividualValue::Str("ContradictionDetected"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/CoherenceViolation",
            type_: "https://uor.foundation/cascade/PipelineFailureReason",
            label: "CoherenceViolation",
            comment: "Failure: coherence constraint violated during extraction.",
            properties: &[
                (
                    "https://uor.foundation/cascade/failureKind",
                    IndividualValue::Str("CoherenceViolation"),
                ),
            ],
        },
        // PipelineSuccess individual (1)
        Individual {
            id: "https://uor.foundation/cascade/FullSaturationSuccess",
            type_: "https://uor.foundation/cascade/PipelineSuccess",
            label: "FullSaturationSuccess",
            comment: "Successful termination: all fibers saturated.",
            properties: &[
                (
                    "https://uor.foundation/cascade/successOutcome",
                    IndividualValue::Str("FullSaturation"),
                ),
                (
                    "https://uor.foundation/cascade/saturationReached",
                    IndividualValue::Bool(true),
                ),
            ],
        },
        // PreflightCheck individuals (3)
        Individual {
            id: "https://uor.foundation/cascade/FeasibilityCheck",
            type_: "https://uor.foundation/cascade/PreflightCheck",
            label: "FeasibilityCheck",
            comment: "Preflight: checks that the cascade can reach convergence.",
            properties: &[
                (
                    "https://uor.foundation/cascade/preflightKind",
                    IndividualValue::Str("Feasibility"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/DispatchCoverageCheck",
            type_: "https://uor.foundation/cascade/PreflightCheck",
            label: "DispatchCoverageCheck",
            comment: "Preflight: checks that every dispatch query has a resolver.",
            properties: &[
                (
                    "https://uor.foundation/cascade/preflightKind",
                    IndividualValue::Str("DispatchCoverage"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/PackageCoherenceCheck",
            type_: "https://uor.foundation/cascade/PreflightCheck",
            label: "PackageCoherenceCheck",
            comment: "Preflight: checks package-level coherence constraints.",
            properties: &[
                (
                    "https://uor.foundation/cascade/preflightKind",
                    IndividualValue::Str("PackageCoherence"),
                ),
            ],
        },
        // ServiceWindow individual (1)
        Individual {
            id: "https://uor.foundation/cascade/default_service_window",
            type_: "https://uor.foundation/cascade/ServiceWindow",
            label: "default_service_window",
            comment: "The default service window: 3 epochs, zero offset.",
            properties: &[
                (
                    "https://uor.foundation/cascade/windowSize",
                    IndividualValue::Int(3),
                ),
                (
                    "https://uor.foundation/cascade/windowOffset",
                    IndividualValue::Int(0),
                ),
            ],
        },
        // StageAdvance individual (1)
        Individual {
            id: "https://uor.foundation/cascade/advance_init_to_declare",
            type_: "https://uor.foundation/cascade/StageAdvance",
            label: "advance_init_to_declare",
            comment: "Advancement from Initialization to Declare.",
            properties: &[
                (
                    "https://uor.foundation/cascade/advanceFrom",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cascade/stage_initialization",
                    ),
                ),
                (
                    "https://uor.foundation/cascade/advanceTo",
                    IndividualValue::IriRef(
                        "https://uor.foundation/cascade/stage_declare",
                    ),
                ),
            ],
        },
        // CascadeTransaction individual (1)
        Individual {
            id: "https://uor.foundation/cascade/atomic_transaction",
            type_: "https://uor.foundation/cascade/CascadeTransaction",
            label: "atomic_transaction",
            comment: "An all-or-nothing atomic cascade transaction.",
            properties: &[
                (
                    "https://uor.foundation/cascade/transactionPolicy",
                    IndividualValue::Str("AllOrNothing"),
                ),
            ],
        },
        // GuardExpression individual (1)
        Individual {
            id: "https://uor.foundation/cascade/empty_guard",
            type_: "https://uor.foundation/cascade/GuardExpression",
            label: "empty_guard",
            comment: "A trivially satisfied guard with no predicates.",
            properties: &[],
        },
        // TransitionEffect individual (1)
        Individual {
            id: "https://uor.foundation/cascade/identity_effect",
            type_: "https://uor.foundation/cascade/TransitionEffect",
            label: "identity_effect",
            comment: "The identity effect: no state changes.",
            properties: &[],
        },
        // PredicateExpression individual (1)
        Individual {
            id: "https://uor.foundation/cascade/true_predicate",
            type_: "https://uor.foundation/cascade/PredicateExpression",
            label: "true_predicate",
            comment: "A predicate that always evaluates to true.",
            properties: &[
                (
                    "https://uor.foundation/cascade/predicateField",
                    IndividualValue::Str("*"),
                ),
                (
                    "https://uor.foundation/cascade/predicateOperator",
                    IndividualValue::Str("true"),
                ),
                (
                    "https://uor.foundation/cascade/predicateValue",
                    IndividualValue::Str("*"),
                ),
            ],
        },
        // PropertyBind individual (1)
        Individual {
            id: "https://uor.foundation/cascade/noop_bind",
            type_: "https://uor.foundation/cascade/PropertyBind",
            label: "noop_bind",
            comment: "A no-op property binding that changes nothing.",
            properties: &[
                (
                    "https://uor.foundation/cascade/bindTarget",
                    IndividualValue::Str("none"),
                ),
                (
                    "https://uor.foundation/cascade/bindValue",
                    IndividualValue::Str("unchanged"),
                ),
            ],
        },
        // Amendment 65: LeaseState individuals (5)
        Individual {
            id: "https://uor.foundation/cascade/Pending",
            type_: "https://uor.foundation/cascade/LeaseState",
            label: "Pending",
            comment: "Lease is pending activation.",
            properties: &[
                (
                    "https://uor.foundation/cascade/leasePhase",
                    IndividualValue::Str("Pending"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/Active",
            type_: "https://uor.foundation/cascade/LeaseState",
            label: "Active",
            comment: "Lease is active and resources are allocated.",
            properties: &[
                (
                    "https://uor.foundation/cascade/leasePhase",
                    IndividualValue::Str("Active"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/Released",
            type_: "https://uor.foundation/cascade/LeaseState",
            label: "Released",
            comment: "Lease has been explicitly released.",
            properties: &[
                (
                    "https://uor.foundation/cascade/leasePhase",
                    IndividualValue::Str("Released"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/Expired",
            type_: "https://uor.foundation/cascade/LeaseState",
            label: "Expired",
            comment: "Lease has expired due to timeout.",
            properties: &[
                (
                    "https://uor.foundation/cascade/leasePhase",
                    IndividualValue::Str("Expired"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/Suspended",
            type_: "https://uor.foundation/cascade/LeaseState",
            label: "Suspended",
            comment: "Lease is temporarily suspended.",
            properties: &[
                (
                    "https://uor.foundation/cascade/leasePhase",
                    IndividualValue::Str("Suspended"),
                ),
            ],
        },
        // FeasibilityResult individuals (2)
        Individual {
            id: "https://uor.foundation/cascade/FeasibilityWitness",
            type_: "https://uor.foundation/cascade/FeasibilityResult",
            label: "FeasibilityWitness",
            comment: "Preflight result: cascade is feasible.",
            properties: &[
                (
                    "https://uor.foundation/cascade/feasibilityKind",
                    IndividualValue::Str("Feasible"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/InfeasibilityWitness",
            type_: "https://uor.foundation/cascade/FeasibilityResult",
            label: "InfeasibilityWitness",
            comment: "Preflight result: cascade is infeasible.",
            properties: &[
                (
                    "https://uor.foundation/cascade/feasibilityKind",
                    IndividualValue::Str("Infeasible"),
                ),
                (
                    "https://uor.foundation/cascade/feasibilityWitness",
                    IndividualValue::Str("obstruction detected"),
                ),
            ],
        },
        // Timing / misc individuals (3)
        Individual {
            id: "https://uor.foundation/cascade/PreflightTiming",
            type_: "https://uor.foundation/cascade/PreflightCheck",
            label: "PreflightTiming",
            comment: "Preflight: timing feasibility check.",
            properties: &[
                (
                    "https://uor.foundation/cascade/preflightKind",
                    IndividualValue::Str("Timing"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/RuntimeTiming",
            type_: "https://uor.foundation/cascade/PreflightCheck",
            label: "RuntimeTiming",
            comment: "Preflight: runtime timing bounds check.",
            properties: &[
                (
                    "https://uor.foundation/cascade/preflightKind",
                    IndividualValue::Str("RuntimeTiming"),
                ),
            ],
        },
        Individual {
            id: "https://uor.foundation/cascade/BackPressureDefault",
            type_: "https://uor.foundation/cascade/BackPressureSignal",
            label: "BackPressureDefault",
            comment: "Default back-pressure signal with medium threshold.",
            properties: &[
                (
                    "https://uor.foundation/cascade/pressureLevel",
                    IndividualValue::Str("Medium"),
                ),
                (
                    "https://uor.foundation/cascade/pressureThreshold",
                    IndividualValue::Str("0.75"),
                ),
            ],
        },
    ]
}
