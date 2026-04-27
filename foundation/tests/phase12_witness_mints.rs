//! Phase 12 verification: every Path-2 `Mint{Foo}` witness mints
//! successfully via `OntologyVerifiedMint::ontology_mint` with the
//! Phase-10 `Mint{Foo}Inputs<H>` placeholder. Each call returns
//! `Ok(witness)` (no `WITNESS_UNIMPLEMENTED_STUB:*` markers) and
//! produces a non-zero, identity-distinguishable fingerprint.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use uor_foundation::witness_scaffolds::{
    MintBornRuleVerification, MintBornRuleVerificationInputs, MintCompletenessWitness,
    MintCompletenessWitnessInputs, MintDisjointnessWitness, MintDisjointnessWitnessInputs,
    MintImpossibilityWitness, MintImpossibilityWitnessInputs, MintInhabitanceImpossibilityWitness,
    MintInhabitanceImpossibilityWitnessInputs, MintLiftObstruction, MintLiftObstructionInputs,
    MintMorphismGroundingWitness, MintMorphismGroundingWitnessInputs, MintProjectionWitness,
    MintProjectionWitnessInputs, MintStateGroundingWitness, MintStateGroundingWitnessInputs,
    MintWitness, MintWitnessInputs, OntologyVerifiedMint,
};
use uor_foundation::DefaultHostTypes;

fn assert_ok_with_fingerprint(witness_label: &str, fp: uor_foundation::ContentFingerprint) {
    assert!(
        !fp.is_zero(),
        "{witness_label}: fingerprint must be non-zero (Phase-12 baseline derives from IRI)"
    );
    assert!(
        fp.width_bytes() > 0,
        "{witness_label}: fingerprint width_bytes must be > 0"
    );
}

#[test]
fn br_mint_born_rule_verification() {
    let inputs = MintBornRuleVerificationInputs::<DefaultHostTypes>::default();
    let w = MintBornRuleVerification::ontology_mint::<DefaultHostTypes>(inputs)
        .expect("BR family verify must succeed for Phase-12 baseline");
    assert_ok_with_fingerprint("MintBornRuleVerification", w.content_fingerprint());
}

#[test]
fn cc_mint_completeness_witness() {
    let inputs = MintCompletenessWitnessInputs::<DefaultHostTypes>::default();
    let w = MintCompletenessWitness::ontology_mint::<DefaultHostTypes>(inputs)
        .expect("CC family verify must succeed");
    assert_ok_with_fingerprint("MintCompletenessWitness", w.content_fingerprint());
}

#[test]
fn dp_mint_disjointness_witness() {
    let inputs = MintDisjointnessWitnessInputs::<DefaultHostTypes>::default();
    let w = MintDisjointnessWitness::ontology_mint::<DefaultHostTypes>(inputs)
        .expect("DP family verify must succeed");
    assert_ok_with_fingerprint("MintDisjointnessWitness", w.content_fingerprint());
}

#[test]
fn ih_mint_impossibility_witness() {
    let inputs = MintImpossibilityWitnessInputs::<DefaultHostTypes>::default();
    let w = MintImpossibilityWitness::ontology_mint::<DefaultHostTypes>(inputs)
        .expect("IH/ImpossibilityWitness verify must succeed");
    assert_ok_with_fingerprint("MintImpossibilityWitness", w.content_fingerprint());
}

#[test]
fn ih_mint_inhabitance_impossibility_witness() {
    let inputs = MintInhabitanceImpossibilityWitnessInputs::<DefaultHostTypes>::default();
    let w = MintInhabitanceImpossibilityWitness::ontology_mint::<DefaultHostTypes>(inputs)
        .expect("IH/InhabitanceImpossibilityWitness verify must succeed");
    assert_ok_with_fingerprint(
        "MintInhabitanceImpossibilityWitness",
        w.content_fingerprint(),
    );
}

#[test]
fn lo_mint_lift_obstruction() {
    let inputs = MintLiftObstructionInputs::<DefaultHostTypes>::default();
    let w = MintLiftObstruction::ontology_mint::<DefaultHostTypes>(inputs)
        .expect("LO family verify must succeed");
    assert_ok_with_fingerprint("MintLiftObstruction", w.content_fingerprint());
}

#[test]
fn oa_mint_morphism_grounding_witness() {
    let inputs = MintMorphismGroundingWitnessInputs::<DefaultHostTypes>::default();
    let w = MintMorphismGroundingWitness::ontology_mint::<DefaultHostTypes>(inputs)
        .expect("OA/morphism::GroundingWitness verify must succeed");
    assert_ok_with_fingerprint("MintMorphismGroundingWitness", w.content_fingerprint());
}

#[test]
fn oa_mint_projection_witness() {
    let inputs = MintProjectionWitnessInputs::<DefaultHostTypes>::default();
    let w = MintProjectionWitness::ontology_mint::<DefaultHostTypes>(inputs)
        .expect("OA/morphism::ProjectionWitness verify must succeed");
    assert_ok_with_fingerprint("MintProjectionWitness", w.content_fingerprint());
}

#[test]
fn oa_mint_state_grounding_witness() {
    let inputs = MintStateGroundingWitnessInputs::<DefaultHostTypes>::default();
    let w = MintStateGroundingWitness::ontology_mint::<DefaultHostTypes>(inputs)
        .expect("OA/state::GroundingWitness verify must succeed");
    assert_ok_with_fingerprint("MintStateGroundingWitness", w.content_fingerprint());
}

#[test]
fn oa_mint_witness() {
    let inputs = MintWitnessInputs::<DefaultHostTypes>::default();
    let w = MintWitness::ontology_mint::<DefaultHostTypes>(inputs)
        .expect("OA/morphism::Witness verify must succeed");
    assert_ok_with_fingerprint("MintWitness", w.content_fingerprint());
}

#[test]
fn fingerprints_distinguish_witnesses_across_families() {
    // Ten different Mint{Foo} witnesses must produce ten distinct
    // fingerprints — the Phase-12 baseline derives each from the
    // class IRI, so each is unique.
    let mut fps = std::collections::HashSet::new();
    fps.insert(
        MintBornRuleVerification::ontology_mint::<DefaultHostTypes>(Default::default())
            .unwrap()
            .content_fingerprint(),
    );
    fps.insert(
        MintCompletenessWitness::ontology_mint::<DefaultHostTypes>(Default::default())
            .unwrap()
            .content_fingerprint(),
    );
    fps.insert(
        MintDisjointnessWitness::ontology_mint::<DefaultHostTypes>(Default::default())
            .unwrap()
            .content_fingerprint(),
    );
    fps.insert(
        MintImpossibilityWitness::ontology_mint::<DefaultHostTypes>(Default::default())
            .unwrap()
            .content_fingerprint(),
    );
    fps.insert(
        MintInhabitanceImpossibilityWitness::ontology_mint::<DefaultHostTypes>(Default::default())
            .unwrap()
            .content_fingerprint(),
    );
    fps.insert(
        MintLiftObstruction::ontology_mint::<DefaultHostTypes>(Default::default())
            .unwrap()
            .content_fingerprint(),
    );
    fps.insert(
        MintMorphismGroundingWitness::ontology_mint::<DefaultHostTypes>(Default::default())
            .unwrap()
            .content_fingerprint(),
    );
    fps.insert(
        MintProjectionWitness::ontology_mint::<DefaultHostTypes>(Default::default())
            .unwrap()
            .content_fingerprint(),
    );
    fps.insert(
        MintStateGroundingWitness::ontology_mint::<DefaultHostTypes>(Default::default())
            .unwrap()
            .content_fingerprint(),
    );
    fps.insert(
        MintWitness::ontology_mint::<DefaultHostTypes>(Default::default())
            .unwrap()
            .content_fingerprint(),
    );
    assert_eq!(
        fps.len(),
        10,
        "10 Path-2 witnesses must produce 10 distinct fingerprints; got {}",
        fps.len()
    );
}
