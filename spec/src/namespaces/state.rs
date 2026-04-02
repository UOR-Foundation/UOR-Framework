//! `state/` namespace — Parameterized address spaces and state model (Amendment 7).
//!
//! The state namespace defines the mutable user-space model for the UOR kernel.
//! State is the user-space overlay onto the kernel's read-only substrate: contexts
//! hold bindings, frames provide visibility windows, and transitions record state
//! changes.
//!
//! Amendment 27 adds the session lifecycle: `Session`, `BindingAccumulator`,
//! `SessionBoundary`, and `SessionBoundaryType` (a typed controlled vocabulary for
//! boundary reasons — ExplicitReset, ConvergenceBoundary, ContradictionBoundary).
//!
//! Amendment 48 adds the multi-session coordination layer: `SharedContext`,
//! `ContextLease`, and `SessionComposition` — enabling concurrent sessions on
//! disjoint fiber leases and composition of completed sessions.
//!
//! **Space classification:** `user` — state is managed by user-space (Prism).

use crate::model::iris::*;
use crate::model::{
    Class, Individual, IndividualValue, Namespace, NamespaceModule, Property, PropertyKind, Space,
};

/// Returns the `state/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "state",
            iri: NS_STATE,
            label: "UOR State",
            comment: "Parameterized address spaces, context management, binding \
                      lifecycle, and state transitions. The user-space overlay \
                      onto the kernel's read-only ring substrate.",
            space: Space::User,
            imports: &[
                NS_U,
                NS_SCHEMA,
                NS_TYPE,
                NS_PARTITION,
                NS_TRACE,
                NS_MORPHISM,
                NS_CERT,
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
            id: "https://uor.foundation/state/Context",
            label: "Context",
            comment: "A bounded set of populated UOR addresses. The parameter space \
                      for a resolution cycle. Contexts hold bindings that map \
                      addresses to datum values.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/state/Binding",
                "https://uor.foundation/state/Frame",
                "https://uor.foundation/state/Transition",
            ],
        },
        Class {
            id: "https://uor.foundation/state/Binding",
            label: "Binding",
            comment: "The association of a datum value with an address in a context. \
                      The write primitive: creating a binding populates an address.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/state/Context",
                "https://uor.foundation/state/Frame",
                "https://uor.foundation/state/Transition",
            ],
        },
        Class {
            id: "https://uor.foundation/state/Frame",
            label: "Frame",
            comment: "The visibility boundary determining which bindings are in scope \
                      for a given resolution. A frame is a view into a context: it \
                      selects which bindings the resolver sees.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/state/Context",
                "https://uor.foundation/state/Binding",
                "https://uor.foundation/state/Transition",
            ],
        },
        Class {
            id: "https://uor.foundation/state/Transition",
            label: "Transition",
            comment: "A state change: the transformation of one context into another \
                      through binding or unbinding. The sequence of transitions is the \
                      application's computation history.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/state/Context",
                "https://uor.foundation/state/Binding",
                "https://uor.foundation/state/Frame",
            ],
        },
        // Amendment 27: Session-Scoped Resolution
        Class {
            id: "https://uor.foundation/state/SessionBoundaryType",
            label: "SessionBoundaryType",
            comment: "A typed controlled vocabulary for session boundary reasons. \
                      Each individual names a specific reason a context-reset boundary \
                      was triggered during a multi-turn session.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/state/Session",
            label: "Session",
            comment: "A bounded sequence of RelationQuery/response pairs sharing \
                      a common state:Context. Sessions are the unit of coherent \
                      multi-turn reasoning in Prism.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/state/BindingAccumulator",
            label: "BindingAccumulator",
            comment: "The mutable accumulator that appends state:Binding instances \
                      to a state:Context as each RelationQuery resolves. Tracks \
                      monotonic reduction of aggregate free fiber space.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/state/SessionBoundary",
            label: "SessionBoundary",
            comment: "Marks a context-reset event within a session stream. \
                      Records why the context was reset and provides a clean \
                      state:Context for subsequent queries.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 33: Saturated Context Limit
        Class {
            id: "https://uor.foundation/state/SaturatedContext",
            label: "SaturatedContext",
            comment: "A context that has reached full saturation: σ = 1, \
                      freeCount = 0, S = 0, T_ctx = 0 (SC_4). The ground \
                      state of the type system. All subsequent queries \
                      resolve in O(1) via SC_5.",
            subclass_of: &["https://uor.foundation/state/Context"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/state/SaturationWitness",
            label: "SaturationWitness",
            comment: "Step-by-step evidence of the saturation process: records \
                      which bindings were applied, in what order, to reach \
                      full saturation.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/state/DomainSaturationRecord",
            label: "DomainSaturationRecord",
            comment: "An informational/monitoring record tracking the saturation \
                      progress of a specific domain within a context. Carries no \
                      formal authority — purely observational.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/state/SaturationPhase",
            label: "SaturationPhase",
            comment: "A typed controlled vocabulary for the three phases of \
                      context saturation: Unsaturated (σ = 0), \
                      PartialSaturation (0 < σ < 1), and FullSaturation (σ = 1).",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        // Amendment 48: Multi-Session Coordination classes
        Class {
            id: "https://uor.foundation/state/SharedContext",
            label: "SharedContext",
            comment: "A Context visible to more than one Session simultaneously. \
                      Holds a set of ContextLease instances that partition its \
                      fiber coordinates among active sessions. Lease disjointness \
                      (SR_9) prevents concurrent write conflicts.",
            subclass_of: &["https://uor.foundation/state/Context"],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/state/ContextLease",
            label: "ContextLease",
            comment: "A bounded, exclusive claim on a set of fiber coordinates \
                      within a SharedContext, held by exactly one Session. When \
                      the session closes or hits a SessionBoundary, the lease is \
                      released and its fibers become available for re-leasing.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[
                "https://uor.foundation/state/Context",
                "https://uor.foundation/state/Binding",
                "https://uor.foundation/state/Frame",
                "https://uor.foundation/state/Transition",
            ],
        },
        Class {
            id: "https://uor.foundation/state/SessionComposition",
            label: "SessionComposition",
            comment: "Records that a Session was formed by merging the binding \
                      sets of two or more predecessor sessions. Valid only if all \
                      predecessor binding sets pass the cross-session consistency \
                      check (SR_8). An invalid composition attempt produces a \
                      ContradictionBoundary on the target session.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        // Binding properties
        Property {
            id: "https://uor.foundation/state/address",
            label: "address",
            comment: "The UOR address being bound in this binding.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/Binding"),
            range: "https://uor.foundation/u/Address",
        },
        Property {
            id: "https://uor.foundation/state/content",
            label: "content",
            comment: "The datum value bound to the address in this binding.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/Binding"),
            range: "https://uor.foundation/schema/Datum",
        },
        Property {
            id: "https://uor.foundation/state/boundType",
            label: "boundType",
            comment: "The type under which this binding's datum is resolved.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/state/Binding"),
            range: "https://uor.foundation/type/TypeDefinition",
        },
        Property {
            id: "https://uor.foundation/state/timestamp",
            label: "timestamp",
            comment: "The time at which this binding was created.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/state/Binding"),
            range: XSD_DATETIME,
        },
        // Context properties
        Property {
            id: "https://uor.foundation/state/binding",
            label: "binding",
            comment: "A binding held in this context.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/state/Context"),
            range: "https://uor.foundation/state/Binding",
        },
        Property {
            id: "https://uor.foundation/state/capacity",
            label: "capacity",
            comment: "The maximum number of bindings this context can hold.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/state/Context"),
            range: XSD_POSITIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/state/contentAddress",
            label: "contentAddress",
            comment: "The content-derived address of this context, uniquely \
                      identifying its current state in the UOR address space.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/state/Context"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/state/quantum",
            label: "quantum",
            comment: "The quantum level of this context's address space.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/state/Context"),
            range: XSD_POSITIVE_INTEGER,
        },
        // Frame properties
        Property {
            id: "https://uor.foundation/state/activeBindings",
            label: "activeBindings",
            comment: "The bindings currently in scope for this frame.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/state/Frame"),
            range: "https://uor.foundation/state/Binding",
        },
        Property {
            id: "https://uor.foundation/state/context",
            label: "context",
            comment: "The context this frame is a view of.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/Frame"),
            range: "https://uor.foundation/state/Context",
        },
        Property {
            id: "https://uor.foundation/state/constraint",
            label: "constraint",
            comment: "The type:Constraint determining which bindings from the context are \
                      visible in this frame. The resolver applies this constraint to filter \
                      the context's binding set, producing the frame's active bindings. \
                      An absent constraint means all bindings are visible.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/Frame"),
            range: "https://uor.foundation/type/Constraint",
        },
        // Transition properties
        Property {
            id: "https://uor.foundation/state/from",
            label: "from",
            comment: "The context before this transition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/Transition"),
            range: "https://uor.foundation/state/Context",
        },
        Property {
            id: "https://uor.foundation/state/to",
            label: "to",
            comment: "The context after this transition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/Transition"),
            range: "https://uor.foundation/state/Context",
        },
        Property {
            id: "https://uor.foundation/state/addedBindings",
            label: "addedBindings",
            comment: "Bindings added to the context in this transition.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/state/Transition"),
            range: "https://uor.foundation/state/Binding",
        },
        Property {
            id: "https://uor.foundation/state/removedBindings",
            label: "removedBindings",
            comment: "Bindings removed from the context in this transition.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/state/Transition"),
            range: "https://uor.foundation/state/Binding",
        },
        Property {
            id: "https://uor.foundation/state/trace",
            label: "trace",
            comment: "The computation trace recording the kernel operations that \
                      effected this state transition.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/Transition"),
            range: "https://uor.foundation/trace/ComputationTrace",
        },
        // Amendment 22: topological snapshot
        Property {
            id: "https://uor.foundation/state/topologicalSnapshot",
            label: "topologicalSnapshot",
            comment: "A snapshot of topological invariants at this transition point.",
            domain: Some("https://uor.foundation/state/Transition"),
            kind: PropertyKind::Object,
            functional: true,
            range: "https://uor.foundation/morphism/TopologicalDelta",
        },
        // Amendment 27: Session-Scoped Resolution properties
        Property {
            id: "https://uor.foundation/state/sessionBindings",
            label: "sessionBindings",
            comment: "The shared context holding all bindings accumulated across \
                      the queries in this session.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/Session"),
            range: "https://uor.foundation/state/Context",
        },
        Property {
            id: "https://uor.foundation/state/sessionQueries",
            label: "sessionQueries",
            comment: "The number of RelationQuery evaluations completed in this session.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/state/Session"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/state/aggregateFiberDeficit",
            label: "aggregateFiberDeficit",
            comment: "The aggregate FiberBudget deficit across all accumulated bindings: \
                      the total remaining free fibers that have not yet been closed by \
                      resolution. Decreases monotonically as the session progresses.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/BindingAccumulator"),
            range: "https://uor.foundation/partition/FiberBudget",
        },
        Property {
            id: "https://uor.foundation/state/accumulatedBindings",
            label: "accumulatedBindings",
            comment: "A binding accumulated by this accumulator from a resolved \
                      RelationQuery.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/state/BindingAccumulator"),
            range: "https://uor.foundation/state/Binding",
        },
        Property {
            id: "https://uor.foundation/state/boundaryReason",
            label: "boundaryReason",
            comment: "A human-readable description of why this session boundary \
                      was triggered.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/state/SessionBoundary"),
            range: XSD_STRING,
        },
        Property {
            id: "https://uor.foundation/state/boundaryType",
            label: "boundaryType",
            comment: "The typed reason category for this session boundary.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/SessionBoundary"),
            range: "https://uor.foundation/state/SessionBoundaryType",
        },
        Property {
            id: "https://uor.foundation/state/priorContext",
            label: "priorContext",
            comment: "The state:Context that was active before this boundary reset.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/SessionBoundary"),
            range: "https://uor.foundation/state/Context",
        },
        Property {
            id: "https://uor.foundation/state/freshContext",
            label: "freshContext",
            comment: "The clean state:Context produced after this boundary reset, \
                      ready for subsequent queries.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/SessionBoundary"),
            range: "https://uor.foundation/state/Context",
        },
        // Amendment 33: Saturated Context Limit properties
        Property {
            id: "https://uor.foundation/state/saturationDegree",
            label: "saturationDegree",
            comment: "The saturation degree σ ∈ \\[0, 1\\] of this context. \
                      Defined by SC_2: σ = (n − freeCount) / n.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/state/Context"),
            range: XSD_DECIMAL,
        },
        Property {
            id: "https://uor.foundation/state/contextTemperature",
            label: "contextTemperature",
            comment: "The context temperature T_ctx ∈ \\[0, ln 2\\]. Defined by \
                      SC_1: T_ctx = freeCount × ln 2 / n. At σ = 1, T_ctx = 0.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/state/Context"),
            range: XSD_DECIMAL,
        },
        Property {
            id: "https://uor.foundation/state/isSaturated",
            label: "isSaturated",
            comment: "Whether this context has reached full saturation (σ = 1). \
                      Equivalent to freeCount = 0, S = 0, T_ctx = 0 per SC_4.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/state/Context"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/state/saturationPhase",
            label: "saturationPhase",
            comment: "The current saturation phase of this context: Unsaturated, \
                      PartialSaturation, or FullSaturation.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/Context"),
            range: "https://uor.foundation/state/SaturationPhase",
        },
        Property {
            id: "https://uor.foundation/state/saturationCertificate",
            label: "saturationCertificate",
            comment: "The SaturationCertificate attesting that this context has \
                      reached full saturation.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/SaturatedContext"),
            range: "https://uor.foundation/cert/SaturationCertificate",
        },
        Property {
            id: "https://uor.foundation/state/witnessBinding",
            label: "witnessBinding",
            comment: "A binding that contributed to the saturation process, \
                      recorded in this SaturationWitness.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/state/SaturationWitness"),
            range: "https://uor.foundation/state/Binding",
        },
        Property {
            id: "https://uor.foundation/state/witnessStep",
            label: "witnessStep",
            comment: "The step index at which a particular binding was applied \
                      during the saturation process.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/state/SaturationWitness"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/state/residualFreeCount",
            label: "residualFreeCount",
            comment: "The number of free (unbound) fibers remaining in this \
                      context. At saturation, residualFreeCount = 0.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/state/Context"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/state/saturatedContext",
            label: "saturatedContext",
            comment: "The SaturatedContext that this DomainSaturationRecord \
                      monitors.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/DomainSaturationRecord"),
            range: "https://uor.foundation/state/SaturatedContext",
        },
        Property {
            id: "https://uor.foundation/state/saturatedDomain",
            label: "saturatedDomain",
            comment: "The domain within the context being tracked by this \
                      DomainSaturationRecord.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/DomainSaturationRecord"),
            range: "https://uor.foundation/type/TypeDefinition",
        },
        Property {
            id: "https://uor.foundation/state/domainFreeCount",
            label: "domainFreeCount",
            comment: "The number of free fibers remaining in the specific domain \
                      tracked by this DomainSaturationRecord.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/state/DomainSaturationRecord"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        // Amendment 48: Multi-Session Coordination properties
        Property {
            id: "https://uor.foundation/state/leasedFibers",
            label: "leasedFibers",
            comment: "The subset of fibers claimed by this lease. Must be disjoint \
                      from all other active leases on the same SharedContext (SR_9).",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/ContextLease"),
            range: "https://uor.foundation/partition/FiberBudget",
        },
        Property {
            id: "https://uor.foundation/state/leaseHolder",
            label: "leaseHolder",
            comment: "The Session that holds this lease.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/ContextLease"),
            range: "https://uor.foundation/state/Session",
        },
        Property {
            id: "https://uor.foundation/state/leaseSet",
            label: "leaseSet",
            comment: "A currently active ContextLease on this SharedContext.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/state/SharedContext"),
            range: "https://uor.foundation/state/ContextLease",
        },
        Property {
            id: "https://uor.foundation/state/composedFrom",
            label: "composedFrom",
            comment: "A predecessor session contributing bindings to this \
                      composition. Non-functional: one composition may merge \
                      two or more sessions.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/state/SessionComposition"),
            range: "https://uor.foundation/state/Session",
        },
        Property {
            id: "https://uor.foundation/state/compositionCompatible",
            label: "compositionCompatible",
            comment: "Whether all predecessor binding sets passed the SR_8 \
                      consistency check. If false, the composition is invalid \
                      and must not be used as a session context.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/state/SessionComposition"),
            range: XSD_BOOLEAN,
        },
        Property {
            id: "https://uor.foundation/state/compositionResult",
            label: "compositionResult",
            comment: "The merged Context produced by a valid composition. Only \
                      present when compositionCompatible = true.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/state/SessionComposition"),
            range: "https://uor.foundation/state/Context",
        },
        Property {
            id: "https://uor.foundation/state/towerConsistencyVerified",
            label: "towerConsistencyVerified",
            comment: "Whether the LiftChain tower consistency check (SR_8 \
                      parametric extension) was performed across all Q_0 \
                      through Q_k levels. Required for compositions involving \
                      sessions at Q_1 or higher.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/state/SessionComposition"),
            range: XSD_BOOLEAN,
        },
    ]
}

// Amendment 27: SessionBoundaryType typed controlled vocabulary individuals
fn individuals() -> Vec<Individual> {
    vec![
        Individual {
            id: "https://uor.foundation/state/ExplicitReset",
            type_: "https://uor.foundation/state/SessionBoundaryType",
            label: "ExplicitReset",
            comment: "The caller explicitly requested a context reset. \
                      All accumulated bindings are discarded.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/state/ConvergenceBoundary",
            type_: "https://uor.foundation/state/SessionBoundaryType",
            label: "ConvergenceBoundary",
            comment: "The session resolver determined that no further queries \
                      can reduce the aggregate fiber deficit.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/state/ContradictionBoundary",
            type_: "https://uor.foundation/state/SessionBoundaryType",
            label: "ContradictionBoundary",
            comment: "A new query produced a type contradiction with an \
                      accumulated binding. Context must reset before \
                      resolution can continue.",
            properties: &[],
        },
        // Amendment 33: SaturationPhase vocabulary individuals
        Individual {
            id: "https://uor.foundation/state/Unsaturated",
            type_: "https://uor.foundation/state/SaturationPhase",
            label: "Unsaturated",
            comment: "The context has σ = 0: no bindings accumulated, all fibers \
                      are free. The initial phase of every session.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/state/PartialSaturation",
            type_: "https://uor.foundation/state/SaturationPhase",
            label: "PartialSaturation",
            comment: "The context has 0 < σ < 1: some fibers are pinned by \
                      accumulated bindings, but free fibers remain. The \
                      accumulation phase.",
            properties: &[],
        },
        Individual {
            id: "https://uor.foundation/state/FullSaturation",
            type_: "https://uor.foundation/state/SaturationPhase",
            label: "FullSaturation",
            comment: "The context has σ = 1: all fibers are pinned, freeCount = 0. \
                      The ground state. All subsequent queries resolve in O(1) \
                      via SC_5.",
            properties: &[],
        },
        // Amendment 33: Canonical ground-state witness
        Individual {
            id: "https://uor.foundation/state/ground_state",
            type_: "https://uor.foundation/state/SaturatedContext",
            label: "ground_state",
            comment: "The canonical ground-state witness: a SaturatedContext at \
                      σ = 1, freeCount = 0, T_ctx = 0, S = 0 (SC_4). Demonstrates \
                      that full saturation is achievable and O(1) resolution (SC_5) \
                      is realized.",
            properties: &[
                (
                    "https://uor.foundation/state/saturationDegree",
                    IndividualValue::Str("1.0"),
                ),
                (
                    "https://uor.foundation/state/contextTemperature",
                    IndividualValue::Str("0.0"),
                ),
                (
                    "https://uor.foundation/state/isSaturated",
                    IndividualValue::Bool(true),
                ),
                (
                    "https://uor.foundation/state/residualFreeCount",
                    IndividualValue::Int(0),
                ),
                (
                    "https://uor.foundation/state/saturationPhase",
                    IndividualValue::IriRef("https://uor.foundation/state/FullSaturation"),
                ),
            ],
        },
    ]
}
