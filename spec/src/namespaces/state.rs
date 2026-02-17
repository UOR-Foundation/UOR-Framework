//! `state/` namespace — Parameterized address spaces and state model (Amendment 7).
//!
//! The state namespace defines the mutable user-space model for the UOR kernel.
//! State is the user-space overlay onto the kernel's read-only substrate: contexts
//! hold bindings, frames provide visibility windows, and transitions record state
//! changes.
//!
//! **Space classification:** `user` — state is managed by user-space (Prism).

use crate::model::{Class, Namespace, NamespaceModule, Property, PropertyKind, Space};
use crate::model::iris::*;

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
            imports: &[NS_U, NS_SCHEMA, NS_TYPE, NS_TRACE],
        },
        classes: classes(),
        properties: properties(),
        individuals: vec![],
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
            comment: "The constraint determining which bindings from the context \
                      are visible in this frame.",
            kind: PropertyKind::Datatype,
            functional: false,
            domain: Some("https://uor.foundation/state/Frame"),
            range: XSD_STRING,
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
    ]
}
