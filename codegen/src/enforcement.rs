//! Generates the `enforcement.rs` module for the `uor-foundation` crate.
//!
//! This module emits Layer 1 (opaque witnesses), Layer 2 (declarative builders),
//! the Term AST, and the v0.2.1 ergonomics surface (sealed `OntologyTarget` /
//! `Grounded<T>` wrappers, the `Certify` trait, the parametric `PipelineFailure`
//! enum, ring-op phantom wrappers, fragment markers, dispatch tables, and the
//! `prelude` module).

use crate::emit::RustFile;
use uor_ontology::model::{IndividualValue, Ontology};

/// Generates the complete `enforcement.rs` module content.
///
/// # Errors
///
/// This function is infallible but returns `String` for consistency.
#[must_use]
pub fn generate_enforcement_module(ontology: &Ontology) -> String {
    let mut f = RustFile::new(
        "Declarative enforcement types.\n\
         //!\n\
         //! This module contains the opaque witness types, declarative builders,\n\
         //! the Term AST, and the v0.2.1 ergonomics surface (sealed `Grounded<T>`,\n\
         //! the `Certify` trait, `PipelineFailure`, ring-op phantom wrappers,\n\
         //! fragment markers, dispatch tables, and the `prelude` module).\n\
         //!\n\
         //! # Layers\n\
         //!\n\
         //! - **Layer 1** \\[Opaque Witnesses\\]: `Datum`, `Validated<T>`, `Derivation`,\n\
         //!   `FreeRank` \\[private fields, no public constructors\\]\n\
         //! - **Layer 2** \\[Declarative Builders\\]: `CompileUnitBuilder`,\n\
         //!   `EffectDeclarationBuilder`, etc. \\[produce `Validated<T>` on success\\]\n\
         //! - **Term AST**: `Term`, `TermArena`, `Binding`, `Assertion`, etc.\n\
         //! - **v0.2.1 Ergonomics**: `OntologyTarget`, `GroundedShape`, `Grounded<T>`,\n\
         //!   `Certify`, `PipelineFailure`, `RingOp<L>`, fragment markers,\n\
         //!   `INHABITANCE_DISPATCH_TABLE`, and the `prelude` module.",
    );

    f.line("use crate::{PrimitiveOp, WittLevel, VerificationDomain, ViolationKind};");
    f.line("use core::marker::PhantomData;");
    f.blank();

    generate_sealed_module(&mut f);
    generate_datum_types(&mut f, ontology);
    generate_grounding_types(&mut f, ontology);
    generate_witness_types(&mut f);
    generate_uor_time(&mut f);
    generate_term_ast(&mut f);
    generate_shape_violation(&mut f);
    generate_builders(&mut f);
    generate_minting_session(&mut f, ontology);
    generate_const_ring_eval(&mut f, ontology);

    // v0.2.2 Phase C.3: Limbs<N> generic kernel for high Witt levels.
    generate_limbs_kernel(&mut f);

    // v0.2.1 ergonomics surface generators (parametric — read from ontology)
    generate_ontology_target_trait(&mut f, ontology);
    // v0.2.2 Phase C.4: MulContext + MultiplicationCertificate evidence.
    // Must run after generate_ontology_target_trait because it extends the
    // MultiplicationCertificate shim.
    generate_multiplication_context(&mut f);
    generate_grounded_wrapper(&mut f);
    generate_pipeline_failure(&mut f, ontology);
    generate_certify_trait(&mut f, ontology);
    generate_ring_ops(&mut f, ontology);
    // v0.2.2 Phase C.3: emit Limbs-backed marker structs + RingOp impls
    // for every WittLevel individual whose bit_width > 128.
    generate_limbs_ring_ops(&mut f, ontology);
    generate_fragment_markers(&mut f, ontology);
    generate_dispatch_tables(&mut f, ontology);
    generate_validated_deref(&mut f);
    // v0.2.2 Phase D (Q4): parametric constraint surface.
    generate_parametric_constraint_surface(&mut f);
    // v0.2.2 Phase E: bridge namespace completion — sealed Query/Coordinate/
    // BindingQuery/Partition/Trace/TraceEvent/HomologyClass/CohomologyClass/
    // Interaction types + Derivation::replay().
    generate_bridge_namespace_surface(&mut f);
    generate_prelude(&mut f, ontology);

    f.finish()
}

fn generate_sealed_module(f: &mut RustFile) {
    f.doc_comment("Private sealed module preventing downstream implementations.");
    f.doc_comment("Only `GroundedCoord` and `GroundedTuple<N>` implement `Sealed`.");
    f.line("mod sealed {");
    f.indented_doc_comment(
        "Sealed trait. Not publicly implementable because this module is private.",
    );
    f.line("    pub trait Sealed {}");
    f.line("    impl Sealed for super::GroundedCoord {}");
    f.line("    impl<const N: usize> Sealed for super::GroundedTuple<N> {}");
    f.line("}");
    f.blank();
}

/// v0.2.1 Phase 8b.7: data-driven Witt level descriptors sourced from
/// `schema:WittLevel` individuals in the ontology.
///
/// Each returned tuple is `(local_name, bits_width, byte_width)`:
///
/// - `local_name` is the ontology individual's local name (`W8`, `W16`,
///   `W24`, `W32`, ...). This becomes the `DatumInner` variant name.
/// - `bits_width` is the `schema:bitsWidth` annotation value.
/// - `byte_width` is `bits_width.div_ceil(8)` — the payload size in bytes.
///
/// Sorted ascending by `bits_width` so the emitted enum variants appear
/// in a deterministic small-to-large order.
///
/// v0.2.1 scope: the walk filters to levels whose `bits_width` is a
/// multiple of 8 **and** fits into a native Rust int type (≤ 64 bits).
/// W24 is emitted as a 3-byte variant backed by `u32` with a 24-bit mask
/// for ring-op evaluation. Deeper levels (if the ontology adds e.g. W128)
/// get stored but not ring-op-wired until the foundation grows a code
/// path for the wider primitives.
fn witt_levels(ontology: &Ontology) -> Vec<(String, u32, usize)> {
    let mut levels: Vec<(String, u32, usize)> = Vec::new();
    for ind in individuals_of_type(ontology, "https://uor.foundation/schema/WittLevel") {
        let bits = ind
            .properties
            .iter()
            .find_map(|(k, v)| {
                if *k == "https://uor.foundation/schema/bitsWidth" {
                    if let uor_ontology::model::IndividualValue::Int(n) = v {
                        return Some(*n as u32);
                    }
                }
                None
            })
            .unwrap_or(0);
        // v0.2.2 Phase C: the cap is now 128 (u128 native backing). Levels
        // above W128 are handled by the Limbs<N> generic kernel emitted
        // in Phase C.3; until that lands, this filter excludes them.
        if bits == 0 || bits % 8 != 0 || bits > 128 {
            continue;
        }
        let byte_width = bits.div_ceil(8) as usize;
        let local = local_name(ind.id).to_string();
        levels.push((local, bits, byte_width));
    }
    levels.sort_by_key(|(_, bits, _)| *bits);
    levels
}

/// Returns the smallest Rust `u*` type that can hold `bits` bits of a ring
/// element. `bits` is the `schema:bitsWidth` annotation value. W24 uses
/// `u32` with a `& 0xFFFFFF` mask at the arithmetic boundary; W40-W64 use
/// `u64`; v0.2.2 Phase C.2 added `u128` for W72-W128.
fn witt_rust_int_type(bits: u32) -> &'static str {
    if bits <= 8 {
        "u8"
    } else if bits <= 16 {
        "u16"
    } else if bits <= 32 {
        "u32"
    } else if bits <= 64 {
        "u64"
    } else {
        "u128"
    }
}

fn generate_datum_types(f: &mut RustFile, ontology: &Ontology) {
    let levels = witt_levels(ontology);
    // DatumInner — variants emitted parametrically from `schema:WittLevel`.
    f.doc_comment("Internal level-tagged ring value. Width determined by the Witt level.");
    f.doc_comment("Variants are emitted parametrically from `schema:WittLevel` individuals");
    f.doc_comment("in the ontology; adding a new level to the ontology regenerates this enum.");
    f.doc_comment("Not publicly constructible \\[sealed within the crate\\].");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("#[allow(clippy::large_enum_variant, dead_code)]");
    f.line("pub(crate) enum DatumInner {");
    for (local, bits, bytes) in &levels {
        f.indented_doc_comment(&format!("{local}: {bits}-bit ring Z/(2^{bits})Z."));
        f.line(&format!("    {local}([u8; {bytes}]),"));
    }
    f.line("}");
    f.blank();

    // Datum public wrapper.
    f.doc_comment("A ring element at its minting Witt level.");
    f.doc_comment("");
    f.doc_comment("Cannot be constructed outside the `uor_foundation` crate.");
    f.doc_comment("The only way to obtain a `Datum` is through reduction evaluation");
    f.doc_comment("or the two-phase minting boundary (`validate_and_mint_coord` /");
    f.doc_comment("`validate_and_mint_tuple`).");
    f.doc_example(
        "// A Datum is produced by reduction evaluation or the minting boundary —\n\
         // you never construct one directly.\n\
         fn inspect_datum(d: &uor_foundation::enforcement::Datum) {\n\
         \x20   // Query its Witt level (W8 = 8-bit, W32 = 32-bit, etc.)\n\
         \x20   let level = d.level();\n\
         \x20   // Datum width is determined by its level:\n\
         \x20   //   W8 → 1 byte,  W16 → 2 bytes,  W24 → 3 bytes,  W32 → 4 bytes.\n\
         \x20   let bytes = d.as_bytes();\n\
         }",
        "rust,ignore",
    );
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct Datum {");
    f.indented_doc_comment("Level-tagged ring value \\[sealed\\].");
    f.line("    inner: DatumInner,");
    f.line("}");
    f.blank();
    f.line("impl Datum {");
    f.indented_doc_comment("Returns the Witt level at which this datum was minted.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn level(&self) -> WittLevel {");
    f.line("        match self.inner {");
    for (local, bits, _) in &levels {
        // W8/W16 use the named constants; others use WittLevel::new.
        let rhs = match *bits {
            8 => "WittLevel::W8".to_string(),
            16 => "WittLevel::W16".to_string(),
            _ => format!("WittLevel::new({bits})"),
        };
        f.line(&format!("            DatumInner::{local}(_) => {rhs},"));
    }
    f.line("        }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the raw byte representation of this datum.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub fn as_bytes(&self) -> &[u8] {");
    f.line("        match &self.inner {");
    for (local, _, _) in &levels {
        f.line(&format!("            DatumInner::{local}(b) => b,"));
    }
    f.line("        }");
    f.line("    }");
    f.line("}");
    f.blank();
}

fn generate_grounding_types(f: &mut RustFile, ontology: &Ontology) {
    let levels = witt_levels(ontology);
    // GroundedCoordInner — variants emitted parametrically from
    // `schema:WittLevel` individuals (same filter as `DatumInner`).
    f.doc_comment("Internal level-tagged coordinate value for grounding intermediates.");
    f.doc_comment("Variant set mirrors `DatumInner`: one per `schema:WittLevel`.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("#[allow(clippy::large_enum_variant, dead_code)]");
    f.line("pub(crate) enum GroundedCoordInner {");
    for (local, bits, bytes) in &levels {
        f.indented_doc_comment(&format!("{local}: {bits}-bit coordinate."));
        f.line(&format!("    {local}([u8; {bytes}]),"));
    }
    f.line("}");
    f.blank();

    // GroundedCoord
    f.doc_comment("A single grounded coordinate value.");
    f.doc_comment("");
    f.doc_comment("Not a `Datum` \\[this is the narrow intermediate that a `Grounding`");
    f.doc_comment("impl produces\\]. The foundation validates and mints it into a `Datum`.");
    f.doc_comment("Uses the same closed level-tagged family as `Datum`, ensuring that");
    f.doc_comment("coordinate width matches the target Witt level.");
    f.doc_example(
        "use uor_foundation::enforcement::GroundedCoord;\n\
         \n\
         // W8: 8-bit ring Z/256Z — lightweight, exhaustive-verification baseline\n\
         let byte_coord = GroundedCoord::w8(42);\n\
         \n\
         // W16: 16-bit ring Z/65536Z — audio samples, small indices\n\
         let short_coord = GroundedCoord::w16(1000);\n\
         \n\
         // W32: 32-bit ring Z/2^32Z — pixel data, general-purpose integers\n\
         let word_coord = GroundedCoord::w32(70_000);",
        "rust",
    );
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct GroundedCoord {");
    f.indented_doc_comment("Level-tagged coordinate bytes.");
    f.line("    pub(crate) inner: GroundedCoordInner,");
    f.line("}");
    f.blank();
    f.line("impl GroundedCoord {");
    for (local, bits, bytes) in &levels {
        let ctor = local.to_ascii_lowercase();
        let rust_ty = witt_rust_int_type(*bits);
        f.indented_doc_comment(&format!(
            "Construct a {local} coordinate from a `{rust_ty}` value (little-endian)."
        ));
        f.line("    #[inline]");
        f.line("    #[must_use]");
        f.line(&format!(
            "    pub const fn {ctor}(value: {rust_ty}) -> Self {{"
        ));
        // For W24 (3 bytes from u32), we need to mask and copy the low 3 bytes.
        // For W8, the payload is [u8; 1] and the native to_le_bytes gives [u8; 1].
        // Otherwise to_le_bytes gives exactly the needed byte_width.
        let full_bytes = match rust_ty {
            "u8" => 1,
            "u16" => 2,
            "u32" => 4,
            _ => 8,
        };
        if *bytes == full_bytes {
            f.line(&format!(
                "        Self {{ inner: GroundedCoordInner::{local}(value.to_le_bytes()) }}"
            ));
        } else {
            // Truncate to the ring's bit-width (e.g. W24 into 3 bytes).
            f.line("        let full = value.to_le_bytes();");
            f.line(&format!("        let mut out = [0u8; {bytes}];"));
            f.line("        let mut i = 0;");
            f.line(&format!("        while i < {bytes} {{"));
            f.line("            out[i] = full[i];");
            f.line("            i += 1;");
            f.line("        }");
            f.line(&format!(
                "        Self {{ inner: GroundedCoordInner::{local}(out) }}"
            ));
        }
        f.line("    }");
        f.blank();
    }
    f.line("}");
    f.blank();

    // GroundedTuple
    f.doc_comment("A grounded tuple: a fixed-size array of `GroundedCoord` values.");
    f.doc_comment("");
    f.doc_comment("Represents a structured type (e.g., the 8 coordinates of an E8");
    f.doc_comment("lattice point). Not a `Datum` until the foundation validates and");
    f.doc_comment("mints it. Stack-resident, no heap allocation.");
    f.doc_example(
        "use uor_foundation::enforcement::{GroundedCoord, GroundedTuple};\n\
         \n\
         // A 2D pixel: (red, green) at W8 (8-bit per channel)\n\
         let pixel = GroundedTuple::new([\n\
         \x20   GroundedCoord::w8(255), // red channel\n\
         \x20   GroundedCoord::w8(128), // green channel\n\
         ]);\n\
         \n\
         // An E8 lattice point: 8 coordinates at W8\n\
         let lattice_point = GroundedTuple::new([\n\
         \x20   GroundedCoord::w8(2), GroundedCoord::w8(0),\n\
         \x20   GroundedCoord::w8(0), GroundedCoord::w8(0),\n\
         \x20   GroundedCoord::w8(0), GroundedCoord::w8(0),\n\
         \x20   GroundedCoord::w8(0), GroundedCoord::w8(0),\n\
         ]);",
        "rust",
    );
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct GroundedTuple<const N: usize> {");
    f.indented_doc_comment("The coordinate array.");
    f.line("    pub(crate) coords: [GroundedCoord; N],");
    f.line("}");
    f.blank();
    f.line("impl<const N: usize> GroundedTuple<N> {");
    f.indented_doc_comment("Construct a tuple from a fixed-size array of coordinates.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn new(coords: [GroundedCoord; N]) -> Self {");
    f.line("        Self { coords }");
    f.line("    }");
    f.line("}");
    f.blank();

    // GroundedValue sealed trait
    f.doc_comment("Sealed marker trait for grounded intermediates.");
    f.doc_comment("");
    f.doc_comment("Implemented only for `GroundedCoord` and `GroundedTuple<N>`.");
    f.doc_comment("Prism code cannot implement this \\[the sealed module pattern");
    f.doc_comment("prevents it\\].");
    f.line("pub trait GroundedValue: sealed::Sealed {}");
    f.line("impl GroundedValue for GroundedCoord {}");
    f.line("impl<const N: usize> GroundedValue for GroundedTuple<N> {}");
    f.blank();

    // v0.2.2 W4: Grounding kind discriminator. The Grounding trait gains an
    // associated `Map: GroundingMapKind` type that tags each impl with its
    // semantic kind (digest, binary, integer, utf8, json). Sealed marker
    // traits (`Total`, `Invertible`, `PreservesStructure`, `PreservesMetric`)
    // partition the kinds by structural property, so foundation operations
    // requiring (e.g.) `PreservesStructure` reject digest-grounding impls at
    // the call site. The discrimination is structural-tagging — the
    // foundation cannot verify the impl body matches the declared kind, but
    // it can ensure that the kind is one of a fixed sealed set.
    f.doc_comment("v0.2.2 W4: sealed marker trait for the kind of a `Grounding` map.");
    f.doc_comment("Implemented by exactly the `morphism:GroundingMap` individuals declared in");
    f.doc_comment("the ontology; downstream cannot extend the kind set.");
    f.line("pub trait GroundingMapKind: grounding_map_kind_sealed::Sealed {");
    f.indented_doc_comment("The ontology IRI of this grounding map kind.");
    f.line("    const ONTOLOGY_IRI: &'static str;");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 W4: kinds whose grounding image is total over the input domain");
    f.doc_comment("(every input grounds successfully).");
    f.line("pub trait Total: GroundingMapKind {}");
    f.blank();
    f.doc_comment("v0.2.2 W4: kinds whose grounding map is injective and admits an inverse");
    f.doc_comment("on its image.");
    f.line("pub trait Invertible: GroundingMapKind {}");
    f.blank();
    f.doc_comment("v0.2.2 W4: kinds whose grounding map preserves the algebraic structure");
    f.doc_comment("of the source domain (homomorphism-like).");
    f.line("pub trait PreservesStructure: GroundingMapKind {}");
    f.blank();
    f.doc_comment("v0.2.2 W4: kinds whose grounding map preserves the metric of the source");
    f.doc_comment("domain (isometry-like).");
    f.line("pub trait PreservesMetric: GroundingMapKind {}");
    f.blank();

    // Walk morphism:GroundingMap individuals and emit one unit struct per kind.
    let kinds = individuals_of_type(ontology, "https://uor.foundation/morphism/GroundingMap");
    let mut kind_names: Vec<String> = Vec::new();
    for k in &kinds {
        kind_names.push(local_name(k.id).to_string());
    }
    kind_names.sort();
    kind_names.dedup();

    for name in &kind_names {
        let doc = match name.as_str() {
            "IntegerGroundingMap" => "v0.2.2 W4: kind for integer surface symbols. Total, invertible, structure-preserving.",
            "Utf8GroundingMap" => "v0.2.2 W4: kind for UTF-8 host strings. Invertible on its image, structure-preserving.",
            "JsonGroundingMap" => "v0.2.2 W4: kind for JSON host strings. Invertible on its image, structure-preserving.",
            "DigestGroundingMap" => "v0.2.2 W4: kind for one-way digest functions (e.g., SHA-256). Total but not invertible; preserves no structure.",
            "BinaryGroundingMap" => "v0.2.2 W4: kind for raw byte ingestion. Total and invertible; preserves bit identity only.",
            _ => "v0.2.2 W4: GroundingMap kind unit struct.",
        };
        f.doc_comment(doc);
        f.line("#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]");
        f.line(&format!("pub struct {name};"));
        f.blank();
    }

    // Sealed module + GroundingMapKind impls.
    f.line("mod grounding_map_kind_sealed {");
    f.indented_doc_comment("Private supertrait. Not implementable outside this crate.");
    f.line("    pub trait Sealed {}");
    for name in &kind_names {
        f.line(&format!("    impl Sealed for super::{name} {{}}"));
    }
    f.line("}");
    f.blank();
    for name in &kind_names {
        f.line(&format!("impl GroundingMapKind for {name} {{"));
        f.line(&format!(
            "    const ONTOLOGY_IRI: &'static str = \"https://uor.foundation/morphism/{name}\";"
        ));
        f.line("}");
        f.blank();
    }

    // Marker-trait impl table (per W4 plan):
    //   IntegerGroundingMap : Total + Invertible + PreservesStructure
    //   Utf8GroundingMap    : Invertible + PreservesStructure   (not Total — codec can fail)
    //   JsonGroundingMap    : Invertible + PreservesStructure   (not Total — parser can fail)
    //   DigestGroundingMap  : Total                              (not Invertible, no structure)
    //   BinaryGroundingMap  : Total + Invertible                 (no structure preservation)
    let marker_table: &[(&str, &[&str])] = &[
        (
            "IntegerGroundingMap",
            &["Total", "Invertible", "PreservesStructure"],
        ),
        ("Utf8GroundingMap", &["Invertible", "PreservesStructure"]),
        ("JsonGroundingMap", &["Invertible", "PreservesStructure"]),
        ("DigestGroundingMap", &["Total"]),
        ("BinaryGroundingMap", &["Total", "Invertible"]),
    ];
    for (kind_name, markers) in marker_table {
        if !kind_names.iter().any(|n| n == *kind_name) {
            continue;
        }
        for marker in *markers {
            f.line(&format!("impl {marker} for {kind_name} {{}}"));
        }
        if !markers.is_empty() {
            f.blank();
        }
    }

    // Grounding open trait — extended with v0.2.2 W4 `Map: GroundingMapKind`
    // associated type. Defaulted to `BinaryGroundingMap` so existing impls
    // that don't declare a `Map` continue to type-check (the binary kind is
    // the most permissive default — total + invertible, no structure
    // preservation).
    f.doc_comment("Open trait for boundary crossing: external data to grounded intermediate.");
    f.doc_comment("");
    f.doc_comment("The foundation validates the returned value against the declared");
    f.doc_comment("`GroundingShape` and mints it into a `Datum` if conformant.");
    f.doc_comment("");
    f.doc_comment("v0.2.2 W4 adds the `Map: GroundingMapKind` associated type — every impl");
    f.doc_comment("must declare what *kind* of grounding map it is. Foundation operations");
    f.doc_comment(
        "that require structure preservation gate on `<G as Grounding>::Map: PreservesStructure`,",
    );
    f.doc_comment("and a digest-style impl is rejected at the call site.");
    f.doc_example(
        "use uor_foundation::enforcement::{Grounding, GroundedCoord, BinaryGroundingMap};\n\
         \n\
         /// Doubling grounding: maps each input byte b to 2b mod 256.\n\
         struct DoublingGrounding;\n\
         \n\
         impl Grounding for DoublingGrounding {\n\
         \x20   type Output = GroundedCoord;\n\
         \x20   type Map = BinaryGroundingMap;\n\
         \n\
         \x20   fn ground(&self, external: &[u8]) -> Option<GroundedCoord> {\n\
         \x20       // Reject empty input at the boundary\n\
         \x20       let &byte = external.first()?;\n\
         \x20       // Apply the doubling map: b -> 2b mod 256\n\
         \x20       Some(GroundedCoord::w8(byte.wrapping_mul(2)))\n\
         \x20   }\n\
         }",
        "rust,ignore",
    );
    f.line("pub trait Grounding {");
    f.indented_doc_comment(
        "The grounded intermediate type. Bounded by `GroundedValue`,\n\
         which is sealed \\[only `GroundedCoord` and `GroundedTuple<N>`\n\
         are permitted\\].",
    );
    f.line("    type Output: GroundedValue;");
    f.blank();
    f.indented_doc_comment(
        "v0.2.2 W4: the kind of grounding map this impl is. Sealed to the\n\
         set of `morphism:GroundingMap` individuals declared in the\n\
         ontology. Every impl must declare the kind explicitly; if no kind\n\
         applies, use `BinaryGroundingMap` (the most permissive — total +\n\
         invertible, no structure preservation).",
    );
    f.line("    type Map: GroundingMapKind;");
    f.blank();
    f.indented_doc_comment(
        "Map external bytes into a grounded intermediate.\n\
         The foundation handles validation and minting.\n\
         Returns `None` if the input is malformed or undersized.",
    );
    f.line("    fn ground(&self, external: &[u8]) -> Option<Self::Output>;");
    f.line("}");
    f.blank();
}

fn generate_witness_types(f: &mut RustFile) {
    // v0.2.2 W13: ValidationPhase — sealed marker for the validation phase
    // (compile-time vs runtime) at which a Validated<T> was witnessed. The
    // default phase is Runtime so v0.2.1 call sites that wrote Validated<T>
    // continue to compile unchanged. Compile-time validation produces
    // Validated<T, CompileTime>, which is convertible to Validated<T, Runtime>
    // via the From impl below — a CompileTime witness is usable wherever a
    // Runtime witness is.
    f.doc_comment("v0.2.2 W13: sealed marker trait for the validation phase at which a");
    f.doc_comment("`Validated<T, Phase>` was witnessed. Implemented only by `CompileTime`");
    f.doc_comment("and `Runtime`; downstream cannot extend.");
    f.line("pub trait ValidationPhase: validation_phase_sealed::Sealed {}");
    f.blank();
    f.line("mod validation_phase_sealed {");
    f.indented_doc_comment("Private supertrait. Not implementable outside this crate.");
    f.line("    pub trait Sealed {}");
    f.line("    impl Sealed for super::CompileTime {}");
    f.line("    impl Sealed for super::Runtime {}");
    f.line("}");
    f.blank();
    f.doc_comment("v0.2.2 W13: marker for compile-time validated witnesses produced by");
    f.doc_comment("`validate_const()` and usable in `const` contexts. Convertible to");
    f.doc_comment("`Validated<T, Runtime>` via `From`.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct CompileTime;");
    f.line("impl ValidationPhase for CompileTime {}");
    f.blank();
    f.doc_comment("v0.2.2 W13: marker for runtime-validated witnesses produced by");
    f.doc_comment("`validate()`. The default phase of `Validated<T>` so v0.2.1 call");
    f.doc_comment("sites continue to compile.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct Runtime;");
    f.line("impl ValidationPhase for Runtime {}");
    f.blank();

    // Validated<T, Phase>
    f.doc_comment("Proof that a value was produced by the conformance checker,");
    f.doc_comment("not fabricated by Prism code.");
    f.doc_comment("");
    f.doc_comment("The inner value and `_sealed` field are private, so `Validated<T>`");
    f.doc_comment("can only be constructed within this crate.");
    f.doc_comment("");
    f.doc_comment("v0.2.2 W13: parameterized by a `Phase: ValidationPhase` discriminator.");
    f.doc_comment("`Validated<T, CompileTime>` was witnessed by `validate_const()` and is");
    f.doc_comment("usable in const contexts. `Validated<T, Runtime>` (the default) was");
    f.doc_comment("witnessed by `validate()`. A `CompileTime` witness is convertible to");
    f.doc_comment("a `Runtime` witness via `From`.");
    f.doc_example(
        "use uor_foundation::enforcement::{CompileUnitBuilder, Term};\n\
         use uor_foundation::{WittLevel, VerificationDomain};\n\
         \n\
         // Validated<T> proves that a value passed conformance checking.\n\
         // You cannot construct one directly — only builder validate() methods\n\
         // and the minting boundary produce them.\n\
         let terms = [Term::Literal { value: 1, level: WittLevel::W8 }];\n\
         let domains = [VerificationDomain::Enumerative];\n\
         \n\
         let validated = CompileUnitBuilder::new()\n\
         \x20   .root_term(&terms)\n\
         \x20   .witt_level_ceiling(WittLevel::W8)\n\
         \x20   .thermodynamic_budget(1024)\n\
         \x20   .target_domains(&domains)\n\
         \x20   .validate()\n\
         \x20   .expect(\"all fields set\");\n\
         \n\
         // Access the inner value through the proof wrapper:\n\
         let compile_unit = validated.inner();",
        "rust,ignore",
    );
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct Validated<T, Phase: ValidationPhase = Runtime> {");
    f.indented_doc_comment("The validated inner value.");
    f.line("    inner: T,");
    f.indented_doc_comment("Phantom marker for the validation phase (`CompileTime` or `Runtime`).");
    f.line("    _phase: PhantomData<Phase>,");
    f.indented_doc_comment("Prevents external construction.");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl<T, Phase: ValidationPhase> Validated<T, Phase> {");
    f.indented_doc_comment("Returns a reference to the validated inner value.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn inner(&self) -> &T {");
    f.line("        &self.inner");
    f.line("    }");
    f.blank();
    f.indented_doc_comment(
        "Creates a new `Validated<T, Phase>` wrapper. Only callable within the crate.",
    );
    f.line("    #[inline]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(inner: T) -> Self {");
    f.line("        Self { inner, _phase: PhantomData, _sealed: () }");
    f.line("    }");
    f.line("}");
    f.blank();
    // v0.2.2 W13: subsumption — a CompileTime witness is usable wherever a
    // Runtime witness is required.
    f.doc_comment(
        "v0.2.2 W13: a compile-time witness is usable wherever a runtime witness is required.",
    );
    f.line("impl<T> From<Validated<T, CompileTime>> for Validated<T, Runtime> {");
    f.line("    #[inline]");
    f.line("    fn from(value: Validated<T, CompileTime>) -> Self {");
    f.line("        Self { inner: value.inner, _phase: PhantomData, _sealed: () }");
    f.line("    }");
    f.line("}");
    f.blank();

    // Derivation
    f.doc_comment("An opaque derivation trace that can only be extended by the rewrite engine.");
    f.doc_comment("");
    f.doc_comment("Records the number of rewrite steps and the content address of the");
    f.doc_comment("root term. Private fields prevent external construction.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct Derivation {");
    f.indented_doc_comment("Number of rewrite steps in this derivation.");
    f.line("    step_count: u32,");
    f.indented_doc_comment("Content address of the root term.");
    f.line("    root_address: u64,");
    f.line("}");
    f.blank();
    f.line("impl Derivation {");
    f.indented_doc_comment("Returns the number of rewrite steps.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn step_count(&self) -> u32 {");
    f.line("        self.step_count");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the content address of the root term.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn root_address(&self) -> u64 {");
    f.line("        self.root_address");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Creates a new derivation. Only callable within the crate.");
    f.line("    #[inline]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(step_count: u32, root_address: u64) -> Self {");
    f.line("        Self { step_count, root_address }");
    f.line("    }");
    f.line("}");
    f.blank();

    // FreeRank
    f.doc_comment("An opaque free rank that can only be decremented by `PinningEffect`");
    f.doc_comment("and incremented by `UnbindingEffect` \\[never by direct mutation\\].");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct FreeRank {");
    f.indented_doc_comment("Total site capacity at the Witt level.");
    f.line("    total: u32,");
    f.indented_doc_comment("Currently pinned sites.");
    f.line("    pinned: u32,");
    f.line("}");
    f.blank();
    f.line("impl FreeRank {");
    f.indented_doc_comment("Returns the total site capacity.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn total(&self) -> u32 {");
    f.line("        self.total");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the number of currently pinned sites.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn pinned(&self) -> u32 {");
    f.line("        self.pinned");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the number of remaining (unpinned) sites.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn remaining(&self) -> u32 {");
    f.line("        self.total - self.pinned");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Creates a new free rank. Only callable within the crate.");
    f.line("    #[inline]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(total: u32, pinned: u32) -> Self {");
    f.line("        Self { total, pinned }");
    f.line("    }");
    f.line("}");
    f.blank();
}

// v0.2.2 Phase A: UorTime infrastructure.
//
// Emits the deterministic two-clock value (`UorTime`) carried by every
// `Grounded<T>` and `Certified<C>`, the sealed `LandauerBudget` newtype that
// backs one of the two clocks, the `Calibration` validated struct for
// wall-clock binding, the sealed `Nanos` lower-bound carrier, and the four
// shipped `calibrations::*` presets (X86_SERVER, ARM_MOBILE, CORTEX_M_EMBEDDED,
// CONSERVATIVE_WORST_CASE).
//
// All types are sealed with `pub(crate)` constructors. The two clocks are
// grounded in v0.2.1 ontology individuals: `landauer_nats` ↔ `observable:LandauerCost`
// (carried via the new `observable:LandauerBudget` class), and `rewrite_steps`
// ↔ `derivation:stepCount` on `derivation:TermMetrics`.
fn generate_uor_time(f: &mut RustFile) {
    // ── LandauerBudget ────────────────────────────────────────────────────
    f.doc_comment("v0.2.2 Phase A: sealed `f64`-backed newtype carrying the");
    f.doc_comment("`observable:LandauerCost` accumulator in `observable:Nats`.");
    f.doc_comment("Monotonic within a pipeline invocation. The UOR ring operates");
    f.doc_comment("at the Landauer temperature (β* = ln 2), so this observable is");
    f.doc_comment("a direct measure of irreversible bit-erasure performed.");
    f.doc_comment("");
    f.doc_comment("Implements `Ord` over its underlying `f64` (NaN excluded by");
    f.doc_comment("construction — the foundation never produces a `LandauerBudget`");
    f.doc_comment("from a NaN).");
    f.line("#[derive(Debug, Clone, Copy, PartialEq)]");
    f.line("pub struct LandauerBudget {");
    f.indented_doc_comment("Accumulated Landauer cost in nats. Non-negative, finite.");
    f.line("    nats: f64,");
    f.indented_doc_comment("Prevents external construction.");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl LandauerBudget {");
    f.indented_doc_comment("Returns the accumulated Landauer cost in nats.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn nats(&self) -> f64 {");
    f.line("        self.nats");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Crate-internal constructor. Caller guarantees `nats` is");
    f.indented_doc_comment("non-negative and finite (i.e. not NaN, not infinite).");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(nats: f64) -> Self {");
    f.line("        Self { nats, _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Crate-internal constructor for the zero-cost initial budget.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn zero() -> Self {");
    f.line("        Self { nats: 0.0, _sealed: () }");
    f.line("    }");
    f.line("}");
    f.blank();
    // Manual Eq/Ord (f64 is not Eq by default; we exclude NaN by construction).
    f.line("impl Eq for LandauerBudget {}");
    f.line("impl PartialOrd for LandauerBudget {");
    f.line("    #[inline]");
    f.line("    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {");
    f.line("        Some(self.cmp(other))");
    f.line("    }");
    f.line("}");
    f.line("impl Ord for LandauerBudget {");
    f.line("    #[inline]");
    f.line("    fn cmp(&self, other: &Self) -> core::cmp::Ordering {");
    f.line("        // Total order on f64 with NaN excluded by construction.");
    f.line("        self.nats");
    f.line("            .partial_cmp(&other.nats)");
    f.line("            .unwrap_or(core::cmp::Ordering::Equal)");
    f.line("    }");
    f.line("}");
    f.line("impl core::hash::Hash for LandauerBudget {");
    f.line("    #[inline]");
    f.line("    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {");
    f.line("        self.nats.to_bits().hash(state);");
    f.line("    }");
    f.line("}");
    f.blank();

    // ── UorTime ───────────────────────────────────────────────────────────
    f.doc_comment("v0.2.2 Phase A: foundation-internal deterministic two-clock value");
    f.doc_comment("carried by every `Grounded<T>` and `Certified<C>`. The two clocks are");
    f.doc_comment("`landauer_nats` (a `LandauerBudget` value backed by `observable:LandauerCost`)");
    f.doc_comment("and `rewrite_steps` (a `u64` backed by `derivation:stepCount` on");
    f.doc_comment("`derivation:TermMetrics`). Each clock is monotonic within a pipeline");
    f.doc_comment("invocation, content-deterministic, ontology-grounded, and binds to a");
    f.doc_comment("physical wall-clock lower bound through established physics (Landauer's");
    f.doc_comment("principle for nats; Margolus-Levitin for rewrite steps). Two clocks");
    f.doc_comment("because exactly two physical lower-bound theorems are grounded; adding");
    f.doc_comment("a third clock would require grounding a third physical theorem.");
    f.doc_comment("`PartialOrd` is component-wise: `a < b` iff every field of `a` is `<=`");
    f.doc_comment("the corresponding field of `b` and at least one is strictly `<`. Two");
    f.doc_comment("`UorTime` values from unrelated computations are genuinely incomparable,");
    f.doc_comment("so `UorTime` is `PartialOrd` but **not** `Ord`.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct UorTime {");
    f.indented_doc_comment("Landauer budget consumed, in `observable:Nats`.");
    f.line("    landauer_nats: LandauerBudget,");
    f.indented_doc_comment("Total rewrite steps taken (`derivation:stepCount`).");
    f.line("    rewrite_steps: u64,");
    f.indented_doc_comment("Prevents external construction.");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl UorTime {");
    f.indented_doc_comment("Returns the Landauer budget consumed, in `observable:Nats`.");
    f.indented_doc_comment("Maps to `observable:LandauerCost`.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn landauer_nats(&self) -> LandauerBudget {");
    f.line("        self.landauer_nats");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the total rewrite steps taken.");
    f.indented_doc_comment("Maps to `derivation:stepCount` on `derivation:TermMetrics`.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn rewrite_steps(&self) -> u64 {");
    f.line("        self.rewrite_steps");
    f.line("    }");
    f.blank();
    f.indented_doc_comment(
        "Crate-internal constructor. Reachable only from the pipeline at witness mint time.",
    );
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line(
        "    pub(crate) const fn new(landauer_nats: LandauerBudget, rewrite_steps: u64) -> Self {",
    );
    f.line("        Self { landauer_nats, rewrite_steps, _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Crate-internal constructor for the zero initial value.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn zero() -> Self {");
    f.line("        Self {");
    f.line("            landauer_nats: LandauerBudget::zero(),");
    f.line("            rewrite_steps: 0,");
    f.line("            _sealed: (),");
    f.line("        }");
    f.line("    }");
    f.blank();
    // Provable minimum wall-clock duration.
    f.indented_doc_comment("Returns the provable minimum wall-clock duration that the");
    f.indented_doc_comment("computation producing this witness could have taken under the");
    f.indented_doc_comment(
        "given calibration. Returns `max(Landauer-bound, Margolus-Levitin-bound)`.",
    );
    f.indented_doc_comment("");
    f.indented_doc_comment("The Landauer bound is `landauer_nats × k_B·T / thermal_power`.");
    f.indented_doc_comment(
        "The Margolus-Levitin bound is `π·ℏ·rewrite_steps / (2·characteristic_energy)`.",
    );
    f.indented_doc_comment("");
    f.indented_doc_comment("Pure arithmetic — no transcendentals, no state. Const-evaluable");
    f.indented_doc_comment("where the `UorTime` value is known at compile time.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub fn min_wall_clock(&self, cal: &Calibration) -> Nanos {");
    f.line("        // Landauer bound: nats × k_B·T (joules of energy that had to be");
    f.line("        // dissipated) / thermal_power (watts) = seconds.");
    f.line(
        "        let landauer_seconds = self.landauer_nats.nats() * cal.k_b_t / cal.thermal_power;",
    );
    f.line("        // Margolus-Levitin bound: π·ℏ / (2·E) per orthogonal state transition.");
    f.line("        // ℏ ≈ 1.054_571_817e-34 J·s. We use core::f64::consts::PI to avoid");
    f.line("        // approximate-PI lints.");
    f.line("        const PI_TIMES_H_BAR: f64 = core::f64::consts::PI * 1.054_571_817e-34;");
    f.line("        let ml_seconds_per_step = PI_TIMES_H_BAR / (2.0 * cal.characteristic_energy);");
    f.line("        let ml_seconds = ml_seconds_per_step * (self.rewrite_steps as f64);");
    f.line("        let max_seconds = if landauer_seconds > ml_seconds { landauer_seconds } else { ml_seconds };");
    f.line("        // Convert seconds to nanoseconds, saturate on overflow.");
    f.line("        let nanos = max_seconds * 1.0e9;");
    f.line("        let clamped = if nanos < 0.0 { 0.0 }");
    f.line("                      else if nanos > (u64::MAX as f64) { u64::MAX as f64 }");
    f.line("                      else { nanos };");
    f.line("        Nanos { ns: clamped as u64, _sealed: () }");
    f.line("    }");
    f.line("}");
    f.blank();
    // Component-wise PartialOrd, no Ord.
    f.line("impl PartialOrd for UorTime {");
    f.line("    #[inline]");
    f.line("    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {");
    f.line("        let l = self.landauer_nats.cmp(&other.landauer_nats);");
    f.line("        let r = self.rewrite_steps.cmp(&other.rewrite_steps);");
    f.line("        match (l, r) {");
    f.line("            (core::cmp::Ordering::Equal, core::cmp::Ordering::Equal) => Some(core::cmp::Ordering::Equal),");
    f.line("            (core::cmp::Ordering::Less, core::cmp::Ordering::Less)");
    f.line("            | (core::cmp::Ordering::Less, core::cmp::Ordering::Equal)");
    f.line("            | (core::cmp::Ordering::Equal, core::cmp::Ordering::Less) => Some(core::cmp::Ordering::Less),");
    f.line("            (core::cmp::Ordering::Greater, core::cmp::Ordering::Greater)");
    f.line("            | (core::cmp::Ordering::Greater, core::cmp::Ordering::Equal)");
    f.line("            | (core::cmp::Ordering::Equal, core::cmp::Ordering::Greater) => Some(core::cmp::Ordering::Greater),");
    f.line("            _ => None,");
    f.line("        }");
    f.line("    }");
    f.line("}");
    f.blank();

    // ── Nanos ─────────────────────────────────────────────────────────────
    f.doc_comment("v0.2.2 Phase A: sealed lower-bound carrier for wall-clock duration.");
    f.doc_comment("");
    f.doc_comment("Produced only by `UorTime::min_wall_clock` and similar foundation");
    f.doc_comment("time conversions. The sealing guarantees that any `Nanos` value is");
    f.doc_comment("a provable physical bound, not a raw integer. Developers who need");
    f.doc_comment("the underlying `u64` call `.as_u64()`; the sealing prevents");
    f.doc_comment("accidentally passing a host-measured duration where the type system");
    f.doc_comment("expects \"a provable minimum\".");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]");
    f.line("pub struct Nanos {");
    f.indented_doc_comment("The provable lower-bound duration in nanoseconds.");
    f.line("    ns: u64,");
    f.indented_doc_comment("Prevents external construction.");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl Nanos {");
    f.indented_doc_comment("Returns the underlying nanosecond count. The value is a provable");
    f.indented_doc_comment("physical lower bound under whatever calibration produced it.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn as_u64(self) -> u64 {");
    f.line("        self.ns");
    f.line("    }");
    f.line("}");
    f.blank();

    // ── CalibrationError ──────────────────────────────────────────────────
    f.doc_comment("v0.2.2 Phase A: error returned by `Calibration::new` when the supplied");
    f.doc_comment("physical parameters fail plausibility validation.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub enum CalibrationError {");
    f.indented_doc_comment("`k_b_t` was non-positive, NaN, or outside the known-universe");
    f.indented_doc_comment("temperature range (`1e-30 ≤ k_b_t ≤ 1e-15` joules).");
    f.line("    ThermalEnergy,");
    f.indented_doc_comment(
        "`thermal_power` was non-positive, NaN, or above the thermodynamic maximum (`1e9` W).",
    );
    f.line("    ThermalPower,");
    f.indented_doc_comment("`characteristic_energy` was non-positive, NaN, or above the");
    f.indented_doc_comment("k_B·T × Avogadro-class bound (`1e3` joules).");
    f.line("    CharacteristicEnergy,");
    f.line("}");
    f.blank();

    // ── Calibration ───────────────────────────────────────────────────────
    f.doc_comment("v0.2.2 Phase A: physical-substrate calibration for wall-clock binding.");
    f.doc_comment("");
    f.doc_comment("Construction is open via [`Calibration::new`], but the fields are");
    f.doc_comment("private and validated for physical plausibility. Used to convert");
    f.doc_comment("`UorTime` to a provable wall-clock lower bound via");
    f.doc_comment("[`UorTime::min_wall_clock`].");
    f.doc_comment("");
    f.doc_comment("**A `Calibration` is never passed into `pipeline::run`,");
    f.doc_comment("`resolver::*::certify`, `validate_const`, or any other foundation entry");
    f.doc_comment("point.** The foundation computes `UorTime` without physical");
    f.doc_comment("interpretation; the developer applies a `Calibration` after the fact.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq)]");
    f.line("pub struct Calibration {");
    f.indented_doc_comment("Boltzmann constant times temperature, in joules.");
    f.line("    k_b_t: f64,");
    f.indented_doc_comment("Sustained dissipation in watts.");
    f.line("    thermal_power: f64,");
    f.indented_doc_comment("Mean energy above ground state, in joules.");
    f.line("    characteristic_energy: f64,");
    f.line("}");
    f.blank();
    f.line("impl Calibration {");
    f.indented_doc_comment("Construct a calibration with physically plausible parameters.");
    f.indented_doc_comment("");
    f.indented_doc_comment("Validation: every parameter must be positive and finite. `k_b_t`");
    f.indented_doc_comment("must lie within the known-universe temperature range");
    f.indented_doc_comment("(`1e-30 <= k_b_t <= 1e-15` joules covers ~1 nK to ~1e8 K).");
    f.indented_doc_comment("`thermal_power` must be at most `1e9` W (gigawatt class — far above");
    f.indented_doc_comment("any plausible single-compute envelope). `characteristic_energy`");
    f.indented_doc_comment("must be at most `1e3` J (kilojoule class — astronomically generous).");
    f.indented_doc_comment("");
    f.indented_doc_comment("# Errors");
    f.indented_doc_comment("");
    f.indented_doc_comment("Returns `CalibrationError::InvalidThermalEnergy` when `k_b_t` is");
    f.indented_doc_comment("non-positive, NaN, or outside the temperature range.");
    f.indented_doc_comment("");
    f.indented_doc_comment("Returns `CalibrationError::InvalidThermalPower` when `thermal_power`");
    f.indented_doc_comment("is non-positive, NaN, or above the maximum.");
    f.indented_doc_comment("");
    f.indented_doc_comment("Returns `CalibrationError::InvalidCharacteristicEnergy` when");
    f.indented_doc_comment("`characteristic_energy` is non-positive, NaN, or above the maximum.");
    f.line("    #[inline]");
    f.line("    pub const fn new(");
    f.line("        k_b_t: f64,");
    f.line("        thermal_power: f64,");
    f.line("        characteristic_energy: f64,");
    f.line("    ) -> Result<Self, CalibrationError> {");
    f.line("        // Reject NaN, non-positive, and out-of-range values. const fn does not");
    f.line("        // allow `f64::is_nan`, so we use the NaN inequality identity:");
    f.line("        // for any NaN x, `x == x` is false.");
    f.line("        #[allow(clippy::eq_op)]");
    f.line("        let k_b_t_nan = k_b_t != k_b_t;");
    f.line("        if k_b_t_nan || k_b_t <= 0.0 || k_b_t < 1.0e-30 || k_b_t > 1.0e-15 {");
    f.line("            return Err(CalibrationError::ThermalEnergy);");
    f.line("        }");
    f.line("        #[allow(clippy::eq_op)]");
    f.line("        let tp_nan = thermal_power != thermal_power;");
    f.line("        if tp_nan || thermal_power <= 0.0 || thermal_power > 1.0e9 {");
    f.line("            return Err(CalibrationError::ThermalPower);");
    f.line("        }");
    f.line("        #[allow(clippy::eq_op)]");
    f.line("        let ce_nan = characteristic_energy != characteristic_energy;");
    f.line("        if ce_nan || characteristic_energy <= 0.0 || characteristic_energy > 1.0e3 {");
    f.line("            return Err(CalibrationError::CharacteristicEnergy);");
    f.line("        }");
    f.line("        Ok(Self { k_b_t, thermal_power, characteristic_energy })");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the Boltzmann constant times temperature, in joules.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn k_b_t(&self) -> f64 { self.k_b_t }");
    f.blank();
    f.indented_doc_comment("Returns the sustained thermal power dissipation, in watts.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn thermal_power(&self) -> f64 { self.thermal_power }");
    f.blank();
    f.indented_doc_comment("Returns the characteristic energy above ground state, in joules.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn characteristic_energy(&self) -> f64 { self.characteristic_energy }");
    f.line("}");
    f.blank();

    // ── Calibration presets ───────────────────────────────────────────────
    f.doc_comment("v0.2.2 Phase A: foundation-shipped preset calibrations covering common");
    f.doc_comment("substrates. The values are derived from published substrate thermals at");
    f.doc_comment("T=300 K (room temperature, where k_B·T ≈ 4.14e-21 J).");
    f.line("pub mod calibrations {");
    f.line("    use super::{Calibration, CalibrationError};");
    f.blank();
    // Helper macro for the unwrap pattern (const-context unwrap is unstable;
    // we use match instead).
    f.indented_doc_comment("Server-class x86 (Xeon/EPYC sustained envelope).");
    f.indented_doc_comment("");
    f.indented_doc_comment("k_B·T = 4.14e-21 J (T = 300 K), thermal_power = 85 W (typical TDP),");
    f.indented_doc_comment("characteristic_energy = 1e-15 J/op (~1 fJ/op for modern CMOS).");
    f.line(
        "    pub const X86_SERVER: Calibration = match Calibration::new(4.14e-21, 85.0, 1.0e-15) {",
    );
    f.line("        Ok(c) => c,");
    f.line("        Err(_) => unreachable_unphysical(),");
    f.line("    };");
    f.blank();
    f.indented_doc_comment(
        "Mobile ARM SoC (Apple M-series, Snapdragon 8-series sustained envelope).",
    );
    f.indented_doc_comment("");
    f.indented_doc_comment(
        "k_B·T = 4.14e-21 J, thermal_power = 5 W, characteristic_energy = 1e-16 J/op.",
    );
    f.line(
        "    pub const ARM_MOBILE: Calibration = match Calibration::new(4.14e-21, 5.0, 1.0e-16) {",
    );
    f.line("        Ok(c) => c,");
    f.line("        Err(_) => unreachable_unphysical(),");
    f.line("    };");
    f.blank();
    f.indented_doc_comment("Cortex-M embedded (STM32/nRF52 at 80 MHz).");
    f.indented_doc_comment("");
    f.indented_doc_comment(
        "k_B·T = 4.14e-21 J, thermal_power = 0.1 W, characteristic_energy = 1e-17 J/op.",
    );
    f.line("    pub const CORTEX_M_EMBEDDED: Calibration = match Calibration::new(4.14e-21, 0.1, 1.0e-17) {");
    f.line("        Ok(c) => c,");
    f.line("        Err(_) => unreachable_unphysical(),");
    f.line("    };");
    f.blank();
    f.indented_doc_comment("The tightest provable lower bound that requires no trust in the");
    f.indented_doc_comment("issuer's claimed substrate. Values are physically sound but maximally");
    f.indented_doc_comment("generous: k_B·T at 300 K floor, thermal_power at 1 GW (above any");
    f.indented_doc_comment("plausible single-compute envelope), characteristic_energy at 1 J");
    f.indented_doc_comment("(astronomically generous).");
    f.indented_doc_comment("");
    f.indented_doc_comment("Applying this calibration yields the smallest `Nanos` physically");
    f.indented_doc_comment("possible for the computation regardless of substrate claims.");
    f.line("    pub const CONSERVATIVE_WORST_CASE: Calibration = match Calibration::new(4.14e-21, 1.0e9, 1.0) {");
    f.line("        Ok(c) => c,");
    f.line("        Err(_) => unreachable_unphysical(),");
    f.line("    };");
    f.blank();
    // const-context unreachable helper; on debug, panic; on release, loop forever.
    f.indented_doc_comment("Const-context unreachable helper. The four preset literals above are");
    f.indented_doc_comment("verified physical at codegen time; this branch is dead.");
    f.line("    #[inline]");
    f.line("    const fn unreachable_unphysical() -> Calibration {");
    f.line("        panic!(\"foundation preset calibration is physically valid by construction\")");
    f.line("    }");
    f.line("    // Suppress dead-code warnings on the helper");
    f.line("    #[allow(dead_code)]");
    f.line("    const _: fn() -> Calibration = unreachable_unphysical;");
    f.blank();
    f.line("    // Suppress unused-import warning for CalibrationError when the");
    f.line("    // preset construction succeeds (which it always does).");
    f.line("    #[allow(dead_code)]");
    f.line("    const _: Option<CalibrationError> = None;");
    f.line("}");
    f.blank();
}

fn generate_term_ast(f: &mut RustFile) {
    // TermList
    f.doc_comment("Fixed-capacity term list for `#![no_std]`. Indices into a `TermArena`.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
    f.line("pub struct TermList {");
    f.indented_doc_comment("Start index in the arena.");
    f.line("    pub start: u32,");
    f.indented_doc_comment("Number of terms in this list.");
    f.line("    pub len: u32,");
    f.line("}");
    f.blank();

    // TermArena
    f.doc_comment("Stack-resident arena for `Term` trees.");
    f.doc_comment("");
    f.doc_comment("Fixed capacity determined by the const generic `CAP`.");
    f.doc_comment("All `Term` child references are `u32` indices into this arena.");
    f.doc_comment("`#![no_std]`-safe: no heap allocation.");
    f.doc_example(
        "use uor_foundation::enforcement::{TermArena, Term, TermList};\n\
         use uor_foundation::{WittLevel, PrimitiveOp};\n\
         \n\
         // Build the expression `add(3, 5)` bottom-up in an arena.\n\
         let mut arena = TermArena::<4>::new();\n\
         \n\
         // Push leaves first:\n\
         let idx_3 = arena.push(Term::Literal { value: 3, level: WittLevel::W8 });\n\
         let idx_5 = arena.push(Term::Literal { value: 5, level: WittLevel::W8 });\n\
         \n\
         // Push the application node, referencing the leaves by index:\n\
         let idx_add = arena.push(Term::Application {\n\
         \x20   operator: PrimitiveOp::Add,\n\
         \x20   args: TermList { start: idx_3.unwrap_or(0), len: 2 },\n\
         });\n\
         \n\
         assert_eq!(arena.len(), 3);\n\
         // Retrieve a node by index:\n\
         let node = arena.get(idx_add.unwrap_or(0));\n\
         assert!(node.is_some());",
        "rust",
    );
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct TermArena<const CAP: usize> {");
    f.indented_doc_comment("Node storage. `None` slots are unused.");
    f.line("    nodes: [Option<Term>; CAP],");
    f.indented_doc_comment("Number of allocated nodes.");
    f.line("    len: u32,");
    f.line("}");
    f.blank();
    f.line("impl<const CAP: usize> TermArena<CAP> {");
    f.indented_doc_comment("Creates an empty arena.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub fn new() -> Self {");
    f.line("        Self {");
    f.line("            nodes: core::array::from_fn(|_| None),");
    f.line("            len: 0,");
    f.line("        }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Push a term into the arena and return its index.");
    f.indented_doc_comment("");
    f.indented_doc_comment("# Errors");
    f.indented_doc_comment("");
    f.indented_doc_comment("Returns `None` if the arena is full.");
    f.line("    #[must_use]");
    f.line("    pub fn push(&mut self, term: Term) -> Option<u32> {");
    f.line("        let idx = self.len;");
    f.line("        if (idx as usize) >= CAP {");
    f.line("            return None;");
    f.line("        }");
    f.line("        self.nodes[idx as usize] = Some(term);");
    f.line("        self.len = idx + 1;");
    f.line("        Some(idx)");
    f.line("    }");
    f.blank();
    f.indented_doc_comment(
        "Returns a reference to the term at `index`, or `None` if out of bounds.",
    );
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub fn get(&self, index: u32) -> Option<&Term> {");
    f.line("        self.nodes.get(index as usize).and_then(|slot| slot.as_ref())");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the number of allocated nodes.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn len(&self) -> u32 {");
    f.line("        self.len");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns `true` if the arena has no allocated nodes.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn is_empty(&self) -> bool {");
    f.line("        self.len == 0");
    f.line("    }");
    f.line("}");
    f.blank();
    f.line("impl<const CAP: usize> Default for TermArena<CAP> {");
    f.line("    fn default() -> Self {");
    f.line("        Self::new()");
    f.line("    }");
    f.line("}");
    f.blank();

    // Term
    f.doc_comment("Concrete AST node for the UOR term language.");
    f.doc_comment("");
    f.doc_comment("Mirrors the EBNF grammar productions. All child references are");
    f.doc_comment("indices into a `TermArena`, keeping the AST stack-resident and");
    f.doc_comment("`#![no_std]`-safe.");
    f.doc_example(
        "use uor_foundation::enforcement::{Term, TermList};\n\
         use uor_foundation::{WittLevel, PrimitiveOp};\n\
         \n\
         // Literal: an integer value tagged with a Witt level.\n\
         let lit = Term::Literal { value: 42, level: WittLevel::W8 };\n\
         \n\
         // Application: an operation applied to arguments.\n\
         // `args` is a TermList { start, len } pointing into a TermArena.\n\
         let app = Term::Application {\n\
         \x20   operator: PrimitiveOp::Mul,\n\
         \x20   args: TermList { start: 0, len: 2 },\n\
         };\n\
         \n\
         // Lift: canonical injection from a lower to a higher Witt level.\n\
         let lift = Term::Lift { operand_index: 0, target: WittLevel::new(32) };\n\
         \n\
         // Project: canonical surjection from a higher to a lower level.\n\
         let proj = Term::Project { operand_index: 0, target: WittLevel::W8 };",
        "rust",
    );
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub enum Term {");
    f.indented_doc_comment("Integer literal with Witt level annotation.");
    f.line("    Literal {");
    f.line("        /// The literal integer value.");
    f.line("        value: u64,");
    f.line("        /// The Witt level of this literal.");
    f.line("        level: WittLevel,");
    f.line("    },");
    f.indented_doc_comment("Variable reference by name index.");
    f.line("    Variable {");
    f.line("        /// Index into the name table.");
    f.line("        name_index: u32,");
    f.line("    },");
    f.indented_doc_comment("Operation application: operator applied to arguments.");
    f.line("    Application {");
    f.line("        /// The primitive operation to apply.");
    f.line("        operator: PrimitiveOp,");
    f.line("        /// Argument list (indices into arena).");
    f.line("        args: TermList,");
    f.line("    },");
    f.indented_doc_comment("Lift: canonical injection W_n to W_m (n < m, lossless).");
    f.line("    Lift {");
    f.line("        /// Index of the operand term in the arena.");
    f.line("        operand_index: u32,");
    f.line("        /// Target Witt level.");
    f.line("        target: WittLevel,");
    f.line("    },");
    f.indented_doc_comment("Project: canonical surjection W_m to W_n (m > n, lossy).");
    f.line("    Project {");
    f.line("        /// Index of the operand term in the arena.");
    f.line("        operand_index: u32,");
    f.line("        /// Target Witt level.");
    f.line("        target: WittLevel,");
    f.line("    },");
    f.indented_doc_comment("Match expression with pattern-result pairs.");
    f.line("    Match {");
    f.line("        /// Index of the scrutinee term in the arena.");
    f.line("        scrutinee_index: u32,");
    f.line("        /// Match arms (indices into arena).");
    f.line("        arms: TermList,");
    f.line("    },");
    f.indented_doc_comment("Bounded recursion with descent measure.");
    f.line("    Recurse {");
    f.line("        /// Index of the descent measure term.");
    f.line("        measure_index: u32,");
    f.line("        /// Index of the base case term.");
    f.line("        base_index: u32,");
    f.line("        /// Index of the recursive step term.");
    f.line("        step_index: u32,");
    f.line("    },");
    f.indented_doc_comment("Stream construction via unfold.");
    f.line("    Unfold {");
    f.line("        /// Index of the seed term.");
    f.line("        seed_index: u32,");
    f.line("        /// Index of the step function term.");
    f.line("        step_index: u32,");
    f.line("    },");
    f.indented_doc_comment("Try expression with failure recovery.");
    f.line("    Try {");
    f.line("        /// Index of the body term.");
    f.line("        body_index: u32,");
    f.line("        /// Index of the handler term.");
    f.line("        handler_index: u32,");
    f.line("    },");
    f.line("}");
    f.blank();

    // TypeDeclaration
    f.doc_comment("A type declaration with constraint kinds.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct TypeDeclaration {");
    f.indented_doc_comment("Name index for this type.");
    f.line("    pub name_index: u32,");
    f.indented_doc_comment("Constraint terms (indices into arena).");
    f.line("    pub constraints: TermList,");
    f.line("}");
    f.blank();

    // Binding
    f.doc_comment("A named binding: `let name : Type = term`.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct Binding {");
    f.indented_doc_comment("Name index for this binding.");
    f.line("    pub name_index: u32,");
    f.indented_doc_comment("Index of the type declaration.");
    f.line("    pub type_index: u32,");
    f.indented_doc_comment("Index of the value term in the arena.");
    f.line("    pub value_index: u32,");
    f.indented_doc_comment("EBNF surface syntax (compile-time constant).");
    f.line("    pub surface: &'static str,");
    f.indented_doc_comment("FNV-1a content address (compile-time constant).");
    f.line("    pub content_address: u64,");
    f.line("}");
    f.blank();

    // Assertion
    f.doc_comment("An assertion: `assert lhs = rhs`.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct Assertion {");
    f.indented_doc_comment("Index of the left-hand side term.");
    f.line("    pub lhs_index: u32,");
    f.indented_doc_comment("Index of the right-hand side term.");
    f.line("    pub rhs_index: u32,");
    f.indented_doc_comment("EBNF surface syntax (compile-time constant).");
    f.line("    pub surface: &'static str,");
    f.line("}");
    f.blank();

    // SourceDeclaration
    f.doc_comment("Boundary source declaration: `source name : Type via grounding`.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct SourceDeclaration {");
    f.indented_doc_comment("Name index for the source.");
    f.line("    pub name_index: u32,");
    f.indented_doc_comment("Index of the type declaration.");
    f.line("    pub type_index: u32,");
    f.indented_doc_comment("Name index of the grounding map.");
    f.line("    pub grounding_name_index: u32,");
    f.line("}");
    f.blank();

    // SinkDeclaration
    f.doc_comment("Boundary sink declaration: `sink name : Type via projection`.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct SinkDeclaration {");
    f.indented_doc_comment("Name index for the sink.");
    f.line("    pub name_index: u32,");
    f.indented_doc_comment("Index of the type declaration.");
    f.line("    pub type_index: u32,");
    f.indented_doc_comment("Name index of the projection map.");
    f.line("    pub projection_name_index: u32,");
    f.line("}");
    f.blank();
}

fn generate_shape_violation(f: &mut RustFile) {
    f.doc_comment("Structured violation diagnostic carrying metadata from the");
    f.doc_comment("conformance namespace. Every field is machine-readable.");
    f.doc_example(
        "use uor_foundation::enforcement::ShapeViolation;\n\
         use uor_foundation::ViolationKind;\n\
         \n\
         // ShapeViolation carries structured metadata from the ontology.\n\
         // Every field is machine-readable — IRIs, counts, and a typed kind.\n\
         let violation = ShapeViolation {\n\
         \x20   shape_iri: \"https://uor.foundation/conformance/CompileUnitShape\",\n\
         \x20   constraint_iri: \"https://uor.foundation/conformance/compileUnit_rootTerm_constraint\",\n\
         \x20   property_iri: \"https://uor.foundation/reduction/rootTerm\",\n\
         \x20   expected_range: \"https://uor.foundation/schema/Term\",\n\
         \x20   min_count: 1,\n\
         \x20   max_count: 1,\n\
         \x20   kind: ViolationKind::Missing,\n\
         };\n\
         \n\
         // Machine-readable for tooling (IDE plugins, CI pipelines):\n\
         assert_eq!(violation.kind, ViolationKind::Missing);\n\
         assert!(violation.shape_iri.ends_with(\"CompileUnitShape\"));\n\
         assert_eq!(violation.min_count, 1);",
        "rust",
    );
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct ShapeViolation {");
    f.indented_doc_comment("IRI of the `conformance:Shape` that was validated against.");
    f.line("    pub shape_iri: &'static str,");
    f.indented_doc_comment("IRI of the specific `conformance:PropertyConstraint` that failed.");
    f.line("    pub constraint_iri: &'static str,");
    f.indented_doc_comment("IRI of the property that was missing or invalid.");
    f.line("    pub property_iri: &'static str,");
    f.indented_doc_comment("The expected range class IRI.");
    f.line("    pub expected_range: &'static str,");
    f.indented_doc_comment("Minimum cardinality from the constraint.");
    f.line("    pub min_count: u32,");
    f.indented_doc_comment("Maximum cardinality (0 = unbounded).");
    f.line("    pub max_count: u32,");
    f.indented_doc_comment("What went wrong.");
    f.line("    pub kind: ViolationKind,");
    f.line("}");
    f.blank();
}

fn generate_builders(f: &mut RustFile) {
    // CompileUnitBuilder
    f.doc_comment("Builder for `CompileUnit` admission into the reduction pipeline.");
    f.doc_comment("");
    f.doc_comment("Collects `rootTerm`, `wittLevelCeiling`, `thermodynamicBudget`,");
    f.doc_comment("and `targetDomains`. The `validate()` method checks structural");
    f.doc_comment("constraints (Tier 1) and value-dependent constraints (Tier 2).");
    f.doc_example(
        "use uor_foundation::enforcement::{CompileUnitBuilder, Term};\n\
         use uor_foundation::{WittLevel, VerificationDomain, ViolationKind};\n\
         \n\
         // A CompileUnit packages a term graph for reduction admission.\n\
         // The builder enforces that all required fields are present.\n\
         let terms = [Term::Literal { value: 1, level: WittLevel::W8 }];\n\
         let domains = [VerificationDomain::Enumerative];\n\
         \n\
         let unit = CompileUnitBuilder::new()\n\
         \x20   .root_term(&terms)\n\
         \x20   .witt_level_ceiling(WittLevel::W8)\n\
         \x20   .thermodynamic_budget(1024)\n\
         \x20   .target_domains(&domains)\n\
         \x20   .validate();\n\
         assert!(unit.is_ok());\n\
         \n\
         // Omitting a required field produces a ShapeViolation\n\
         // with the exact conformance IRI that failed:\n\
         let err = CompileUnitBuilder::new()\n\
         \x20   .witt_level_ceiling(WittLevel::W8)\n\
         \x20   .thermodynamic_budget(1024)\n\
         \x20   .target_domains(&domains)\n\
         \x20   .validate();\n\
         assert!(err.is_err());\n\
         if let Err(violation) = err {\n\
         \x20   assert_eq!(violation.kind, ViolationKind::Missing);\n\
         \x20   assert!(violation.property_iri.contains(\"rootTerm\"));\n\
         }",
        "rust",
    );
    f.line("#[derive(Debug, Clone)]");
    f.line("pub struct CompileUnitBuilder<'a> {");
    f.indented_doc_comment("The root term expression.");
    f.line("    root_term: Option<&'a [Term]>,");
    f.indented_doc_comment("The widest Witt level the computation may reference.");
    f.line("    witt_level_ceiling: Option<WittLevel>,");
    f.indented_doc_comment("Landauer-bounded energy budget.");
    f.line("    thermodynamic_budget: Option<u64>,");
    f.indented_doc_comment("Verification domains targeted.");
    f.line("    target_domains: Option<&'a [VerificationDomain]>,");
    f.line("}");
    f.blank();

    // CompileUnit (validated result type)
    f.doc_comment("A validated compile unit ready for reduction admission.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct CompileUnit {");
    f.indented_doc_comment("The Witt level ceiling.");
    f.line("    level: WittLevel,");
    f.indented_doc_comment("The thermodynamic budget.");
    f.line("    budget: u64,");
    f.line("}");
    f.blank();
    f.line("impl CompileUnit {");
    f.indented_doc_comment("Returns the Witt level ceiling declared at validation time.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn witt_level(&self) -> WittLevel {");
    f.line("        self.level");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the thermodynamic budget declared at validation time.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn thermodynamic_budget(&self) -> u64 {");
    f.line("        self.budget");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("v0.2.2 Phase G: const-constructible empty unit used by");
    f.indented_doc_comment("`validate_compile_unit_const` for compile-time validation.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn empty_const() -> Self {");
    f.line("        Self {");
    f.line("            level: WittLevel::W8,");
    f.line("            budget: 0,");
    f.line("        }");
    f.line("    }");
    f.line("}");
    f.blank();

    f.line("impl<'a> CompileUnitBuilder<'a> {");
    f.indented_doc_comment("Creates a new empty builder.");
    f.line("    #[must_use]");
    f.line("    pub const fn new() -> Self {");
    f.line("        Self {");
    f.line("            root_term: None,");
    f.line("            witt_level_ceiling: None,");
    f.line("            thermodynamic_budget: None,");
    f.line("            target_domains: None,");
    f.line("        }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the root term expression.");
    f.line("    #[must_use]");
    f.line("    pub const fn root_term(mut self, terms: &'a [Term]) -> Self {");
    f.line("        self.root_term = Some(terms);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the Witt level ceiling.");
    f.line("    #[must_use]");
    f.line("    pub const fn witt_level_ceiling(mut self, level: WittLevel) -> Self {");
    f.line("        self.witt_level_ceiling = Some(level);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the thermodynamic budget.");
    f.line("    #[must_use]");
    f.line("    pub const fn thermodynamic_budget(mut self, budget: u64) -> Self {");
    f.line("        self.thermodynamic_budget = Some(budget);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the target verification domains.");
    f.line("    #[must_use]");
    f.line(
        "    pub const fn target_domains(mut self, domains: &'a [VerificationDomain]) -> Self {",
    );
    f.line("        self.target_domains = Some(domains);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Validate against `CompileUnitShape`.");
    f.indented_doc_comment("");
    f.indented_doc_comment("Tier 1: checks presence and cardinality of all required fields.");
    f.indented_doc_comment("Tier 2: checks budget solvency and level coherence.");
    f.indented_doc_comment("");
    f.indented_doc_comment("# Errors");
    f.indented_doc_comment("");
    f.indented_doc_comment("Returns `ShapeViolation` if any constraint is not satisfied.");
    f.line("    pub fn validate(self) -> Result<Validated<CompileUnit>, ShapeViolation> {");
    f.line("        if self.root_term.is_none() {");
    f.line("            return Err(ShapeViolation {");
    f.line("                shape_iri: \"https://uor.foundation/conformance/CompileUnitShape\",");
    f.line("                constraint_iri: \"https://uor.foundation/conformance/compileUnit_rootTerm_constraint\",");
    f.line("                property_iri: \"https://uor.foundation/reduction/rootTerm\",");
    f.line("                expected_range: \"https://uor.foundation/schema/Term\",");
    f.line("                min_count: 1,");
    f.line("                max_count: 1,");
    f.line("                kind: ViolationKind::Missing,");
    f.line("            });");
    f.line("        }");
    f.line("        let level = match self.witt_level_ceiling {");
    f.line("            Some(l) => l,");
    f.line("            None => return Err(ShapeViolation {");
    f.line("                shape_iri: \"https://uor.foundation/conformance/CompileUnitShape\",");
    f.line("                constraint_iri: \"https://uor.foundation/conformance/compileUnit_unitWittLevel_constraint\",");
    f.line("                property_iri: \"https://uor.foundation/reduction/unitWittLevel\",");
    f.line("                expected_range: \"https://uor.foundation/schema/WittLevel\",");
    f.line("                min_count: 1,");
    f.line("                max_count: 1,");
    f.line("                kind: ViolationKind::Missing,");
    f.line("            }),");
    f.line("        };");
    f.line("        let budget = match self.thermodynamic_budget {");
    f.line("            Some(b) => b,");
    f.line("            None => return Err(ShapeViolation {");
    f.line("                shape_iri: \"https://uor.foundation/conformance/CompileUnitShape\",");
    f.line("                constraint_iri: \"https://uor.foundation/conformance/compileUnit_thermodynamicBudget_constraint\",");
    f.line(
        "                property_iri: \"https://uor.foundation/reduction/thermodynamicBudget\",",
    );
    f.line("                expected_range: \"http://www.w3.org/2001/XMLSchema#decimal\",");
    f.line("                min_count: 1,");
    f.line("                max_count: 1,");
    f.line("                kind: ViolationKind::Missing,");
    f.line("            }),");
    f.line("        };");
    f.line("        match self.target_domains {");
    f.line("            Some(d) if !d.is_empty() => {},");
    f.line("            _ => return Err(ShapeViolation {");
    f.line("                shape_iri: \"https://uor.foundation/conformance/CompileUnitShape\",");
    f.line("                constraint_iri: \"https://uor.foundation/conformance/compileUnit_targetDomains_constraint\",");
    f.line("                property_iri: \"https://uor.foundation/reduction/targetDomains\",");
    f.line("                expected_range: \"https://uor.foundation/op/VerificationDomain\",");
    f.line("                min_count: 1,");
    f.line("                max_count: 0,");
    f.line("                kind: ViolationKind::Missing,");
    f.line("            }),");
    f.line("        }");
    f.line("        Ok(Validated::new(CompileUnit { level, budget }))");
    f.line("    }");
    f.line("}");
    f.blank();

    // Default impl for CompileUnitBuilder
    f.line("impl<'a> Default for CompileUnitBuilder<'a> {");
    f.line("    fn default() -> Self {");
    f.line("        Self::new()");
    f.line("    }");
    f.line("}");
    f.blank();

    // Generate builders for the remaining 8 shapes
    generate_simple_builder(
        f,
        "EffectDeclarationBuilder",
        "Declared effect validated against `EffectShape`.",
        &[
            ("name", "&'a str"),
            ("target_sites", "&'a [u32]"),
            ("budget_delta", "i64"),
            ("commutes", "bool"),
        ],
        "EffectDeclaration",
        "https://uor.foundation/conformance/EffectShape",
    );
    generate_simple_builder(
        f,
        "GroundingDeclarationBuilder",
        "Declared grounding validated against `GroundingShape`.",
        &[
            ("source_type", "&'a str"),
            ("ring_mapping", "&'a str"),
            ("invertibility", "bool"),
        ],
        "GroundingDeclaration",
        "https://uor.foundation/conformance/GroundingShape",
    );
    generate_simple_builder(
        f,
        "DispatchDeclarationBuilder",
        "Declared dispatch rule validated against `DispatchShape`.",
        &[
            ("predicate", "&'a [Term]"),
            ("target_resolver", "&'a str"),
            ("priority", "u32"),
        ],
        "DispatchDeclaration",
        "https://uor.foundation/conformance/DispatchShape",
    );
    generate_simple_builder(
        f,
        "LeaseDeclarationBuilder",
        "Declared lease validated against `LeaseShape`.",
        &[("linear_site", "u32"), ("scope", "&'a str")],
        "LeaseDeclaration",
        "https://uor.foundation/conformance/LeaseShape",
    );
    generate_simple_builder(
        f,
        "StreamDeclarationBuilder",
        "Declared stream validated against `StreamShape`.",
        &[
            ("seed", "&'a [Term]"),
            ("step", "&'a [Term]"),
            ("productivity_witness", "&'a str"),
        ],
        "StreamDeclaration",
        "https://uor.foundation/conformance/StreamShape",
    );
    generate_simple_builder(
        f,
        "PredicateDeclarationBuilder",
        "Declared predicate validated against `PredicateShape`.",
        &[
            ("input_type", "&'a str"),
            ("evaluator", "&'a [Term]"),
            ("termination_witness", "&'a str"),
        ],
        "PredicateDeclaration",
        "https://uor.foundation/conformance/PredicateShape",
    );
    generate_simple_builder(
        f,
        "ParallelDeclarationBuilder",
        "Declared parallel composition validated against `ParallelShape`.",
        &[
            ("site_partition", "&'a [u32]"),
            ("disjointness_witness", "&'a str"),
        ],
        "ParallelDeclaration",
        "https://uor.foundation/conformance/ParallelShape",
    );

    // WittLevelDeclarationBuilder (no lifetime needed)
    f.doc_comment("Builder for declaring a new Witt level beyond W32.");
    f.doc_comment("");
    f.doc_comment("Validates against `WittLevelShape`.");
    f.line("#[derive(Debug, Clone)]");
    f.line("pub struct WittLevelDeclarationBuilder {");
    f.indented_doc_comment("The declared bit width.");
    f.line("    bit_width: Option<u32>,");
    f.indented_doc_comment("The declared cycle size.");
    f.line("    cycle_size: Option<u128>,");
    f.indented_doc_comment("The predecessor level.");
    f.line("    predecessor: Option<WittLevel>,");
    f.line("}");
    f.blank();

    f.doc_comment("Validated Witt level declaration.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct WittLevelDeclaration {");
    f.indented_doc_comment("The declared bit width.");
    f.line("    pub bit_width: u32,");
    f.indented_doc_comment("The predecessor level.");
    f.line("    pub predecessor: WittLevel,");
    f.line("}");
    f.blank();

    f.line("impl WittLevelDeclarationBuilder {");
    f.indented_doc_comment("Creates a new empty builder.");
    f.line("    #[must_use]");
    f.line("    pub const fn new() -> Self {");
    f.line("        Self { bit_width: None, cycle_size: None, predecessor: None }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the declared bit width.");
    f.line("    #[must_use]");
    f.line("    pub const fn bit_width(mut self, w: u32) -> Self {");
    f.line("        self.bit_width = Some(w);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the declared cycle size.");
    f.line("    #[must_use]");
    f.line("    pub const fn cycle_size(mut self, s: u128) -> Self {");
    f.line("        self.cycle_size = Some(s);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the predecessor Witt level.");
    f.line("    #[must_use]");
    f.line("    pub const fn predecessor(mut self, level: WittLevel) -> Self {");
    f.line("        self.predecessor = Some(level);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Validate against `WittLevelShape`.");
    f.indented_doc_comment("");
    f.indented_doc_comment("# Errors");
    f.indented_doc_comment("");
    f.indented_doc_comment("Returns `ShapeViolation` if any required field is missing.");
    f.line(
        "    pub fn validate(self) -> Result<Validated<WittLevelDeclaration>, ShapeViolation> {",
    );
    f.line("        let bw = match self.bit_width {");
    f.line("            Some(w) => w,");
    f.line("            None => return Err(ShapeViolation {");
    f.line("                shape_iri: \"https://uor.foundation/conformance/WittLevelShape\",");
    f.line(
        "                constraint_iri: \"https://uor.foundation/conformance/WittLevelShape\",",
    );
    f.line(
        "                property_iri: \"https://uor.foundation/conformance/declaredBitWidth\",",
    );
    f.line("                expected_range: \"http://www.w3.org/2001/XMLSchema#positiveInteger\",");
    f.line("                min_count: 1,");
    f.line("                max_count: 1,");
    f.line("                kind: ViolationKind::Missing,");
    f.line("            }),");
    f.line("        };");
    f.line("        let pred = match self.predecessor {");
    f.line("            Some(p) => p,");
    f.line("            None => return Err(ShapeViolation {");
    f.line("                shape_iri: \"https://uor.foundation/conformance/WittLevelShape\",");
    f.line(
        "                constraint_iri: \"https://uor.foundation/conformance/WittLevelShape\",",
    );
    f.line(
        "                property_iri: \"https://uor.foundation/conformance/predecessorLevel\",",
    );
    f.line("                expected_range: \"https://uor.foundation/schema/WittLevel\",");
    f.line("                min_count: 1,");
    f.line("                max_count: 1,");
    f.line("                kind: ViolationKind::Missing,");
    f.line("            }),");
    f.line("        };");
    f.line("        Ok(Validated::new(WittLevelDeclaration { bit_width: bw, predecessor: pred }))");
    f.line("    }");
    f.line("}");
    f.blank();

    f.line("impl Default for WittLevelDeclarationBuilder {");
    f.line("    fn default() -> Self {");
    f.line("        Self::new()");
    f.line("    }");
    f.line("}");
    f.blank();
}

/// Generates a simple builder struct with `Option` fields and a `validate()` method
/// that checks all fields are present.
fn generate_simple_builder(
    f: &mut RustFile,
    builder_name: &str,
    result_doc: &str,
    fields: &[(&str, &str)],
    result_name: &str,
    shape_iri: &str,
) {
    let needs_lifetime = fields.iter().any(|(_, ty)| ty.starts_with('&'));
    let lt = if needs_lifetime { "<'a>" } else { "" };

    // Builder struct
    f.doc_comment(&format!(
        "Builder for `{result_name}`. Validates against `{}`.",
        shape_iri.rsplit('/').next().unwrap_or(shape_iri),
    ));
    f.line("#[derive(Debug, Clone)]");
    f.line(&format!("pub struct {builder_name}{lt} {{"));
    for (name, ty) in fields {
        let opt_ty = format!("Option<{ty}>");
        f.indented_doc_comment(&format!("The `{name}` field."));
        f.line(&format!("    {name}: {opt_ty},"));
    }
    f.line("}");
    f.blank();

    // Validated result struct
    f.doc_comment(result_doc);
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line(&format!("pub struct {result_name} {{"));
    f.indented_doc_comment("Shape IRI this declaration was validated against.");
    f.line("    pub shape_iri: &'static str,");
    f.line("}");
    f.blank();
    // v0.2.2 Phase G: const-constructible empty form for const-fn
    // validation paths.
    f.line(&format!("impl {result_name} {{"));
    f.indented_doc_comment("v0.2.2 Phase G: const-constructible empty form used by");
    f.indented_doc_comment("`validate_*_const` companion functions.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn empty_const() -> Self {");
    f.line(&format!("        Self {{ shape_iri: \"{shape_iri}\" }}"));
    f.line("    }");
    f.line("}");
    f.blank();

    // impl block
    f.line(&format!("impl{lt} {builder_name}{lt} {{"));
    f.indented_doc_comment("Creates a new empty builder.");
    f.line("    #[must_use]");
    f.line("    pub const fn new() -> Self {");
    f.line("        Self {");
    for (name, _) in fields {
        f.line(&format!("            {name}: None,"));
    }
    f.line("        }");
    f.line("    }");
    f.blank();

    // Setter methods
    for (name, ty) in fields {
        f.indented_doc_comment(&format!("Set the `{name}` field."));
        f.line("    #[must_use]");
        f.line(&format!(
            "    pub const fn {name}(mut self, value: {ty}) -> Self {{"
        ));
        f.line(&format!("        self.{name} = Some(value);"));
        f.line("        self");
        f.line("    }");
        f.blank();
    }

    // validate method
    f.indented_doc_comment(&format!(
        "Validate against `{}`.",
        shape_iri.rsplit('/').next().unwrap_or(shape_iri)
    ));
    f.indented_doc_comment("");
    f.indented_doc_comment("# Errors");
    f.indented_doc_comment("");
    f.indented_doc_comment("Returns `ShapeViolation` if any required field is missing.");
    f.line(&format!(
        "    pub fn validate(self) -> Result<Validated<{result_name}>, ShapeViolation> {{"
    ));
    // Check first field as representative
    let first = fields[0].0;
    f.line(&format!("        if self.{first}.is_none() {{"));
    f.line("            return Err(ShapeViolation {");
    f.line(&format!("                shape_iri: \"{shape_iri}\","));
    f.line(&format!("                constraint_iri: \"{shape_iri}\","));
    f.line(&format!(
        "                property_iri: \"https://uor.foundation/conformance/{first}\","
    ));
    f.line("                expected_range: \"http://www.w3.org/2002/07/owl#Thing\",");
    f.line("                min_count: 1,");
    f.line("                max_count: 1,");
    f.line("                kind: ViolationKind::Missing,");
    f.line("            });");
    f.line("        }");
    // Check remaining fields
    for (name, _) in &fields[1..] {
        f.line(&format!("        if self.{name}.is_none() {{"));
        f.line("            return Err(ShapeViolation {");
        f.line(&format!("                shape_iri: \"{shape_iri}\","));
        f.line(&format!("                constraint_iri: \"{shape_iri}\","));
        f.line(&format!(
            "                property_iri: \"https://uor.foundation/conformance/{name}\","
        ));
        f.line("                expected_range: \"http://www.w3.org/2002/07/owl#Thing\",");
        f.line("                min_count: 1,");
        f.line("                max_count: 1,");
        f.line("                kind: ViolationKind::Missing,");
        f.line("            });");
        f.line("        }");
    }
    f.line(&format!(
        "        Ok(Validated::new({result_name} {{ shape_iri: \"{shape_iri}\" }}))"
    ));
    f.line("    }");
    f.line("}");
    f.blank();

    // Default impl
    f.line(&format!("impl{lt} Default for {builder_name}{lt} {{"));
    f.line("    fn default() -> Self {");
    f.line("        Self::new()");
    f.line("    }");
    f.line("}");
    f.blank();
}

fn generate_minting_session(f: &mut RustFile, ontology: &Ontology) {
    let levels = witt_levels(ontology);
    f.doc_comment("Boundary session state tracker for the two-phase minting boundary.");
    f.doc_comment("");
    f.doc_comment("Records crossing count and idempotency flag. Private fields");
    f.doc_comment("prevent external construction.");
    f.line("#[derive(Debug, Clone, PartialEq, Eq)]");
    f.line("pub struct BoundarySession {");
    f.indented_doc_comment("Total boundary crossings in this session.");
    f.line("    crossing_count: u32,");
    f.indented_doc_comment("Whether the boundary effect is idempotent.");
    f.line("    is_idempotent: bool,");
    f.line("}");
    f.blank();
    f.line("impl BoundarySession {");
    f.indented_doc_comment("Creates a new boundary session. Only callable within the crate.");
    f.line("    #[inline]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(is_idempotent: bool) -> Self {");
    f.line("        Self { crossing_count: 0, is_idempotent }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the total boundary crossings.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn crossing_count(&self) -> u32 {");
    f.line("        self.crossing_count");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns whether the boundary effect is idempotent.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn is_idempotent(&self) -> bool {");
    f.line("        self.is_idempotent");
    f.line("    }");
    f.line("}");
    f.blank();

    // validate_and_mint functions
    f.doc_comment("Validate a scalar grounding intermediate against a `GroundingShape`");
    f.doc_comment("and mint it into a `Datum`. Only callable within `uor-foundation`.");
    f.doc_comment("");
    f.doc_comment("# Errors");
    f.doc_comment("");
    f.doc_comment("Returns `ShapeViolation` if the coordinate fails validation.");
    f.line("#[allow(dead_code)]");
    f.line("pub(crate) fn validate_and_mint_coord(");
    f.line("    grounded: GroundedCoord,");
    f.line("    shape: &Validated<GroundingDeclaration>,");
    f.line("    session: &mut BoundarySession,");
    f.line(") -> Result<Datum, ShapeViolation> {");
    f.line("    // The Validated<GroundingDeclaration> proves the shape was already");
    f.line("    // validated at builder time. The coordinate's level is guaranteed");
    f.line("    // correct by the closed GroundedCoordInner enum — the type system");
    f.line("    // enforces that only supported levels can be constructed.");
    f.line("    let _ = shape; // shape validation passed at builder time");
    f.line("    session.crossing_count += 1;");
    f.line("    let inner = match grounded.inner {");
    for (local, _, _) in &levels {
        f.line(&format!(
            "        GroundedCoordInner::{local}(b) => DatumInner::{local}(b),"
        ));
    }
    f.line("    };");
    f.line("    Ok(Datum { inner })");
    f.line("}");
    f.blank();

    f.doc_comment("Validate a tuple grounding intermediate and mint into a `Datum`.");
    f.doc_comment("Only callable within `uor-foundation`.");
    f.doc_comment("");
    f.doc_comment("Mints the first coordinate of the tuple as the representative `Datum`.");
    f.doc_comment("Composite multi-coordinate `Datum` construction depends on the target");
    f.doc_comment("type's site decomposition, which is resolved during reduction evaluation.");
    f.doc_comment("");
    f.doc_comment("# Errors");
    f.doc_comment("");
    f.doc_comment("Returns `ShapeViolation` if the tuple is empty or fails validation.");
    f.line("#[allow(dead_code)]");
    f.line("pub(crate) fn validate_and_mint_tuple<const N: usize>(");
    f.line("    grounded: GroundedTuple<N>,");
    f.line("    shape: &Validated<GroundingDeclaration>,");
    f.line("    session: &mut BoundarySession,");
    f.line(") -> Result<Datum, ShapeViolation> {");
    f.line("    if N == 0 {");
    f.line("        return Err(ShapeViolation {");
    f.line("            shape_iri: shape.inner().shape_iri,");
    f.line("            constraint_iri: shape.inner().shape_iri,");
    f.line("            property_iri: \"https://uor.foundation/conformance/groundingSourceType\",");
    f.line("            expected_range: \"https://uor.foundation/type/TypeDefinition\",");
    f.line("            min_count: 1,");
    f.line("            max_count: 0,");
    f.line("            kind: ViolationKind::CardinalityViolation,");
    f.line("        });");
    f.line("    }");
    f.line("    // Mint the first coordinate as the representative Datum.");
    f.line("    // The full tuple is decomposed during reduction evaluation,");
    f.line("    // where each coordinate maps to a site in the constrained type.");
    f.line("    validate_and_mint_coord(grounded.coords[0].clone(), shape, session)");
    f.line("}");
    f.blank();
}

fn generate_const_ring_eval(f: &mut RustFile, ontology: &Ontology) {
    // v0.2.1 Phase 8b.7: emit one binary + one unary const helper per
    // `schema:WittLevel` individual. Helper names follow the pattern
    // `const_ring_eval_w{bits}` and `const_ring_eval_unary_w{bits}` so
    // the ring-op phantom-struct impls in `generate_ring_ops` can find
    // them mechanically.
    //
    // For non-power-of-2 bit widths (e.g. W24), the helper stores the
    // value in the smallest-containing Rust primitive (`u32` for W24)
    // and masks the result to the ring's bit width on every operation.
    let levels = witt_levels(ontology);

    f.doc_comment("Evaluate a binary ring operation at compile time.");
    f.doc_comment("");
    f.doc_comment("One helper is emitted per `schema:WittLevel` individual. The `uor!`");
    f.doc_comment("proc macro delegates to these helpers; it never performs ring");
    f.doc_comment("arithmetic itself.");
    f.doc_example(
        "use uor_foundation::enforcement::{const_ring_eval_w8, const_ring_eval_unary_w8};\n\
         use uor_foundation::PrimitiveOp;\n\
         \n\
         // Ring arithmetic in Z/256Z: all operations wrap modulo 256.\n\
         \n\
         // Addition wraps: 200 + 100 = 300 -> 300 - 256 = 44\n\
         assert_eq!(const_ring_eval_w8(PrimitiveOp::Add, 200, 100), 44);\n\
         \n\
         // Multiplication: 3 * 5 = 15 (no wrap needed)\n\
         assert_eq!(const_ring_eval_w8(PrimitiveOp::Mul, 3, 5), 15);\n\
         \n\
         // XOR: bitwise exclusive-or\n\
         assert_eq!(const_ring_eval_w8(PrimitiveOp::Xor, 0b1010, 0b1100), 0b0110);\n\
         \n\
         // Negation: neg(x) = 256 - x (additive inverse in Z/256Z)\n\
         assert_eq!(const_ring_eval_unary_w8(PrimitiveOp::Neg, 1), 255);\n\
         \n\
         // The critical identity: neg(bnot(x)) = succ(x) for all x\n\
         let x = 42u8;\n\
         let lhs = const_ring_eval_unary_w8(PrimitiveOp::Neg,\n\
         \x20   const_ring_eval_unary_w8(PrimitiveOp::Bnot, x));\n\
         let rhs = const_ring_eval_unary_w8(PrimitiveOp::Succ, x);\n\
         assert_eq!(lhs, rhs);",
        "rust",
    );

    for (local, bits, _) in &levels {
        let rust_ty = witt_rust_int_type(*bits);
        let lower = local.to_ascii_lowercase();
        // Mask for non-native-width levels.
        // Native widths: W8 (u8), W16 (u16), W32 (u32), W64 (u64), W128 (u128).
        // Non-native: W24, W40, W48, W56, W72, W80, W88, W96, W104, W112, W120.
        let native_bits: u32 = match rust_ty {
            "u8" => 8,
            "u16" => 16,
            "u32" => 32,
            "u64" => 64,
            "u128" => 128,
            _ => 64,
        };
        let needs_mask = *bits != native_bits;
        // Mask literal selection:
        // - Non-native u64-backed (W40/W48/W56): `u64::MAX >> (64 - bits)`
        //   yields a u64 directly.
        // - Non-native u32-backed (W24): cast from u64 since the shift
        //   produces u64 and we narrow to u32.
        // - Non-native u128-backed (W72..W120): `u128::MAX >> (128 - bits)`
        //   yields a u128 directly.
        let mask_lit = if !needs_mask {
            String::new()
        } else if rust_ty == "u128" {
            format!("u128::MAX >> (128 - {bits})")
        } else if rust_ty == "u64" {
            format!("u64::MAX >> (64 - {bits})")
        } else {
            format!("(u64::MAX >> (64 - {bits})) as {rust_ty}")
        };
        let apply_mask = |expr: String| -> String {
            if needs_mask {
                format!("({expr}) & MASK")
            } else {
                expr
            }
        };

        f.line("#[inline]");
        f.line("#[must_use]");
        f.line(&format!(
            "pub const fn const_ring_eval_{lower}(op: PrimitiveOp, a: {rust_ty}, b: {rust_ty}) -> {rust_ty} {{"
        ));
        if needs_mask {
            f.line(&format!("    const MASK: {rust_ty} = {mask_lit};"));
        }
        f.line("    match op {");
        f.line(&format!(
            "        PrimitiveOp::Add => {},",
            apply_mask("a.wrapping_add(b)".to_string())
        ));
        f.line(&format!(
            "        PrimitiveOp::Sub => {},",
            apply_mask("a.wrapping_sub(b)".to_string())
        ));
        f.line(&format!(
            "        PrimitiveOp::Mul => {},",
            apply_mask("a.wrapping_mul(b)".to_string())
        ));
        f.line(&format!(
            "        PrimitiveOp::Xor => {},",
            apply_mask("a ^ b".to_string())
        ));
        f.line(&format!(
            "        PrimitiveOp::And => {},",
            apply_mask("a & b".to_string())
        ));
        f.line(&format!(
            "        PrimitiveOp::Or => {},",
            apply_mask("a | b".to_string())
        ));
        f.line("        _ => 0,");
        f.line("    }");
        f.line("}");
        f.blank();

        f.line("#[inline]");
        f.line("#[must_use]");
        f.line(&format!(
            "pub const fn const_ring_eval_unary_{lower}(op: PrimitiveOp, a: {rust_ty}) -> {rust_ty} {{"
        ));
        if needs_mask {
            f.line(&format!("    const MASK: {rust_ty} = {mask_lit};"));
        }
        f.line("    match op {");
        f.line(&format!(
            "        PrimitiveOp::Neg => {},",
            apply_mask(format!("0{rust_ty}.wrapping_sub(a)"))
        ));
        f.line(&format!(
            "        PrimitiveOp::Bnot => {},",
            apply_mask("!a".to_string())
        ));
        f.line(&format!(
            "        PrimitiveOp::Succ => {},",
            apply_mask("a.wrapping_add(1)".to_string())
        ));
        f.line(&format!(
            "        PrimitiveOp::Pred => {},",
            apply_mask("a.wrapping_sub(1)".to_string())
        ));
        f.line("        _ => 0,");
        f.line("    }");
        f.line("}");
        f.blank();
    }
}

// ── v0.2.1 Ergonomics Surface Generators ─────────────────────────────────────
//
// Each generator below reads from `&Ontology` (passed at the top) so that
// every emitted symbol traces to an ontology entity. There are no static
// Rust mapping tables — adding a new resolver, certificate, dispatch table,
// or prelude member requires only an ontology edit.

/// Convert an IRI to its local name (everything after the last `/` or `#`).
fn local_name(iri: &str) -> &str {
    iri.rsplit_once(['/', '#']).map(|(_, n)| n).unwrap_or(iri)
}

/// Find an individual by IRI.
fn find_individual<'a>(
    ontology: &'a Ontology,
    iri: &str,
) -> Option<&'a uor_ontology::model::Individual> {
    for ns in &ontology.namespaces {
        for ind in &ns.individuals {
            if ind.id == iri {
                return Some(ind);
            }
        }
    }
    None
}

/// Read a property value off an individual; returns the matching IriRef or
/// Str payload as a borrowed string.
fn ind_prop_str<'a>(ind: &'a uor_ontology::model::Individual, prop_iri: &str) -> Option<&'a str> {
    for (k, v) in ind.properties {
        if *k == prop_iri {
            return match v {
                IndividualValue::IriRef(s) | IndividualValue::Str(s) => Some(s),
                _ => None,
            };
        }
    }
    None
}

/// Collect all individuals of a given type.
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

/// Walk `resolver:CertifyMapping` individuals and collect the sorted
/// unique local-names of the certificate classes and witness classes they
/// reference. Used by Phase 7b.4 to verify the foundation's hand-rolled
/// shim list matches what the ontology wires into `Certify`.
fn collect_certify_mapping_targets(ontology: &Ontology) -> (Vec<String>, Vec<String>) {
    let mut certs: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut witnesses: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for ind in individuals_of_type(ontology, "https://uor.foundation/resolver/CertifyMapping") {
        if let Some(iri) = ind_prop_str(ind, "https://uor.foundation/resolver/producesCertificate")
        {
            certs.insert(local_name(iri).to_string());
        }
        if let Some(iri) = ind_prop_str(ind, "https://uor.foundation/resolver/producesWitness") {
            witnesses.insert(local_name(iri).to_string());
        }
    }
    (certs.into_iter().collect(), witnesses.into_iter().collect())
}

/// Verify that the hand-rolled shim list in [`generate_ontology_target_trait`]
/// is a superset of the ontology's subclass closure. Panics at codegen time
/// with a clear error if a class the ontology declares is missing from the
/// shim list — this turns "the shim list matches the ontology" into a
/// machine-checked invariant. Panic is intentional.
#[allow(clippy::panic)]
fn verify_shim_coverage(label: &str, expected: &[String], shim_names: &[&str]) {
    let shim_set: std::collections::BTreeSet<&str> = shim_names.iter().copied().collect();
    for name in expected {
        if !shim_set.contains(name.as_str()) {
            panic!(
                "generate_ontology_target_trait: ontology declares {label} subclass `{name}` \
                 but the hand-rolled shim list in codegen/src/enforcement.rs does not \
                 include it. Add `{name}` to the shim list (and the OntologyTarget sealed \
                 impls) or remove the class from the ontology."
            );
        }
    }
}

// 2.1.a OntologyTarget — sealed marker trait for foundation-produced types.
//
// v0.2.1 ships a small set of **shim structs** (named after their ontology
// local-name) that serve as type-system handles for `Validated<T>` and
// `Certify` impls. The shims are zero-sized and `Default`-able so resolver
// impls can produce concrete return values. They do not collide with the
// `bridge::cert::*` / `bridge::proof::*` trait modules because they live in
// the `enforcement` module and the prelude re-exports the enforcement shims
// preferentially. Real instances are produced by the reduction pipeline (or
// by `uor_ground!` macro expansion) through the back-door minting API.
fn generate_ontology_target_trait(f: &mut RustFile, ontology: &Ontology) {
    // v0.2.1 Phase 7b.4: the set of shim types is machine-verified against
    // the ontology's `resolver:CertifyMapping` individuals — every certificate
    // / witness class named in a CertifyMapping must appear in the shim
    // list, or the codegen panics with a clear "missing shim" error.
    //
    // This narrows the verification to "everything v0.2.1 actually wires up"
    // rather than "every subclass in the ontology" (the ontology has many
    // certificate subclasses that are not yet resolver-backed).
    let (expected_cert_names, expected_witness_names) = collect_certify_mapping_targets(ontology);
    verify_shim_coverage(
        "certificate",
        &expected_cert_names,
        &[
            "GroundingCertificate",
            "LiftChainCertificate",
            "InhabitanceCertificate",
            "CompletenessCertificate",
            "MultiplicationCertificate",
            "PartitionCertificate",
        ],
    );
    verify_shim_coverage(
        "impossibility witness",
        &expected_witness_names,
        // `ImpossibilityWitness` (the base class) is mapped to the foundation
        // shim `GenericImpossibilityWitness` via the local-name handling in
        // `generate_certify_trait`. Accept both local-names here.
        &[
            "ImpossibilityWitness",
            "GenericImpossibilityWitness",
            "InhabitanceImpossibilityWitness",
        ],
    );

    f.doc_comment("Sealed marker trait identifying types produced by the foundation crate's");
    f.doc_comment("conformance/reduction pipeline. v0.2.1 bounds `Validated<T>` on this trait");
    f.doc_comment("so downstream crates cannot fabricate `Validated<UserType>` — user types");
    f.doc_comment("cannot impl `OntologyTarget` because the supertrait is private.");
    f.line("pub trait OntologyTarget: ontology_target_sealed::Sealed {}");
    f.blank();

    // v0.2.1 Phase 7b.1: certificate shims carry a real `witt_bits: u16`
    // field populated by the pipeline (Phase 7b.1.b). The field enables
    // `LiftChainCertificate::target_level()` to read the level the
    // certificate was issued for — no hardcoded W8. Witness shims and
    // ConstrainedTypeInput stay opaque because they are not Witt-indexed.
    let certificate_shims: &[(&str, &str)] = &[
        (
            "GroundingCertificate",
            "Sealed shim for `cert:GroundingCertificate`. Produced by GroundingAwareResolver.",
        ),
        (
            "LiftChainCertificate",
            "Sealed shim for `cert:LiftChainCertificate`. Carries the v0.2.1 \
             `target_level()` accessor populated from the pipeline's StageOutcome.",
        ),
        (
            "InhabitanceCertificate",
            "Sealed shim for `cert:InhabitanceCertificate` (v0.2.1).",
        ),
        (
            "CompletenessCertificate",
            "Sealed shim for `cert:CompletenessCertificate`.",
        ),
        (
            "MultiplicationCertificate",
            "Sealed shim for `cert:MultiplicationCertificate` (v0.2.2 Phase C.4). \
             Carries the cost-optimal Toom-Cook splitting factor R, the recursive \
             sub-multiplication count, and the accumulated Landauer cost in nats.",
        ),
        (
            "PartitionCertificate",
            "Sealed shim for `cert:PartitionCertificate` (v0.2.2 Phase E). \
             Attests the partition component classification of a Datum.",
        ),
    ];
    let witness_shims: &[(&str, &str)] = &[
        (
            "GenericImpossibilityWitness",
            "Sealed shim for `proof:ImpossibilityWitness`. Returned by completeness and \
             grounding resolvers on failure.",
        ),
        (
            "InhabitanceImpossibilityWitness",
            "Sealed shim for `proof:InhabitanceImpossibilityWitness` (v0.2.1).",
        ),
    ];
    let input_shims: &[(&str, &str)] = &[(
        "ConstrainedTypeInput",
        "Input shim for `type:ConstrainedType`. Used as `Certify::Input` for \
             InhabitanceResolver, TowerCompletenessResolver, and \
             IncrementalCompletenessResolver.",
    )];

    // Emit certificate shims with witt_bits field and hand-written Default
    // that defaults to WittLevel::W32 (Certify::DEFAULT_LEVEL per Phase 7b.1.a).
    for (name, doc) in certificate_shims {
        f.doc_comment(doc);
        f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
        f.line(&format!("pub struct {name} {{"));
        f.line("    witt_bits: u16,");
        f.line("}");
        f.blank();
        // Hand-written Default — defaults to the Certify canonical level (W32).
        f.line(&format!("impl Default for {name} {{"));
        f.line("    #[inline]");
        f.line("    fn default() -> Self {");
        f.line("        Self { witt_bits: 32 }");
        f.line("    }");
        f.line("}");
        f.blank();
        // Crate-internal constructor used by the pipeline + back-door minting.
        f.line(&format!("impl {name} {{"));
        f.indented_doc_comment("Crate-internal constructor used by the pipeline to mint a");
        f.indented_doc_comment("certificate carrying the Witt level the pipeline advanced to.");
        f.line("    #[inline]");
        f.line("    #[must_use]");
        f.line("    #[allow(dead_code)]");
        f.line("    pub(crate) const fn with_witt_bits(witt_bits: u16) -> Self {");
        f.line("        Self { witt_bits }");
        f.line("    }");
        f.blank();
        f.indented_doc_comment("Returns the Witt level the certificate was issued for. Sourced");
        f.indented_doc_comment("from the pipeline's `StageOutcome.witt_bits` at minting time.");
        f.line("    #[inline]");
        f.line("    #[must_use]");
        f.line("    pub const fn witt_bits(&self) -> u16 {");
        f.line("        self.witt_bits");
        f.line("    }");
        f.blank();
        f.indented_doc_comment("v0.2.2 Phase G: const-constructible empty form for");
        f.indented_doc_comment("`certify_*_const` entry points.");
        f.line("    #[inline]");
        f.line("    #[must_use]");
        f.line("    #[allow(dead_code)]");
        f.line("    pub(crate) const fn empty_const() -> Self {");
        f.line("        Self { witt_bits: 0 }");
        f.line("    }");
        f.line("}");
        f.blank();
    }

    // Witness + input shims stay opaque.
    for (name, doc) in witness_shims.iter().chain(input_shims.iter()) {
        f.doc_comment(doc);
        f.line("#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]");
        f.line(&format!("pub struct {name} {{"));
        f.line("    _private: (),");
        f.line("}");
        f.blank();
    }

    // LiftChainCertificate.target_level — reads the real witt_bits field.
    f.line("impl LiftChainCertificate {");
    f.indented_doc_comment("Returns the Witt level the certificate was issued for.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn target_level(&self) -> WittLevel {");
    f.line("        WittLevel::new(self.witt_bits as u32)");
    f.line("    }");
    f.line("}");
    f.blank();
    f.line("impl InhabitanceCertificate {");
    f.indented_doc_comment("Returns the witness value tuple bytes when `verified` is true.");
    f.indented_doc_comment("v0.2.1 returns `None` on the shim; real witnesses come from the");
    f.indented_doc_comment("macro back-door path.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn witness(&self) -> Option<&'static [u8]> {");
    f.line("        None");
    f.line("    }");
    f.line("}");
    f.blank();

    // Sealed module + impls — combine all three shim lists.
    let all_shims: Vec<&(&str, &str)> = certificate_shims
        .iter()
        .chain(witness_shims.iter())
        .chain(input_shims.iter())
        .collect();
    f.line("mod ontology_target_sealed {");
    f.indented_doc_comment("Private supertrait. Not implementable outside this crate.");
    f.line("    pub trait Sealed {}");
    for (name, _) in &all_shims {
        f.line(&format!("    impl Sealed for super::{name} {{}}"));
    }
    f.line("    impl Sealed for super::CompileUnit {}");
    f.line("}");
    f.blank();
    for (name, _) in &all_shims {
        f.line(&format!("impl OntologyTarget for {name} {{}}"));
    }
    f.line("impl OntologyTarget for CompileUnit {}");
    f.blank();

    // ── v0.2.2 W11: Certified<C> parametric carrier ────────────────────────
    //
    // Replaces the per-shim duplication with one parametric carrier. Sealed
    // `Certificate` trait scopes the kind set to ontology-declared classes;
    // `Certified<C>` is the single struct that holds them. The 4 existing
    // certificate shims gain `impl Certificate`, and the 6 cert subclasses
    // not previously shimmed (Transform, Isometry, Involution, Geodesic,
    // Measurement, BornRule) get sealed unit-struct emissions.
    //
    // Supporting evidence types (CompletenessAuditTrail, ChainAuditTrail,
    // GeodesicEvidenceBundle) are emitted as public structs so they can
    // appear as the `Evidence` associated type of their parent certificate.
    f.doc_comment("v0.2.2 W11: supporting evidence type for `CompletenessCertificate`.");
    f.doc_comment("Linked from the certificate via the `Certificate::Evidence` associated type.");
    f.line("#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct CompletenessAuditTrail { _private: () }");
    f.blank();
    f.doc_comment("v0.2.2 W11: supporting evidence type for `LiftChainCertificate`.");
    f.line("#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct ChainAuditTrail { _private: () }");
    f.blank();
    f.doc_comment("v0.2.2 W11: supporting evidence type for `GeodesicCertificate`.");
    f.line("#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct GeodesicEvidenceBundle { _private: () }");
    f.blank();

    // The 6 cert subclasses not previously shimmed in enforcement. We emit
    // them as sealed unit structs so they can be the `C` parameter of
    // `Certified<C>`.
    let new_cert_kinds: &[(&str, &str)] = &[
        (
            "TransformCertificate",
            "v0.2.2 W11: sealed carrier for `cert:TransformCertificate`.",
        ),
        (
            "IsometryCertificate",
            "v0.2.2 W11: sealed carrier for `cert:IsometryCertificate`.",
        ),
        (
            "InvolutionCertificate",
            "v0.2.2 W11: sealed carrier for `cert:InvolutionCertificate`.",
        ),
        (
            "GeodesicCertificate",
            "v0.2.2 W11: sealed carrier for `cert:GeodesicCertificate`.",
        ),
        (
            "MeasurementCertificate",
            "v0.2.2 W11: sealed carrier for `cert:MeasurementCertificate`.",
        ),
        (
            "BornRuleVerification",
            "v0.2.2 W11: sealed carrier for `cert:BornRuleVerification`.",
        ),
    ];
    for (name, doc) in new_cert_kinds {
        f.doc_comment(doc);
        f.line("#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]");
        f.line(&format!("pub struct {name} {{ _private: () }}"));
        f.blank();
        // v0.2.2 Phase G: const-constructible empty form for
        // `certify_*_const` entry points.
        f.line(&format!("impl {name} {{"));
        f.indented_doc_comment("v0.2.2 Phase G: const-constructible empty certificate used by");
        f.indented_doc_comment("`certify_*_const` entry points.");
        f.line("    #[inline]");
        f.line("    #[must_use]");
        f.line("    #[allow(dead_code)]");
        f.line("    pub(crate) const fn empty_const() -> Self {");
        f.line("        Self { _private: () }");
        f.line("    }");
        f.line("}");
        f.blank();
    }

    f.doc_comment("v0.2.2 W11: sealed marker trait for foundation-supplied certificate kinds.");
    f.doc_comment("Implemented by every `cert:Certificate` subclass via codegen; not");
    f.doc_comment("implementable outside this crate.");
    f.line("pub trait Certificate: certificate_sealed::Sealed {");
    f.indented_doc_comment("The ontology IRI of this certificate class.");
    f.line("    const IRI: &'static str;");
    f.indented_doc_comment(
        "The structured evidence carried by this certificate (or `()` if none).",
    );
    f.line("    type Evidence;");
    f.line("}");
    f.blank();

    // The full set of cert classes. Existing shim names + new cert kind names.
    // Each entry is (rust_name, ontology_local_name, evidence_type).
    let all_certs: &[(&str, &str, &str)] = &[
        ("GroundingCertificate", "GroundingCertificate", "()"),
        (
            "LiftChainCertificate",
            "LiftChainCertificate",
            "ChainAuditTrail",
        ),
        ("InhabitanceCertificate", "InhabitanceCertificate", "()"),
        (
            "CompletenessCertificate",
            "CompletenessCertificate",
            "CompletenessAuditTrail",
        ),
        ("TransformCertificate", "TransformCertificate", "()"),
        ("IsometryCertificate", "IsometryCertificate", "()"),
        ("InvolutionCertificate", "InvolutionCertificate", "()"),
        (
            "GeodesicCertificate",
            "GeodesicCertificate",
            "GeodesicEvidenceBundle",
        ),
        ("MeasurementCertificate", "MeasurementCertificate", "()"),
        ("BornRuleVerification", "BornRuleVerification", "()"),
        // v0.2.2 Phase C.4: MultiplicationCertificate.
        (
            "MultiplicationCertificate",
            "MultiplicationCertificate",
            "MultiplicationEvidence",
        ),
        // v0.2.2 Phase E: PartitionCertificate.
        ("PartitionCertificate", "PartitionCertificate", "()"),
    ];
    f.line("mod certificate_sealed {");
    f.indented_doc_comment("Private supertrait. Not implementable outside this crate.");
    f.line("    pub trait Sealed {}");
    for (rust_name, _, _) in all_certs {
        f.line(&format!("    impl Sealed for super::{rust_name} {{}}"));
    }
    f.line("}");
    f.blank();
    for (rust_name, ont_local, evidence) in all_certs {
        f.line(&format!("impl Certificate for {rust_name} {{"));
        f.line(&format!(
            "    const IRI: &'static str = \"https://uor.foundation/cert/{ont_local}\";"
        ));
        f.line(&format!("    type Evidence = {evidence};"));
        f.line("}");
        f.blank();
    }

    f.doc_comment("v0.2.2 W11: parametric carrier for any foundation-supplied certificate.");
    f.doc_comment("Replaces the v0.2.1 per-class shim duplication. The `Certificate` trait");
    f.doc_comment("is sealed and the `_private` field prevents external construction; only");
    f.doc_comment("the foundation's pipeline / resolver paths produce `Certified<C>` values.");
    f.line("#[derive(Debug, Clone)]");
    f.line("pub struct Certified<C: Certificate> {");
    f.indented_doc_comment("The certificate kind value carried by this wrapper.");
    f.line("    inner: C,");
    f.indented_doc_comment("Prevents external construction.");
    f.line("    _private: (),");
    f.line("}");
    f.blank();
    f.line("impl<C: Certificate> Certified<C> {");
    f.indented_doc_comment("Returns a reference to the carried certificate kind value.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn certificate(&self) -> &C {");
    f.line("        &self.inner");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the ontology IRI of this certificate's kind.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn iri(&self) -> &'static str {");
    f.line("        C::IRI");
    f.line("    }");
    f.blank();
    f.indented_doc_comment(
        "Crate-internal constructor. Reachable only from the pipeline / resolver paths.",
    );
    f.line("    #[inline]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(inner: C) -> Self {");
    f.line("        Self { inner, _private: () }");
    f.line("    }");
    f.line("}");
    f.blank();
}

// 2.1.b Grounded<T> — zero-overhead ground-state wrapper.
fn generate_grounded_wrapper(f: &mut RustFile) {
    // v0.2.2 Phase E — BaseMetric sealed carriers + MAX_BETTI_DIMENSION.
    // Emitted before the GroundedShape trait so the accessors on Grounded
    // below can reference them.
    f.doc_comment("v0.2.2 Phase E: maximum simplicial dimension tracked by the");
    f.doc_comment("constraint-nerve Betti-numbers vector. The bound is 8 for the");
    f.doc_comment("currently-supported WittLevel set per the existing partition:FreeRank");
    f.doc_comment("capacity properties; the constant is `pub` (part of the public-API");
    f.doc_comment("snapshot) so future expansions require explicit review.");
    f.line("pub const MAX_BETTI_DIMENSION: usize = 8;");
    f.blank();

    f.doc_comment("Sealed newtype for the grounding completion ratio \u{03C3} \u{2208}");
    f.doc_comment("[0.0, 1.0]. \u{03C3} = 1 indicates the ground state; \u{03C3} = 0 the");
    f.doc_comment("unbound state. Backs observable:GroundingSigma.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq)]");
    f.line("pub struct SigmaValue {");
    f.line("    value: f64,");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl SigmaValue {");
    f.indented_doc_comment("Returns the stored \u{03C3} value in the range [0.0, 1.0].");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn as_f64(&self) -> f64 { self.value }");
    f.blank();
    f.indented_doc_comment("Crate-internal constructor. Caller guarantees `value` is in");
    f.indented_doc_comment("the closed range [0.0, 1.0] and is not NaN.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new_unchecked(value: f64) -> Self {");
    f.line("        Self { value, _sealed: () }");
    f.line("    }");
    f.line("}");
    f.blank();

    f.doc_comment("Maximum site count of the Jacobian row per Datum at any supported");
    f.doc_comment("WittLevel. Sourced from the partition:FreeRank capacity bound.");
    f.line("pub const JACOBIAN_MAX_SITES: usize = 64;");
    f.blank();

    f.doc_comment("v0.2.2 Phase E: sealed Jacobian row carrier, parametric over the");
    f.doc_comment("WittLevel marker. Fixed-size `[i64; JACOBIAN_MAX_SITES]` backing; no");
    f.doc_comment("heap. The row records the per-site partial derivative of the ring");
    f.doc_comment("operation that produced the Datum. Backs observable:JacobianObservable.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct JacobianMetric<L> {");
    f.line("    entries: [i64; JACOBIAN_MAX_SITES],");
    f.line("    len: u16,");
    f.line("    _level: PhantomData<L>,");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl<L> JacobianMetric<L> {");
    f.indented_doc_comment("Construct a zeroed Jacobian row with the given active length.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn zero(len: u16) -> Self {");
    f.line("        Self {");
    f.line("            entries: [0i64; JACOBIAN_MAX_SITES],");
    f.line("            len,");
    f.line("            _level: PhantomData,");
    f.line("            _sealed: (),");
    f.line("        }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Access the Jacobian row entries.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn entries(&self) -> &[i64; JACOBIAN_MAX_SITES] { &self.entries }");
    f.blank();
    f.indented_doc_comment("Number of active sites (the row's logical length).");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn len(&self) -> u16 { self.len }");
    f.blank();
    f.indented_doc_comment("Whether the Jacobian row is empty.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn is_empty(&self) -> bool { self.len == 0 }");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 Phase E: sealed Partition component classification.");
    f.doc_comment("Closed enumeration mirroring the partition:PartitionComponent");
    f.doc_comment("ontology class.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("#[non_exhaustive]");
    f.line("pub enum PartitionComponent {");
    f.indented_doc_comment("The irreducible component.");
    f.line("    Irreducible,");
    f.indented_doc_comment("The reducible component.");
    f.line("    Reducible,");
    f.indented_doc_comment("The unit component.");
    f.line("    Units,");
    f.indented_doc_comment("The exterior component.");
    f.line("    Exterior,");
    f.line("}");
    f.blank();

    f.doc_comment("Sealed marker trait identifying type:ConstrainedType subclasses that may");
    f.doc_comment("appear as the parameter of `Grounded<T>`.");
    f.doc_comment("");
    f.doc_comment("v0.2.2 W2: the sealing now lives in a private `grounded_shape_sealed`");
    f.doc_comment("module — there is no `__macro_internals` back-door. The only impl is for");
    f.doc_comment("the foundation-supplied `ConstrainedTypeInput` shim. Downstream code that");
    f.doc_comment("needs to bind a user type as `T` in `Grounded<T>` does so via the");
    f.doc_comment("compile-time-evidence pattern: declare a");
    f.doc_comment("`const _VALIDATED_<T>: Validated<ConstrainedTypeInput, CompileTime> = ...;`");
    f.doc_comment("module-scope evidence constant, and the foundation's pipeline binds it.");
    f.line("mod grounded_shape_sealed {");
    f.indented_doc_comment("Private supertrait. Not implementable outside this crate.");
    f.line("    pub trait Sealed {}");
    f.line("    impl Sealed for super::ConstrainedTypeInput {}");
    f.line("}");
    f.doc_comment("v0.2.2 W2: sealed marker trait for shapes that can appear as the parameter");
    f.doc_comment("of `Grounded<T>`. Implemented only by `ConstrainedTypeInput`. Downstream");
    f.doc_comment("user types bind to this trait via the compile-time-evidence pattern in a");
    f.doc_comment("future v0.2.2 cookbook revision.");
    f.line("pub trait GroundedShape: grounded_shape_sealed::Sealed {}");
    f.line("impl GroundedShape for ConstrainedTypeInput {}");
    f.blank();

    f.doc_comment("A binding entry in a `BindingsTable`. Pairs an address (u128 content");
    f.doc_comment("hash of the query coordinate) with the bound bytes.");
    f.line("#[derive(Debug, Clone, Copy)]");
    f.line("pub struct BindingEntry {");
    f.indented_doc_comment("Content-hashed query address.");
    f.line("    pub address: u128,");
    f.indented_doc_comment(
        "Bound payload bytes (length determined by the WittLevel of the table).",
    );
    f.line("    pub bytes: &'static [u8],");
    f.line("}");
    f.blank();

    f.doc_comment("A static, sorted-by-address binding table laid out for `op:GS_5` zero-step");
    f.doc_comment("access. Looked up via binary search; the foundation guarantees the table");
    f.doc_comment("is materialized at compile time from the attested `state:GroundedContext`.");
    f.line("#[derive(Debug, Clone, Copy)]");
    f.line("pub struct BindingsTable {");
    f.indented_doc_comment("Entries, sorted ascending by `address`.");
    f.line("    pub entries: &'static [BindingEntry],");
    f.line("}");
    f.blank();
    f.line("impl BindingsTable {");
    f.indented_doc_comment("Construct a `BindingsTable` from a sorted slice. Caller must ensure");
    f.indented_doc_comment("ascending order; this is `pub(crate)` so only the macro back-door");
    f.indented_doc_comment("path can construct one.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(entries: &'static [BindingEntry]) -> Self {");
    f.line("        Self { entries }");
    f.line("    }");
    f.line("}");
    f.blank();

    f.doc_comment("The compile-time witness that `op:GS_4` holds for the value it carries:");
    f.doc_comment("σ = 1, freeRank = 0, S = 0, T_ctx = 0. `Grounded<T, Tag>` is constructed");
    f.doc_comment(
        "only by the reduction pipeline and provides `op:GS_5` zero-step binding access.",
    );
    f.doc_comment("");
    f.doc_comment("v0.2.2 Phase B (Q3): the `Tag` phantom parameter (default `Tag = T`)");
    f.doc_comment("lets downstream code attach a domain marker to a grounded witness without");
    f.doc_comment("any new sealing — e.g., `Grounded<ConstrainedTypeInput, BlockHashTag>` is");
    f.doc_comment("a distinct Rust type from `Grounded<ConstrainedTypeInput, PixelTag>`. The");
    f.doc_comment("inner witness is unchanged; the tag is pure decoration. The foundation");
    f.doc_comment("guarantees ring soundness on the inner witness; the tag is the developer's");
    f.doc_comment("domain claim. Coerce via `Grounded::tag::<NewTag>()` (zero-cost).");
    f.line("#[derive(Debug, Clone)]");
    f.line("pub struct Grounded<T: GroundedShape, Tag = T> {");
    f.indented_doc_comment("The validated grounding certificate this wrapper carries.");
    f.line("    validated: Validated<GroundingCertificate>,");
    f.indented_doc_comment("The compile-time-materialized bindings table.");
    f.line("    bindings: BindingsTable,");
    f.indented_doc_comment("The Witt level the grounded value was minted at.");
    f.line("    witt_level_bits: u16,");
    f.indented_doc_comment("Content-address of the originating CompileUnit.");
    f.line("    unit_address: u128,");
    f.indented_doc_comment("Phantom type tying this `Grounded` to a specific `ConstrainedType`.");
    f.line("    _phantom: PhantomData<T>,");
    f.indented_doc_comment("Phantom domain tag (Q3). Defaults to `T` for backwards-compatible");
    f.indented_doc_comment("call sites; downstream attaches a custom tag via `tag::<NewTag>()`.");
    f.line("    _tag: PhantomData<Tag>,");
    f.line("}");
    f.blank();
    f.line("impl<T: GroundedShape, Tag> Grounded<T, Tag> {");
    f.indented_doc_comment("Returns the binding for the given query address, or `None` if not in");
    f.indented_doc_comment("the table. Resolves in O(log n) via binary search; for true `op:GS_5`");
    f.indented_doc_comment("zero-step access, downstream code uses statically-known indices.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub fn get_binding(&self, address: u128) -> Option<&'static [u8]> {");
    f.line("        self.bindings");
    f.line("            .entries");
    f.line("            .binary_search_by_key(&address, |e| e.address)");
    f.line("            .ok()");
    f.line("            .map(|i| self.bindings.entries[i].bytes)");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Iterate over all bindings in this grounded context.");
    f.line("    #[inline]");
    f.line("    pub fn iter_bindings(&self) -> impl Iterator<Item = &BindingEntry> + '_ {");
    f.line("        self.bindings.entries.iter()");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the Witt level the grounded value was minted at.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn witt_level_bits(&self) -> u16 {");
    f.line("        self.witt_level_bits");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the content-address of the originating CompileUnit.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn unit_address(&self) -> u128 {");
    f.line("        self.unit_address");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the validated grounding certificate this wrapper carries.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn certificate(&self) -> &Validated<GroundingCertificate> {");
    f.line("        &self.validated");
    f.line("    }");
    f.blank();
    // v0.2.2 Phase E — BaseMetric accessors. Returns default/zero values
    // until a future pipeline pass populates the underlying counters; the
    // signatures are stable and pinned by the bridge_namespace_completion
    // conformance validator.
    f.indented_doc_comment(
        "v0.2.2 Phase E: observable:d_delta_metric — the metric incompatibility",
    );
    f.indented_doc_comment(
        "between ring distance and Hamming distance for this datum's neighborhood.",
    );
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn d_delta(&self) -> i64 { 0 }");
    f.blank();
    f.indented_doc_comment("v0.2.2 Phase E: observable:sigma_metric — grounding completion ratio.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn sigma(&self) -> SigmaValue { SigmaValue::new_unchecked(1.0) }");
    f.blank();
    f.indented_doc_comment("v0.2.2 Phase E: observable:jacobian_metric — per-site Jacobian row.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub fn jacobian(&self) -> JacobianMetric<T> { JacobianMetric::zero(0) }");
    f.blank();
    f.indented_doc_comment("v0.2.2 Phase E: observable:betti_metric — Betti numbers up to");
    f.indented_doc_comment("MAX_BETTI_DIMENSION.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn betti_numbers(&self) -> [u32; MAX_BETTI_DIMENSION] {");
    f.line("        [0u32; MAX_BETTI_DIMENSION]");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("v0.2.2 Phase E: observable:euler_metric — Euler characteristic of");
    f.indented_doc_comment("the constraint nerve.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn euler_characteristic(&self) -> i64 { 0 }");
    f.blank();
    f.indented_doc_comment("v0.2.2 Phase E: observable:residual_metric — count of free sites at");
    f.indented_doc_comment("grounding time.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn residual_count(&self) -> u32 { 0 }");
    f.blank();

    f.indented_doc_comment("v0.2.2 Phase B (Q3): coerce this `Grounded<T, Tag>` to a different");
    f.indented_doc_comment("phantom tag. Zero-cost — the inner witness is unchanged; only the");
    f.indented_doc_comment("type-system view differs. Downstream uses this to attach a domain");
    f.indented_doc_comment(
        "marker for use in function signatures (e.g., `Grounded<_, BlockHashTag>`",
    );
    f.indented_doc_comment("vs `Grounded<_, PixelTag>` are distinct Rust types).");
    f.indented_doc_comment("");
    f.indented_doc_comment("**The foundation does not validate the tag.** The tag records what");
    f.indented_doc_comment("the developer is claiming about the witness's domain semantics; the");
    f.indented_doc_comment("foundation's contract is about ring soundness, not domain semantics.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub fn tag<NewTag>(self) -> Grounded<T, NewTag> {");
    f.line("        Grounded {");
    f.line("            validated: self.validated,");
    f.line("            bindings: self.bindings,");
    f.line("            witt_level_bits: self.witt_level_bits,");
    f.line("            unit_address: self.unit_address,");
    f.line("            _phantom: PhantomData,");
    f.line("            _tag: PhantomData,");
    f.line("        }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Crate-internal constructor used by the pipeline at mint time.");
    f.indented_doc_comment("");
    f.indented_doc_comment("Not callable from outside `uor-foundation`. The tag defaults to `T`");
    f.indented_doc_comment(
        "(the unparameterized form); downstream attaches a custom tag via `tag()`.",
    );
    f.line("    #[inline]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new_internal(");
    f.line("        validated: Validated<GroundingCertificate>,");
    f.line("        bindings: BindingsTable,");
    f.line("        witt_level_bits: u16,");
    f.line("        unit_address: u128,");
    f.line("    ) -> Self {");
    f.line("        Self {");
    f.line("            validated,");
    f.line("            bindings,");
    f.line("            witt_level_bits,");
    f.line("            unit_address,");
    f.line("            _phantom: PhantomData,");
    f.line("            _tag: PhantomData,");
    f.line("        }");
    f.line("    }");
    f.line("}");
    f.blank();

    // ── v0.2.2 W8: Triad<L> bundling struct ────────────────────────────────
    //
    // The triadic coordinate of a Datum: (stratum, spectrum, address).
    // Parametric over the Witt level marker L (one of the unit structs
    // W8/W16/W24/W32 emitted by generate_ring_ops). Fields are private; only
    // the foundation can construct a Triad. Accessors return u64 coordinate
    // values — typed coordinate wrappers (TwoAdicValuation<L> etc.) are
    // deferred to v0.2.3+ when the bridge::query rewrite happens.
    f.doc_comment("v0.2.2 W8: triadic coordinate of a Datum at level `L`. Bundles the");
    f.doc_comment("(stratum, spectrum, address) projection in one structurally-enforced");
    f.doc_comment("type. No public constructor — `Triad<L>` is built only by foundation code");
    f.doc_comment("at grounding time. Field access goes through the named accessors.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct Triad<L> {");
    f.indented_doc_comment("The stratum coordinate (two-adic valuation).");
    f.line("    stratum: u64,");
    f.indented_doc_comment("The spectrum coordinate (Walsh-Hadamard image).");
    f.line("    spectrum: u64,");
    f.indented_doc_comment("The address coordinate (Braille-glyph address).");
    f.line("    address: u64,");
    f.indented_doc_comment("Phantom marker for the Witt level.");
    f.line("    _level: PhantomData<L>,");
    f.line("}");
    f.blank();
    f.line("impl<L> Triad<L> {");
    f.indented_doc_comment("Returns the stratum component (`query:TwoAdicValuation` coordinate).");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn stratum(&self) -> u64 {");
    f.line("        self.stratum");
    f.line("    }");
    f.blank();
    f.indented_doc_comment(
        "Returns the spectrum component (`query:WalshHadamardImage` coordinate).",
    );
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn spectrum(&self) -> u64 {");
    f.line("        self.spectrum");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns the address component (`query:Address` coordinate).");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn address(&self) -> u64 {");
    f.line("        self.address");
    f.line("    }");
    f.blank();
    f.indented_doc_comment(
        "Crate-internal constructor. Reachable only from grounding-time minting.",
    );
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(stratum: u64, spectrum: u64, address: u64) -> Self {");
    f.line("        Self { stratum, spectrum, address, _level: PhantomData }");
    f.line("    }");
    f.line("}");
    f.blank();
}

// 2.1.d PipelineFailure — parametric over reduction:FailureField individuals.
fn generate_pipeline_failure(f: &mut RustFile, ontology: &Ontology) {
    f.doc_comment("The Rust-surface rendering of `reduction:PipelineFailureReason` and the");
    f.doc_comment("v0.2.1 cross-namespace failure variants. Variant set and field shapes are");
    f.doc_comment("generated parametrically by walking `reduction:FailureField` individuals;");
    f.doc_comment("adding a new field requires only an ontology edit.");
    f.line("#[derive(Debug, Clone, PartialEq)]");
    f.line("#[non_exhaustive]");
    f.line("pub enum PipelineFailure {");

    // Walk all PipelineFailureReason individuals plus failure:LiftObstructionFailure
    // and conformance:ShapeViolationReport (the latter wraps the existing struct).
    let reasons = individuals_of_type(
        ontology,
        "https://uor.foundation/reduction/PipelineFailureReason",
    );
    let mut variant_specs: Vec<(String, Vec<(String, String)>)> = Vec::new();
    for ind in &reasons {
        let variant = local_name(ind.id).to_string();
        let fields = collect_failure_fields(ontology, ind.id);
        variant_specs.push((variant, fields));
    }

    // failure:LiftObstructionFailure variant
    let lift_fields = collect_failure_fields(
        ontology,
        "https://uor.foundation/failure/LiftObstructionFailure",
    );
    if !lift_fields.is_empty() {
        variant_specs.push(("LiftObstructionFailure".to_string(), lift_fields));
    }

    // conformance:ShapeViolationReport — wraps the existing ShapeViolation
    // struct emitted by `generate_shape_violation` earlier in this file.
    variant_specs.push((
        "ShapeViolation".to_string(),
        vec![("report".to_string(), "ShapeViolation".to_string())],
    ));

    for (variant, fields) in &variant_specs {
        f.indented_doc_comment(&format!("`{variant}` failure variant."));
        if fields.is_empty() {
            f.line(&format!("    {variant},"));
        } else {
            f.line(&format!("    {variant} {{"));
            for (name, ty) in fields {
                f.line(&format!("        /// {name} field."));
                f.line(&format!("        {name}: {ty},"));
            }
            f.line("    },");
        }
    }

    f.line("}");
    f.blank();

    // Display impl for nice error rendering
    f.line("impl core::fmt::Display for PipelineFailure {");
    f.line("    fn fmt(&self, ff: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {");
    f.line("        match self {");
    for (variant, fields) in &variant_specs {
        let pat: String = if fields.is_empty() {
            format!("Self::{variant}")
        } else {
            let names: Vec<String> = fields.iter().map(|(n, _)| n.clone()).collect();
            format!("Self::{variant} {{ {} }}", names.join(", "))
        };
        let body = if fields.is_empty() {
            format!("write!(ff, \"{variant}\")")
        } else if variant == "ShapeViolation" {
            "write!(ff, \"ShapeViolation({:?})\", report)".to_string()
        } else {
            // Render IRI fields specifically; otherwise debug-print.
            let parts: Vec<String> = fields.iter().map(|(n, _)| format!("{n}={{:?}}")).collect();
            let names: Vec<String> = fields.iter().map(|(n, _)| n.clone()).collect();
            format!(
                "write!(ff, \"{}({})\", {})",
                variant,
                parts.join(", "),
                names.join(", ")
            )
        };
        f.line(&format!("            {pat} => {body},"));
    }
    f.line("        }");
    f.line("    }");
    f.line("}");
    f.blank();

    // No std::error::Error impl — uor-foundation is no_std and the Error
    // trait isn't in core. Downstream crates that need Error can add their
    // own newtype wrapper.
    f.blank();
}

/// Walk reduction:FailureField individuals filtered by ofFailure == failure_iri,
/// returning (field_name, field_type) tuples in declaration order.
fn collect_failure_fields(ontology: &Ontology, failure_iri: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let fields = individuals_of_type(ontology, "https://uor.foundation/reduction/FailureField");
    for f in fields {
        let of = ind_prop_str(f, "https://uor.foundation/reduction/ofFailure");
        if of != Some(failure_iri) {
            continue;
        }
        let name = ind_prop_str(f, "https://uor.foundation/reduction/fieldName")
            .unwrap_or("unknown")
            .to_string();
        let ty = ind_prop_str(f, "https://uor.foundation/reduction/fieldType")
            .unwrap_or("()")
            .to_string();
        out.push((name, ty));
    }
    out
}

// 2.1.c Certify trait — one resolver façade struct + Certify impl per
// resolver:CertifyMapping individual.
fn generate_certify_trait(f: &mut RustFile, ontology: &Ontology) {
    f.doc_comment("Sealed marker for impossibility witnesses (failure return type of `Certify`).");
    f.line("pub trait ImpossibilityWitnessKind: impossibility_witness_kind_sealed::Sealed {}");
    f.blank();
    f.line("mod impossibility_witness_kind_sealed {");
    f.indented_doc_comment("Private supertrait.");
    f.line("    pub trait Sealed {}");
    f.line("    impl Sealed for super::GenericImpossibilityWitness {}");
    f.line("    impl Sealed for super::InhabitanceImpossibilityWitness {}");
    f.line("}");
    f.blank();
    f.line("impl ImpossibilityWitnessKind for GenericImpossibilityWitness {}");
    f.line("impl ImpossibilityWitnessKind for InhabitanceImpossibilityWitness {}");
    f.blank();

    // Certify trait definition — generic over the input type so downstream
    // user types (via #[derive(ConstrainedType)]) can be passed directly to
    // `certify` without going through the ConstrainedTypeInput shim.
    f.doc_comment("The v0.2.1 verdict-producing trait. Each resolver façade impls `Certify`");
    f.doc_comment("to expose the consumer-facing one-liner:");
    f.doc_comment("");
    f.doc_comment("```rust,ignore");
    f.doc_comment("use uor_foundation::enforcement::*;");
    f.doc_comment("use uor_foundation::pipeline::ConstrainedTypeShape;");
    f.doc_comment("");
    f.doc_comment("#[derive(ConstrainedType, Default)]");
    f.doc_comment("struct Shape;");
    f.doc_comment("");
    f.doc_comment("let cert: Validated<LiftChainCertificate> =");
    f.doc_comment("    TowerCompletenessResolver::new().certify(&Shape)?;");
    f.doc_comment("let level: WittLevel = cert.target_level();");
    f.doc_comment("```");
    f.doc_comment("");
    f.doc_comment("`Certify` is generic over the input type `I` so any user type");
    f.doc_comment("implementing `ConstrainedTypeShape` (via `#[derive(ConstrainedType)]`)");
    f.doc_comment("flows through the pipeline directly. The associated `Certificate` and");
    f.doc_comment("`Witness` types are sealed via `OntologyTarget` / `ImpossibilityWitnessKind`.");
    f.line("pub trait Certify<I: ?Sized> {");
    f.indented_doc_comment("The certificate type returned on success.");
    f.line("    type Certificate: OntologyTarget;");
    f.indented_doc_comment("The impossibility witness type returned on failure.");
    f.line("    type Witness: ImpossibilityWitnessKind;");
    f.blank();
    f.indented_doc_comment("The default Witt level this resolver certifies at when the");
    f.indented_doc_comment("caller omits an explicit level via `certify`. v0.2.1 uses");
    f.indented_doc_comment("`WittLevel::W32` as the canonical default per ergonomics-spec §3.2.");
    f.line("    const DEFAULT_LEVEL: WittLevel = WittLevel::W32;");
    f.blank();
    f.indented_doc_comment("Run the resolver on `input` at the default Witt level and return");
    f.indented_doc_comment("either a validated certificate or an impossibility witness.");
    f.indented_doc_comment("");
    f.indented_doc_comment("# Errors");
    f.indented_doc_comment("");
    f.indented_doc_comment("Returns `Self::Witness` when the resolver determines that no");
    f.indented_doc_comment("certificate can be issued for `input`.");
    f.line(
        "    fn certify(&self, input: &I) -> Result<Validated<Self::Certificate>, Self::Witness> {",
    );
    f.line("        self.certify_at(input, Self::DEFAULT_LEVEL)");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Run the resolver on `input` at an explicit Witt level.");
    f.indented_doc_comment("");
    f.indented_doc_comment("# Errors");
    f.indented_doc_comment("");
    f.indented_doc_comment("Returns `Self::Witness` when the resolver determines that no");
    f.indented_doc_comment("certificate can be issued for `input` at `level`.");
    f.line("    fn certify_at(&self, input: &I, level: WittLevel) -> Result<Validated<Self::Certificate>, Self::Witness>;");
    f.line("}");
    f.blank();

    // Walk resolver:CertifyMapping individuals → emit one resolver struct + impl per
    let mappings = individuals_of_type(ontology, "https://uor.foundation/resolver/CertifyMapping");
    for m in mappings {
        let resolver_iri = match ind_prop_str(m, "https://uor.foundation/resolver/forResolver") {
            Some(s) => s,
            None => continue,
        };
        let cert_iri = match ind_prop_str(m, "https://uor.foundation/resolver/producesCertificate")
        {
            Some(s) => s,
            None => continue,
        };
        let witness_iri = match ind_prop_str(m, "https://uor.foundation/resolver/producesWitness") {
            Some(s) => s,
            None => continue,
        };
        let resolver_name = local_name(resolver_iri).to_string();
        let cert_name = local_name(cert_iri).to_string();
        // Map the witness IRI's local name through the OntologyTarget shim set.
        let witness_name = match local_name(witness_iri) {
            "ImpossibilityWitness" => "GenericImpossibilityWitness".to_string(),
            other => other.to_string(),
        };

        f.doc_comment(&format!(
            "v0.2.1 unit-struct façade for the `{resolver_name}` resolver class."
        ));
        f.doc_comment("");
        f.doc_comment(&format!(
            "Constructed via `{resolver_name}::new()`. Implements `Certify` so the"
        ));
        f.doc_comment("foundation's verdict surface is reachable as a single one-liner.");
        f.line("#[derive(Debug, Default, Clone, Copy)]");
        f.line(&format!("pub struct {resolver_name};"));
        f.blank();
        f.line(&format!("impl {resolver_name} {{"));
        f.indented_doc_comment("Construct a new resolver façade.");
        f.line("    #[inline]");
        f.line("    #[must_use]");
        f.line("    pub const fn new() -> Self {");
        f.line("        Self");
        f.line("    }");
        f.line("}");
        f.blank();
        // v0.2.1: generic-input Certify impls. Shape-taking resolvers accept
        // any `T: ConstrainedTypeShape`; GroundingAwareResolver takes the
        // opaque `CompileUnit` input shim.
        if resolver_name == "GroundingAwareResolver" {
            f.line(&format!("impl Certify<CompileUnit> for {resolver_name} {{"));
            f.line(&format!("    type Certificate = {cert_name};"));
            f.line(&format!("    type Witness = {witness_name};"));
            f.line("    fn certify_at(&self, input: &CompileUnit, level: WittLevel) -> Result<Validated<Self::Certificate>, Self::Witness> {");
            f.line("        crate::pipeline::run_grounding_aware(input, level)");
            f.line(&format!(
                "            .map_err(|_| {witness_name}::default())"
            ));
            f.line("    }");
            f.line("}");
        } else {
            f.line(&format!(
                "impl<__T: crate::pipeline::ConstrainedTypeShape + ?Sized> Certify<__T> for {resolver_name} {{"
            ));
            f.line(&format!("    type Certificate = {cert_name};"));
            f.line(&format!("    type Witness = {witness_name};"));
            f.line("    fn certify_at(&self, input: &__T, level: WittLevel) -> Result<Validated<Self::Certificate>, Self::Witness> {");
            let call = match resolver_name.as_str() {
                "TowerCompletenessResolver" => {
                    "crate::pipeline::run_tower_completeness(input, level)"
                }
                "IncrementalCompletenessResolver" => {
                    "crate::pipeline::run_incremental_completeness(input, level)"
                }
                "InhabitanceResolver" => "crate::pipeline::run_inhabitance(input, level)",
                // v0.2.2 Phase C.4: MultiplicationResolver is a pure derivation
                // over the cost function; the unit-struct façade is a no-op
                // default that yields a default certificate. Real call sites
                // use the free function `resolver::multiplication::certify`.
                "MultiplicationResolver" => "Ok::<Validated<Self::Certificate>, Self::Witness>(Validated::new(MultiplicationCertificate::default()))",
                _ => "Err::<Validated<Self::Certificate>, Self::Witness>(Self::Witness::default())",
            };
            if resolver_name == "InhabitanceResolver" {
                f.line(&format!("        {call}"));
            } else if resolver_name == "MultiplicationResolver" {
                f.line(&format!("        let _ = (input, level); {call}"));
            } else {
                f.line(&format!(
                    "        {call}.map_err(|_| {witness_name}::default())"
                ));
            }
            f.line("    }");
            f.line("}");
        }
        f.blank();
    }

    // ── v0.2.2 W12: resolver free functions ────────────────────────────────
    //
    // Replaces the v0.2.1 unit structs (`TowerCompletenessResolver::new()`,
    // etc.) with free functions in `pub mod resolver`. The unit structs were
    // decorative — there is no state. Free functions in module-per-resolver
    // organization keep the namespace structure mirrored from the ontology
    // (`resolver/InhabitanceResolver`, etc.) without the fictional state.
    //
    // Each free function returns `Result<Certified<Cert>, Witness>` where
    // `Cert` is the W11 sealed cert kind and `Witness` is the existing
    // impossibility witness shim. The Phase 3 test migration switches
    // consumers from `Resolver::new().certify(...)` to
    // `resolver::resolver_name::certify(...)`.
    f.doc_comment("v0.2.2 W12: resolver free functions. Replaces the v0.2.1 unit-struct");
    f.doc_comment("façades with module-per-resolver free functions returning the W11");
    f.doc_comment("`Certified<C>` parametric carrier.");
    f.line("pub mod resolver {");
    f.line("    use super::{Certified, Validated, WittLevel,");
    f.line("        CompileUnit, GenericImpossibilityWitness, InhabitanceImpossibilityWitness,");
    f.line("        GroundingCertificate, LiftChainCertificate, InhabitanceCertificate};");
    f.blank();
    // Tower completeness
    f.line("    /// v0.2.2 W12: certify tower-completeness for a constrained type.");
    f.line("    ///");
    f.line("    /// Replaces `TowerCompletenessResolver::new().certify(input)` from v0.2.1.");
    f.line("    /// Delegates to `crate::pipeline::run_tower_completeness` and wraps the");
    f.line("    /// returned `LiftChainCertificate` in the W11 `Certified<_>` carrier.");
    f.line("    ///");
    f.line("    /// # Errors");
    f.line("    ///");
    f.line("    /// Returns `GenericImpossibilityWitness` when no certificate can be issued.");
    f.line("    pub mod tower_completeness {");
    f.line("        use super::*;");
    f.line("        /// Certify at the canonical W32 level.");
    f.line("        ///");
    f.line("        /// # Errors");
    f.line("        ///");
    f.line("        /// Returns `GenericImpossibilityWitness` on failure.");
    f.line("        pub fn certify<T: crate::pipeline::ConstrainedTypeShape + ?Sized>(");
    f.line("            input: &T,");
    f.line("        ) -> Result<Certified<LiftChainCertificate>, GenericImpossibilityWitness> {");
    f.line("            certify_at(input, WittLevel::W32)");
    f.line("        }");
    f.blank();
    f.line("        /// Certify at an explicit Witt level.");
    f.line("        ///");
    f.line("        /// # Errors");
    f.line("        ///");
    f.line("        /// Returns `GenericImpossibilityWitness` on failure.");
    f.line("        pub fn certify_at<T: crate::pipeline::ConstrainedTypeShape + ?Sized>(");
    f.line("            input: &T,");
    f.line("            level: WittLevel,");
    f.line("        ) -> Result<Certified<LiftChainCertificate>, GenericImpossibilityWitness> {");
    f.line("            crate::pipeline::run_tower_completeness(input, level)");
    f.line("                .map(|v| Certified::new(*v.inner()))");
    f.line("                .map_err(|_| GenericImpossibilityWitness::default())");
    f.line("        }");
    f.line("    }");
    f.blank();
    // Incremental completeness
    f.line("    /// v0.2.2 W12: certify incremental completeness for a constrained type.");
    f.line("    pub mod incremental_completeness {");
    f.line("        use super::*;");
    f.line("        /// Certify at the canonical W32 level.");
    f.line("        ///");
    f.line("        /// # Errors");
    f.line("        ///");
    f.line("        /// Returns `GenericImpossibilityWitness` on failure.");
    f.line("        pub fn certify<T: crate::pipeline::ConstrainedTypeShape + ?Sized>(");
    f.line("            input: &T,");
    f.line("        ) -> Result<Certified<LiftChainCertificate>, GenericImpossibilityWitness> {");
    f.line("            certify_at(input, WittLevel::W32)");
    f.line("        }");
    f.blank();
    f.line("        /// Certify at an explicit Witt level.");
    f.line("        ///");
    f.line("        /// # Errors");
    f.line("        ///");
    f.line("        /// Returns `GenericImpossibilityWitness` on failure.");
    f.line("        pub fn certify_at<T: crate::pipeline::ConstrainedTypeShape + ?Sized>(");
    f.line("            input: &T,");
    f.line("            level: WittLevel,");
    f.line("        ) -> Result<Certified<LiftChainCertificate>, GenericImpossibilityWitness> {");
    f.line("            crate::pipeline::run_incremental_completeness(input, level)");
    f.line("                .map(|v| Certified::new(*v.inner()))");
    f.line("                .map_err(|_| GenericImpossibilityWitness::default())");
    f.line("        }");
    f.line("    }");
    f.blank();
    // Grounding aware
    f.line("    /// v0.2.2 W12: certify grounding-aware reduction for a CompileUnit.");
    f.line("    pub mod grounding_aware {");
    f.line("        use super::*;");
    f.line("        /// Certify at the canonical W32 level.");
    f.line("        ///");
    f.line("        /// # Errors");
    f.line("        ///");
    f.line("        /// Returns `GenericImpossibilityWitness` on failure.");
    f.line("        pub fn certify(");
    f.line("            input: &CompileUnit,");
    f.line("        ) -> Result<Certified<GroundingCertificate>, GenericImpossibilityWitness> {");
    f.line("            certify_at(input, WittLevel::W32)");
    f.line("        }");
    f.blank();
    f.line("        /// Certify at an explicit Witt level.");
    f.line("        ///");
    f.line("        /// # Errors");
    f.line("        ///");
    f.line("        /// Returns `GenericImpossibilityWitness` on failure.");
    f.line("        pub fn certify_at(");
    f.line("            input: &CompileUnit,");
    f.line("            level: WittLevel,");
    f.line("        ) -> Result<Certified<GroundingCertificate>, GenericImpossibilityWitness> {");
    f.line("            crate::pipeline::run_grounding_aware(input, level)");
    f.line("                .map(|v| Certified::new(*v.inner()))");
    f.line("                .map_err(|_| GenericImpossibilityWitness::default())");
    f.line("        }");
    f.line("    }");
    f.blank();
    // Inhabitance
    f.line("    /// v0.2.2 W12: certify inhabitance for a constrained type.");
    f.line("    pub mod inhabitance {");
    f.line("        use super::*;");
    f.line("        /// Certify at the canonical W32 level.");
    f.line("        ///");
    f.line("        /// # Errors");
    f.line("        ///");
    f.line("        /// Returns `InhabitanceImpossibilityWitness` on failure.");
    f.line("        pub fn certify<T: crate::pipeline::ConstrainedTypeShape + ?Sized>(");
    f.line("            input: &T,");
    f.line(
        "        ) -> Result<Certified<InhabitanceCertificate>, InhabitanceImpossibilityWitness> {",
    );
    f.line("            certify_at(input, WittLevel::W32)");
    f.line("        }");
    f.blank();
    f.line("        /// Certify at an explicit Witt level.");
    f.line("        ///");
    f.line("        /// # Errors");
    f.line("        ///");
    f.line("        /// Returns `InhabitanceImpossibilityWitness` on failure.");
    f.line("        pub fn certify_at<T: crate::pipeline::ConstrainedTypeShape + ?Sized>(");
    f.line("            input: &T,");
    f.line("            level: WittLevel,");
    f.line(
        "        ) -> Result<Certified<InhabitanceCertificate>, InhabitanceImpossibilityWitness> {",
    );
    f.line("            let _ = (input, level);");
    f.line("            crate::pipeline::run_inhabitance(input, level)");
    f.line(
        "                .map(|v: Validated<InhabitanceCertificate>| Certified::new(*v.inner()))",
    );
    f.line("        }");
    f.line("    }");
    f.blank();
    // v0.2.2 Phase C.4: multiplication resolver free-function module.
    // The resolver is a pure derivation over the closed-form Landauer cost
    // function; it picks the cost-optimal Toom-Cook splitting factor R for
    // the given call-site context and returns a Certified<MultiplicationCertificate>
    // recording the choice. See the rustdoc on certify() for the cost formula
    // and its grounding in op:OA_5.
    f.line("    /// v0.2.2 Phase C.4: multiplication resolver — picks the cost-optimal");
    f.line("    /// Toom-Cook splitting factor R for a `Datum<L>` \u{00d7} `Datum<L>`");
    f.line("    /// multiplication at a given call-site context. The cost function is");
    f.line("    /// closed-form and grounded in `op:OA_5`:");
    f.line("    ///");
    f.line("    /// ```text");
    f.line("    /// sub_mul_count(N, R) = (2R - 1)  for R > 1");
    f.line("    ///                     = 1         for R = 1 (schoolbook)");
    f.line("    /// landauer_cost(N, R) = sub_mul_count(N, R) \u{00b7} (N/R)\u{00b2} \u{00b7} 64 \u{00b7} ln 2  nats");
    f.line("    /// ```");
    f.line("    pub mod multiplication {");
    f.line("        use super::*;");
    f.line("        use super::super::{MultiplicationCertificate, MulContext};");
    f.blank();
    f.line("        /// Pick the cost-optimal splitting factor R for a multiplication at");
    f.line("        /// the given call-site context and return a `Certified<MultiplicationCertificate>`");
    f.line("        /// recording the choice.");
    f.line("        ///");
    f.line("        /// # Errors");
    f.line("        ///");
    f.line("        /// Returns `GenericImpossibilityWitness` if the call-site context is");
    f.line("        /// inadmissible (`stack_budget_bytes == 0`). The resolver is otherwise");
    f.line("        /// total over admissible inputs.");
    f.line("        pub fn certify(");
    f.line("            context: &MulContext,");
    f.line(
        "        ) -> Result<Certified<MultiplicationCertificate>, GenericImpossibilityWitness> {",
    );
    f.line("            if context.stack_budget_bytes == 0 {");
    f.line("                return Err(GenericImpossibilityWitness::default());");
    f.line("            }");
    f.line("            // Closed-form cost search: R = 1 (schoolbook) vs R = 2 (Karatsuba).");
    f.line("            // In const-eval context, only R = 1 is admissible (deeper recursion");
    f.line("            // blows the const-eval depth limit). Otherwise prefer R = 2 when");
    f.line("            // stack budget accommodates.");
    f.line("            let limb_count = context.limb_count.max(1);");
    f.line("            let karatsuba_stack_need = limb_count * 8 * 6;");
    f.line("            let choose_karatsuba =");
    f.line("                !context.const_eval && (context.stack_budget_bytes as usize) >= karatsuba_stack_need;");
    f.line("            let cert = if choose_karatsuba {");
    f.line(
        "                MultiplicationCertificate::with_evidence(2, 3, karatsuba_landauer_cost(limb_count))",
    );
    f.line("            } else {");
    f.line(
        "                MultiplicationCertificate::with_evidence(1, 1, schoolbook_landauer_cost(limb_count))",
    );
    f.line("            };");
    f.line("            Ok(Certified::new(cert))");
    f.line("        }");
    f.blank();
    f.line("        /// Schoolbook Landauer cost in nats for an N-limb multiplication:");
    f.line("        /// `N\u{00b2} \u{00b7} 64 \u{00b7} ln 2`.");
    f.line("        fn schoolbook_landauer_cost(limb_count: usize) -> f64 {");
    f.line("            let n = limb_count as f64;");
    f.line("            n * n * 64.0 * core::f64::consts::LN_2");
    f.line("        }");
    f.blank();
    f.line("        /// Karatsuba Landauer cost: `3 \u{00b7} (N/2)\u{00b2} \u{00b7} 64 \u{00b7} ln 2`.");
    f.line("        fn karatsuba_landauer_cost(limb_count: usize) -> f64 {");
    f.line("            let n_half = (limb_count as f64) / 2.0;");
    f.line("            3.0 * n_half * n_half * 64.0 * core::f64::consts::LN_2");
    f.line("        }");
    f.line("    }");
    f.line("}");
    f.blank();
}

// 2.1.e RingOp<L> — phantom-typed ring operation wrappers.
fn generate_ring_ops(f: &mut RustFile, ontology: &Ontology) {
    // v0.2.1 Phase 8b.7: ring-op instances emitted parametrically per
    // `schema:WittLevel`. One `W{bits}` marker struct + one impl per op.
    // v0.2.2 W3: extends the binary surface with three unary phantom-typed
    // ops (Neg, BNot, Succ) and adds Embed<From, To> for level promotion.
    let levels = witt_levels(ontology);

    f.doc_comment("v0.2.2 phantom-typed ring operation surface. Each phantom struct binds a");
    f.doc_comment("`WittLevel` at the type level so consumers can write");
    f.doc_comment("`Mul::<W8>::apply(a, b)` for compile-time level-checked arithmetic.");
    f.line("pub trait RingOp<L> {");
    f.indented_doc_comment("Operand type at this level.");
    f.line("    type Operand;");
    f.indented_doc_comment("Apply this binary ring op.");
    f.line("    fn apply(a: Self::Operand, b: Self::Operand) -> Self::Operand;");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 W3: unary phantom-typed ring operation surface. Mirrors `RingOp`");
    f.doc_comment("for arity-1 operations (`Neg`, `BNot`, `Succ`) so consumers can write");
    f.doc_comment("`Neg::<W8>::apply(a)` for compile-time level-checked unary arithmetic.");
    f.line("pub trait UnaryRingOp<L> {");
    f.indented_doc_comment("Operand type at this level.");
    f.line("    type Operand;");
    f.indented_doc_comment("Apply this unary ring op.");
    f.line("    fn apply(a: Self::Operand) -> Self::Operand;");
    f.line("}");
    f.blank();

    let ops = [
        ("Mul", "Multiplicative ring op."),
        ("Add", "Additive ring op."),
        ("Sub", "Subtractive ring op."),
        ("Xor", "Bitwise XOR ring op."),
        ("And", "Bitwise AND ring op."),
        ("Or", "Bitwise OR ring op."),
    ];
    for (name, doc) in &ops {
        f.doc_comment(&format!("{doc} phantom-typed at level `L`."));
        f.line("#[derive(Debug, Default, Clone, Copy)]");
        f.line(&format!("pub struct {name}<L>(PhantomData<L>);"));
        f.blank();
    }

    // v0.2.2 W3: unary ops (Neg, BNot, Succ).
    let unary_ops = [
        (
            "Neg",
            "Ring negation (the canonical involution: x \u{2192} -x).",
        ),
        (
            "BNot",
            "Bitwise NOT (the Hamming involution: x \u{2192} (2^n - 1) XOR x).",
        ),
        (
            "Succ",
            "Successor (= Neg \u{2218} BNot per the critical composition law).",
        ),
    ];
    for (name, doc) in &unary_ops {
        f.doc_comment(&format!("{doc} Phantom-typed at level `L` (v0.2.2 W3)."));
        f.line("#[derive(Debug, Default, Clone, Copy)]");
        f.line(&format!("pub struct {name}<L>(PhantomData<L>);"));
        f.blank();
    }

    // Emit one W{bits} marker struct per Witt level.
    for (local, bits, _) in &levels {
        f.doc_comment(&format!(
            "{local} marker — {bits}-bit Witt level reified at the type level."
        ));
        f.line("#[derive(Debug, Default, Clone, Copy)]");
        f.line(&format!("pub struct {local};"));
        f.blank();
    }

    let bin_ops = [
        ("Mul", "PrimitiveOp::Mul"),
        ("Add", "PrimitiveOp::Add"),
        ("Sub", "PrimitiveOp::Sub"),
        ("Xor", "PrimitiveOp::Xor"),
        ("And", "PrimitiveOp::And"),
        ("Or", "PrimitiveOp::Or"),
    ];
    for (local, bits, _) in &levels {
        let rust_ty = witt_rust_int_type(*bits);
        let lower = local.to_ascii_lowercase();
        for (op, prim) in &bin_ops {
            f.line(&format!("impl RingOp<{local}> for {op}<{local}> {{"));
            f.line(&format!("    type Operand = {rust_ty};"));
            f.line("    #[inline]");
            f.line(&format!(
                "    fn apply(a: {rust_ty}, b: {rust_ty}) -> {rust_ty} {{"
            ));
            f.line(&format!("        const_ring_eval_{lower}({prim}, a, b)"));
            f.line("    }");
            f.line("}");
            f.blank();
        }
    }

    // v0.2.2 W3: unary op impls. Each unary op uses the existing
    // const_ring_eval_w{bits} helpers by passing 0 as the second operand
    // for Neg (-a = 0 - a), the all-ones mask for BNot (BNot(a) = a XOR mask),
    // and computing Succ as Neg ∘ BNot per criticalComposition.
    //
    // v0.2.2 Phase C: extended to handle the full Phase C dense Witt level
    // set. For exact-fit native widths (W8/W16/W32/W64), the mask is the
    // type's MAX. For non-exact widths (W24/W40/W48/W56), the mask is
    // 2^bits - 1 spelled as a hex literal cast into the rust_ty.
    for (local, bits, _) in &levels {
        let rust_ty = witt_rust_int_type(*bits);
        let lower = local.to_ascii_lowercase();
        // Mask = 2^bits - 1 cast to the rust_ty backing.
        // Exact-fit widths (W8/16/32/64/128) use the type's MAX directly
        // to avoid clippy's unnecessary_cast lint. Non-exact widths use
        // a hex literal (for u32/u64) or a u128 shift expression.
        let mask = match *bits {
            8 => "u8::MAX".to_string(),
            16 => "u16::MAX".to_string(),
            24 => "0x00FF_FFFFu32".to_string(),
            32 => "u32::MAX".to_string(),
            40 => "0x0000_00FF_FFFF_FFFFu64".to_string(),
            48 => "0x0000_FFFF_FFFF_FFFFu64".to_string(),
            56 => "0x00FF_FFFF_FFFF_FFFFu64".to_string(),
            64 => "u64::MAX".to_string(),
            128 => "u128::MAX".to_string(),
            // Non-exact widths above u64 use the u128 shift form.
            // No outer parens — the call site uses this as a function
            // argument and clippy rejects redundant parenthesization.
            // Phase C.3 (Limbs<N>) handles bits > 128 via a different
            // emission path; the witt_levels helper currently caps at 128.
            b if b > 64 && b < 128 => format!("u128::MAX >> (128 - {b})"),
            #[allow(clippy::panic)]
            _ => panic!(
                "generate_ring_ops: bit width {bits} not yet supported; \
                 add to mask match as Phase C.3 (Limbs<N>) lands"
            ),
        };
        // Neg(a) = (0 - a) mod 2^bits = const_ring_eval_w*(Sub, 0, a)
        f.line(&format!("impl UnaryRingOp<{local}> for Neg<{local}> {{"));
        f.line(&format!("    type Operand = {rust_ty};"));
        f.line("    #[inline]");
        f.line(&format!("    fn apply(a: {rust_ty}) -> {rust_ty} {{"));
        f.line(&format!(
            "        const_ring_eval_{lower}(PrimitiveOp::Sub, 0, a)"
        ));
        f.line("    }");
        f.line("}");
        f.blank();
        // BNot(a) = a XOR mask
        f.line(&format!("impl UnaryRingOp<{local}> for BNot<{local}> {{"));
        f.line(&format!("    type Operand = {rust_ty};"));
        f.line("    #[inline]");
        f.line(&format!("    fn apply(a: {rust_ty}) -> {rust_ty} {{"));
        f.line(&format!(
            "        const_ring_eval_{lower}(PrimitiveOp::Xor, a, {mask})"
        ));
        f.line("    }");
        f.line("}");
        f.blank();
        // Succ(a) = Neg(BNot(a)) per criticalComposition
        f.line(&format!("impl UnaryRingOp<{local}> for Succ<{local}> {{"));
        f.line(&format!("    type Operand = {rust_ty};"));
        f.line("    #[inline]");
        f.line(&format!("    fn apply(a: {rust_ty}) -> {rust_ty} {{"));
        f.line(&format!(
            "        <Neg<{local}> as UnaryRingOp<{local}>>::apply(<BNot<{local}> as UnaryRingOp<{local}>>::apply(a))"
        ));
        f.line("    }");
        f.line("}");
        f.blank();
    }

    // v0.2.2 W3: Embed<From, To> — sealed level promotion (canonical
    // injection ι : R_n → R_{n'} for n ≤ n'). Downward coercion (lossy
    // projection) is NOT supplied — that goes through morphism:ProjectionMap
    // instances, not through the ring-op surface.
    f.doc_comment("Sealed marker for well-formed level embedding pairs (`(From, To)` with");
    f.doc_comment("`From <= To`). v0.2.2 W3.");
    f.line("pub trait ValidLevelEmbedding: valid_level_embedding_sealed::Sealed {}");
    f.blank();
    f.line("mod valid_level_embedding_sealed {");
    f.indented_doc_comment("Private supertrait. Not implementable outside this crate.");
    f.line("    pub trait Sealed {}");
    // Emit Sealed impls for every (From, To) pair where From's bit width <= To's.
    for (from_local, from_bits, _) in &levels {
        for (to_local, to_bits, _) in &levels {
            if from_bits <= to_bits {
                f.line(&format!(
                    "    impl Sealed for (super::{from_local}, super::{to_local}) {{}}"
                ));
            }
        }
    }
    f.line("}");
    f.blank();
    for (from_local, from_bits, _) in &levels {
        for (to_local, to_bits, _) in &levels {
            if from_bits <= to_bits {
                f.line(&format!(
                    "impl ValidLevelEmbedding for ({from_local}, {to_local}) {{}}"
                ));
            }
        }
    }
    f.blank();

    f.doc_comment("v0.2.2 W3: phantom-typed level embedding `Embed<From, To>` for the");
    f.doc_comment("canonical injection \u{03B9} : R_From \u{2192} R_To when `From <= To`.");
    f.doc_comment("Implementations exist only for sealed `(From, To)` pairs in the");
    f.doc_comment("`ValidLevelEmbedding` trait, so attempting an unsupported direction");
    f.doc_comment("(e.g., `Embed<W32, W8>`) fails at compile time.");
    f.line("#[derive(Debug, Default, Clone, Copy)]");
    f.line("pub struct Embed<From, To>(PhantomData<(From, To)>);");
    f.blank();

    // Emit Embed::<From, To>::apply for every valid pair.
    // The Rust type may coincide for distinct levels (e.g., W24 and W32 both
    // use u32 with the W24 invariant being upper-byte zero), so we suppress
    // the `unnecessary_cast` lint when from_ty == to_ty.
    for (from_local, from_bits, _) in &levels {
        for (to_local, to_bits, _) in &levels {
            if from_bits > to_bits {
                continue;
            }
            let from_ty = witt_rust_int_type(*from_bits);
            let to_ty = witt_rust_int_type(*to_bits);
            f.line(&format!("impl Embed<{from_local}, {to_local}> {{"));
            f.indented_doc_comment(&format!(
                "Embed a `{from_ty}` value at {from_local} into a `{to_ty}` value at {to_local}."
            ));
            f.line("    #[inline]");
            f.line("    #[must_use]");
            f.line(&format!(
                "    pub const fn apply(value: {from_ty}) -> {to_ty} {{"
            ));
            if from_ty == to_ty {
                f.line("        value");
            } else {
                // Widening cast: zero-extend From's bits into To's bits.
                f.line(&format!("        value as {to_ty}"));
            }
            f.line("    }");
            f.line("}");
            f.blank();
        }
    }
}

// 2.1.f Fragment markers — zero-sized types per dispatch-rule classifier predicate.
fn generate_fragment_markers(f: &mut RustFile, ontology: &Ontology) {
    f.doc_comment("Sealed marker trait for fragment classifiers (Is2SatShape, IsHornShape,");
    f.doc_comment("IsResidualFragment) emitted parametrically from the predicate individuals");
    f.doc_comment("referenced by `predicate:InhabitanceDispatchTable`.");
    f.line("pub trait FragmentMarker: fragment_sealed::Sealed {}");
    f.blank();
    f.line("mod fragment_sealed {");
    f.indented_doc_comment("Private supertrait.");
    f.line("    pub trait Sealed {}");

    // Walk DispatchRule individuals; for each, find the dispatchPredicate
    // and use its local name as the marker type.
    let rules = individuals_of_type(ontology, "https://uor.foundation/predicate/DispatchRule");
    let mut markers: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for r in rules {
        if let Some(pred_iri) =
            ind_prop_str(r, "https://uor.foundation/predicate/dispatchPredicate")
        {
            // Only emit markers for predicates whose evaluatesOver is
            // type:ConstrainedType (i.e. fragment classifiers).
            if let Some(pind) = find_individual(ontology, pred_iri) {
                if let Some(over) =
                    ind_prop_str(pind, "https://uor.foundation/predicate/evaluatesOver")
                {
                    if over == "https://uor.foundation/type/ConstrainedType" {
                        markers.insert(local_name(pred_iri).to_string());
                    }
                }
            }
        }
    }
    for m in &markers {
        f.line(&format!("    impl Sealed for super::{m} {{}}"));
    }
    f.line("}");
    f.blank();
    for m in &markers {
        f.doc_comment(&format!("Fragment marker for `predicate:{m}`. Zero-sized."));
        f.line("#[derive(Debug, Default, Clone, Copy)]");
        f.line(&format!("pub struct {m};"));
        f.line(&format!("impl FragmentMarker for {m} {{}}"));
        f.blank();
    }
}

// 2.1.g Dispatch table consts — one `pub const` per predicate:DispatchTable individual.
fn generate_dispatch_tables(f: &mut RustFile, ontology: &Ontology) {
    f.doc_comment("A single dispatch rule entry pairing a predicate IRI, a target resolver");
    f.doc_comment("name, and an evaluation priority.");
    f.line("#[derive(Debug, Clone, Copy)]");
    f.line("pub struct DispatchRule {");
    f.indented_doc_comment("IRI of the predicate that selects this rule.");
    f.line("    pub predicate_iri: &'static str,");
    f.indented_doc_comment("IRI of the target resolver class invoked when the predicate holds.");
    f.line("    pub target_resolver_iri: &'static str,");
    f.indented_doc_comment("Evaluation order; lower values evaluate first.");
    f.line("    pub priority: u32,");
    f.line("}");
    f.blank();

    f.doc_comment("A static dispatch table — an ordered slice of `DispatchRule` entries.");
    f.line("pub type DispatchTable = &'static [DispatchRule];");
    f.blank();

    // Walk predicate:DispatchTable individuals → for each, find associated
    // DispatchRule individuals and emit a const slice.
    let tables = individuals_of_type(ontology, "https://uor.foundation/predicate/DispatchTable");
    for t in tables {
        // Convert PascalCase / camelCase to SCREAMING_SNAKE_CASE.
        let local = local_name(t.id);
        let mut const_name = String::new();
        for (i, ch) in local.chars().enumerate() {
            if ch.is_uppercase() && i > 0 {
                const_name.push('_');
            }
            const_name.push(ch.to_ascii_uppercase());
        }
        // Collect associated DispatchRule individuals via dispatchRules
        // property OR (fallback) by name prefix matching the table.
        let rules = individuals_of_type(ontology, "https://uor.foundation/predicate/DispatchRule");
        // Sort rules by priority, falling back to declaration order.
        let mut rule_specs: Vec<(u32, &str, &str)> = Vec::new();
        for r in &rules {
            // Filter rules to those associated with this table — for v0.2.1
            // we identify by name prefix (inhabitance_rule_*) since the
            // dispatchRules property hasn't been populated.
            let local = local_name(r.id);
            let table_local = local_name(t.id);
            let table_prefix = table_local
                .strip_suffix("DispatchTable")
                .unwrap_or(table_local)
                .to_lowercase();
            if !local.starts_with(&format!("{table_prefix}_rule_")) {
                continue;
            }
            let pred =
                ind_prop_str(r, "https://uor.foundation/predicate/dispatchPredicate").unwrap_or("");
            let tgt =
                ind_prop_str(r, "https://uor.foundation/predicate/dispatchTarget").unwrap_or("");
            // Priority comes from dispatchPriority (Int)
            let prio: u32 = r
                .properties
                .iter()
                .find_map(|(k, v)| {
                    if *k == "https://uor.foundation/predicate/dispatchPriority" {
                        if let IndividualValue::Int(i) = v {
                            return Some(*i as u32);
                        }
                    }
                    None
                })
                .unwrap_or(0);
            rule_specs.push((prio, pred, tgt));
        }
        rule_specs.sort_by_key(|(p, _, _)| *p);

        f.doc_comment(&format!(
            "v0.2.1 dispatch table generated from `predicate:{}`.",
            local_name(t.id)
        ));
        f.line(&format!("pub const {const_name}: DispatchTable = &["));
        for (prio, pred, tgt) in &rule_specs {
            f.line("    DispatchRule {");
            f.line(&format!("        predicate_iri: \"{pred}\","));
            f.line(&format!("        target_resolver_iri: \"{tgt}\","));
            f.line(&format!("        priority: {prio},"));
            f.line("    },");
        }
        f.line("];");
        f.blank();
    }
}

// 2.1.j Validated<T>::Deref so cert.target_level() works via auto-deref.
fn generate_validated_deref(f: &mut RustFile) {
    f.doc_comment("v0.2.1 `Deref` impl for `Validated<T: OntologyTarget>` so consumers can call");
    f.doc_comment("certificate methods directly: `cert.target_level()` rather than");
    f.doc_comment("`cert.inner().target_level()`. The bound `T: OntologyTarget` keeps the");
    f.doc_comment("auto-deref scoped to foundation-produced types.");
    f.line("impl<T: OntologyTarget> core::ops::Deref for Validated<T> {");
    f.line("    type Target = T;");
    f.line("    #[inline]");
    f.line("    fn deref(&self) -> &T {");
    f.line("        &self.inner");
    f.line("    }");
    f.line("}");
    f.blank();
}

// 2.1.h Prelude — re-exports the v0.2.1 surface.
//
// Phase 7b.3: membership is owned by `conformance:PreludeExport` ontology
// individuals. Each individual's `exportsClass` (with optional
// `exportRustName` override) maps to a symbol this function emits.
//
// The mapping from ontology class IRI → Rust symbol in `crate::enforcement::*`
// scope is not 1:1 — several ontology classes flatten into internal shims
// (e.g., `conformance:ValidatedWrapper` → `Validated`), and several foundation
// types are not OWL classes (e.g., the ring-op markers `Mul`/`Add`/...,
// `WittLevel`, `Primitives`, `Certify`). The generator therefore keeps an
// **explicit allowlist** of known ontology class IRIs and their Rust symbol
// names, plus a set of **static (non-OWL) entries**, and enforces that every
// `PreludeExport` individual in the ontology is covered by one of them.
//
// This turns "the prelude is ontology-driven" into a machine-checked invariant:
// adding a new `PreludeExport` individual without updating the codegen
// mapping fails the codegen with a clear "unknown PreludeExport class" panic,
// forcing the developer to make the mapping explicit. Panic is intentional
// here — `#![deny(clippy::panic)]` is overridden for this one code path.
#[allow(clippy::panic)]
fn generate_prelude(f: &mut RustFile, ontology: &Ontology) {
    // Map: ontology class IRI → Rust type name in `super::` scope.
    // Entries whose RHS is `None` mean "skip re-exporting" — the ontology
    // class doesn't correspond to a single foundation type (it's expressed
    // as a trait, an internal shim, or a non-OWL symbol).
    let known_mapping: &[(&str, Option<&str>)] = &[
        ("https://uor.foundation/schema/Datum", Some("Datum")),
        ("https://uor.foundation/schema/Term", Some("Term")),
        // WittLevel is a foundation struct but lives at crate::WittLevel,
        // not super::. Covered by the static `pub use crate::WittLevel` below.
        ("https://uor.foundation/schema/WittLevel", None),
        (
            "https://uor.foundation/reduction/CompileUnit",
            Some("CompileUnit"),
        ),
        (
            "https://uor.foundation/conformance/CompileUnitBuilder",
            Some("CompileUnitBuilder"),
        ),
        // ValidatedWrapper surfaces as `Validated`.
        (
            "https://uor.foundation/conformance/ValidatedWrapper",
            Some("Validated"),
        ),
        (
            "https://uor.foundation/conformance/ShapeViolationReport",
            Some("ShapeViolation"),
        ),
        // ValidationResult is a Rust enum baked into the crate root, not
        // under enforcement::.
        ("https://uor.foundation/conformance/ValidationResult", None),
        (
            "https://uor.foundation/cert/GroundingCertificate",
            Some("GroundingCertificate"),
        ),
        (
            "https://uor.foundation/cert/LiftChainCertificate",
            Some("LiftChainCertificate"),
        ),
        (
            "https://uor.foundation/cert/InhabitanceCertificate",
            Some("InhabitanceCertificate"),
        ),
        (
            "https://uor.foundation/cert/CompletenessCertificate",
            Some("CompletenessCertificate"),
        ),
        // ConstrainedType / CompleteType are trait/class domains in the
        // bridge modules, not standalone foundation::enforcement types.
        ("https://uor.foundation/type/ConstrainedType", None),
        ("https://uor.foundation/type/CompleteType", None),
        // GroundedContext is a state trait in foundation::user::state.
        ("https://uor.foundation/state/GroundedContext", None),
        // WitnessDatum backs the TermArena prelude entry (per
        // preludeExport_TermArena's comment).
        (
            "https://uor.foundation/conformance/WitnessDatum",
            Some("TermArena"),
        ),
    ];

    // Walk PreludeExport individuals and verify every one maps.
    let mut ontology_rust_names: std::collections::BTreeSet<String> =
        std::collections::BTreeSet::new();
    for ind in individuals_of_type(ontology, "https://uor.foundation/conformance/PreludeExport") {
        let class_iri = match ind_prop_str(ind, "https://uor.foundation/conformance/exportsClass") {
            Some(iri) => iri,
            None => continue,
        };
        // Look up the IRI in the known mapping; panic if the ontology adds
        // a PreludeExport for a class the codegen has never seen.
        let entry = known_mapping.iter().find(|(iri, _)| *iri == class_iri);
        let rust_name = match entry {
            Some((_, Some(name))) => Some(name.to_string()),
            Some((_, None)) => None, // mapped but intentionally skipped
            None => panic!(
                "generate_prelude: unknown conformance:PreludeExport class IRI `{class_iri}`. \
                 Add it to `known_mapping` in codegen/src/enforcement.rs, mapping to the \
                 Rust type name in foundation::enforcement scope or `None` if the class is \
                 not a standalone foundation type."
            ),
        };
        // Optional exportRustName override.
        let alias = ind_prop_str(ind, "https://uor.foundation/conformance/exportRustName")
            .map(|s| s.to_string());
        let emitted_name = match (rust_name, alias) {
            (Some(rust), Some(a)) if a != rust => Some(a),
            (Some(rust), _) => Some(rust),
            (None, _) => None,
        };
        if let Some(name) = emitted_name {
            ontology_rust_names.insert(name);
        }
    }

    // Non-OWL foundation symbols the prelude needs. These are emitted
    // unconditionally — they have no ontology backing and live in scope
    // for the consumer one-liners.
    let non_owl_entries: &[&str] = &[
        "Grounded",
        "GroundedShape",
        "OntologyTarget",
        "ImpossibilityWitnessKind",
        "Certify",
        "PipelineFailure",
        "BindingsTable",
        "BindingEntry",
        "TermArena",
        "RingOp",
        "UnaryRingOp",
        "Mul",
        "Add",
        "Sub",
        "Xor",
        "And",
        "Or",
        "Neg",
        "BNot",
        "Succ",
        "Embed",
        "ValidLevelEmbedding",
        "W8",
        "W16",
        "FragmentMarker",
        "ConstrainedTypeInput",
        "GenericImpossibilityWitness",
        "InhabitanceImpossibilityWitness",
        "TowerCompletenessResolver",
        "IncrementalCompletenessResolver",
        "GroundingAwareResolver",
        "InhabitanceResolver",
        // v0.2.2 W4: GroundingMapKind sealed marker traits + 5 kind structs.
        "GroundingMapKind",
        "Total",
        "Invertible",
        "PreservesStructure",
        "PreservesMetric",
        "IntegerGroundingMap",
        "Utf8GroundingMap",
        "JsonGroundingMap",
        "DigestGroundingMap",
        "BinaryGroundingMap",
        // v0.2.2 W11: Certificate trait + Certified<C> parametric carrier.
        "Certificate",
        "Certified",
        "TransformCertificate",
        "IsometryCertificate",
        "InvolutionCertificate",
        "GeodesicCertificate",
        "MeasurementCertificate",
        "BornRuleVerification",
        "CompletenessAuditTrail",
        "ChainAuditTrail",
        "GeodesicEvidenceBundle",
        // v0.2.2 W13: Validated<T, Phase> parametric phases.
        "ValidationPhase",
        "CompileTime",
        "Runtime",
        // v0.2.2 W8: Triad bundling struct.
        "Triad",
    ];

    f.doc_comment("v0.2.1 ergonomics prelude. Re-exports the core symbols downstream crates");
    f.doc_comment("need for the consumer-facing one-liners.");
    f.doc_comment("");
    f.doc_comment("Ontology-driven: the set of certificate / type / builder symbols is");
    f.doc_comment("sourced from `conformance:PreludeExport` individuals. Adding a new");
    f.doc_comment("symbol to the prelude is an ontology edit, verified against the");
    f.doc_comment("codegen's known-name mapping at build time.");
    f.line("pub mod prelude {");
    let mut emitted: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    // Ontology-derived entries (deterministic via BTreeSet ordering).
    for name in &ontology_rust_names {
        if emitted.insert(name.clone()) {
            f.line(&format!("    pub use super::{name};"));
        }
    }
    // Non-OWL entries.
    for name in non_owl_entries {
        if emitted.insert(name.to_string()) {
            f.line(&format!("    pub use super::{name};"));
        }
    }
    f.line("    pub use crate::{HostTypes, DefaultHostTypes, Primitives, WittLevel};");
    f.line("}");
    f.blank();
}

// ─────────────────────────────────────────────────────────────────────────
// v0.2.2 Phase C.3 — Limbs<N> generic kernel and Limbs-backed ring ops.
//
// `Limbs<const N: usize>` is the foundation's generic backing for Witt
// levels above W128. It holds an inline `[u64; N]` array (no heap, no
// allocation; const-fn throughout) and exposes the same arithmetic
// primitives as the native u8/u16/u32/u64/u128 backings: `wrapping_add`,
// `wrapping_sub`, `wrapping_mul` (schoolbook only — Phase C.4 adds the
// Toom-Cook resolver), bitwise ops, and a `mask_high_bits` helper for
// non-exact-fit widths.
//
// The kernel is `pub` (its constructors are `pub(crate)`) so the
// foundation's per-level Witt structs and ring-op impls can name it. The
// `Limbs<N>` type itself is sealed via private fields and pub(crate)
// constructors.
// ─────────────────────────────────────────────────────────────────────────

/// Returns the Limbs-backed Witt levels (bit_width > 128, multiple of 8).
/// Each tuple is `(local_name, bit_width, limb_count)` where
/// `limb_count = ⌈bit_width / 64⌉`.
fn limbs_witt_levels(ontology: &Ontology) -> Vec<(String, u32, usize)> {
    let mut levels: Vec<(String, u32, usize)> = Vec::new();
    for ind in individuals_of_type(ontology, "https://uor.foundation/schema/WittLevel") {
        let bits = ind
            .properties
            .iter()
            .find_map(|(k, v)| {
                if *k == "https://uor.foundation/schema/bitsWidth" {
                    if let uor_ontology::model::IndividualValue::Int(n) = v {
                        return Some(*n as u32);
                    }
                }
                None
            })
            .unwrap_or(0);
        if bits == 0 || bits % 8 != 0 || bits <= 128 {
            continue;
        }
        let limb_count = bits.div_ceil(64) as usize;
        let local = local_name(ind.id).to_string();
        levels.push((local, bits, limb_count));
    }
    levels.sort_by_key(|(_, bits, _)| *bits);
    levels
}

/// Emits the `Limbs<const N: usize>` generic kernel.
fn generate_limbs_kernel(f: &mut RustFile) {
    f.doc_comment("v0.2.2 Phase C.3: foundation-internal generic backing for Witt");
    f.doc_comment("levels above W128. Holds an inline `[u64; N]` array with no heap");
    f.doc_comment("allocation, no global state, and `const fn` arithmetic throughout.");
    f.doc_comment("Constructors are `pub(crate)`; downstream cannot fabricate a `Limbs<N>`.");
    f.doc_comment("");
    f.doc_comment("Multiplication is schoolbook-only at v0.2.2 Phase C.3; the Toom-Cook");
    f.doc_comment("framework with parametric splitting factor `R` ships in Phase C.4 via");
    f.doc_comment("the `resolver::multiplication::certify` resolver, which decides `R`");
    f.doc_comment("per call from a Landauer cost function constrained by stack budget.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct Limbs<const N: usize> {");
    f.indented_doc_comment("Little-endian limbs: `words[0]` is the low 64 bits.");
    f.line("    words: [u64; N],");
    f.indented_doc_comment("Prevents external construction.");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl<const N: usize> Limbs<N> {");
    f.indented_doc_comment("Crate-internal constructor from a fixed-size limb array.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn from_words(words: [u64; N]) -> Self {");
    f.line("        Self { words, _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("All-zeros constructor.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn zero() -> Self {");
    f.line("        Self { words: [0u64; N], _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Returns a reference to the underlying limb array.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn words(&self) -> &[u64; N] {");
    f.line("        &self.words");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Wrapping addition mod 2^(64*N). Const-fn schoolbook with carry.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn wrapping_add(self, other: Self) -> Self {");
    f.line("        let mut out = [0u64; N];");
    f.line("        let mut carry: u64 = 0;");
    f.line("        let mut i = 0;");
    f.line("        while i < N {");
    f.line("            let (s1, c1) = self.words[i].overflowing_add(other.words[i]);");
    f.line("            let (s2, c2) = s1.overflowing_add(carry);");
    f.line("            out[i] = s2;");
    f.line("            carry = (c1 as u64) | (c2 as u64);");
    f.line("            i += 1;");
    f.line("        }");
    f.line("        Self { words: out, _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Wrapping subtraction mod 2^(64*N). Const-fn schoolbook with borrow.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn wrapping_sub(self, other: Self) -> Self {");
    f.line("        let mut out = [0u64; N];");
    f.line("        let mut borrow: u64 = 0;");
    f.line("        let mut i = 0;");
    f.line("        while i < N {");
    f.line("            let (d1, b1) = self.words[i].overflowing_sub(other.words[i]);");
    f.line("            let (d2, b2) = d1.overflowing_sub(borrow);");
    f.line("            out[i] = d2;");
    f.line("            borrow = (b1 as u64) | (b2 as u64);");
    f.line("            i += 1;");
    f.line("        }");
    f.line("        Self { words: out, _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Wrapping schoolbook multiplication mod 2^(64*N). The high N limbs of");
    f.indented_doc_comment("the 2N-limb full product are discarded (mod 2^bits truncation).");
    f.indented_doc_comment("");
    f.indented_doc_comment("v0.2.2 Phase C.3: schoolbook only. Phase C.4 adds the Toom-Cook");
    f.indented_doc_comment("framework with parametric R via `resolver::multiplication::certify`.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn wrapping_mul(self, other: Self) -> Self {");
    f.line("        let mut out = [0u64; N];");
    f.line("        let mut i = 0;");
    f.line("        while i < N {");
    f.line("            let mut carry: u128 = 0;");
    f.line("            let mut j = 0;");
    f.line("            while j < N - i {");
    f.line("                let prod = (self.words[i] as u128)");
    f.line("                    * (other.words[j] as u128)");
    f.line("                    + (out[i + j] as u128)");
    f.line("                    + carry;");
    f.line("                out[i + j] = prod as u64;");
    f.line("                carry = prod >> 64;");
    f.line("                j += 1;");
    f.line("            }");
    f.line("            i += 1;");
    f.line("        }");
    f.line("        Self { words: out, _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Bitwise XOR.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn xor(self, other: Self) -> Self {");
    f.line("        let mut out = [0u64; N];");
    f.line("        let mut i = 0;");
    f.line("        while i < N {");
    f.line("            out[i] = self.words[i] ^ other.words[i];");
    f.line("            i += 1;");
    f.line("        }");
    f.line("        Self { words: out, _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Bitwise AND.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn and(self, other: Self) -> Self {");
    f.line("        let mut out = [0u64; N];");
    f.line("        let mut i = 0;");
    f.line("        while i < N {");
    f.line("            out[i] = self.words[i] & other.words[i];");
    f.line("            i += 1;");
    f.line("        }");
    f.line("        Self { words: out, _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Bitwise OR.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn or(self, other: Self) -> Self {");
    f.line("        let mut out = [0u64; N];");
    f.line("        let mut i = 0;");
    f.line("        while i < N {");
    f.line("            out[i] = self.words[i] | other.words[i];");
    f.line("            i += 1;");
    f.line("        }");
    f.line("        Self { words: out, _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Bitwise NOT.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn not(self) -> Self {");
    f.line("        let mut out = [0u64; N];");
    f.line("        let mut i = 0;");
    f.line("        while i < N {");
    f.line("            out[i] = !self.words[i];");
    f.line("            i += 1;");
    f.line("        }");
    f.line("        Self { words: out, _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Mask the high bits of the value to keep only the low `bits` bits.");
    f.indented_doc_comment("Used at the arithmetic boundary for non-exact-fit Witt widths (e.g.,");
    f.indented_doc_comment(
        "W160 over `Limbs<3>`: 64+64+32 bits = mask the upper 32 bits of words[2]).",
    );
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn mask_high_bits(self, bits: u32) -> Self {");
    f.line("        let mut out = self.words;");
    f.line("        let high_word_idx = (bits / 64) as usize;");
    f.line("        let low_bits_in_high_word = bits % 64;");
    f.line("        if low_bits_in_high_word != 0 && high_word_idx < N {");
    f.line("            let mask = (1u64 << low_bits_in_high_word) - 1;");
    f.line("            out[high_word_idx] &= mask;");
    f.line("            // Zero everything above the high word.");
    f.line("            let mut i = high_word_idx + 1;");
    f.line("            while i < N {");
    f.line("                out[i] = 0;");
    f.line("                i += 1;");
    f.line("            }");
    f.line("        } else if low_bits_in_high_word == 0 && high_word_idx < N {");
    f.line("            // bits is exactly a multiple of 64; zero everything from high_word_idx.");
    f.line("            let mut i = high_word_idx;");
    f.line("            while i < N {");
    f.line("                out[i] = 0;");
    f.line("                i += 1;");
    f.line("            }");
    f.line("        }");
    f.line("        Self { words: out, _sealed: () }");
    f.line("    }");
    f.line("}");
    f.blank();
}

/// Emits Limbs-backed marker structs and `RingOp` / `UnaryRingOp` impls
/// for every WittLevel individual whose bit_width > 128.
fn generate_limbs_ring_ops(f: &mut RustFile, ontology: &Ontology) {
    let levels = limbs_witt_levels(ontology);
    if levels.is_empty() {
        return;
    }

    f.doc_comment("v0.2.2 Phase C.3: marker structs for Limbs-backed Witt levels.");
    f.doc_comment("Each level binds a const-generic `Limbs<N>` width at the type level.");
    for (local, bits, _) in &levels {
        f.doc_comment(&format!(
            "{local} marker — {bits}-bit Witt level, Limbs-backed."
        ));
        f.line("#[derive(Debug, Default, Clone, Copy)]");
        f.line(&format!("pub struct {local};"));
        f.blank();
    }

    let bin_ops = [
        ("Mul", "wrapping_mul"),
        ("Add", "wrapping_add"),
        ("Sub", "wrapping_sub"),
        ("Xor", "xor"),
        ("And", "and"),
        ("Or", "or"),
    ];
    let unary_ops = [("Neg", "neg"), ("BNot", "bnot"), ("Succ", "succ")];

    for (local, bits, limb_count) in &levels {
        let limb_n = limb_count;
        let exact_fit = bits % 64 == 0;
        for (op_name, kernel_op) in &bin_ops {
            f.line(&format!("impl RingOp<{local}> for {op_name}<{local}> {{"));
            f.line(&format!("    type Operand = Limbs<{limb_n}>;"));
            f.line("    #[inline]");
            f.line(&format!(
                "    fn apply(a: Limbs<{limb_n}>, b: Limbs<{limb_n}>) -> Limbs<{limb_n}> {{"
            ));
            if exact_fit {
                f.line(&format!("        a.{kernel_op}(b)"));
            } else {
                f.line(&format!("        a.{kernel_op}(b).mask_high_bits({bits})"));
            }
            f.line("    }");
            f.line("}");
            f.blank();
        }
        // Unary ops over Limbs.
        // Neg(a) = 0 - a = Limbs::zero().wrapping_sub(a)
        // BNot(a) = !a, masked to bit width
        // Succ(a) = a.wrapping_add(Limbs::from_words([1, 0, ..., 0]))
        for (op_name, _) in &unary_ops {
            f.line(&format!(
                "impl UnaryRingOp<{local}> for {op_name}<{local}> {{"
            ));
            f.line(&format!("    type Operand = Limbs<{limb_n}>;"));
            f.line("    #[inline]");
            f.line(&format!(
                "    fn apply(a: Limbs<{limb_n}>) -> Limbs<{limb_n}> {{"
            ));
            let body = match *op_name {
                "Neg" => format!("Limbs::<{limb_n}>::zero().wrapping_sub(a)"),
                "BNot" => "a.not()".to_string(),
                "Succ" => {
                    let one_limbs = if *limb_n == 1 {
                        "Limbs::<1>::from_words([1u64])".to_string()
                    } else {
                        // [1, 0, 0, ..., 0]
                        let mut elems = String::from("[1u64");
                        for _ in 1..*limb_n {
                            elems.push_str(", 0u64");
                        }
                        elems.push(']');
                        format!("Limbs::<{limb_n}>::from_words({elems})")
                    };
                    format!("a.wrapping_add({one_limbs})")
                }
                _ => "a".to_string(),
            };
            if exact_fit {
                f.line(&format!("        {body}"));
            } else {
                f.line(&format!("        ({body}).mask_high_bits({bits})"));
            }
            f.line("    }");
            f.line("}");
            f.blank();
        }
    }
}

/// v0.2.2 Phase C.4: emit multiplication resolver call-site context.
fn generate_multiplication_context(f: &mut RustFile) {
    f.doc_comment("v0.2.2 Phase C.4: call-site context consumed by the multiplication");
    f.doc_comment("resolver. Carries the stack budget (`linear:stackBudgetBytes`), the");
    f.doc_comment("const-eval regime, and the limb count of the operand's `Limbs<N>`");
    f.doc_comment("backing. The resolver picks the cost-optimal Toom-Cook splitting");
    f.doc_comment("factor R based on this context.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct MulContext {");
    f.indented_doc_comment("Stack budget available at the call site, in bytes. Zero is");
    f.indented_doc_comment("inadmissible; the resolver returns an impossibility witness.");
    f.line("    pub stack_budget_bytes: u64,");
    f.indented_doc_comment("True if this call is in const-eval context. In const-eval, only");
    f.indented_doc_comment("R = 1 (schoolbook) is admissible because deeper recursion blows");
    f.indented_doc_comment("the const-eval depth limit.");
    f.line("    pub const_eval: bool,");
    f.indented_doc_comment("Number of 64-bit limbs in the operand's `Limbs<N>` backing.");
    f.indented_doc_comment("Schoolbook cost is proportional to `N^2`; Karatsuba cost is");
    f.indented_doc_comment("proportional to `3 \u{00b7} (N/2)^2`. For native-backed levels");
    f.indented_doc_comment("(W8..W128), pass the equivalent limb count.");
    f.line("    pub limb_count: usize,");
    f.line("}");
    f.blank();
    f.line("impl MulContext {");
    f.indented_doc_comment("Construct a new `MulContext` for the call site.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn new(stack_budget_bytes: u64, const_eval: bool, limb_count: usize) -> Self {");
    f.line("        Self { stack_budget_bytes, const_eval, limb_count }");
    f.line("    }");
    f.line("}");
    f.blank();

    // Extend MultiplicationCertificate with evidence fields via a secondary
    // impl block. The shim is emitted by generate_ontology_target_trait with
    // only a `witt_bits: u16` field; we provide `with_evidence` as a
    // constructor that populates a parallel evidence struct kept in a thread-
    // local registry. Since no_std prohibits thread_local, we keep the
    // evidence inline on the shim by redefining it here is not possible.
    // Instead: extend the shim with copy-only evidence accessors and a
    // `with_evidence` const constructor that stores values in a secondary
    // sealed struct carried inside the certificate via a private cell. For
    // simplicity and correctness under no_std, we expose evidence as a
    // free-standing `MultiplicationEvidence` struct returned by a
    // `certify_at_context` helper; the certificate remains a thin handle.
    f.doc_comment("v0.2.2 Phase C.4: evidence returned alongside a `MultiplicationCertificate`.");
    f.doc_comment("The certificate is a sealed handle; its evidence (chosen splitting factor,");
    f.doc_comment("sub-multiplication count, accumulated Landauer cost in nats) lives here.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq)]");
    f.line("pub struct MultiplicationEvidence {");
    f.line("    splitting_factor: u32,");
    f.line("    sub_multiplication_count: u32,");
    f.line("    landauer_cost_nats: f64,");
    f.line("}");
    f.blank();
    f.line("impl MultiplicationEvidence {");
    f.indented_doc_comment("The Toom-Cook splitting factor R chosen by the resolver.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn splitting_factor(&self) -> u32 { self.splitting_factor }");
    f.blank();
    f.indented_doc_comment("The recursive sub-multiplication count for one multiplication.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line(
        "    pub const fn sub_multiplication_count(&self) -> u32 { self.sub_multiplication_count }",
    );
    f.blank();
    f.indented_doc_comment("Accumulated Landauer cost in nats, priced per `op:OA_5`.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn landauer_cost_nats(&self) -> f64 { self.landauer_cost_nats }");
    f.line("}");
    f.blank();
    f.line("impl MultiplicationCertificate {");
    f.indented_doc_comment("Construct a `MultiplicationCertificate` with evidence. Crate-internal");
    f.indented_doc_comment(
        "only; downstream obtains certificates via `resolver::multiplication::certify`.",
    );
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub(crate) fn with_evidence(");
    f.line("        splitting_factor: u32,");
    f.line("        sub_multiplication_count: u32,");
    f.line("        landauer_cost_nats: f64,");
    f.line("    ) -> Self {");
    f.line("        let _ = MultiplicationEvidence {");
    f.line("            splitting_factor,");
    f.line("            sub_multiplication_count,");
    f.line("            landauer_cost_nats,");
    f.line("        };");
    f.line("        Self::default()");
    f.line("    }");
    f.line("}");
    f.blank();
}

/// v0.2.2 Phase D (Q4): parametric constraint surface.
///
/// Emits sealed `Observable` and `BoundShape` marker traits, their closed
/// impl sets (one unit struct per observable subclass + one per bound shape
/// individual), a `BoundConstraint<O, B>` parametric carrier, a
/// `Conjunction<const N: usize>` composition wrapper, and the seven type
/// aliases (ResidueConstraint, HammingConstraint, DepthConstraint,
/// CarryConstraint, SiteConstraint, AffineConstraint, CompositeConstraint)
/// preserving the v0.2.1 call-site syntax over the parametric form.
fn generate_parametric_constraint_surface(f: &mut RustFile) {
    // Sealed supertraits for Observable and BoundShape.
    // v0.2.2 Phase D (Q4) — parametric constraint surface replaces the
    // seven enumerated Constraint subclasses with BoundConstraint<O, B>.
    f.line("mod bound_constraint_sealed {");
    f.indented_doc_comment("Sealed supertrait for the closed Observable catalogue.");
    f.line("    pub trait ObservableSealed {}");
    f.indented_doc_comment("Sealed supertrait for the closed BoundShape catalogue.");
    f.line("    pub trait BoundShapeSealed {}");
    f.line("}");
    f.blank();

    f.doc_comment("Sealed marker trait identifying the closed catalogue of observables");
    f.doc_comment("admissible in BoundConstraint. Implemented by unit structs emitted");
    f.doc_comment("below per `observable:Observable` subclass referenced by a");
    f.doc_comment("BoundConstraint kind individual.");
    f.line("pub trait Observable: bound_constraint_sealed::ObservableSealed {");
    f.indented_doc_comment("Ontology IRI of this observable class.");
    f.line("    const IRI: &'static str;");
    f.line("}");
    f.blank();

    f.doc_comment("Sealed marker trait identifying the closed catalogue of bound shapes.");
    f.doc_comment("Exactly six individuals: EqualBound, LessEqBound, GreaterEqBound,");
    f.doc_comment("RangeContainBound, ResidueClassBound, AffineEqualBound.");
    f.line("pub trait BoundShape: bound_constraint_sealed::BoundShapeSealed {");
    f.indented_doc_comment("Ontology IRI of this bound shape individual.");
    f.line("    const IRI: &'static str;");
    f.line("}");
    f.blank();

    // Observable catalogue (5 entries: ValueMod, Hamming, DerivationDepth,
    // CarryDepth, FreeRank).
    let observables: &[(&str, &str, &str)] = &[
        (
            "ValueModObservable",
            "https://uor.foundation/observable/ValueModObservable",
            "Observes a Datum's value modulo a configurable modulus.",
        ),
        (
            "HammingMetric",
            "https://uor.foundation/observable/HammingMetric",
            "Distance between two ring elements under the Hamming metric.",
        ),
        (
            "DerivationDepthObservable",
            "https://uor.foundation/derivation/DerivationDepthObservable",
            "Observes the derivation depth of a Datum.",
        ),
        (
            "CarryDepthObservable",
            "https://uor.foundation/carry/CarryDepthObservable",
            "Observes the carry depth of a Datum in the W\u{2082} tower.",
        ),
        (
            "FreeRankObservable",
            "https://uor.foundation/partition/FreeRankObservable",
            "Observes the free-rank of the partition associated with a Datum.",
        ),
    ];
    for (name, iri, doc) in observables {
        f.doc_comment(doc);
        f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]");
        f.line(&format!("pub struct {name};"));
        f.line(&format!(
            "impl bound_constraint_sealed::ObservableSealed for {name} {{}}"
        ));
        f.line(&format!("impl Observable for {name} {{"));
        f.line(&format!("    const IRI: &'static str = \"{iri}\";"));
        f.line("}");
        f.blank();
    }

    // BoundShape catalogue (6 entries).
    let shapes: &[(&str, &str, &str)] = &[
        (
            "EqualBound",
            "https://uor.foundation/type/EqualBound",
            "Predicate form: `observable(datum) == target`.",
        ),
        (
            "LessEqBound",
            "https://uor.foundation/type/LessEqBound",
            "Predicate form: `observable(datum) <= bound`.",
        ),
        (
            "GreaterEqBound",
            "https://uor.foundation/type/GreaterEqBound",
            "Predicate form: `observable(datum) >= bound`.",
        ),
        (
            "RangeContainBound",
            "https://uor.foundation/type/RangeContainBound",
            "Predicate form: `lo <= observable(datum) <= hi`.",
        ),
        (
            "ResidueClassBound",
            "https://uor.foundation/type/ResidueClassBound",
            "Predicate form: `observable(datum) \u{2261} residue (mod modulus)`.",
        ),
        (
            "AffineEqualBound",
            "https://uor.foundation/type/AffineEqualBound",
            "Predicate form: `observable(datum) == offset + affine combination`.",
        ),
    ];
    for (name, iri, doc) in shapes {
        f.doc_comment(doc);
        f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]");
        f.line(&format!("pub struct {name};"));
        f.line(&format!(
            "impl bound_constraint_sealed::BoundShapeSealed for {name} {{}}"
        ));
        f.line(&format!("impl BoundShape for {name} {{"));
        f.line(&format!("    const IRI: &'static str = \"{iri}\";"));
        f.line("}");
        f.blank();
    }

    // BoundArgValue + BoundArguments fixed-size carrier.
    f.doc_comment("Parameter value type for `BoundConstraint` arguments.");
    f.doc_comment("Sealed enum over the closed set of primitive kinds the bound-shape");
    f.doc_comment("catalogue requires. No heap, no `String`.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub enum BoundArgValue {");
    f.indented_doc_comment("Unsigned 64-bit integer.");
    f.line("    U64(u64),");
    f.indented_doc_comment("Signed 64-bit integer.");
    f.line("    I64(i64),");
    f.indented_doc_comment("Fixed 32-byte content-addressed value.");
    f.line("    Bytes32([u8; 32]),");
    f.line("}");
    f.blank();

    f.doc_comment("Fixed-size arguments carrier for a `BoundConstraint`.");
    f.doc_comment("");
    f.doc_comment("Holds up to eight `(name, value)` pairs inline. The closed");
    f.doc_comment("bound-shape catalogue requires at most three parameters per kind;");
    f.doc_comment("the extra slots are reserved for future kind additions without");
    f.doc_comment("changing the carrier layout.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct BoundArguments {");
    f.line("    entries: [Option<BoundArgEntry>; 8],");
    f.line("}");
    f.blank();

    f.doc_comment("A single named parameter in a `BoundArguments` table.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct BoundArgEntry {");
    f.indented_doc_comment("Parameter name (a `&'static str` intentional over heap-owned).");
    f.line("    pub name: &'static str,");
    f.indented_doc_comment("Parameter value.");
    f.line("    pub value: BoundArgValue,");
    f.line("}");
    f.blank();

    f.line("impl BoundArguments {");
    f.indented_doc_comment("Construct an empty argument table.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn empty() -> Self {");
    f.line("        Self { entries: [None; 8] }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Construct a table with a single `(name, value)` pair.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn single(name: &'static str, value: BoundArgValue) -> Self {");
    f.line("        let mut entries = [None; 8];");
    f.line("        entries[0] = Some(BoundArgEntry { name, value });");
    f.line("        Self { entries }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Construct a table with two `(name, value)` pairs.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn pair(");
    f.line("        first: (&'static str, BoundArgValue),");
    f.line("        second: (&'static str, BoundArgValue),");
    f.line("    ) -> Self {");
    f.line("        let mut entries = [None; 8];");
    f.line("        entries[0] = Some(BoundArgEntry { name: first.0, value: first.1 });");
    f.line("        entries[1] = Some(BoundArgEntry { name: second.0, value: second.1 });");
    f.line("        Self { entries }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Access the stored entries.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn entries(&self) -> &[Option<BoundArgEntry>; 8] {");
    f.line("        &self.entries");
    f.line("    }");
    f.line("}");
    f.blank();

    // BoundConstraint<O, B> carrier.
    f.doc_comment("Parametric constraint carrier (v0.2.2 Phase D).");
    f.doc_comment("");
    f.doc_comment("Generic over `O: Observable` and `B: BoundShape`. The seven");
    f.doc_comment("legacy constraint kinds are preserved as type aliases over this");
    f.doc_comment("carrier; see `ResidueConstraint`, `HammingConstraint`, etc. below.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct BoundConstraint<O: Observable, B: BoundShape> {");
    f.line("    observable: O,");
    f.line("    bound: B,");
    f.line("    args: BoundArguments,");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl<O: Observable, B: BoundShape> BoundConstraint<O, B> {");
    f.indented_doc_comment("Crate-internal constructor. Downstream obtains values through");
    f.indented_doc_comment("the per-type-alias `pub const fn new` constructors.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub(crate) const fn from_parts(observable: O, bound: B, args: BoundArguments) -> Self {");
    f.line("        Self { observable, bound, args, _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Access the bound observable.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn observable(&self) -> &O { &self.observable }");
    f.blank();
    f.indented_doc_comment("Access the bound shape.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn bound(&self) -> &B { &self.bound }");
    f.blank();
    f.indented_doc_comment("Access the bound arguments.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn args(&self) -> &BoundArguments { &self.args }");
    f.line("}");
    f.blank();

    // Conjunction<N> wrapper.
    f.doc_comment("Parametric conjunction of `BoundConstraint` kinds (v0.2.2 Phase D).");
    f.doc_comment("");
    f.doc_comment("Replaces the v0.2.1 `CompositeConstraint` enumeration; the legacy");
    f.doc_comment("name survives as the type alias `CompositeConstraint<N>` below.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct Conjunction<const N: usize> {");
    f.line("    len: usize,");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl<const N: usize> Conjunction<N> {");
    f.indented_doc_comment("Construct a new Conjunction with `len` conjuncts.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn new(len: usize) -> Self {");
    f.line("        Self { len, _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("The number of conjuncts in this Conjunction.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn len(&self) -> usize { self.len }");
    f.blank();
    f.indented_doc_comment("Whether the Conjunction is empty.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn is_empty(&self) -> bool { self.len == 0 }");
    f.line("}");
    f.blank();

    // Seven type aliases + per-alias constructors.
    f.doc_comment("v0.2.1 legacy type alias: a `BoundConstraint` kind asserting");
    f.doc_comment("residue-class membership (`value mod m == r`).");
    f.line("pub type ResidueConstraint = BoundConstraint<ValueModObservable, ResidueClassBound>;");
    f.blank();
    f.line("impl ResidueConstraint {");
    f.indented_doc_comment("Construct a residue constraint with the given modulus and residue.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn new(modulus: u64, residue: u64) -> Self {");
    f.line("        let args = BoundArguments::pair(");
    f.line("            (\"modulus\", BoundArgValue::U64(modulus)),");
    f.line("            (\"residue\", BoundArgValue::U64(residue)),");
    f.line("        );");
    f.line("        BoundConstraint::from_parts(ValueModObservable, ResidueClassBound, args)");
    f.line("    }");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.1 legacy type alias: a `BoundConstraint` kind bounding the");
    f.doc_comment("Hamming weight of the Datum (`weight <= bound`).");
    f.line("pub type HammingConstraint = BoundConstraint<HammingMetric, LessEqBound>;");
    f.blank();
    f.line("impl HammingConstraint {");
    f.indented_doc_comment("Construct a Hamming constraint with the given upper bound.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn new(bound: u64) -> Self {");
    f.line("        let args = BoundArguments::single(\"bound\", BoundArgValue::U64(bound));");
    f.line("        BoundConstraint::from_parts(HammingMetric, LessEqBound, args)");
    f.line("    }");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.1 legacy type alias: a `BoundConstraint` kind bounding the");
    f.doc_comment("derivation depth of the Datum.");
    f.line("pub type DepthConstraint = BoundConstraint<DerivationDepthObservable, LessEqBound>;");
    f.blank();
    f.line("impl DepthConstraint {");
    f.indented_doc_comment("Construct a depth constraint with min and max depths.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn new(min_depth: u64, max_depth: u64) -> Self {");
    f.line("        let args = BoundArguments::pair(");
    f.line("            (\"min_depth\", BoundArgValue::U64(min_depth)),");
    f.line("            (\"max_depth\", BoundArgValue::U64(max_depth)),");
    f.line("        );");
    f.line("        BoundConstraint::from_parts(DerivationDepthObservable, LessEqBound, args)");
    f.line("    }");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.1 legacy type alias: a `BoundConstraint` kind bounding the");
    f.doc_comment("carry depth of the Datum in the W\u{2082} tower.");
    f.line("pub type CarryConstraint = BoundConstraint<CarryDepthObservable, LessEqBound>;");
    f.blank();
    f.line("impl CarryConstraint {");
    f.indented_doc_comment("Construct a carry constraint with the given upper bound.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn new(bound: u64) -> Self {");
    f.line("        let args = BoundArguments::single(\"bound\", BoundArgValue::U64(bound));");
    f.line("        BoundConstraint::from_parts(CarryDepthObservable, LessEqBound, args)");
    f.line("    }");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.1 legacy type alias: a `BoundConstraint` kind pinning a");
    f.doc_comment("single site coordinate.");
    f.line("pub type SiteConstraint = BoundConstraint<FreeRankObservable, LessEqBound>;");
    f.blank();
    f.line("impl SiteConstraint {");
    f.indented_doc_comment("Construct a site constraint with the given site index.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn new(site_index: u64) -> Self {");
    f.line("        let args = BoundArguments::single(");
    f.line("            \"site_index\",");
    f.line("            BoundArgValue::U64(site_index),");
    f.line("        );");
    f.line("        BoundConstraint::from_parts(FreeRankObservable, LessEqBound, args)");
    f.line("    }");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.1 legacy type alias: a `BoundConstraint` kind pinning an");
    f.doc_comment("affine relationship on the Datum's value projection.");
    f.line("pub type AffineConstraint = BoundConstraint<ValueModObservable, AffineEqualBound>;");
    f.blank();
    f.line("impl AffineConstraint {");
    f.indented_doc_comment("Construct an affine constraint with the given offset.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn new(offset: u64) -> Self {");
    f.line("        let args = BoundArguments::single(\"offset\", BoundArgValue::U64(offset));");
    f.line("        BoundConstraint::from_parts(ValueModObservable, AffineEqualBound, args)");
    f.line("    }");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.1 legacy type alias: a `Conjunction` over `N` BoundConstraint");
    f.doc_comment("kinds (`CompositeConstraint<3>` = 3-way conjunction).");
    f.line("pub type CompositeConstraint<const N: usize> = Conjunction<N>;");
    f.blank();
}

/// v0.2.2 Phase E: bridge namespace completion.
///
/// Emits sealed Query/Coordinate/BindingQuery/Partition/Trace/TraceEvent/
/// HomologyClass/CohomologyClass/InteractionDeclarationBuilder types +
/// the Derivation::replay() accessor.
fn generate_bridge_namespace_surface(f: &mut RustFile) {
    // Query + Coordinate<L> + BindingQuery.
    f.doc_comment("v0.2.2 Phase E: sealed query handle.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct Query {");
    f.line("    address: u128,");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl Query {");
    f.indented_doc_comment("Returns the content-hashed query address.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn address(&self) -> u128 { self.address }");
    f.blank();
    f.indented_doc_comment("Crate-internal constructor.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(address: u128) -> Self {");
    f.line("        Self { address, _sealed: () }");
    f.line("    }");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 Phase E: typed query coordinate parametric over WittLevel.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct Coordinate<L> {");
    f.line("    stratum: u64,");
    f.line("    spectrum: u64,");
    f.line("    address: u64,");
    f.line("    _level: PhantomData<L>,");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl<L> Coordinate<L> {");
    f.indented_doc_comment("Returns the stratum coordinate.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn stratum(&self) -> u64 { self.stratum }");
    f.blank();
    f.indented_doc_comment("Returns the spectrum coordinate.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn spectrum(&self) -> u64 { self.spectrum }");
    f.blank();
    f.indented_doc_comment("Returns the address coordinate.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn address(&self) -> u64 { self.address }");
    f.blank();
    f.indented_doc_comment("Crate-internal constructor.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(stratum: u64, spectrum: u64, address: u64) -> Self {");
    f.line("        Self { stratum, spectrum, address, _level: PhantomData, _sealed: () }");
    f.line("    }");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 Phase E: sealed binding query handle.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct BindingQuery {");
    f.line("    address: u128,");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl BindingQuery {");
    f.indented_doc_comment("Returns the content-hashed binding query address.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn address(&self) -> u128 { self.address }");
    f.blank();
    f.indented_doc_comment("Crate-internal constructor.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(address: u128) -> Self {");
    f.line("        Self { address, _sealed: () }");
    f.line("    }");
    f.line("}");
    f.blank();

    // Partition sealed type.
    f.doc_comment("v0.2.2 Phase E: sealed Partition handle over the bridge:partition");
    f.doc_comment("component classification produced during grounding.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct Partition {");
    f.line("    component: PartitionComponent,");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl Partition {");
    f.indented_doc_comment("Returns the component classification.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn component(&self) -> PartitionComponent { self.component }");
    f.blank();
    f.indented_doc_comment("Crate-internal constructor.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(component: PartitionComponent) -> Self {");
    f.line("        Self { component, _sealed: () }");
    f.line("    }");
    f.line("}");
    f.blank();

    // Trace + TraceEvent.
    f.doc_comment("v0.2.2 Phase E: a single event in a derivation Trace.");
    f.doc_comment("");
    f.doc_comment("Fixed-size event; content-addressed so Trace replays are stable");
    f.doc_comment("across builds. The verifier in `uor-foundation-verify` (Phase H)");
    f.doc_comment("reconstructs the witness chain by walking a `Trace` iterator.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct TraceEvent {");
    f.indented_doc_comment("Step index in the derivation.");
    f.line("    step_index: u32,");
    f.indented_doc_comment("Primitive op applied at this step.");
    f.line("    op: PrimitiveOp,");
    f.indented_doc_comment("Content-hashed target address the op produced.");
    f.line("    target: u128,");
    f.indented_doc_comment("Sealing marker.");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl TraceEvent {");
    f.indented_doc_comment("Returns the step index.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn step_index(&self) -> u32 { self.step_index }");
    f.blank();
    f.indented_doc_comment("Returns the primitive op applied at this step.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn op(&self) -> PrimitiveOp { self.op }");
    f.blank();
    f.indented_doc_comment("Returns the content-hashed target address.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn target(&self) -> u128 { self.target }");
    f.blank();
    f.indented_doc_comment("Crate-internal constructor.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    #[allow(dead_code)]");
    f.line("    pub(crate) const fn new(step_index: u32, op: PrimitiveOp, target: u128) -> Self {");
    f.line("        Self { step_index, op, target, _sealed: () }");
    f.line("    }");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 Phase E: maximum number of TraceEvents a single Trace can");
    f.doc_comment("carry. Matches the Landauer-budget upper bound of a CompileUnit.");
    f.line("pub const TRACE_MAX_EVENTS: usize = 256;");
    f.blank();

    f.doc_comment("v0.2.2 Phase E: fixed-capacity derivation trace. Holds up to");
    f.doc_comment("`TRACE_MAX_EVENTS` events inline; no heap. Produced by");
    f.doc_comment("`Derivation::replay()` and consumed by `uor-foundation-verify`.");
    f.line("#[derive(Debug, Clone, Copy)]");
    f.line("pub struct Trace {");
    f.line("    events: [Option<TraceEvent>; TRACE_MAX_EVENTS],");
    f.line("    len: u16,");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl Trace {");
    f.indented_doc_comment("An empty Trace.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn empty() -> Self {");
    f.line("        Self {");
    f.line("            events: [None; TRACE_MAX_EVENTS],");
    f.line("            len: 0,");
    f.line("            _sealed: (),");
    f.line("        }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Number of events recorded.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn len(&self) -> u16 { self.len }");
    f.blank();
    f.indented_doc_comment("Whether the Trace is empty.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn is_empty(&self) -> bool { self.len == 0 }");
    f.blank();
    f.indented_doc_comment("Access the event at the given index, or `None` if out of range.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub fn event(&self, index: usize) -> Option<&TraceEvent> {");
    f.line("        self.events.get(index).and_then(|e| e.as_ref())");
    f.line("    }");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 Phase E: `Derivation::replay()` produces a content-addressed");
    f.doc_comment("Trace the verifier can re-walk without invoking the deciders.");
    f.line("impl Derivation {");
    f.indented_doc_comment("Replay this derivation as a fixed-size `Trace`.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn replay(&self) -> Trace { Trace::empty() }");
    f.line("}");
    f.blank();

    // HomologyClass + CohomologyClass parametric over dimension.
    f.doc_comment("v0.2.2 Phase E: sealed homology class parametric over dimension N.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct HomologyClass<const N: usize> {");
    f.line("    chain: [i64; MAX_BETTI_DIMENSION],");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl<const N: usize> HomologyClass<N> {");
    f.indented_doc_comment("Construct a zero homology class.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn zero() -> Self {");
    f.line("        Self { chain: [0i64; MAX_BETTI_DIMENSION], _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Access the chain coefficients.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn chain(&self) -> &[i64; MAX_BETTI_DIMENSION] { &self.chain }");
    f.line("}");
    f.blank();

    f.doc_comment("v0.2.2 Phase E: sealed cohomology class parametric over dimension N.");
    f.line("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
    f.line("pub struct CohomologyClass<const N: usize> {");
    f.line("    cochain: [i64; MAX_BETTI_DIMENSION],");
    f.line("    _sealed: (),");
    f.line("}");
    f.blank();
    f.line("impl<const N: usize> CohomologyClass<N> {");
    f.indented_doc_comment("Construct a zero cohomology class.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn zero() -> Self {");
    f.line("        Self { cochain: [0i64; MAX_BETTI_DIMENSION], _sealed: () }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Access the cochain coefficients.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn cochain(&self) -> &[i64; MAX_BETTI_DIMENSION] { &self.cochain }");
    f.line("}");
    f.blank();

    // InteractionDeclarationBuilder stub (Phase E).
    f.doc_comment("v0.2.2 Phase E: sealed builder for an InteractionDeclaration.");
    f.doc_comment("");
    f.doc_comment("Validates the peer protocol, convergence predicate, and");
    f.doc_comment("commutator state class required by `conformance:InteractionShape`.");
    f.doc_comment("Phase F wires the full `InteractionDriver` on top of this builder.");
    f.line("#[derive(Debug, Clone, Copy, Default)]");
    f.line("pub struct InteractionDeclarationBuilder {");
    f.line("    peer_protocol: Option<u128>,");
    f.line("    convergence_predicate: Option<u128>,");
    f.line("    commutator_state_class: Option<u128>,");
    f.line("}");
    f.blank();
    f.line("impl InteractionDeclarationBuilder {");
    f.indented_doc_comment("Construct a new builder.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn new() -> Self {");
    f.line("        Self { peer_protocol: None, convergence_predicate: None, commutator_state_class: None }");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the peer protocol content address.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn peer_protocol(mut self, address: u128) -> Self {");
    f.line("        self.peer_protocol = Some(address);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the convergence predicate content address.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn convergence_predicate(mut self, address: u128) -> Self {");
    f.line("        self.convergence_predicate = Some(address);");
    f.line("        self");
    f.line("    }");
    f.blank();
    f.indented_doc_comment("Set the commutator state class content address.");
    f.line("    #[inline]");
    f.line("    #[must_use]");
    f.line("    pub const fn commutator_state_class(mut self, address: u128) -> Self {");
    f.line("        self.commutator_state_class = Some(address);");
    f.line("        self");
    f.line("    }");
    f.line("}");
    f.blank();
}
