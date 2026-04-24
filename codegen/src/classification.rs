//! Phase 0 classification — maps every ontology class to a `PathKind`.
//!
//! The classification drives every subsequent phase's codegen. Design notes
//! in `docs/orphan-closure/phase-0-classification.md`; the overall 4-path
//! strategy in `docs/orphan-closure/overview.md`.
//!
//! `classify` is a pure, deterministic function. `classify_all` runs it over
//! every class in the ontology. `write_report` emits a human-readable table
//! to `docs/orphan-closure/classification_report.md` — regenerated on every
//! `cargo run --bin uor-crate` and gated by `git diff --exit-code`.

use std::fmt::Write as FmtWrite;
use std::path::Path;

use anyhow::{Context, Result};
use uor_ontology::model::iris::{
    NS_PARALLEL, NS_STREAM, OWL_CLASS, OWL_THING, RDF_LIST, XSD_BOOLEAN, XSD_DECIMAL,
    XSD_HEX_BINARY, XSD_INTEGER, XSD_NON_NEGATIVE_INTEGER, XSD_POSITIVE_INTEGER, XSD_STRING,
};
use uor_ontology::{Class, Ontology, Property, PropertyKind};

use crate::mapping::local_name;

/// Which of the four orphan-closure paths a class belongs to.
///
/// See `docs/orphan-closure/overview.md` for the taxonomy and
/// `docs/orphan-closure/phase-0-classification.md` for the decision
/// procedure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathKind {
    /// Enum classes or `Primitives` — no trait emitted, so not an orphan.
    Skip,

    /// Class already has a concrete impl in `foundation/src/` (Certificate
    /// subclasses, Partition-algebra witnesses). No codegen-time work.
    AlreadyImplemented,

    /// Theory-deferred — cohomology / operad / parallel / stream machinery
    /// awaiting theoretical grounding. Traits stay orphan by design until
    /// theory lands; Phase 6 pairs each with a tracking issue.
    Path4TheoryDeferred,

    /// Theorem-backed witness. Phase 3 emits `{Foo}Witness` +
    /// `{Foo}MintInputs<H>` + `impl VerifiedMint for {Foo}Witness`; Phase 5
    /// fills the verification body per theorem family.
    Path2TheoremWitness {
        /// Whether any of the class's properties carries entropy (per R7).
        /// Determines whether `Hash` is dropped from the witness derives.
        entropy_bearing: bool,
        /// `op:Identity` IRI whose theorem this witness attests. Empty
        /// until R6 is fully wired; Phase 3 uses it in the stub-body
        /// `WITNESS_UNIMPLEMENTED_STUB:{IRI}` marker.
        theorem_identity: String,
    },

    /// Primitive-backed — Phase 4 emits a hand-written blanket impl
    /// delegating to a `primitive_*` function. R13: the named primitive
    /// must exist at classification time.
    Path3PrimitiveBacked {
        /// Name of the `primitive_*` function in `foundation/src/enforcement.rs`.
        primitive_name: String,
    },

    /// Fallthrough — Phase 2 emits `{Foo}Handle` + `{Foo}Resolver` +
    /// `{Foo}Record` + `Resolved{Foo}` and a single `impl {Foo}<H> for
    /// Resolved{Foo}<'r, R, H>`.
    Path1HandleResolver,
}

impl PathKind {
    /// Short textual label, used by the report and by tests.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            PathKind::Skip => "Skip",
            PathKind::AlreadyImplemented => "AlreadyImplemented",
            PathKind::Path4TheoryDeferred => "Path4TheoryDeferred",
            PathKind::Path2TheoremWitness { .. } => "Path2TheoremWitness",
            PathKind::Path3PrimitiveBacked { .. } => "Path3PrimitiveBacked",
            PathKind::Path1HandleResolver => "Path1HandleResolver",
        }
    }
}

/// One classification record.
#[derive(Debug, Clone)]
pub struct ClassificationEntry {
    /// Full class IRI (e.g. `https://uor.foundation/partition/Partition`).
    pub class_iri: &'static str,
    /// Local class name (last IRI segment).
    pub class_local: &'static str,
    /// Namespace prefix (e.g. `partition`, `observable`).
    pub namespace: &'static str,
    /// Assigned path.
    pub path_kind: PathKind,
    /// Short human-readable rationale for the classification.
    pub rationale: String,
}

// ─── Allow-lists — explicit, no heuristics ──────────────────────────────

/// Full class IRIs of ontology-derived traits that already have a concrete
/// `impl` in `foundation/src/`. Verified by greping for
/// `impl(<...>)? crate::<ns>::<module>::<Class><...>? for <Type>`.
///
/// Local names collide across namespaces (`cert::GroundingCertificate`
/// vs `morphism::GroundingCertificate`) so this list uses full IRIs.
///
/// Phase 0 baseline (after the Product/Coproduct Amendment §845c0ff):
/// only the four partition-algebra traits are closed. The enforcement.rs
/// `Certificate` trait family is a *local* sealed trait distinct from the
/// ontology-derived `cert::Certificate<H>` trait; the 17 `impl Certificate
/// for <Struct>` hits in enforcement.rs do not close any ontology trait.
const ALREADY_IMPLEMENTED: &[&str] = &[
    "https://uor.foundation/partition/Partition",
    "https://uor.foundation/partition/PartitionProduct",
    "https://uor.foundation/partition/PartitionCoproduct",
    "https://uor.foundation/partition/CartesianPartitionProduct",
];

/// Class local names deferred until theory lands (strategy doc §Path 4).
///
/// Extended by every class in `kernel/parallel` and `kernel/stream`
/// namespaces — see `classify()`.
const THEORY_DEFERRED_LOCAL_NAMES: &[&str] = &[
    // Cohomology machinery (OB_P1/P2/P3 not grounded computationally).
    "CochainComplex",
    "CohomologyGroup",
    "Sheaf",
    "RestrictionMap",
    "Section",
    "Stalk",
    "GluingObstruction",
    // Monoidal / operad (OP_3 Leibniz-rule grounding missing).
    "MonoidalProduct",
    "MonoidalComposition",
    "OperadComposition",
    // Coboundary / boundary machinery that depends on cohomology grounding.
    "Coboundary",
    "Cocycle",
];

/// Property labels whose presence marks a Path-2 witness as entropy-bearing
/// (R7). Witnesses carrying any of these cannot derive `Hash`.
const ENTROPY_PROPERTY_LABELS: &[&str] = &[
    "bits",
    "bitsDissipated",
    "landauerCost",
    "landauerNats",
    "entropy",
    "crossEntropy",
    "freeEnergy",
];

/// Substring matched against a class name to flag it as a theorem-witness
/// candidate (R7 heuristic #1).
const THEOREM_WITNESS_SUFFIXES: &[&str] = &["Witness", "Obstruction", "Verification"];

/// Explicit Path-3 allow-list, keyed by class local name. Each entry names
/// the `primitive_*` function the Phase-4 blanket impl will delegate to.
/// R13: this list is empty at Phase 0 close — Phase 4 populates it as
/// blanket impls land.
const PATH3_ALLOW_LIST: &[(&str, &str)] = &[];

// ─── Classification ──────────────────────────────────────────────────────

/// Classifies a single class.
///
/// Deterministic and pure: same `(class, ontology)` always yields the same
/// `ClassificationEntry`. Ordering: first match in the decision procedure
/// wins; see `docs/orphan-closure/phase-0-classification.md`.
#[must_use]
pub fn classify(class: &Class, ontology: &Ontology) -> ClassificationEntry {
    let class_iri: &'static str = class.id;
    let class_local: &'static str = static_local_name(class_iri);
    let namespace = namespace_prefix(class_iri, ontology).unwrap_or("");

    // 1. Skip — enum classes + Primitives
    if is_skipped_class(class_local) {
        return ClassificationEntry {
            class_iri,
            class_local,
            namespace,
            path_kind: PathKind::Skip,
            rationale: format!("{class_local} is an enum class or Primitives"),
        };
    }

    // 2. AlreadyImplemented (full-IRI match — local names collide)
    if ALREADY_IMPLEMENTED.contains(&class_iri) {
        return ClassificationEntry {
            class_iri,
            class_local,
            namespace,
            path_kind: PathKind::AlreadyImplemented,
            rationale: "hand-written impl exists in foundation/src/".to_string(),
        };
    }

    // 3. Path4TheoryDeferred (allow-list + parallel/stream namespaces)
    if THEORY_DEFERRED_LOCAL_NAMES.contains(&class_local)
        || class_iri.starts_with(NS_PARALLEL)
        || class_iri.starts_with(NS_STREAM)
    {
        let reason = if THEORY_DEFERRED_LOCAL_NAMES.contains(&class_local) {
            "theory-deferred per strategy doc §Path 4"
        } else if class_iri.starts_with(NS_PARALLEL) {
            "kernel/parallel awaits runtime-integration grounding"
        } else {
            "kernel/stream awaits reactive-semantics grounding"
        };
        return ClassificationEntry {
            class_iri,
            class_local,
            namespace,
            path_kind: PathKind::Path4TheoryDeferred,
            rationale: reason.to_string(),
        };
    }

    // 4. Path2TheoremWitness — name ends in Witness/Obstruction/Verification
    if let Some(suffix) = matching_theorem_suffix(class_local) {
        let entropy_bearing = has_entropy_property(class_iri, ontology);
        let theorem_identity = resolve_theorem_identity(class_iri, ontology).unwrap_or_default();
        return ClassificationEntry {
            class_iri,
            class_local,
            namespace,
            path_kind: PathKind::Path2TheoremWitness {
                entropy_bearing,
                theorem_identity,
            },
            rationale: format!("class name ends in '{suffix}' — theorem-witness shape"),
        };
    }

    // 5. Path3PrimitiveBacked — explicit allow-list only (R13 loud failure
    //    for mismatches is enforced by the Phase-0 tests: any allow-list
    //    entry whose primitive is absent fails classification_counts).
    if let Some((_, prim)) = PATH3_ALLOW_LIST
        .iter()
        .find(|(name, _)| *name == class_local)
    {
        return ClassificationEntry {
            class_iri,
            class_local,
            namespace,
            path_kind: PathKind::Path3PrimitiveBacked {
                primitive_name: (*prim).to_string(),
            },
            rationale: format!("primitive-backed: {prim}"),
        };
    }

    // 6. Path1HandleResolver fallthrough — verify R4 (every property's range
    //    maps to a known absent-sentinel). If a property has no absent
    //    sentinel, demote to Path4.
    if let Some(unsupported_range) = property_without_absent_sentinel(class_iri, ontology) {
        return ClassificationEntry {
            class_iri,
            class_local,
            namespace,
            path_kind: PathKind::Path4TheoryDeferred,
            rationale: format!("no-absent-semantics: {unsupported_range}"),
        };
    }

    ClassificationEntry {
        class_iri,
        class_local,
        namespace,
        path_kind: PathKind::Path1HandleResolver,
        rationale: "pure-accessor bundle (default)".to_string(),
    }
}

/// Classifies every class in the ontology.
#[must_use]
pub fn classify_all(ontology: &Ontology) -> Vec<ClassificationEntry> {
    let mut out: Vec<ClassificationEntry> = ontology
        .namespaces
        .iter()
        .flat_map(|m| m.classes.iter())
        .map(|c| classify(c, ontology))
        .collect();
    out.sort_by(|a, b| {
        a.namespace
            .cmp(b.namespace)
            .then_with(|| a.class_local.cmp(b.class_local))
    });
    out
}

/// Per-variant counts, returned for `spec/src/counts.rs` cross-check.
#[derive(Debug, Clone, Copy, Default)]
pub struct ClassificationCounts {
    /// Total `PathKind::Skip`.
    pub skip: usize,
    /// Total `PathKind::AlreadyImplemented`.
    pub already_implemented: usize,
    /// Total `PathKind::Path1HandleResolver`.
    pub path1: usize,
    /// Total `PathKind::Path2TheoremWitness`.
    pub path2: usize,
    /// Total `PathKind::Path3PrimitiveBacked`.
    pub path3: usize,
    /// Total `PathKind::Path4TheoryDeferred`.
    pub path4: usize,
}

impl ClassificationCounts {
    /// Sum of every variant.
    #[must_use]
    pub fn total(&self) -> usize {
        self.skip + self.already_implemented + self.path1 + self.path2 + self.path3 + self.path4
    }
}

/// Tallies classification counts.
#[must_use]
pub fn count(entries: &[ClassificationEntry]) -> ClassificationCounts {
    let mut c = ClassificationCounts::default();
    for e in entries {
        match e.path_kind {
            PathKind::Skip => c.skip += 1,
            PathKind::AlreadyImplemented => c.already_implemented += 1,
            PathKind::Path1HandleResolver => c.path1 += 1,
            PathKind::Path2TheoremWitness { .. } => c.path2 += 1,
            PathKind::Path3PrimitiveBacked { .. } => c.path3 += 1,
            PathKind::Path4TheoryDeferred => c.path4 += 1,
        }
    }
    c
}

// ─── Report emission ─────────────────────────────────────────────────────

/// Writes the human-readable classification report to `out_path`.
///
/// Format: Markdown table, one row per class, sorted by namespace then class
/// name. Regenerated on every `cargo run --bin uor-crate` and gated by `git
/// diff --exit-code docs/orphan-closure/classification_report.md`.
///
/// # Errors
///
/// Returns an error if the file cannot be written.
pub fn write_report(entries: &[ClassificationEntry], out_path: &Path) -> Result<()> {
    let counts = count(entries);
    let mut s = String::with_capacity(4096 + entries.len() * 256);

    s.push_str("<!-- @generated by uor-crate from uor-codegen::classification — do not edit manually -->\n\n");
    s.push_str("# Orphan-trait classification report\n\n");
    s.push_str(
        "Generated by `cargo run --bin uor-crate`. See \
         [phase-0-classification.md](./phase-0-classification.md) for the \
         decision procedure.\n\n",
    );

    s.push_str("## Totals\n\n");
    s.push_str("| PathKind | Count |\n|---|---|\n");
    let _ = writeln!(s, "| Skip | {} |", counts.skip);
    let _ = writeln!(s, "| AlreadyImplemented | {} |", counts.already_implemented);
    let _ = writeln!(s, "| Path1HandleResolver | {} |", counts.path1);
    let _ = writeln!(s, "| Path2TheoremWitness | {} |", counts.path2);
    let _ = writeln!(s, "| Path3PrimitiveBacked | {} |", counts.path3);
    let _ = writeln!(s, "| Path4TheoryDeferred | {} |", counts.path4);
    let _ = writeln!(s, "| **Total** | **{}** |", counts.total());
    s.push('\n');

    s.push_str("## Per-class\n\n");
    s.push_str(
        "| Namespace | Class | PathKind | Entropy | Theorem identity | Primitive | Rationale |\n\
         |---|---|---|---|---|---|---|\n",
    );
    for e in entries {
        let (entropy, theorem, primitive) = match &e.path_kind {
            PathKind::Path2TheoremWitness {
                entropy_bearing,
                theorem_identity,
            } => (
                if *entropy_bearing { "yes" } else { "no" },
                theorem_identity.as_str(),
                "",
            ),
            PathKind::Path3PrimitiveBacked { primitive_name } => ("", "", primitive_name.as_str()),
            _ => ("", "", ""),
        };
        let _ = writeln!(
            s,
            "| `{}` | `{}` | {} | {} | {} | {} | {} |",
            e.namespace,
            e.class_local,
            e.path_kind.label(),
            entropy,
            theorem,
            primitive,
            e.rationale.replace('|', r"\|"),
        );
    }

    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }
    std::fs::write(out_path, s)
        .with_context(|| format!("Failed to write report: {}", out_path.display()))?;
    Ok(())
}

// ─── Internal helpers ────────────────────────────────────────────────────

fn is_skipped_class(class_local: &str) -> bool {
    if class_local == "Primitives" {
        return true;
    }
    Ontology::enum_class_names().contains(&class_local)
}

fn matching_theorem_suffix(class_local: &str) -> Option<&'static str> {
    THEOREM_WITNESS_SUFFIXES
        .iter()
        .copied()
        .find(|suffix| class_local.ends_with(*suffix))
}

fn has_entropy_property(class_iri: &str, ontology: &Ontology) -> bool {
    properties_with_domain(class_iri, ontology).any(|p| {
        // Decimal range OR property label is in the entropy set.
        p.range == XSD_DECIMAL || ENTROPY_PROPERTY_LABELS.contains(&p.label)
    })
}

fn resolve_theorem_identity(class_iri: &str, ontology: &Ontology) -> Option<String> {
    // Phase 0: a class's `op:Identity` linkage is not yet asserted in the
    // ontology as a direct back-reference. The mapping is populated via
    // R6's lookup in Phase 3 when the witness scaffolds need it. For
    // Phase 0 we leave it blank and let Phase 3 resolve. Return a hint
    // when the class name matches an `op:Identity` individual directly.
    let class_local = local_name(class_iri);
    ontology
        .namespaces
        .iter()
        .flat_map(|m| m.individuals.iter())
        .find(|ind| {
            ind.type_ == "https://uor.foundation/op/Identity" && local_name(ind.id) == class_local
        })
        .map(|ind| ind.id.to_string())
}

fn property_without_absent_sentinel(class_iri: &str, ontology: &Ontology) -> Option<String> {
    for p in properties_with_domain(class_iri, ontology) {
        if !range_has_absent_sentinel(p.range, ontology) {
            return Some(format!("{} (property {})", p.range, p.label));
        }
    }
    None
}

fn range_has_absent_sentinel(range_iri: &str, ontology: &Ontology) -> bool {
    // Known XSD primitive types — all have absent sentinels per R4.
    match range_iri {
        XSD_STRING
        | XSD_INTEGER
        | XSD_NON_NEGATIVE_INTEGER
        | XSD_POSITIVE_INTEGER
        | XSD_BOOLEAN
        | XSD_DECIMAL
        | XSD_HEX_BINARY => return true,
        _ => {}
    }
    // `xsd:dateTime` maps to `H::WitnessBytes` per `mapping::xsd_to_primitives_type`.
    if range_iri == "http://www.w3.org/2001/XMLSchema#dateTime" {
        return true;
    }
    // Generic object ranges (`owl:Thing`, `owl:Class`, `rdf:List`) are mapped
    // by `codegen/src/traits.rs` to `&H::HostString` or `count/_at` forms;
    // both have absent sentinels (`EMPTY_HOST_STRING`).
    if range_iri == OWL_THING || range_iri == OWL_CLASS || range_iri == RDF_LIST {
        return true;
    }
    // Ontology classes — handle-typed fields, with `ContentFingerprint::zero()`
    // sentinel per R4. Accept any class declared in the ontology.
    if ontology.find_class(range_iri).is_some() {
        return true;
    }
    false
}

fn properties_with_domain<'a>(
    class_iri: &'a str,
    ontology: &'a Ontology,
) -> impl Iterator<Item = &'a Property> + 'a {
    ontology
        .namespaces
        .iter()
        .flat_map(|m| m.properties.iter())
        .filter(move |p| p.kind != PropertyKind::Annotation && p.domain == Some(class_iri))
}

fn namespace_prefix(class_iri: &str, ontology: &Ontology) -> Option<&'static str> {
    ontology
        .namespaces
        .iter()
        .find(|m| class_iri.starts_with(m.namespace.iri))
        .map(|m| m.namespace.prefix)
}

fn static_local_name(iri: &'static str) -> &'static str {
    // `local_name` returns `&str` tied to its input; since input is 'static,
    // the return is 'static too — but the compiler doesn't prove it through
    // rsplit. We reconstruct with the borrow at the 'static boundary.
    if let Some(pos) = iri.rfind('/') {
        return &iri[pos + 1..];
    }
    if let Some(pos) = iri.rfind('#') {
        return &iri[pos + 1..];
    }
    iri
}

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn skip_detects_enum_classes() {
        let ontology = Ontology::full();
        let witt = match ontology.find_class_by_local_name("WittLevel") {
            Some(c) => c,
            None => panic!("WittLevel class must exist"),
        };
        let e = classify(witt, ontology);
        assert!(matches!(e.path_kind, PathKind::Skip));
    }

    #[test]
    fn path1_default_for_plain_class() {
        // `carry/CarryChain` is guaranteed NOT in the AlreadyImplemented or
        // Path-4 allow-lists, and its name doesn't match Path-2's suffix
        // heuristic. A plain accessor bundle.
        let ontology = Ontology::full();
        let class = match ontology.find_class("https://uor.foundation/carry/CarryChain") {
            Some(c) => c,
            None => panic!("carry/CarryChain class must exist"),
        };
        let e = classify(class, ontology);
        assert_eq!(e.path_kind.label(), "Path1HandleResolver");
    }

    #[test]
    fn already_implemented_for_partition_product() {
        let ontology = Ontology::full();
        let pp = match ontology.find_class("https://uor.foundation/partition/PartitionProduct") {
            Some(c) => c,
            None => panic!("PartitionProduct class must exist"),
        };
        let e = classify(pp, ontology);
        assert!(matches!(e.path_kind, PathKind::AlreadyImplemented));
    }

    #[test]
    fn counts_sum_to_class_count() {
        let ontology = Ontology::full();
        let entries = classify_all(ontology);
        let counts = count(&entries);
        assert_eq!(counts.total(), ontology.class_count());
    }

    #[test]
    fn classification_is_deterministic() {
        let ontology = Ontology::full();
        let a = classify_all(ontology);
        let b = classify_all(ontology);
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(b.iter()) {
            assert_eq!(x.class_iri, y.class_iri);
            assert_eq!(x.path_kind.label(), y.path_kind.label());
        }
    }
}
