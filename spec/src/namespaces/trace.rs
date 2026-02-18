//! `trace/` namespace — Computation execution traces.
//!
//! Traces record the actual execution path of a kernel computation: which
//! operations were applied, in what order, and what the intermediate results
//! were. They are the runtime log of kernel activity.
//!
//! **Space classification:** `bridge` — kernel-produced, user-consumed.

use crate::model::iris::*;
use crate::model::{Class, Namespace, NamespaceModule, Property, PropertyKind, Space};

/// Returns the `trace/` namespace module.
#[must_use]
pub fn module() -> NamespaceModule {
    NamespaceModule {
        namespace: Namespace {
            prefix: "trace",
            iri: NS_TRACE,
            label: "UOR Computation Traces",
            comment: "Execution traces recording the sequence of kernel operations, \
                      intermediate results, and accumulated metrics for a computation.",
            space: Space::Bridge,
            imports: &[NS_SCHEMA, NS_OP],
        },
        classes: classes(),
        properties: properties(),
        individuals: vec![],
    }
}

fn classes() -> Vec<Class> {
    vec![
        Class {
            id: "https://uor.foundation/trace/ComputationTrace",
            label: "ComputationTrace",
            comment: "A complete record of a kernel computation: the input, output, \
                      every operation step, and accumulated metrics.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/trace/ComputationStep",
            label: "ComputationStep",
            comment: "A single step in a computation trace: one operation applied \
                      to produce one output from one or more inputs.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
        Class {
            id: "https://uor.foundation/trace/TraceMetrics",
            label: "TraceMetrics",
            comment: "Summary metrics for a computation trace: total steps, \
                      accumulated ring distance, and accumulated Hamming distance.",
            subclass_of: &[OWL_THING],
            disjoint_with: &[],
        },
    ]
}

fn properties() -> Vec<Property> {
    vec![
        Property {
            id: "https://uor.foundation/trace/input",
            label: "input",
            comment: "The input datum of this computation.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/trace/ComputationTrace"),
            range: "https://uor.foundation/schema/Datum",
        },
        Property {
            id: "https://uor.foundation/trace/output",
            label: "output",
            comment: "The output datum of this computation.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/trace/ComputationTrace"),
            range: "https://uor.foundation/schema/Datum",
        },
        Property {
            id: "https://uor.foundation/trace/step",
            label: "step",
            comment: "A computation step in this trace.",
            kind: PropertyKind::Object,
            functional: false,
            domain: Some("https://uor.foundation/trace/ComputationTrace"),
            range: "https://uor.foundation/trace/ComputationStep",
        },
        Property {
            id: "https://uor.foundation/trace/monodromy",
            label: "monodromy",
            comment: "The monodromy accumulated by this computation: the net \
                      dihedral group element produced by the full operation sequence.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/trace/ComputationTrace"),
            range: "https://uor.foundation/observable/DihedralElement",
        },
        Property {
            id: "https://uor.foundation/trace/from",
            label: "from",
            comment: "The input datum of this computation step.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/trace/ComputationStep"),
            range: "https://uor.foundation/schema/Datum",
        },
        Property {
            id: "https://uor.foundation/trace/to",
            label: "to",
            comment: "The output datum of this computation step.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/trace/ComputationStep"),
            range: "https://uor.foundation/schema/Datum",
        },
        Property {
            id: "https://uor.foundation/trace/operation",
            label: "operation",
            comment: "The operation applied in this computation step.",
            kind: PropertyKind::Object,
            functional: true,
            domain: Some("https://uor.foundation/trace/ComputationStep"),
            range: "https://uor.foundation/op/Operation",
        },
        Property {
            id: "https://uor.foundation/trace/index",
            label: "index",
            comment: "The zero-based sequential index of this step within its trace.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/trace/ComputationStep"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/trace/stepCount",
            label: "stepCount",
            comment: "Total number of computation steps in this trace.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/trace/TraceMetrics"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/trace/totalRingDistance",
            label: "totalRingDistance",
            comment: "Total ring-metric distance accumulated across all steps.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/trace/TraceMetrics"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
        Property {
            id: "https://uor.foundation/trace/totalHammingDistance",
            label: "totalHammingDistance",
            comment: "Total Hamming-metric distance accumulated across all steps.",
            kind: PropertyKind::Datatype,
            functional: true,
            domain: Some("https://uor.foundation/trace/TraceMetrics"),
            range: XSD_NON_NEGATIVE_INTEGER,
        },
    ]
}
