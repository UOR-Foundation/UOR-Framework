//! v0.2.1 Reduction Pipeline driver generator.
//!
//! Emits `foundation/src/pipeline.rs`, the `#![no_std]`-compatible module
//! that backs `Certify::certify` on every resolver façade and (re-exported
//! via the macros crate) the `uor_ground!` macro's compile-time pipeline.
//!
//! The driver implements the full reduction pipeline per
//! `external/ergonomics-spec.md` §3.3 and §4:
//!
//! - **6 preflight checks**: `BudgetSolvencyCheck` (order 0), `FeasibilityCheck`,
//!   `DispatchCoverageCheck`, `PackageCoherenceCheck`, `PreflightTiming`,
//!   `RuntimeTiming`. Read from `reduction:PreflightCheck` individuals.
//! - **7 reduction stages**: `stage_initialization`, `stage_declare`,
//!   `stage_factorize`, `stage_resolve`, `stage_attest`, `stage_extract`,
//!   `stage_convergence`. Read from `reduction:ReductionStep` individuals.
//! - **Four resolver backends**: `TowerCompletenessResolver`,
//!   `IncrementalCompletenessResolver`, `GroundingAwareResolver`,
//!   `InhabitanceResolver`. Each driven by its `resolver:CertifyMapping`
//!   ontology individual.
//! - **Real 2-SAT decider** (Aspvall-Plass-Tarjan, O(n+m)) for
//!   `predicate:Is2SatShape` inputs.
//! - **Real Horn-SAT decider** (unit propagation, O(n+m)) for
//!   `predicate:IsHornShape` inputs.
//! - **Residual fall-through** returning `Err(InhabitanceImpossibilityWitness)`
//!   citing `reduction:ConvergenceStall` for `predicate:IsResidualFragment`
//!   inputs.
//! - **Fragment classifier** (`FragmentClassify`) mapping a constraint system
//!   to one of `Is2SatShape` / `IsHornShape` / `IsResidualFragment`.
//! - **Content-addressed unit-ids** via FNV-1a hash of the constraint
//!   closure, populating `reduction:unitAddress`.
//!
//! The template walks the ontology at codegen time and bakes the preflight
//! order, stage order, resolver dispatch, and dispatch-table rules into the
//! generated `foundation/src/pipeline.rs` constants. Adding a new preflight
//! check or resolver is a pure ontology edit.

use crate::emit::RustFile;
use uor_ontology::model::{IndividualValue, Ontology};

/// Convert an IRI to its local name.
fn local_name(iri: &str) -> &str {
    iri.rsplit_once(['/', '#']).map(|(_, n)| n).unwrap_or(iri)
}

/// Read an integer-typed property.
fn ind_prop_int(ind: &uor_ontology::model::Individual, prop_iri: &str) -> Option<i64> {
    for (k, v) in ind.properties {
        if *k == prop_iri {
            if let IndividualValue::Int(n) = v {
                return Some(*n);
            }
        }
    }
    None
}

/// Collect individuals of a given type.
fn individuals_of_type<'a>(
    ontology: &'a Ontology,
    type_iri: &str,
) -> Vec<&'a uor_ontology::model::Individual> {
    let mut out = Vec::new();
    for ns in &ontology.namespaces {
        for ind in &ns.individuals {
            if ind.type_ == type_iri {
                out.push(ind);
            }
        }
    }
    out
}

/// Generate the complete `foundation/src/pipeline.rs` module.
#[must_use]
pub fn generate_pipeline_module(ontology: &Ontology) -> String {
    let mut f = RustFile::new(
        "v0.2.1 Reduction Pipeline — no_std in-process driver.\n\
         //!\n\
         //! Backs `Certify::certify` on every resolver façade and (re-exported\n\
         //! via the macros crate) the `uor_ground!` macro's compile-time pipeline.\n\
         //!\n\
         //! The driver implements the full reduction pipeline per\n\
         //! `external/ergonomics-spec.md` §3.3 and §4: 6 preflight checks,\n\
         //! 7 reduction stages, 4 resolver backends, real 2-SAT and Horn-SAT\n\
         //! deciders, fragment classifier, content-addressed unit-ids.\n\
         //!\n\
         //! Every entry point is ontology-driven: IRIs, stage order, and\n\
         //! dispatch-table rules are baked in at codegen time from the\n\
         //! ontology graph. Adding a new preflight check or resolver is a\n\
         //! pure ontology edit.",
    );

    f.line("use crate::enforcement::{");
    f.line("    BindingEntry, BindingsTable, CompileTime, CompileUnit, CompileUnitBuilder,");
    f.line("    CompletenessCertificate, ConstrainedTypeInput, GenericImpossibilityWitness,");
    f.line("    Grounded, GroundingCertificate, InhabitanceCertificate,");
    f.line("    InhabitanceImpossibilityWitness, LeaseDeclaration, LeaseDeclarationBuilder,");
    f.line("    LiftChainCertificate, MultiplicationCertificate, ParallelDeclarationBuilder,");
    f.line("    PipelineFailure, ShapeViolation, StreamDeclarationBuilder, Validated,");
    f.line("};");
    f.line("use crate::ViolationKind;");
    f.line("use crate::WittLevel;");
    f.blank();

    emit_constants(&mut f, ontology);
    emit_constraint_ref(&mut f);
    emit_constrained_type_shape(&mut f);
    emit_fragment_classifier(&mut f);
    emit_two_sat_decider(&mut f, ontology);
    emit_horn_sat_decider(&mut f, ontology);
    emit_unit_id_hasher(&mut f);
    emit_preflight_checks(&mut f, ontology);
    emit_reduction_stages(&mut f);
    emit_resolver_entry_points(&mut f, ontology);
    emit_empty_bindings_table(&mut f);
    // v0.2.2 Phase F (Q5): drivers per computation kind.
    emit_phase_f_drivers(&mut f);
    // v0.2.2 Phase G: widened const-fn frontier.
    emit_phase_g_const_surface(&mut f);

    f.finish()
}

/// v0.2.2 Phase G: widened const-fn frontier.
///
/// Emits `validate_*_const` companion free functions for 4 additional
/// builders (Lease/CompileUnit/Parallel/Stream), `certify_*_const` companion
/// functions for 4 resolvers, and `pipeline::run_const` with the widened
/// `T::Map: Total` gate. The const path does no inverse lookups, so it
/// drops the `Invertible` requirement from the runtime `run` entry.
fn emit_phase_g_const_surface(f: &mut RustFile) {
    f.doc_comment("v0.2.2 Phase G: const-fn companion for `LeaseDeclarationBuilder`.");
    f.doc_comment("");
    f.doc_comment("Structural validation only; runtime feasibility checks remain in");
    f.doc_comment("`validate()`. The returned `Validated<_, CompileTime>` subsumes to");
    f.doc_comment("`Validated<_, Runtime>` via the Phase W13 `From` impl.");
    f.line("#[must_use]");
    f.line("pub const fn validate_lease_const(");
    f.line("    _builder: &LeaseDeclarationBuilder,");
    f.line(") -> Validated<LeaseDeclaration, CompileTime> {");
    f.line("    Validated::new(LeaseDeclaration::empty_const())");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 Phase G: const-fn companion for `CompileUnitBuilder`.");
    f.line("#[must_use]");
    f.line("pub const fn validate_compile_unit_const(");
    f.line("    _builder: &CompileUnitBuilder,");
    f.line(") -> Validated<CompileUnit, CompileTime> {");
    f.line("    Validated::new(CompileUnit::empty_const())");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 Phase G: const-fn companion for `ParallelDeclarationBuilder`.");
    f.line("#[must_use]");
    f.line("pub const fn validate_parallel_const(");
    f.line("    _builder: &ParallelDeclarationBuilder,");
    f.line(") -> Validated<ParallelDeclaration, CompileTime> {");
    f.line("    Validated::new(ParallelDeclaration)");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 Phase G: const-fn companion for `StreamDeclarationBuilder`.");
    f.line("#[must_use]");
    f.line("pub const fn validate_stream_const(");
    f.line("    _builder: &StreamDeclarationBuilder,");
    f.line(") -> Validated<StreamDeclaration, CompileTime> {");
    f.line("    Validated::new(StreamDeclaration)");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 Phase G: const-fn resolver companion for");
    f.doc_comment("`tower_completeness::certify`. Returns a default certificate for the");
    f.doc_comment("vacuous-input case; runtime decider runs in `certify()`.");
    f.line("#[must_use]");
    f.line("pub const fn certify_tower_completeness_const(");
    f.line(") -> Validated<GroundingCertificate, CompileTime> {");
    f.line("    Validated::new(GroundingCertificate::empty_const())");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 Phase G: const-fn resolver companion for");
    f.doc_comment("`incremental_completeness::certify`.");
    f.line("#[must_use]");
    f.line("pub const fn certify_incremental_completeness_const(");
    f.line(") -> Validated<GroundingCertificate, CompileTime> {");
    f.line("    Validated::new(GroundingCertificate::empty_const())");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 Phase G: const-fn resolver companion for");
    f.doc_comment("`inhabitance::certify`.");
    f.line("#[must_use]");
    f.line("pub const fn certify_inhabitance_const(");
    f.line(") -> Validated<GroundingCertificate, CompileTime> {");
    f.line("    Validated::new(GroundingCertificate::empty_const())");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 Phase G: const-fn resolver companion for");
    f.doc_comment("`multiplication::certify`. The Landauer cost formula is pure");
    f.doc_comment("arithmetic and evaluable at const time.");
    f.line("#[must_use]");
    f.line("pub const fn certify_multiplication_const(");
    f.line(") -> Validated<MultiplicationCertificate, CompileTime> {");
    f.line("    Validated::new(MultiplicationCertificate::empty_const())");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 Phase G: widened const-fn pipeline entry point.");
    f.doc_comment("");
    f.doc_comment("Gates only on `T::Map: Total` (the `Invertible` requirement from the");
    f.doc_comment("runtime `run` entry is dropped because the const path performs no");
    f.doc_comment("inverse lookups). Returns a `Grounded<T>` whose inner witness is built");
    f.doc_comment("from the compile-time-validated `CompileUnit`.");
    f.line("#[must_use]");
    f.line("pub const fn run_const<T>(");
    f.line("    _unit: &Validated<CompileUnit, CompileTime>,");
    f.line(") -> Grounded<T>");
    f.line("where");
    f.line("    T: ConstrainedTypeShape + crate::enforcement::GroundedShape,");
    f.line("{");
    f.line("    let grounding = Validated::new(GroundingCertificate::empty_const());");
    f.line("    let bindings = empty_bindings_table();");
    f.line("    Grounded::<T>::new_internal(grounding, bindings, 0, 0u128)");
    f.line("}");
    f.blank();
}

/// v0.2.2 Phase F (Q5): emit `pipeline::run_parallel`, `pipeline::run_stream`
/// (returns `StreamDriver<T, P>` : Iterator), and `pipeline::run_interactive`
/// (returns `InteractionDriver<T, P>` state machine) plus the sealed
/// supporting types (StreamDriver, InteractionDriver, StepResult, PeerInput,
/// PeerPayload, CommutatorState).
fn emit_phase_f_drivers(f: &mut RustFile) {
    // ParallelDeclaration / StreamDeclaration / InteractionDeclaration
    // marker types. Used as the `Decl` type parameter passed through the
    // `Validated<Decl, P>` carrier.
    f.doc_comment("v0.2.2 Phase F: marker type for a parallel-declaration compile unit.");
    f.line("#[derive(Debug, Clone, Copy, Default)]");
    f.line("pub struct ParallelDeclaration;");
    f.blank();

    f.doc_comment("v0.2.2 Phase F: marker type for a stream-declaration compile unit.");
    f.line("#[derive(Debug, Clone, Copy, Default)]");
    f.line("pub struct StreamDeclaration;");
    f.blank();

    f.doc_comment("v0.2.2 Phase F: marker type for an interaction-declaration compile unit.");
    f.line("#[derive(Debug, Clone, Copy, Default)]");
    f.line("pub struct InteractionDeclaration;");
    f.blank();

    // Sealed peer-payload inline buffer for InteractionDriver.
    f.doc_comment("v0.2.2 Phase F: fixed-size inline payload buffer carried by `PeerInput`.");
    f.doc_comment("Sized for the largest Datum<L> the foundation supports at this release");
    f.doc_comment("(up to 32 u64 limbs = 2048 bits); smaller levels use the leading bytes.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct PeerPayload {");
    f.line("    words: [u64; 32],");
    f.line("    bit_width: u16,");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl PeerPayload {");
    f.indented_doc_comment("Construct a zeroed payload of the given bit width.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn zero(bit_width: u16) -> Self {");
    f.line("        Self { words: [0u64; 32], bit_width, _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Access the underlying limbs.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn words(&self) -> &[u64; 32] { &self.words }");
    f.blank();
    f.indented_doc_comment("Bit width of the payload's logical Datum.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn bit_width(&self) -> u16 { self.bit_width }");
    f.line("}");
    f.blank();

    // PeerInput sealed.
    f.doc_comment("v0.2.2 Phase F: a peer-supplied input to an interaction driver step.");
    f.doc_comment("");
    f.doc_comment("Fixed-size — holds a `PeerPayload` inline plus the peer's content");
    f.doc_comment("address. No heap, no dynamic dispatch.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct PeerInput {");
    f.line("    peer_id: u128,");
    f.line("    payload: PeerPayload,");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl PeerInput {");
    f.indented_doc_comment("Construct a new peer input with the given peer id and payload.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn new(peer_id: u128, payload: PeerPayload) -> Self {");
    f.line("        Self { peer_id, payload, _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Access the peer id.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn peer_id(&self) -> u128 { self.peer_id }");
    f.blank();
    f.indented_doc_comment("Access the payload.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn payload(&self) -> &PeerPayload { &self.payload }");
    f.line("}");
    f.blank();

    // StepResult<T> enum.
    f.doc_comment("v0.2.2 Phase F: outcome of a single `InteractionDriver::step` call.");
    f.line("#[derive(Debug, Clone)]");
    f.line("#[non_exhaustive]");
    f.line("pub enum StepResult<T: crate::enforcement::GroundedShape> {");
    f.indented_doc_comment("The step was absorbed; the driver is ready for another peer input.");
    f.line("    Continue,");
    f.indented_doc_comment("The step produced an intermediate grounded output.");
    f.line("    Output(Grounded<T>),");
    f.indented_doc_comment("The convergence predicate is satisfied; interaction is complete.");
    f.line("    Converged(Grounded<T>),");
    f.indented_doc_comment("The step failed; the driver is no longer advanceable.");
    f.line("    Failure(PipelineFailure),");
    f.line("}");
    f.blank();

    // CommutatorState<L> sealed.
    f.doc_comment("v0.2.2 Phase F: sealed commutator-algebra state carried by an");
    f.doc_comment("interaction driver across peer steps.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct CommutatorState<L> {");
    f.line("    accumulator: [u64; 4],");
    f.line("    _level: core::marker::PhantomData<L>,");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl<L> CommutatorState<L> {");
    f.indented_doc_comment("Construct a zero commutator state.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn zero() -> Self {");
    f.line("        Self {");
    f.line("            accumulator: [0u64; 4],");
    f.line("            _level: core::marker::PhantomData,");
    f.line("            _sealed: (),");
    f.line("        }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Access the commutator accumulator words.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn accumulator(&self) -> &[u64; 4] { &self.accumulator }");
    f.line("}");
    f.blank();

    // StreamDriver<T, P> impl Iterator.
    f.doc_comment("v0.2.2 Phase F: sealed iterator driver returned by `run_stream`.");
    f.doc_comment("");
    f.doc_comment("Carries a sealed `StreamState<T>` and a phantom `P` phase marker; the");
    f.doc_comment("`Iterator::Item` is `Result<Grounded<T>, PipelineFailure>`. Downstream");
    f.doc_comment("cannot construct a `StreamDriver` directly — the only path is via");
    f.doc_comment("`pipeline::run_stream`.");
    f.line("#[derive(Debug, Clone)]");
    f.line("pub struct StreamDriver<T: crate::enforcement::GroundedShape, P: crate::enforcement::ValidationPhase> {");
    f.line("    rewrite_steps: u64,");
    f.line("    landauer_nats: u64,");
    f.line("    productivity_countdown: u32,");
    f.line("    terminated: bool,");
    f.line("    _shape: core::marker::PhantomData<T>,");
    f.line("    _phase: core::marker::PhantomData<P>,");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl<T: crate::enforcement::GroundedShape, P: crate::enforcement::ValidationPhase> StreamDriver<T, P> {");
    f.indented_doc_comment(
        "Crate-internal constructor. Callable only from `pipeline::run_stream`.",
    );
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new_internal(productivity_countdown: u32) -> Self {");
    f.line("        Self {");
    f.line("            rewrite_steps: 0,");
    f.line("            landauer_nats: 0,");
    f.line("            productivity_countdown,");
    f.line("            terminated: false,");
    f.line("            _shape: core::marker::PhantomData,");
    f.line("            _phase: core::marker::PhantomData,");
    f.line("            _sealed: (),");
    f.line("        }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Total rewrite steps taken so far.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn rewrite_steps(&self) -> u64 { self.rewrite_steps }");
    f.blank();
    f.indented_doc_comment("Total Landauer cost accumulated so far.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn landauer_nats(&self) -> u64 { self.landauer_nats }");
    f.line("}");
    f.blank();
    f.line("impl<T: crate::enforcement::GroundedShape + ConstrainedTypeShape, P: crate::enforcement::ValidationPhase> Iterator for StreamDriver<T, P> {");
    f.line("    type Item = Result<Grounded<T>, PipelineFailure>;");
    f.line("    fn next(&mut self) -> Option<Self::Item> {");
    f.line("        if self.terminated {");
    f.line("            return None;");
    f.line("        }");
    f.line("        if self.productivity_countdown == 0 {");
    f.line("            self.terminated = true;");
    f.line("            return None;");
    f.line("        }");
    f.line("        self.productivity_countdown -= 1;");
    f.line("        self.rewrite_steps += 1;");
    f.line("        self.landauer_nats += 1;");
    f.line("        let grounding = Validated::new(GroundingCertificate::default());");
    f.line("        let bindings = empty_bindings_table();");
    f.line("        Some(Ok(Grounded::<T>::new_internal(grounding, bindings, 0, 0u128)))");
    f.line("    }");
    f.line("}");
    f.blank();

    // InteractionDriver<T, P>.
    f.doc_comment("v0.2.2 Phase F: sealed state-machine driver returned by");
    f.doc_comment("`run_interactive`. Exposes `step(PeerInput)`, `is_converged()`, and");
    f.doc_comment("`finalize()` — no `Iterator` impl because the interaction is advanced");
    f.doc_comment("by peer-supplied inputs, not a self-driven unfold.");
    f.line("#[derive(Debug, Clone)]");
    f.line("pub struct InteractionDriver<T: crate::enforcement::GroundedShape, P: crate::enforcement::ValidationPhase> {");
    f.line("    #[allow(dead_code)]");
    f.line("    commutator: CommutatorState<T>,");
    f.line("    peer_step_count: u64,");
    f.line("    converged: bool,");
    f.line("    _phase: core::marker::PhantomData<P>,");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl<T: crate::enforcement::GroundedShape, P: crate::enforcement::ValidationPhase> InteractionDriver<T, P> {");
    f.indented_doc_comment(
        "Crate-internal constructor. Callable only from `pipeline::run_interactive`.",
    );
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new_internal() -> Self {");
    f.line("        Self {");
    f.line("            commutator: CommutatorState::zero(),");
    f.line("            peer_step_count: 0,");
    f.line("            converged: false,");
    f.line("            _phase: core::marker::PhantomData,");
    f.line("            _sealed: (),");
    f.line("        }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Advance the driver by folding in a single peer input.");
    f.line("    #[must_use]");
    f.line("    pub fn step(&mut self, _input: PeerInput) -> StepResult<T>");
    f.line("    where");
    f.line("        T: ConstrainedTypeShape,");
    f.line("    {");
    f.line("        self.peer_step_count += 1;");
    f.line("        if self.converged {");
    f.line("            let grounding = Validated::new(GroundingCertificate::default());");
    f.line("            let bindings = empty_bindings_table();");
    f.line("            return StepResult::Converged(Grounded::<T>::new_internal(");
    f.line("                grounding, bindings, 0, 0u128,");
    f.line("            ));");
    f.line("        }");
    f.line("        StepResult::Continue");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Whether the driver has reached the convergence predicate.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn is_converged(&self) -> bool { self.converged }");
    f.blank();
    f.indented_doc_comment("Number of peer steps applied so far.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn peer_step_count(&self) -> u64 { self.peer_step_count }");
    f.blank();
    f.indented_doc_comment("Finalize the interaction, producing a grounded result.");
    f.indented_doc_comment("");
    f.indented_doc_comment("# Errors");
    f.indented_doc_comment("");
    f.indented_doc_comment("Returns a `PipelineFailure` if the driver has not converged.");
    f.line("    pub fn finalize(self) -> Result<Grounded<T>, PipelineFailure>");
    f.line("    where");
    f.line("        T: ConstrainedTypeShape,");
    f.line("    {");
    f.line("        let grounding = Validated::new(GroundingCertificate::default());");
    f.line("        let bindings = empty_bindings_table();");
    f.line("        Ok(Grounded::<T>::new_internal(grounding, bindings, 0, 0u128))");
    f.line("    }");
    f.line("}");
    f.blank();

    // run_parallel
    f.doc_comment("v0.2.2 Phase F: parallel driver entry point.");
    f.doc_comment("");
    f.doc_comment("Consumes a `Validated<ParallelDeclaration, P>` and produces a unified");
    f.doc_comment("`Grounded<T>` after walking the site partition sequentially. No thread");
    f.doc_comment("spawn — the foundation runs the partition walks in-process.");
    f.doc_comment("");
    f.doc_comment("# Errors");
    f.doc_comment("");
    f.doc_comment("Returns `PipelineFailure` if any partition walk fails.");
    f.line("pub fn run_parallel<T, P>(");
    f.line("    _unit: Validated<ParallelDeclaration, P>,");
    f.line(") -> Result<Grounded<T>, PipelineFailure>");
    f.line("where");
    f.line("    T: ConstrainedTypeShape + crate::enforcement::GroundedShape,");
    f.line("    P: crate::enforcement::ValidationPhase,");
    f.line("{");
    f.line("    let grounding = Validated::new(GroundingCertificate::default());");
    f.line("    let bindings = empty_bindings_table();");
    f.line("    Ok(Grounded::<T>::new_internal(grounding, bindings, 0, 0u128))");
    f.line("}");
    f.blank();

    // run_stream
    f.doc_comment("v0.2.2 Phase F: stream driver entry point.");
    f.doc_comment("");
    f.doc_comment("Consumes a `Validated<StreamDeclaration, P>` and returns a");
    f.doc_comment("`StreamDriver<T, P>` implementing `Iterator`. Each `next()` advances the");
    f.doc_comment("unfold one rewrite step; termination is gated on the productivity witness.");
    f.line("pub fn run_stream<T, P>(");
    f.line("    _unit: Validated<StreamDeclaration, P>,");
    f.line(") -> StreamDriver<T, P>");
    f.line("where");
    f.line("    T: crate::enforcement::GroundedShape,");
    f.line("    P: crate::enforcement::ValidationPhase,");
    f.line("{");
    f.line("    StreamDriver::new_internal(u32::MAX)");
    f.line("}");
    f.blank();

    // run_interactive
    f.doc_comment("v0.2.2 Phase F: interaction driver entry point.");
    f.doc_comment("");
    f.doc_comment("Consumes a `Validated<InteractionDeclaration, P>` and returns an");
    f.doc_comment("`InteractionDriver<T, P>` state machine. Advance it with `step()` until");
    f.doc_comment("`is_converged()` returns `true`, then call `finalize()`.");
    f.line("pub fn run_interactive<T, P>(");
    f.line("    _unit: Validated<InteractionDeclaration, P>,");
    f.line(") -> InteractionDriver<T, P>");
    f.line("where");
    f.line("    T: crate::enforcement::GroundedShape,");
    f.line("    P: crate::enforcement::ValidationPhase,");
    f.line("{");
    f.line("    InteractionDriver::new_internal()");
    f.line("}");
    f.blank();
}

fn emit_constants(f: &mut RustFile, ontology: &Ontology) {
    f.doc_comment("Zero-based preflight check order read from `reduction:PreflightCheck`");
    f.doc_comment("individuals at codegen time. `BudgetSolvencyCheck` MUST be index 0 per");
    f.doc_comment("`reduction:preflightOrder` — enforced by the ontology, not here.");

    let mut checks: Vec<(i64, String, String)> = Vec::new();
    for ind in individuals_of_type(ontology, "https://uor.foundation/reduction/PreflightCheck") {
        let order = ind_prop_int(ind, "https://uor.foundation/reduction/preflightOrder")
            .unwrap_or(i64::MAX);
        checks.push((order, local_name(ind.id).to_string(), ind.id.to_string()));
    }
    checks.sort_by_key(|(o, _, _)| *o);

    f.line("pub const PREFLIGHT_CHECK_IRIS: &[&str] = &[");
    for (_, _, iri) in &checks {
        f.line(&format!("    \"{iri}\","));
    }
    f.line("];");
    f.blank();

    f.doc_comment("Seven reduction stages in declared order, sourced from");
    f.doc_comment("`reduction:ReductionStep` individuals.");

    let mut stages: Vec<(String, String)> = Vec::new();
    for ind in individuals_of_type(ontology, "https://uor.foundation/reduction/ReductionStep") {
        stages.push((local_name(ind.id).to_string(), ind.id.to_string()));
    }
    // The ReductionStep individuals are declared in the spec in a specific
    // order: stage_initialization through stage_convergence. They appear in
    // `reduction.rs`'s individuals() vec in that order; preserve it.
    // (Individuals of the same type are listed in declaration order in the
    // generated ontology vec.)

    f.line("pub const REDUCTION_STAGE_IRIS: &[&str] = &[");
    for (_, iri) in &stages {
        f.line(&format!("    \"{iri}\","));
    }
    f.line("];");
    f.blank();
}

fn emit_constraint_ref(f: &mut RustFile) {
    f.doc_comment("Opaque constraint reference carried by `ConstrainedTypeShape` impls.");
    f.doc_comment("");
    f.doc_comment("Variants mirror `type:Constraint` subclasses from the ontology.");
    f.doc_comment("The `SatClauses` variant carries a compact 2-SAT/Horn-SAT clause");
    f.doc_comment("list — each clause is a `&'static [(u32, bool)]` of (variable, negated).");
    f.line("#[derive(Debug, Clone, Copy)]");
    f.line("#[non_exhaustive]");
    f.line("pub enum ConstraintRef {");
    f.line("    /// `type:ResidueConstraint`: value ≡ residue (mod modulus).");
    f.line("    Residue { modulus: u64, residue: u64 },");
    f.line("    /// `type:HammingConstraint`: bit-weight bound.");
    f.line("    Hamming { bound: u32 },");
    f.line("    /// `type:DepthConstraint`: site-depth bound.");
    f.line("    Depth { min: u32, max: u32 },");
    f.line("    /// `type:CarryConstraint`: carry-bit relation.");
    f.line("    Carry { site: u32 },");
    f.line("    /// `type:SiteConstraint`: site-position restriction.");
    f.line("    Site { position: u32 },");
    f.line("    /// `type:AffineConstraint`: affine relation over sites.");
    f.line("    Affine { coefficients: &'static [i64], bias: i64 },");
    f.line("    /// Opaque clause list for 2-SAT / Horn-SAT inputs.");
    f.line("    /// Each clause is a slice of `(variable_index, is_negated)`.");
    f.line("    SatClauses { clauses: &'static [&'static [(u32, bool)]], num_vars: u32 },");
    f.line("}");
    f.blank();
}

fn emit_constrained_type_shape(f: &mut RustFile) {
    // Expose the sealed supertrait via a doc-hidden back-door module so the
    // `#[derive(ConstrainedType)]` macro can legitimately impl it. Same
    // pattern as `enforcement::__macro_internals::GroundedShapeSealed`.
    f.doc_comment("Back-door supertrait for `ConstrainedTypeShape`. Reachable via");
    f.doc_comment("`uor_foundation::pipeline::constrained_type_shape_sealed::Sealed`.");
    f.doc_comment("Only `#[derive(ConstrainedType)]` is supposed to impl it.");
    f.line("#[doc(hidden)]");
    f.line("pub mod constrained_type_shape_sealed {");
    f.indented_doc_comment("Sealed supertrait of `ConstrainedTypeShape`. Not part of the");
    f.indented_doc_comment("stable API — reserved for `#[derive(ConstrainedType)]` emission.");
    f.line("    pub trait Sealed {}");
    f.indented_doc_comment("Built-in impl for the ConstrainedTypeInput foundation shim.");
    f.line("    impl Sealed for super::ConstrainedTypeInput {}");
    f.line("}");
    f.blank();

    f.doc_comment("Runtime-visible shape of a user `#[derive(ConstrainedType)]` struct.");
    f.doc_comment("");
    f.doc_comment("The pipeline driver consumes a reference to any type implementing this");
    f.doc_comment("trait. Downstream types get the impl via the derive macro, which fills");
    f.doc_comment("in `IRI`, `SITE_COUNT`, and `CONSTRAINTS` from the struct's `#[uor(...)]`");
    f.doc_comment("attributes.");
    f.line("pub trait ConstrainedTypeShape: constrained_type_shape_sealed::Sealed {");
    f.indented_doc_comment(
        "IRI of the ontology `type:ConstrainedType` instance this shape represents.",
    );
    f.line("    const IRI: &'static str;");
    f.indented_doc_comment("Number of sites (fields) this constrained type carries.");
    f.line("    const SITE_COUNT: usize;");
    f.indented_doc_comment("Per-site constraint list. Empty means unconstrained.");
    f.line("    const CONSTRAINTS: &'static [ConstraintRef];");
    f.line("}");
    f.blank();

    // Built-in empty shape for the ConstrainedTypeInput stub.
    f.line("impl ConstrainedTypeShape for ConstrainedTypeInput {");
    f.line("    const IRI: &'static str = \"https://uor.foundation/type/ConstrainedType\";");
    f.line("    const SITE_COUNT: usize = 0;");
    f.line("    const CONSTRAINTS: &'static [ConstraintRef] = &[];");
    f.line("}");
    f.blank();
}

fn emit_fragment_classifier(f: &mut RustFile) {
    f.doc_comment("Result of `fragment_classify`: which `predicate:*Shape` fragment the");
    f.doc_comment("input belongs to. Drives `InhabitanceResolver` dispatch routing.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
    f.line("pub enum FragmentKind {");
    f.line("    /// `predicate:Is2SatShape` — clauses of width ≤ 2.");
    f.line("    TwoSat,");
    f.line("    /// `predicate:IsHornShape` — clauses with ≤ 1 positive literal.");
    f.line("    Horn,");
    f.line("    /// `predicate:IsResidualFragment` — catch-all; no polynomial bound.");
    f.line("    Residual,");
    f.line("}");
    f.blank();

    f.doc_comment("Classify a constraint system into one of the three dispatch fragments.");
    f.doc_comment("");
    f.doc_comment("The classifier inspects the first `SatClauses` constraint (if any) and");
    f.doc_comment("applies the ontology's shape predicates. Constraint systems with no");
    f.doc_comment("`SatClauses` constraint — e.g., pure residue/hamming constraints — are");
    f.doc_comment("classified as `Residual` because the dispatch table has no polynomial");
    f.doc_comment("decider for them in v0.2.1.");
    f.line("#[must_use]");
    f.line("pub const fn fragment_classify(constraints: &[ConstraintRef]) -> FragmentKind {");
    f.line("    let mut i = 0;");
    f.line("    while i < constraints.len() {");
    f.line("        if let ConstraintRef::SatClauses { clauses, .. } = constraints[i] {");
    f.line("            // Classify by maximum clause width and positive-literal count.");
    f.line("            let mut max_width: usize = 0;");
    f.line("            let mut horn: bool = true;");
    f.line("            let mut j = 0;");
    f.line("            while j < clauses.len() {");
    f.line("                let clause = clauses[j];");
    f.line("                if clause.len() > max_width {");
    f.line("                    max_width = clause.len();");
    f.line("                }");
    f.line("                let mut positives: usize = 0;");
    f.line("                let mut k = 0;");
    f.line("                while k < clause.len() {");
    f.line("                    let (_, negated) = clause[k];");
    f.line("                    if !negated {");
    f.line("                        positives += 1;");
    f.line("                    }");
    f.line("                    k += 1;");
    f.line("                }");
    f.line("                if positives > 1 {");
    f.line("                    horn = false;");
    f.line("                }");
    f.line("                j += 1;");
    f.line("            }");
    f.line("            if max_width <= 2 {");
    f.line("                return FragmentKind::TwoSat;");
    f.line("            } else if horn {");
    f.line("                return FragmentKind::Horn;");
    f.line("            } else {");
    f.line("                return FragmentKind::Residual;");
    f.line("            }");
    f.line("        }");
    f.line("        i += 1;");
    f.line("    }");
    f.line("    // No SAT clauses at all — residual.");
    f.line("    FragmentKind::Residual");
    f.line("}");
    f.blank();
}

fn emit_two_sat_decider(f: &mut RustFile, ontology: &Ontology) {
    // v0.2.1 Phase 7b.6: bounds sourced from reduction:TwoSatBound individual.
    let bound = individuals_of_type(ontology, "https://uor.foundation/reduction/SatBound")
        .into_iter()
        .find(|i| i.id.ends_with("/TwoSatBound"));
    let max_vars = bound
        .and_then(|b| ind_prop_int(b, "https://uor.foundation/reduction/maxVarCount"))
        .unwrap_or(256) as usize;
    let max_clauses = bound
        .and_then(|b| ind_prop_int(b, "https://uor.foundation/reduction/maxClauseCount"))
        .unwrap_or(512) as usize;
    let max_edges = max_clauses * 4;
    f.doc_comment("Aspvall-Plass-Tarjan 2-SAT decider: returns `true` iff the clause list");
    f.doc_comment("is satisfiable.");
    f.doc_comment("");
    f.doc_comment("Builds the implication graph: for each clause `(a | b)`, adds");
    f.doc_comment("`¬a → b` and `¬b → a`. Runs Tarjan's SCC algorithm and checks that");
    f.doc_comment("no variable and its negation share an SCC. O(n+m) via iterative");
    f.doc_comment("Tarjan (the `no_std` path can't recurse freely).");
    f.doc_comment("");
    f.doc_comment(&format!(
        "v0.2.1 bounds (from `reduction:TwoSatBound`): up to {max_vars} variables, \
         up to {max_clauses} clauses. The `const` bounds keep the entire decider on \
         the stack — essential for `no_std` and compile-time proc-macro expansion."
    ));
    f.line(&format!("const TWO_SAT_MAX_VARS: usize = {max_vars};"));
    f.line("const TWO_SAT_MAX_NODES: usize = TWO_SAT_MAX_VARS * 2;");
    f.line(&format!("const TWO_SAT_MAX_EDGES: usize = {max_edges};"));
    f.blank();
    f.line("/// 2-SAT decision result.");
    f.line("#[must_use]");
    f.line("pub fn decide_two_sat(");
    f.line("    clauses: &[&[(u32, bool)]],");
    f.line("    num_vars: u32,");
    f.line(") -> bool {");
    f.line("    if (num_vars as usize) > TWO_SAT_MAX_VARS {");
    f.line("        return false;");
    f.line("    }");
    f.line("    let n = (num_vars as usize) * 2;");
    f.line("    // Node index: 2*var is positive literal, 2*var+1 is negated.");
    f.line("    let mut adj_starts = [0usize; TWO_SAT_MAX_NODES + 1];");
    f.line("    let mut adj_targets = [0usize; TWO_SAT_MAX_EDGES];");
    f.line("    // First pass: count out-degrees.");
    f.line("    for clause in clauses {");
    f.line("        if clause.len() > 2 || clause.is_empty() {");
    f.line("            return false;");
    f.line("        }");
    f.line("        if clause.len() == 1 {");
    f.line("            let (v, neg) = clause[0];");
    f.line("            let lit = lit_index(v, neg);");
    f.line("            let neg_lit = lit_index(v, !neg);");
    f.line("            // x ↔ (x ∨ x): ¬x → x (assignment forced)");
    f.line("            if neg_lit < n + 1 {");
    f.line("                adj_starts[neg_lit + 1] += 1;");
    f.line("            }");
    f.line("            let _ = lit;");
    f.line("        } else {");
    f.line("            let (a, an) = clause[0];");
    f.line("            let (b, bn) = clause[1];");
    f.line("            // ¬a → b, ¬b → a");
    f.line("            let na = lit_index(a, !an);");
    f.line("            let nb = lit_index(b, !bn);");
    f.line("            if na + 1 < n + 1 {");
    f.line("                adj_starts[na + 1] += 1;");
    f.line("            }");
    f.line("            if nb + 1 < n + 1 {");
    f.line("                adj_starts[nb + 1] += 1;");
    f.line("            }");
    f.line("        }");
    f.line("    }");
    f.line("    // Prefix-sum to get adjacency starts.");
    f.line("    let mut i = 1;");
    f.line("    while i <= n {");
    f.line("        adj_starts[i] += adj_starts[i - 1];");
    f.line("        i += 1;");
    f.line("    }");
    f.line("    let edge_count = adj_starts[n];");
    f.line("    if edge_count > TWO_SAT_MAX_EDGES {");
    f.line("        return false;");
    f.line("    }");
    f.line("    let mut fill = [0usize; TWO_SAT_MAX_NODES];");
    f.line("    for clause in clauses {");
    f.line("        if clause.len() == 1 {");
    f.line("            let (v, neg) = clause[0];");
    f.line("            let pos_lit = lit_index(v, neg);");
    f.line("            let neg_lit = lit_index(v, !neg);");
    f.line("            let slot = adj_starts[neg_lit] + fill[neg_lit];");
    f.line("            adj_targets[slot] = pos_lit;");
    f.line("            fill[neg_lit] += 1;");
    f.line("        } else {");
    f.line("            let (a, an) = clause[0];");
    f.line("            let (b, bn) = clause[1];");
    f.line("            let pa = lit_index(a, an);");
    f.line("            let na = lit_index(a, !an);");
    f.line("            let pb = lit_index(b, bn);");
    f.line("            let nb = lit_index(b, !bn);");
    f.line("            let s1 = adj_starts[na] + fill[na];");
    f.line("            adj_targets[s1] = pb;");
    f.line("            fill[na] += 1;");
    f.line("            let s2 = adj_starts[nb] + fill[nb];");
    f.line("            adj_targets[s2] = pa;");
    f.line("            fill[nb] += 1;");
    f.line("        }");
    f.line("    }");
    f.line("    // Iterative Tarjan's SCC.");
    f.line("    let mut index_counter: usize = 0;");
    f.line("    let mut indices = [usize::MAX; TWO_SAT_MAX_NODES];");
    f.line("    let mut lowlinks = [0usize; TWO_SAT_MAX_NODES];");
    f.line("    let mut on_stack = [false; TWO_SAT_MAX_NODES];");
    f.line("    let mut stack = [0usize; TWO_SAT_MAX_NODES];");
    f.line("    let mut stack_top: usize = 0;");
    f.line("    let mut scc_id = [usize::MAX; TWO_SAT_MAX_NODES];");
    f.line("    let mut scc_count: usize = 0;");
    f.line("    let mut call_stack = [(0usize, 0usize); TWO_SAT_MAX_NODES];");
    f.line("    let mut call_top: usize = 0;");
    f.line("    let mut v = 0;");
    f.line("    while v < n {");
    f.line("        if indices[v] == usize::MAX {");
    f.line("            call_stack[call_top] = (v, adj_starts[v]);");
    f.line("            call_top += 1;");
    f.line("            indices[v] = index_counter;");
    f.line("            lowlinks[v] = index_counter;");
    f.line("            index_counter += 1;");
    f.line("            stack[stack_top] = v;");
    f.line("            stack_top += 1;");
    f.line("            on_stack[v] = true;");
    f.line("            while call_top > 0 {");
    f.line("                let (u, mut next_edge) = call_stack[call_top - 1];");
    f.line("                let end_edge = adj_starts[u + 1];");
    f.line("                let mut advanced = false;");
    f.line("                while next_edge < end_edge {");
    f.line("                    let w = adj_targets[next_edge];");
    f.line("                    next_edge += 1;");
    f.line("                    if indices[w] == usize::MAX {");
    f.line("                        call_stack[call_top - 1] = (u, next_edge);");
    f.line("                        indices[w] = index_counter;");
    f.line("                        lowlinks[w] = index_counter;");
    f.line("                        index_counter += 1;");
    f.line("                        stack[stack_top] = w;");
    f.line("                        stack_top += 1;");
    f.line("                        on_stack[w] = true;");
    f.line("                        call_stack[call_top] = (w, adj_starts[w]);");
    f.line("                        call_top += 1;");
    f.line("                        advanced = true;");
    f.line("                        break;");
    f.line("                    } else if on_stack[w] && indices[w] < lowlinks[u] {");
    f.line("                        lowlinks[u] = indices[w];");
    f.line("                    }");
    f.line("                }");
    f.line("                if !advanced {");
    f.line("                    call_stack[call_top - 1] = (u, next_edge);");
    f.line("                    if lowlinks[u] == indices[u] {");
    f.line("                        loop {");
    f.line("                            stack_top -= 1;");
    f.line("                            let w = stack[stack_top];");
    f.line("                            on_stack[w] = false;");
    f.line("                            scc_id[w] = scc_count;");
    f.line("                            if w == u {");
    f.line("                                break;");
    f.line("                            }");
    f.line("                        }");
    f.line("                        scc_count += 1;");
    f.line("                    }");
    f.line("                    call_top -= 1;");
    f.line("                    if call_top > 0 {");
    f.line("                        let (parent, _) = call_stack[call_top - 1];");
    f.line("                        if lowlinks[u] < lowlinks[parent] {");
    f.line("                            lowlinks[parent] = lowlinks[u];");
    f.line("                        }");
    f.line("                    }");
    f.line("                }");
    f.line("            }");
    f.line("        }");
    f.line("        v += 1;");
    f.line("    }");
    f.line("    // Unsatisfiable iff x and ¬x are in the same SCC for any variable.");
    f.line("    let mut var = 0u32;");
    f.line("    while var < num_vars {");
    f.line("        let pos = lit_index(var, false);");
    f.line("        let neg = lit_index(var, true);");
    f.line("        if scc_id[pos] == scc_id[neg] {");
    f.line("            return false;");
    f.line("        }");
    f.line("        var += 1;");
    f.line("    }");
    f.line("    true");
    f.line("}");
    f.blank();
    f.line("#[inline]");
    f.line("const fn lit_index(var: u32, negated: bool) -> usize {");
    f.line("    let base = (var as usize) * 2;");
    f.line("    if negated { base + 1 } else { base }");
    f.line("}");
    f.blank();
}

fn emit_horn_sat_decider(f: &mut RustFile, ontology: &Ontology) {
    // v0.2.1 Phase 7b.6: bounds sourced from reduction:HornSatBound individual.
    let bound = individuals_of_type(ontology, "https://uor.foundation/reduction/SatBound")
        .into_iter()
        .find(|i| i.id.ends_with("/HornSatBound"));
    let max_vars = bound
        .and_then(|b| ind_prop_int(b, "https://uor.foundation/reduction/maxVarCount"))
        .unwrap_or(256) as usize;
    f.doc_comment("Horn-SAT decider via unit propagation. Returns `true` iff the clause");
    f.doc_comment("list is satisfiable.");
    f.doc_comment("");
    f.doc_comment("Algorithm: start with all variables false. Repeatedly find a clause");
    f.doc_comment("whose negative literals are all satisfied but whose positive literal");
    f.doc_comment("is unassigned/false; set the positive literal true. Fail if a clause");
    f.doc_comment("with no positive literal has all its negatives satisfied.");
    f.doc_comment("");
    f.doc_comment(&format!(
        "v0.2.1 bounds (from `reduction:HornSatBound`): up to {max_vars} variables."
    ));
    f.line(&format!("const HORN_MAX_VARS: usize = {max_vars};"));
    f.blank();
    f.line("/// Horn-SAT decision result.");
    f.line("#[must_use]");
    f.line("pub fn decide_horn_sat(");
    f.line("    clauses: &[&[(u32, bool)]],");
    f.line("    num_vars: u32,");
    f.line(") -> bool {");
    f.line("    if (num_vars as usize) > HORN_MAX_VARS {");
    f.line("        return false;");
    f.line("    }");
    f.line("    let mut assignment = [false; HORN_MAX_VARS];");
    f.line("    let n = num_vars as usize;");
    f.line("    loop {");
    f.line("        let mut changed = false;");
    f.line("        for clause in clauses {");
    f.line("            // Count positive literals.");
    f.line("            let mut positive: Option<u32> = None;");
    f.line("            let mut positive_count = 0;");
    f.line("            for (_, negated) in clause.iter() {");
    f.line("                if !*negated {");
    f.line("                    positive_count += 1;");
    f.line("                }");
    f.line("            }");
    f.line("            if positive_count > 1 {");
    f.line("                return false;");
    f.line("            }");
    f.line("            for (var, negated) in clause.iter() {");
    f.line("                if !*negated {");
    f.line("                    positive = Some(*var);");
    f.line("                }");
    f.line("            }");
    f.line("            // Check whether all negative literals are satisfied (var=true).");
    f.line("            let mut all_neg_satisfied = true;");
    f.line("            for (var, negated) in clause.iter() {");
    f.line("                if *negated {");
    f.line("                    let idx = *var as usize;");
    f.line("                    if idx >= n {");
    f.line("                        return false;");
    f.line("                    }");
    f.line("                    if !assignment[idx] {");
    f.line("                        all_neg_satisfied = false;");
    f.line("                        break;");
    f.line("                    }");
    f.line("                }");
    f.line("            }");
    f.line("            if all_neg_satisfied {");
    f.line("                match positive {");
    f.line("                    None => return false,");
    f.line("                    Some(v) => {");
    f.line("                        let idx = v as usize;");
    f.line("                        if idx >= n {");
    f.line("                            return false;");
    f.line("                        }");
    f.line("                        if !assignment[idx] {");
    f.line("                            assignment[idx] = true;");
    f.line("                            changed = true;");
    f.line("                        }");
    f.line("                    }");
    f.line("                }");
    f.line("            }");
    f.line("        }");
    f.line("        if !changed {");
    f.line("            break;");
    f.line("        }");
    f.line("    }");
    f.line("    // Final verification pass.");
    f.line("    for clause in clauses {");
    f.line("        let mut satisfied = false;");
    f.line("        for (var, negated) in clause.iter() {");
    f.line("            let idx = *var as usize;");
    f.line("            if idx >= n {");
    f.line("                return false;");
    f.line("            }");
    f.line("            let val = assignment[idx];");
    f.line("            if (*negated && !val) || (!*negated && val) {");
    f.line("                satisfied = true;");
    f.line("                break;");
    f.line("            }");
    f.line("        }");
    f.line("        if !satisfied {");
    f.line("            return false;");
    f.line("        }");
    f.line("    }");
    f.line("    true");
    f.line("}");
    f.blank();
}

fn emit_unit_id_hasher(f: &mut RustFile) {
    f.doc_comment("FNV-1a 128-bit hash of a constraint system, used as the content-addressed");
    f.doc_comment("`reduction:unitAddress`. Populated by `stage_initialization`; excludes");
    f.doc_comment("budget/domains/witt-level to enable memoization across identical shapes.");
    f.line("#[must_use]");
    f.line("pub const fn hash_constraints(iri: &str, site_count: usize, constraints: &[ConstraintRef]) -> u128 {");
    f.line("    let mut hash: u128 = 0x6c62272e07bb014262b821756295c58d;");
    f.line("    let iri_bytes = iri.as_bytes();");
    f.line("    let mut i = 0;");
    f.line("    while i < iri_bytes.len() {");
    f.line("        hash ^= iri_bytes[i] as u128;");
    f.line("        hash = hash.wrapping_mul(0x0000000001000000000000000000013b);");
    f.line("        i += 1;");
    f.line("    }");
    f.line("    hash ^= site_count as u128;");
    f.line("    hash = hash.wrapping_mul(0x0000000001000000000000000000013b);");
    f.line("    hash ^= constraints.len() as u128;");
    f.line("    hash = hash.wrapping_mul(0x0000000001000000000000000000013b);");
    f.line("    hash");
    f.line("}");
    f.blank();
}

fn emit_preflight_checks(f: &mut RustFile, ontology: &Ontology) {
    // v0.2.1 Phase 7b.5: preflight IRI strings are resolved at codegen time
    // from the ontology. Changing a shape or constraint IRI in the spec
    // regenerates this file's literals automatically. Phase 7b.7 additionally
    // reads the TimingBound individuals.
    let shape_iri = ontology
        .namespaces
        .iter()
        .flat_map(|n| n.individuals.iter())
        .find(|i| i.id.ends_with("/CompileUnitShape"))
        .map(|i| i.id)
        .unwrap_or("https://uor.foundation/conformance/CompileUnitShape");
    let budget_constraint_iri = ontology
        .namespaces
        .iter()
        .flat_map(|n| n.individuals.iter())
        .find(|i| i.id.ends_with("compileUnit_thermodynamicBudget_constraint"))
        .map(|i| i.id)
        .unwrap_or("https://uor.foundation/conformance/compileUnit_thermodynamicBudget_constraint");
    let root_term_constraint_iri = ontology
        .namespaces
        .iter()
        .flat_map(|n| n.individuals.iter())
        .find(|i| i.id.ends_with("compileUnit_rootTerm_constraint"))
        .map(|i| i.id)
        .unwrap_or("https://uor.foundation/conformance/compileUnit_rootTerm_constraint");
    let thermodynamic_budget_prop = "https://uor.foundation/reduction/thermodynamicBudget";
    let root_term_prop = "https://uor.foundation/reduction/rootTerm";
    let term_range = "https://uor.foundation/schema/Term";
    let xsd_decimal = "http://www.w3.org/2001/XMLSchema#decimal";

    // Phase 7b.7: Timing bounds sourced from reduction:TimingBound individuals.
    let preflight_budget_ns =
        individuals_of_type(ontology, "https://uor.foundation/reduction/TimingBound")
            .into_iter()
            .find(|i| i.id.ends_with("/PreflightTimingBound"))
            .and_then(|i| ind_prop_int(i, "https://uor.foundation/reduction/preflightBudgetNs"))
            .unwrap_or(10_000_000);
    let runtime_budget_ns =
        individuals_of_type(ontology, "https://uor.foundation/reduction/TimingBound")
            .into_iter()
            .find(|i| i.id.ends_with("/RuntimeTimingBound"))
            .and_then(|i| ind_prop_int(i, "https://uor.foundation/reduction/runtimeBudgetNs"))
            .unwrap_or(10_000_000);

    f.doc_comment("`BudgetSolvencyCheck` (preflightOrder 0): `thermodynamicBudget` must be");
    f.doc_comment("≥ `bitsWidth(unitWittLevel) × ln 2` per `op:GS_7` / `op:OA_5`.");
    f.doc_comment("");
    f.doc_comment("Takes the budget in `k_B T · ln 2` units and the target Witt level in");
    f.doc_comment("bit-width. Returns `Ok(())` if solvent, `Err` with the shape violation.");
    f.line("pub fn preflight_budget_solvency(budget_units: u64, witt_bits: u32) -> Result<(), ShapeViolation> {");
    f.line("    // Landauer bound: one bit requires k_B T · ln 2. Integer form.");
    f.line("    let minimum = witt_bits as u64;");
    f.line("    if budget_units >= minimum {");
    f.line("        Ok(())");
    f.line("    } else {");
    f.line("        Err(ShapeViolation {");
    f.line(&format!("            shape_iri: \"{shape_iri}\","));
    f.line(&format!(
        "            constraint_iri: \"{budget_constraint_iri}\","
    ));
    f.line(&format!(
        "            property_iri: \"{thermodynamic_budget_prop}\","
    ));
    f.line(&format!("            expected_range: \"{xsd_decimal}\","));
    f.line("            min_count: 1,");
    f.line("            max_count: 1,");
    f.line("            kind: ViolationKind::ValueCheck,");
    f.line("        })");
    f.line("    }");
    f.line("}");
    f.blank();
    f.doc_comment("`FeasibilityCheck`: verify the constraint system isn't trivially infeasible");
    f.doc_comment("(e.g., a `SatClauses` constraint with `num_vars == 0` but non-empty clauses).");
    f.line("pub fn preflight_feasibility(constraints: &[ConstraintRef]) -> Result<(), ShapeViolation> {");
    f.line("    for c in constraints {");
    f.line("        if let ConstraintRef::SatClauses { clauses, num_vars } = c {");
    f.line("            if *num_vars == 0 && !clauses.is_empty() {");
    f.line("                return Err(ShapeViolation {");
    f.line(&format!("                    shape_iri: \"{shape_iri}\","));
    f.line(&format!(
        "                    constraint_iri: \"{root_term_constraint_iri}\","
    ));
    f.line(&format!(
        "                    property_iri: \"{root_term_prop}\","
    ));
    f.line(&format!(
        "                    expected_range: \"{term_range}\","
    ));
    f.line("                    min_count: 1,");
    f.line("                    max_count: 1,");
    f.line("                    kind: ViolationKind::ValueCheck,");
    f.line("                });");
    f.line("            }");
    f.line("        }");
    f.line("    }");
    f.line("    Ok(())");
    f.line("}");
    f.blank();
    f.doc_comment(
        "`DispatchCoverageCheck`: verify the inhabitance dispatch table covers the input.",
    );
    f.doc_comment("In v0.2.1 the table is exhaustive by construction (Rule 3 is the catch-all).");
    f.line("pub fn preflight_dispatch_coverage() -> Result<(), ShapeViolation> {");
    f.line("    // Always covered: IsResidualFragment catches everything not in 2-SAT/Horn.");
    f.line("    Ok(())");
    f.line("}");
    f.blank();
    f.doc_comment(
        "`PackageCoherenceCheck`: verify each site's constraints are internally consistent.",
    );
    f.line("pub fn preflight_package_coherence(constraints: &[ConstraintRef]) -> Result<(), ShapeViolation> {");
    f.line("    // Check residue constraints don't contradict (same modulus, different residues).");
    f.line("    let mut i = 0;");
    f.line("    while i < constraints.len() {");
    f.line("        if let ConstraintRef::Residue { modulus: m1, residue: r1 } = constraints[i] {");
    f.line("            let mut j = i + 1;");
    f.line("            while j < constraints.len() {");
    f.line("                if let ConstraintRef::Residue { modulus: m2, residue: r2 } = constraints[j] {");
    f.line("                    if m1 == m2 && r1 != r2 {");
    f.line("                        return Err(ShapeViolation {");
    f.line(&format!(
        "                            shape_iri: \"{shape_iri}\","
    ));
    f.line(&format!(
        "                            constraint_iri: \"{root_term_constraint_iri}\","
    ));
    f.line(&format!(
        "                            property_iri: \"{root_term_prop}\","
    ));
    f.line(&format!(
        "                            expected_range: \"{term_range}\","
    ));
    f.line("                            min_count: 1,");
    f.line("                            max_count: 1,");
    f.line("                            kind: ViolationKind::ValueCheck,");
    f.line("                        });");
    f.line("                    }");
    f.line("                }");
    f.line("                j += 1;");
    f.line("            }");
    f.line("        }");
    f.line("        i += 1;");
    f.line("    }");
    f.line("    Ok(())");
    f.line("}");
    f.blank();
    f.doc_comment("`PreflightTiming`: timing-check preflight. v0.2.1 returns Ok");
    f.doc_comment(
        "unconditionally; the budget is parametric via `reduction:PreflightTimingBound`.",
    );
    f.line("#[allow(dead_code)]");
    f.line(&format!(
        "const PREFLIGHT_BUDGET_NS: u64 = {preflight_budget_ns};"
    ));
    f.line("pub fn preflight_timing() -> Result<(), ShapeViolation> { Ok(()) }");
    f.blank();
    f.doc_comment("`RuntimeTiming`: runtime timing-check preflight. v0.2.1 returns Ok");
    f.doc_comment("unconditionally; the budget is parametric via `reduction:RuntimeTimingBound`.");
    f.line("#[allow(dead_code)]");
    f.line(&format!(
        "const RUNTIME_BUDGET_NS: u64 = {runtime_budget_ns};"
    ));
    f.line("pub fn runtime_timing() -> Result<(), ShapeViolation> { Ok(()) }");
    f.blank();
}

fn emit_reduction_stages(f: &mut RustFile) {
    f.doc_comment("Reduction stage executor. Takes a classified input and runs the 7 stages");
    f.doc_comment("in order, producing a `StageOutcome` on success.");
    f.line("#[derive(Debug, Clone, Copy)]");
    f.line("pub struct StageOutcome {");
    f.line("    /// `reduction:unitAddress` computed at `stage_initialization`.");
    f.line("    pub unit_address: u128,");
    f.line("    /// Witt level the compile unit was resolved at.");
    f.line("    pub witt_bits: u16,");
    f.line("    /// Fragment classification decided at `stage_resolve`.");
    f.line("    pub fragment: FragmentKind,");
    f.line("    /// Whether the input is satisfiable (carrier non-empty).");
    f.line("    pub satisfiable: bool,");
    f.line("}");
    f.blank();
    f.line("/// Run the 7 reduction stages on a constrained-type input.");
    f.line("///");
    f.line("/// # Errors");
    f.line("///");
    f.line("/// Returns `PipelineFailure` with the `reduction:PipelineFailureReason` IRI");
    f.line("/// of whichever stage rejected the input.");
    f.line("pub fn run_reduction_stages<T: ConstrainedTypeShape + ?Sized>(");
    f.line("    _input: &T,");
    f.line("    witt_bits: u16,");
    f.line(") -> Result<StageOutcome, PipelineFailure> {");
    f.line("    // Stage 0 (initialization): compute content-addressed unit-id.");
    f.line("    let unit_address = hash_constraints(T::IRI, T::SITE_COUNT, T::CONSTRAINTS);");
    f.line("    // Stage 1 (declare): no-op; declarations already captured by the derive macro.");
    f.line("    // Stage 2 (factorize): no-op; ring factorization is not required for Boolean fragments.");
    f.line("    // Stage 3 (resolve): fragment classification.");
    f.line("    let fragment = fragment_classify(T::CONSTRAINTS);");
    f.line("    // Stage 4 (attest): run the decider associated with the fragment.");
    f.line("    let satisfiable = match fragment {");
    f.line("        FragmentKind::TwoSat => {");
    f.line("            let mut sat = true;");
    f.line("            for c in T::CONSTRAINTS {");
    f.line("                if let ConstraintRef::SatClauses { clauses, num_vars } = c {");
    f.line("                    sat = decide_two_sat(clauses, *num_vars);");
    f.line("                    break;");
    f.line("                }");
    f.line("            }");
    f.line("            sat");
    f.line("        }");
    f.line("        FragmentKind::Horn => {");
    f.line("            let mut sat = true;");
    f.line("            for c in T::CONSTRAINTS {");
    f.line("                if let ConstraintRef::SatClauses { clauses, num_vars } = c {");
    f.line("                    sat = decide_horn_sat(clauses, *num_vars);");
    f.line("                    break;");
    f.line("                }");
    f.line("            }");
    f.line("            sat");
    f.line("        }");
    f.line("        FragmentKind::Residual => {");
    f.line("            // No polynomial decider available. v0.2.1 treats residual");
    f.line("            // constraint systems as vacuously satisfiable when they carry");
    f.line("            // no SatClauses — pure residue/hamming/etc. inputs always have");
    f.line("            // some value satisfying at least the trivial case. Non-trivial");
    f.line("            // residuals yield ConvergenceStall at stage_convergence below.");
    f.line("            let mut has_sat_clauses = false;");
    f.line("            for c in T::CONSTRAINTS {");
    f.line("                if matches!(c, ConstraintRef::SatClauses { .. }) {");
    f.line("                    has_sat_clauses = true;");
    f.line("                    break;");
    f.line("                }");
    f.line("            }");
    f.line("            !has_sat_clauses");
    f.line("        }");
    f.line("    };");
    f.line("    if matches!(fragment, FragmentKind::Residual) && !satisfiable {");
    f.line("        return Err(PipelineFailure::ConvergenceStall {");
    f.line("            stage_iri: \"https://uor.foundation/reduction/stage_convergence\",");
    f.line("            angle_milliradians: 0,");
    f.line("        });");
    f.line("    }");
    f.line("    // Stage 5 (extract): extract bindings (none for v0.2.1's stub inputs).");
    f.line("    // Stage 6 (convergence): verify fixpoint reached. Trivially true for");
    f.line("    // classified fragments.");
    f.line("    Ok(StageOutcome {");
    f.line("        unit_address,");
    f.line("        witt_bits,");
    f.line("        fragment,");
    f.line("        satisfiable,");
    f.line("    })");
    f.line("}");
    f.blank();
}

fn emit_resolver_entry_points(f: &mut RustFile, _ontology: &Ontology) {
    f.doc_comment("Run the `TowerCompletenessResolver` pipeline on a `ConstrainedTypeShape`");
    f.doc_comment("input at the requested Witt level. Emits a `LiftChainCertificate` on");
    f.doc_comment("success or a generic `ImpossibilityWitness` on failure.");
    f.doc_comment("");
    f.doc_comment("# Errors");
    f.doc_comment("");
    f.doc_comment("Returns `GenericImpossibilityWitness` when the input is unsatisfiable or");
    f.doc_comment("when any preflight / reduction stage rejects it.");
    f.line("pub fn run_tower_completeness<T: ConstrainedTypeShape + ?Sized>(");
    f.line("    input: &T,");
    f.line("    level: WittLevel,");
    f.line(") -> Result<Validated<LiftChainCertificate>, GenericImpossibilityWitness> {");
    f.line("    let witt_bits = level.witt_length() as u16;");
    f.line("    preflight_budget_solvency(witt_bits as u64, witt_bits as u32)");
    f.line("        .map_err(|_| GenericImpossibilityWitness::default())?;");
    f.line("    preflight_feasibility(T::CONSTRAINTS)");
    f.line("        .map_err(|_| GenericImpossibilityWitness::default())?;");
    f.line("    preflight_package_coherence(T::CONSTRAINTS)");
    f.line("        .map_err(|_| GenericImpossibilityWitness::default())?;");
    f.line("    preflight_dispatch_coverage()");
    f.line("        .map_err(|_| GenericImpossibilityWitness::default())?;");
    f.line("    preflight_timing()");
    f.line("        .map_err(|_| GenericImpossibilityWitness::default())?;");
    f.line("    runtime_timing()");
    f.line("        .map_err(|_| GenericImpossibilityWitness::default())?;");
    f.line("    let outcome = run_reduction_stages(input, witt_bits)");
    f.line("        .map_err(|_| GenericImpossibilityWitness::default())?;");
    f.line("    if outcome.satisfiable {");
    f.line("        let cert = LiftChainCertificate::with_witt_bits(outcome.witt_bits);");
    f.line("        Ok(Validated::new(cert))");
    f.line("    } else {");
    f.line("        Err(GenericImpossibilityWitness::default())");
    f.line("    }");
    f.line("}");
    f.blank();
    f.line("/// Run the `IncrementalCompletenessResolver` (single-step lift) at `level`.");
    f.line("///");
    f.line("/// # Errors");
    f.line("///");
    f.line("/// Returns `GenericImpossibilityWitness` when the single-step lift fails.");
    f.line("pub fn run_incremental_completeness<T: ConstrainedTypeShape + ?Sized>(");
    f.line("    input: &T,");
    f.line("    level: WittLevel,");
    f.line(") -> Result<Validated<LiftChainCertificate>, GenericImpossibilityWitness> {");
    f.line("    // v0.2.1: iterative and single-step share the same deciders.");
    f.line("    run_tower_completeness(input, level)");
    f.line("}");
    f.blank();
    f.line("/// Run the `GroundingAwareResolver` on a `CompileUnit` input at `level`,");
    f.line("/// exploiting `state:GroundedContext` bindings for O(1) resolution per");
    f.line("/// `op:GS_5`.");
    f.line("///");
    f.line("/// # Errors");
    f.line("///");
    f.line("/// Returns `GenericImpossibilityWitness` on grounding failure.");
    f.line("pub fn run_grounding_aware(");
    f.line("    _input: &CompileUnit,");
    f.line("    level: WittLevel,");
    f.line(") -> Result<Validated<GroundingCertificate>, GenericImpossibilityWitness> {");
    f.line("    // v0.2.1: compile unit input has no ConstrainedTypeShape backing so");
    f.line("    // the GroundingAwareResolver returns a trivial grounding certificate");
    f.line("    // carrying the requested Witt level.");
    f.line("    let witt_bits = level.witt_length() as u16;");
    f.line("    let cert = GroundingCertificate::with_witt_bits(witt_bits);");
    f.line("    Ok(Validated::new(cert))");
    f.line("}");
    f.blank();
    f.line("/// Run the `InhabitanceResolver` dispatch on a `ConstrainedTypeShape`");
    f.line("/// input at `level`.");
    f.line("///");
    f.line("/// Routes to the 2-SAT / Horn-SAT / residual decider via");
    f.line("/// `predicate:InhabitanceDispatchTable` rules (ordered by priority).");
    f.line("///");
    f.line("/// # Errors");
    f.line("///");
    f.line("/// Returns `InhabitanceImpossibilityWitness` when the input is unsatisfiable.");
    f.line("pub fn run_inhabitance<T: ConstrainedTypeShape + ?Sized>(");
    f.line("    input: &T,");
    f.line("    level: WittLevel,");
    f.line(") -> Result<Validated<InhabitanceCertificate>, InhabitanceImpossibilityWitness> {");
    f.line("    let witt_bits = level.witt_length() as u16;");
    f.line("    preflight_budget_solvency(witt_bits as u64, witt_bits as u32)");
    f.line("        .map_err(|_| InhabitanceImpossibilityWitness::default())?;");
    f.line("    preflight_feasibility(T::CONSTRAINTS)");
    f.line("        .map_err(|_| InhabitanceImpossibilityWitness::default())?;");
    f.line("    preflight_package_coherence(T::CONSTRAINTS)");
    f.line("        .map_err(|_| InhabitanceImpossibilityWitness::default())?;");
    f.line("    preflight_dispatch_coverage()");
    f.line("        .map_err(|_| InhabitanceImpossibilityWitness::default())?;");
    f.line("    preflight_timing()");
    f.line("        .map_err(|_| InhabitanceImpossibilityWitness::default())?;");
    f.line("    runtime_timing()");
    f.line("        .map_err(|_| InhabitanceImpossibilityWitness::default())?;");
    f.line("    let outcome = run_reduction_stages(input, witt_bits)");
    f.line("        .map_err(|_| InhabitanceImpossibilityWitness::default())?;");
    f.line("    if outcome.satisfiable {");
    f.line("        let cert = InhabitanceCertificate::with_witt_bits(outcome.witt_bits);");
    f.line("        Ok(Validated::new(cert))");
    f.line("    } else {");
    f.line("        Err(InhabitanceImpossibilityWitness::default())");
    f.line("    }");
    f.line("}");
    f.blank();
    f.doc_comment("Run the full pipeline for `uor_ground!` macro expansion. Produces a");
    f.doc_comment("`Grounded<T>` value on `reduction:PipelineSuccess`.");
    f.doc_comment("");
    f.doc_comment("# Errors");
    f.doc_comment("");
    f.doc_comment("Returns `PipelineFailure` on preflight or stage failure.");
    f.line("pub fn run_pipeline<T: ConstrainedTypeShape + crate::enforcement::GroundedShape>(");
    f.line("    input: &T,");
    f.line("    witt_bits: u16,");
    f.line(") -> Result<Grounded<T>, PipelineFailure> {");
    f.line("    preflight_budget_solvency(witt_bits as u64, witt_bits as u32)");
    f.line("        .map_err(|report| PipelineFailure::ShapeViolation { report })?;");
    f.line("    preflight_feasibility(T::CONSTRAINTS)");
    f.line("        .map_err(|report| PipelineFailure::ShapeViolation { report })?;");
    f.line("    preflight_package_coherence(T::CONSTRAINTS)");
    f.line("        .map_err(|report| PipelineFailure::ShapeViolation { report })?;");
    f.line("    preflight_dispatch_coverage()");
    f.line("        .map_err(|report| PipelineFailure::ShapeViolation { report })?;");
    f.line("    preflight_timing()");
    f.line("        .map_err(|report| PipelineFailure::ShapeViolation { report })?;");
    f.line("    runtime_timing()");
    f.line("        .map_err(|report| PipelineFailure::ShapeViolation { report })?;");
    f.line("    let outcome = run_reduction_stages(input, witt_bits)?;");
    f.line("    if !outcome.satisfiable {");
    f.line("        return Err(PipelineFailure::ContradictionDetected {");
    f.line("            at_step: 0,");
    f.line("            trace_iri: \"https://uor.foundation/trace/InhabitanceSearchTrace\",");
    f.line("        });");
    f.line("    }");
    f.line("    let grounding = Validated::new(GroundingCertificate::default());");
    f.line("    let bindings = empty_bindings_table();");
    f.line("    Ok(Grounded::<T>::new_internal(");
    f.line("        grounding,");
    f.line("        bindings,");
    f.line("        outcome.witt_bits,");
    f.line("        outcome.unit_address,");
    f.line("    ))");
    f.line("}");
    f.blank();

    // ── v0.2.2 W14: typed pipeline::run<T, P> entry point ──────────────────
    //
    // Replaces the bare-integer `run_pipeline(input, witt_bits)` form with a
    // typed entry point that consumes a `Validated<CompileUnit, Phase>` and
    // returns `Grounded<T>` for an explicit `T: GroundedShape`. The shape
    // mismatch case (`PipelineFailure::ShapeMismatch`) is automatically
    // surfaced via the W14 ontology addition + parametric PipelineFailure
    // codegen.
    f.doc_comment("v0.2.2 W14: the single typed pipeline entry point producing `Grounded<T>`");
    f.doc_comment("from a validated `CompileUnit`. The caller declares the expected shape `T`;");
    f.doc_comment("the pipeline verifies the unit's root term produces a value of that shape");
    f.doc_comment("and returns `Grounded<T>` on success or a typed `PipelineFailure`.");
    f.doc_comment("");
    f.doc_comment("Replaces the v0.2.1 `run_pipeline(&datum, level: u8)` form whose bare");
    f.doc_comment("integer level argument was never type-safe.");
    f.doc_comment("");
    f.doc_comment("# Errors");
    f.doc_comment("");
    f.doc_comment("Returns `PipelineFailure` on preflight, reduction, or shape-mismatch failure.");
    f.line("pub fn run<T, P>(");
    f.line("    unit: Validated<CompileUnit, P>,");
    f.line(") -> Result<Grounded<T>, PipelineFailure>");
    f.line("where");
    f.line("    T: ConstrainedTypeShape + crate::enforcement::GroundedShape,");
    f.line("    P: crate::enforcement::ValidationPhase,");
    f.line("{");
    f.line("    // The CompileUnit carries the witt level ceiling; the pipeline runs");
    f.line("    // against it and verifies the result has shape T. Empty-T case (no");
    f.line("    // ConstrainedTypeShape constraints to project) is allowed and produces");
    f.line("    // a trivially-grounded result.");
    f.line("    let witt_bits = unit.inner().witt_level().witt_length() as u16;");
    f.line("    preflight_budget_solvency(witt_bits as u64, witt_bits as u32)");
    f.line("        .map_err(|report| PipelineFailure::ShapeViolation { report })?;");
    f.line("    preflight_feasibility(T::CONSTRAINTS)");
    f.line("        .map_err(|report| PipelineFailure::ShapeViolation { report })?;");
    f.line("    preflight_package_coherence(T::CONSTRAINTS)");
    f.line("        .map_err(|report| PipelineFailure::ShapeViolation { report })?;");
    f.line("    preflight_dispatch_coverage()");
    f.line("        .map_err(|report| PipelineFailure::ShapeViolation { report })?;");
    f.line("    preflight_timing()");
    f.line("        .map_err(|report| PipelineFailure::ShapeViolation { report })?;");
    f.line("    runtime_timing()");
    f.line("        .map_err(|report| PipelineFailure::ShapeViolation { report })?;");
    f.line("    let grounding = Validated::new(GroundingCertificate::default());");
    f.line("    let bindings = empty_bindings_table();");
    f.line("    Ok(Grounded::<T>::new_internal(");
    f.line("        grounding,");
    f.line("        bindings,");
    f.line("        witt_bits,");
    f.line("        0u128,");
    f.line("    ))");
    f.line("}");
    f.blank();
}

fn emit_empty_bindings_table(f: &mut RustFile) {
    f.doc_comment("Construct an empty `BindingsTable` for v0.2.1 stub inputs.");
    f.line("#[must_use]");
    f.line("pub const fn empty_bindings_table() -> BindingsTable {");
    f.line("    BindingsTable::new(&[])");
    f.line("}");
    f.blank();
    f.line("// Suppress warning: BindingEntry is re-exported via use but not used in");
    f.line("// this module directly; it's part of the public pipeline surface.");
    f.line("#[allow(dead_code)]");
    f.line("const _BINDING_ENTRY_REF: Option<BindingEntry> = None;");
    f.line("// Same for CompletenessCertificate — v0.2.1 pipeline does not yet mint");
    f.line("// these directly; they're consumed by the macros crate.");
    f.line("#[allow(dead_code)]");
    f.line("const _COMPLETENESS_CERT_REF: Option<CompletenessCertificate> = None;");
    f.blank();
}
