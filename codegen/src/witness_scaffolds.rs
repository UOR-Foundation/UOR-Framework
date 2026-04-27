//! Phase 10 codegen: VerifiedMint witness scaffolds for Path-2 classes.
//!
//! For every class classified `Path2TheoremWitness` (excluding the four
//! `AlreadyImplemented` partition-algebra witnesses), this module emits:
//!
//! * `Mint{Foo}` — a sealed witness struct (`Copy + Clone + Debug + Eq +
//!   PartialEq` always; `Hash` only when the class is non-entropy-bearing
//!   per Phase 0 R7).
//! * `Mint{Foo}Inputs<H: HostTypes>` — caller-supplied input bundle. Phase
//!   10 emits a minimal `PhantomData<H>` placeholder; Phase 12 fills in
//!   the per-class field mapping when the verify primitive needs it.
//! * `impl Certificate for Mint{Foo}` — registers the witness as a sealed
//!   ontology certificate carrier (`const IRI` + `type Evidence = ()`).
//! * `impl OntologyVerifiedMint for Mint{Foo}` — wires the type-erased
//!   `ontology_mint::<H>` entry point to the appropriate primitive
//!   stub (`crate::primitives::{family}::verify_{ident}`).
//! * Stubbed primitive bodies under `crate::primitives::{family}` that
//!   return `Err(GenericImpossibilityWitness::for_identity(
//!   "WITNESS_UNIMPLEMENTED_STUB:{IRI}"))`. Phase 12 replaces each stub
//!   with a real verification body.
//!
//! Plus a module-level `OntologyVerifiedMint` trait declaration. The
//! pre-existing `VerifiedMint` trait (used by the partition-algebra
//! amendment witnesses) is left untouched per the carve-out clause.

use std::collections::BTreeMap;

use uor_ontology::Ontology;

use crate::classification::{classify_all, primitive_module_for_identity, PathKind};
use crate::emit::RustFile;
use crate::mapping::to_snake_case;

/// One Path-2 emission descriptor.
struct Path2Emission {
    /// Class local name, e.g. `BornRuleVerification`.
    class_local: String,
    /// Class IRI, used in the WITNESS_UNIMPLEMENTED_STUB marker.
    class_iri: String,
    /// Namespace prefix (e.g. `morphism`, `state`) — included in the
    /// verify-function name to disambiguate cross-namespace collisions
    /// like `morphism::GroundingWitness` vs `state::GroundingWitness`.
    namespace: String,
    /// Theorem `op:Identity` IRI from `classification::resolve_theorem_identity`.
    theorem_identity: String,
    /// Family-routed primitive module (`pt`/`st`/.../`oa`).
    primitive_module: String,
    /// Snake-cased verify-function name. Format:
    /// `<namespace>_<class_local_snake>` so two classes with the same
    /// local name (cross-namespace) collide-free.
    verify_ident: String,
    /// Whether the class is entropy-bearing (R7) — controls Hash derive.
    entropy_bearing: bool,
}

/// Returns the Path-2 emission set for `ontology`, sorted by class IRI for
/// determinism.
fn path2_emissions(ontology: &Ontology) -> Vec<Path2Emission> {
    let mut out: Vec<Path2Emission> = Vec::new();
    for entry in classify_all(ontology) {
        if let PathKind::Path2TheoremWitness {
            entropy_bearing,
            theorem_identity,
        } = &entry.path_kind
        {
            let primitive_module = primitive_module_for_identity(theorem_identity).to_string();
            // Verify function name: <namespace>_<class_local_snake>. This
            // disambiguates morphism::GroundingWitness vs state::GroundingWitness
            // and keeps the function name 1:1 with the class.
            let verify_ident = format!("{}_{}", entry.namespace, to_snake_case(entry.class_local));
            out.push(Path2Emission {
                class_local: entry.class_local.to_string(),
                class_iri: entry.class_iri.to_string(),
                namespace: entry.namespace.to_string(),
                theorem_identity: theorem_identity.clone(),
                primitive_module,
                verify_ident,
                entropy_bearing: *entropy_bearing,
            });
        }
    }
    out.sort_by(|a, b| a.class_iri.cmp(&b.class_iri));
    out
}

/// Mint-struct name for an emission. `Mint{Foo}` for namespaces with
/// unique class local names; `Mint{TitleCase(namespace)}{Foo}` when
/// the local name collides cross-namespace.
fn mint_struct_name(e: &Path2Emission) -> String {
    if needs_namespace_qualifier(&e.class_local) {
        let mut ns = e.namespace.clone();
        if let Some(c) = ns.get_mut(0..1) {
            c.make_ascii_uppercase();
        }
        format!("Mint{ns}{}", e.class_local)
    } else {
        format!("Mint{}", e.class_local)
    }
}

/// Class local names that appear in two or more namespaces in the
/// ontology Path-2 set. Hard-coded because the set is small and stable;
/// the Phase 10 verification test re-checks against the live
/// classification.
const COLLIDING_CLASS_LOCALS: &[&str] = &["GroundingWitness"];

fn needs_namespace_qualifier(class_local: &str) -> bool {
    COLLIDING_CLASS_LOCALS.contains(&class_local)
}

/// Returns the unique-by-(module,verify_ident) primitive stub set,
/// indexed by module name.
fn primitive_stub_groups(emissions: &[Path2Emission]) -> BTreeMap<String, Vec<&Path2Emission>> {
    let mut by_module: BTreeMap<String, Vec<&Path2Emission>> = BTreeMap::new();
    for e in emissions {
        by_module
            .entry(e.primitive_module.clone())
            .or_default()
            .push(e);
    }
    by_module
}

/// Generates `foundation/src/witness_scaffolds.rs`.
#[must_use]
pub fn generate_witness_scaffolds_module(ontology: &Ontology) -> String {
    let mut f = RustFile::new(
        "Phase 10 — Path-2 VerifiedMint witness scaffolds. Generated from \
         every `Path2TheoremWitness` classification; one `Mint{Foo}` + \
         `Mint{Foo}Inputs<H>` + `Certificate` + `OntologyVerifiedMint` \
         per class. Routes to per-family primitive stubs in \
         `crate::primitives::*`.",
    );
    f.line("#![allow(clippy::module_name_repetitions)]");
    f.blank();
    f.line("use crate::enforcement::{");
    f.line("    Certificate, ContentFingerprint, GenericImpossibilityWitness, OntologyTarget,");
    f.line("};");
    f.line("use crate::HostTypes;");
    f.line("use core::marker::PhantomData;");
    f.blank();

    // OntologyVerifiedMint trait declaration. Sealed via the
    // `Certificate` supertrait (which is itself sealed by
    // `crate::enforcement::certificate_sealed::Sealed`). The trait
    // carries a generic associated type `Inputs<H>` and the
    // `ontology_mint<H>` entry point — distinct from the existing
    // non-generic `VerifiedMint` trait used by the partition-algebra
    // amendment witnesses.
    f.doc_comment("Phase 10 — sealed mint trait for ontology-derived Path-2 witnesses.");
    f.doc_comment("");
    f.doc_comment("Distinct from `VerifiedMint` (used by the partition-algebra");
    f.doc_comment("amendment): the new trait carries a `HostTypes`-parameterized GAT");
    f.doc_comment("`Inputs<H>` so witness inputs can hold `H::Decimal` /");
    f.doc_comment("`{Range}Handle<H>` fields without leaking f64 into the trait shape.");
    f.doc_comment("Sealed via the `Certificate` supertrait.");
    f.line("pub trait OntologyVerifiedMint: Certificate {");
    f.line("    /// Caller-supplied input bundle, parameterized over the host's");
    f.line("    /// chosen `HostTypes` so witness inputs can carry `H::Decimal`,");
    f.line("    /// `{Range}Handle<H>`, etc.");
    f.line("    type Inputs<H: HostTypes>;");
    f.blank();
    f.line("    /// Op-namespace identity that this witness attests. Phase 10a");
    f.line("    /// resolves this via `proof:provesIdentity` inverse lookup.");
    f.line("    const THEOREM_IDENTITY: &'static str;");
    f.blank();
    f.line("    /// Verify the inputs and mint a witness. The default Phase-10");
    f.line("    /// stub returns the `WITNESS_UNIMPLEMENTED_STUB:{IRI}` marker;");
    f.line("    /// Phase 12 replaces each per-family stub with the real body.");
    f.line("    /// # Errors");
    f.line("    /// Returns `GenericImpossibilityWitness::for_identity(iri)` whenever");
    f.line("    /// the underlying primitive rejects the inputs.");
    f.line("    fn ontology_mint<H: HostTypes>(");
    f.line("        inputs: Self::Inputs<H>,");
    f.line("    ) -> Result<Self, GenericImpossibilityWitness>");
    f.line("    where");
    f.line("        Self: Sized;");
    f.line("}");
    f.blank();

    let emissions = path2_emissions(ontology);

    for e in &emissions {
        emit_one_witness_scaffold(&mut f, e);
    }

    f.finish()
}

/// Emit one Path-2 emission's full scaffolding in-place.
fn emit_one_witness_scaffold(f: &mut RustFile, e: &Path2Emission) {
    let name = mint_struct_name(e);
    let inputs = format!("{name}Inputs");
    let stub_marker = format!("WITNESS_UNIMPLEMENTED_STUB:{}", e.class_iri);

    f.doc_comment(&format!(
        "Phase 10 sealed witness for `{}`. Attests `{}`.",
        e.class_iri, e.theorem_identity
    ));
    f.doc_comment(&format!(
        "Carries a single `ContentFingerprint` and a private constructor; \
         mint via `OntologyVerifiedMint::ontology_mint`. Phase 12 fills the \
         primitive body in `crate::primitives::{}::verify_{}`.",
        e.primitive_module, e.verify_ident
    ));
    let derives = if e.entropy_bearing {
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]"
    } else {
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]"
    };
    f.line(derives);
    f.line(&format!("pub struct {name} {{"));
    f.line("    content_fingerprint: ContentFingerprint,");
    f.line("}");
    f.blank();

    f.line(&format!("impl {name} {{"));
    f.line("    /// Content fingerprint of the witnessed structure.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn content_fingerprint(&self) -> ContentFingerprint {");
    f.line("        self.content_fingerprint");
    f.line("    }");
    f.blank();
    f.line("    /// Crate-internal constructor — only the verify-* primitive in");
    f.line("    /// `crate::primitives::*` may instantiate the witness.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn from_fingerprint(content_fingerprint: ContentFingerprint) -> Self {");
    f.line("        Self { content_fingerprint }");
    f.line("    }");
    f.line("}");
    f.blank();

    // Sealed registration (so Mint{Foo}: Certificate via its sealed
    // supertrait certificate_sealed::Sealed, and OntologyTarget via
    // ontology_target_sealed::Sealed).
    f.line(&format!(
        "impl crate::enforcement::certificate_sealed::Sealed for {name} {{}}"
    ));
    f.line(&format!(
        "impl crate::enforcement::ontology_target_sealed::Sealed for {name} {{}}"
    ));
    f.line(&format!("impl OntologyTarget for {name} {{}}"));
    f.blank();

    // Certificate impl — `const IRI` + `type Evidence = ()` per the
    // existing trait shape.
    f.line(&format!("impl Certificate for {name} {{"));
    f.line(&format!(
        "    const IRI: &'static str = \"{}\";",
        e.class_iri
    ));
    f.line("    type Evidence = ();");
    f.line("}");
    f.blank();

    // Mint{Foo}Inputs<H> — Phase 10 placeholder. Phase 12 will replace
    // PhantomData with the real per-class field mapping per R5.
    f.doc_comment(&format!(
        "Inputs to `{name}::ontology_mint`. Phase 10 placeholder — Phase \
         12 will populate per-property fields per R5 when the verify body \
         is filled in."
    ));
    f.line("#[derive(Debug, Clone, Copy)]");
    f.line(&format!("pub struct {inputs}<H: HostTypes> {{"));
    f.line("    /// Phase-10 placeholder. Phase 12 replaces with the real");
    f.line("    /// per-property fields (object props → `{Range}Handle<H>`,");
    f.line("    /// datatype props → `H::Decimal` / `u64` / `bool` / `&'static str`).");
    f.line("    pub _phantom: PhantomData<H>,");
    f.line("}");
    f.blank();

    f.line(&format!("impl<H: HostTypes> Default for {inputs}<H> {{"));
    f.line("    #[inline]");
    f.line("    fn default() -> Self {");
    f.line("        Self { _phantom: PhantomData }");
    f.line("    }");
    f.line("}");
    f.blank();

    // OntologyVerifiedMint impl.
    f.line(&format!("impl OntologyVerifiedMint for {name} {{"));
    f.line(&format!("    type Inputs<H: HostTypes> = {inputs}<H>;"));
    f.line(&format!(
        "    const THEOREM_IDENTITY: &'static str = \"{}\";",
        e.theorem_identity
    ));
    f.blank();
    f.line("    #[inline]");
    f.line("    fn ontology_mint<H: HostTypes>(");
    f.line("        inputs: Self::Inputs<H>,");
    f.line("    ) -> Result<Self, GenericImpossibilityWitness> {");
    f.line(&format!(
        "        crate::primitives::{}::verify_{}::<H>(inputs)",
        e.primitive_module, e.verify_ident
    ));
    f.line("    }");
    f.line("}");
    f.blank();

    // Marker constant — used by tests, public-API surface, and the
    // Phase-12 verify body's stub return.
    f.line(&format!(
        "/// `WITNESS_UNIMPLEMENTED_STUB:{}` marker recorded by the Phase-10 stub.",
        e.class_iri
    ));
    f.line(&format!(
        "pub const {}_STUB_MARKER: &str = \"{}\";",
        name.to_uppercase(),
        stub_marker
    ));
    f.blank();
}

/// Generates the `foundation/src/primitives/` module tree.
///
/// Each module hosts the family's stubbed `verify_*` primitives — Phase
/// 10 close emits one stub per Path-2 class, returning
/// `Err(GenericImpossibilityWitness::for_identity(IRI))`. Phase 12
/// replaces each stub with a real verification body.
///
/// Returns a vector of `(relative_path, content)` pairs.
#[must_use]
pub fn generate_primitives_modules(ontology: &Ontology) -> Vec<(String, String)> {
    let emissions = path2_emissions(ontology);
    let by_module = primitive_stub_groups(&emissions);
    let mut out: Vec<(String, String)> = Vec::new();

    // mod.rs — declares submodules in alphabetical order.
    let mut mod_rs = String::with_capacity(1024);
    mod_rs.push_str("// @generated by uor-crate from uor-ontology — do not edit manually\n\n");
    mod_rs.push_str(
        "//! Phase 10 / 12 — per-family verify primitives for Path-2 witnesses.\n\
         //!\n\
         //! Each submodule hosts the stubbed (Phase 10) or real (Phase 12)\n\
         //! `verify_*` primitives that back the\n\
         //! `OntologyVerifiedMint::ontology_mint` impls in\n\
         //! `crate::witness_scaffolds`. Stub bodies return\n\
         //! `Err(GenericImpossibilityWitness::for_identity(\"WITNESS_UNIMPLEMENTED_STUB:{IRI}\"))`.\n\n",
    );
    for module in by_module.keys() {
        mod_rs.push_str(&format!("pub mod {module};\n"));
    }
    out.push(("primitives/mod.rs".to_string(), mod_rs));

    for (module, ems) in &by_module {
        let mut content = String::with_capacity(2048);
        content.push_str("// @generated by uor-crate from uor-ontology — do not edit manually\n\n");
        content.push_str(&format!(
            "//! Phase 10 stubs for the `{module}` theorem family.\n\
             //!\n\
             //! Each `verify_*` is a Phase-10 placeholder returning\n\
             //! `Err(GenericImpossibilityWitness::for_identity(\"WITNESS_UNIMPLEMENTED_STUB:{{IRI}}\"))`.\n\
             //! Phase 12 replaces each body with the real verification.\n\n",
        ));
        content.push_str(
            "use crate::enforcement::GenericImpossibilityWitness;\n\
             use crate::HostTypes;\n\
             use crate::witness_scaffolds::{",
        );

        // Import every Mint{Foo} + Mint{Foo}Inputs that this module
        // provides verify functions for.
        let mut imports: Vec<String> = Vec::new();
        for e in ems {
            let mint = mint_struct_name(e);
            imports.push(format!("{mint}Inputs"));
            imports.push(mint);
        }
        imports.sort();
        imports.dedup();
        for (i, name) in imports.iter().enumerate() {
            if i > 0 {
                content.push_str(", ");
            }
            content.push_str(name);
        }
        content.push_str("};\n\n");

        for e in ems {
            let mint = mint_struct_name(e);
            let inputs = format!("{mint}Inputs");
            let _ = std::fmt::Write::write_fmt(
                &mut content,
                format_args!(
                    "/// Phase-10 stub for `{}`.\n\
                     ///\n\
                     /// # Errors\n\
                     ///\n\
                     /// Always returns `WITNESS_UNIMPLEMENTED_STUB:{}` until\n\
                     /// Phase 12 fills the body.\n\
                     pub fn verify_{}<H: HostTypes>(\n\
                     \x20   inputs: {}<H>,\n\
                     ) -> Result<{}, GenericImpossibilityWitness> {{\n\
                     \x20   let _ = inputs;\n\
                     \x20   Err(GenericImpossibilityWitness::for_identity(\n\
                     \x20       \"WITNESS_UNIMPLEMENTED_STUB:{}\",\n\
                     \x20   ))\n\
                     }}\n\n",
                    e.theorem_identity, e.class_iri, e.verify_ident, inputs, mint, e.class_iri,
                ),
            );
        }
        out.push((format!("primitives/{module}.rs"), content));
    }

    out
}

/// Returns the Path-2 class local names + their resolved theorem
/// identities for use by tests and the witness_scaffold_emission
/// validator.
#[must_use]
pub fn path2_summary(ontology: &Ontology) -> Vec<(String, String, String, bool)> {
    path2_emissions(ontology)
        .into_iter()
        .map(|e| {
            (
                e.class_local,
                e.theorem_identity,
                e.primitive_module,
                e.entropy_bearing,
            )
        })
        .collect()
}
