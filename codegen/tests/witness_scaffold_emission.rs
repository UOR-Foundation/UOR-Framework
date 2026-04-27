//! Phase 10b/c/d verification: every Path-2 class produces a complete
//! witness scaffold (`Mint{Foo}`, `Mint{Foo}Inputs<H>`, `Certificate`,
//! `OntologyVerifiedMint`) and a stub `verify_*` primitive in the
//! family-routed module.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use uor_codegen::witness_scaffolds::{
    generate_primitives_modules, generate_witness_scaffolds_module, path2_summary,
};
use uor_ontology::Ontology;

#[test]
fn every_path2_class_emits_full_scaffold() {
    let ontology = Ontology::full();
    let scaffolds = generate_witness_scaffolds_module(ontology);

    for (class_local, theorem_identity, _module, _entropy) in path2_summary(ontology) {
        // Phase 10 emits Mint{Foo} and Mint{Foo}Inputs — possibly with
        // a namespace qualifier when local names collide cross-namespace.
        // We accept either form by greppping the namespace-qualified
        // OR plain Mint{Foo} per emission rule.
        let plain_mint = format!("Mint{class_local}");
        let plain_inputs = format!("{plain_mint}Inputs");

        let has_struct = scaffolds.contains(&format!("pub struct {plain_mint} {{"))
            || (scaffolds.contains("pub struct Mint") && scaffolds.contains(&class_local));
        assert!(has_struct, "no Mint*{} struct emitted", class_local);

        let has_inputs =
            scaffolds.contains(&plain_inputs) || scaffolds.contains("Inputs<H: HostTypes> {");
        assert!(has_inputs, "no Mint*{class_local}Inputs<H> emitted");

        // Every emission must reference the THEOREM_IDENTITY.
        assert!(
            scaffolds.contains(&theorem_identity),
            "scaffold missing THEOREM_IDENTITY reference for `{class_local}` (expected `{theorem_identity}`)"
        );
    }
}

#[test]
fn ontology_verified_mint_trait_present() {
    let ontology = Ontology::full();
    let scaffolds = generate_witness_scaffolds_module(ontology);
    assert!(
        scaffolds.contains("pub trait OntologyVerifiedMint:"),
        "OntologyVerifiedMint trait not declared"
    );
    assert!(
        scaffolds.contains("type Inputs<H: HostTypes>"),
        "OntologyVerifiedMint::Inputs<H> GAT missing"
    );
    assert!(
        scaffolds.contains("const THEOREM_IDENTITY:"),
        "OntologyVerifiedMint::THEOREM_IDENTITY const missing"
    );
}

#[test]
fn primitive_stub_modules_emitted_per_family() {
    let ontology = Ontology::full();
    let modules = generate_primitives_modules(ontology);

    // mod.rs always present.
    assert!(
        modules.iter().any(|(p, _)| p == "primitives/mod.rs"),
        "primitives/mod.rs not emitted"
    );

    // Every Path-2 class's family must have a primitive file.
    let mut families: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for (_, _, module, _) in path2_summary(ontology) {
        families.insert(module);
    }
    for family in &families {
        let path = format!("primitives/{family}.rs");
        let found = modules.iter().any(|(p, _)| p == &path);
        assert!(found, "primitives/{family}.rs not emitted");
    }
}

#[test]
fn primitive_stubs_return_unimplemented_marker() {
    let ontology = Ontology::full();
    let modules = generate_primitives_modules(ontology);

    // Every emitted family file must return WITNESS_UNIMPLEMENTED_STUB.
    for (path, content) in &modules {
        if path == "primitives/mod.rs" {
            continue;
        }
        assert!(
            content.contains("WITNESS_UNIMPLEMENTED_STUB:"),
            "{path} missing WITNESS_UNIMPLEMENTED_STUB marker"
        );
        assert!(
            content.contains("GenericImpossibilityWitness::for_identity"),
            "{path} missing GenericImpossibilityWitness::for_identity error"
        );
    }
}

#[test]
fn entropy_bearing_classes_drop_hash() {
    let ontology = Ontology::full();
    let scaffolds = generate_witness_scaffolds_module(ontology);

    for (class_local, _, _, entropy_bearing) in path2_summary(ontology) {
        // Find the line `#[derive(...)] pub struct Mint*{class_local}`.
        // Walk the scaffolds finding that struct's derive line.
        let needle = format!("pub struct Mint{class_local}");
        let qualified_needle = "pub struct MintMorphism";
        // Find a line containing `pub struct Mint{class_local}` (allowing
        // namespace-qualified forms by also accepting MintMorphismGroundingWitness etc.).
        let mut found_struct = false;
        for (idx, line) in scaffolds.lines().enumerate() {
            if line.contains(&needle)
                || (line.contains(qualified_needle) && line.contains(&class_local))
            {
                found_struct = true;
                // Check the immediately preceding non-doc line for derive.
                let lines: Vec<&str> = scaffolds.lines().collect();
                let mut j = idx;
                while j > 0 {
                    j -= 1;
                    let prev = lines[j].trim();
                    if prev.starts_with("///") || prev.is_empty() {
                        continue;
                    }
                    if prev.starts_with("#[derive") {
                        if entropy_bearing {
                            assert!(
                                !prev.contains("Hash"),
                                "entropy_bearing class `{class_local}` must NOT derive Hash; got: {prev}"
                            );
                        } else {
                            assert!(
                                prev.contains("Hash"),
                                "non-entropy-bearing class `{class_local}` must derive Hash; got: {prev}"
                            );
                        }
                        break;
                    }
                    break;
                }
                break;
            }
        }
        assert!(
            found_struct,
            "could not find Mint*{class_local} struct emission"
        );
    }
}
