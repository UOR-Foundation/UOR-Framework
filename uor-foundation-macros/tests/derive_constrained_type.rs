//! v0.2.1 integration test: `#[derive(ConstrainedType)]`.

use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef};
use uor_foundation_macros::ConstrainedType;

#[derive(ConstrainedType, Default)]
#[uor(residue = 255, hamming = 8)]
#[allow(dead_code)]
struct Pixel(u8);

#[derive(ConstrainedType, Default)]
#[uor(residue = 65535, hamming = 16)]
#[allow(dead_code)]
struct MatVecRow<const M: usize, const K: usize> {
    _marker: core::marker::PhantomData<([u16; M], [(); K])>,
}

#[test]
fn pixel_iri_constant_present() {
    assert_eq!(
        Pixel::UOR_CONSTRAINED_TYPE_IRI,
        "https://uor.foundation/type/ConstrainedType"
    );
}

#[test]
fn matvec_iri_constant_present() {
    assert_eq!(
        MatVecRow::<64, 2048>::UOR_CONSTRAINED_TYPE_IRI,
        "https://uor.foundation/type/ConstrainedType"
    );
}

#[test]
fn pixel_implements_constrained_type_shape() {
    // The derive emits a ConstrainedTypeShape impl carrying the residue + hamming
    // constraints as a static array. Verify both entries flow through.
    assert_eq!(Pixel::SITE_COUNT, 8);
    assert_eq!(Pixel::CONSTRAINTS.len(), 2);
    let has_residue = Pixel::CONSTRAINTS.iter().any(|c| {
        matches!(
            c,
            ConstraintRef::Residue {
                modulus: 256,
                residue: 255
            }
        )
    });
    let has_hamming = Pixel::CONSTRAINTS
        .iter()
        .any(|c| matches!(c, ConstraintRef::Hamming { bound: 8 }));
    assert!(has_residue, "residue constraint missing");
    assert!(has_hamming, "hamming constraint missing");
}

#[test]
fn pixel_flows_through_certify_one_liner() {
    // The v0.2.1 consumer-facing one-liner: the derive's impls let Pixel
    // flow through `TowerCompletenessResolver::new().certify(&pixel)` and
    // `InhabitanceResolver::new().certify(&pixel)` directly, matching the
    // ergonomics-spec §3.2 snippet.
    use uor_foundation::enforcement::{Certify, InhabitanceResolver, TowerCompletenessResolver};
    let pixel = Pixel::default();
    let tower = TowerCompletenessResolver::new().certify(&pixel);
    assert!(
        tower.is_ok(),
        "Pixel (residue + hamming, no SatClauses) classifies as residual-vacuous and certifies"
    );
    let inhab = InhabitanceResolver::new().certify(&pixel);
    assert!(
        inhab.is_ok(),
        "Pixel inhabitance query succeeds on the same classification"
    );
}

#[test]
fn pixel_implements_grounded_shape_via_macro_backdoor() {
    // Compile-time verification: the derive successfully opens the sealed
    // GroundedShape supertrait so Pixel can be used as the parameter of
    // `Grounded<T>` anywhere in downstream code. If the derive's
    // `__macro_internals::GroundedShapeSealed` impl is missing, this fails
    // to compile.
    fn takes_grounded_shape<T: uor_foundation::enforcement::GroundedShape>() {}
    takes_grounded_shape::<Pixel>();
    takes_grounded_shape::<MatVecRow<64, 2048>>();
}

// ==========================================================================
// v0.2.1 Phase 8a.3: 6-constraint-kind coverage.
//
// Every constraint kind supported by `#[derive(ConstrainedType)]` must have
// at least one test that asserts the emitted `CONSTRAINTS` slice carries
// the expected `ConstraintRef` variant. Residue + Hamming are already
// covered above by `Pixel`. The four remaining kinds (Carry, Depth, Site,
// Affine) and the nested `#[uor(residue(modulus=X, residue=Y))]` form
// are tested here.
// ==========================================================================

#[derive(ConstrainedType, Default)]
#[uor(residue(modulus = 1024, residue = 511))]
#[allow(dead_code)]
struct NestedResidueShape;

#[test]
fn nested_residue_form_emits_explicit_modulus() {
    let has_residue = NestedResidueShape::CONSTRAINTS.iter().any(|c| {
        matches!(
            c,
            ConstraintRef::Residue {
                modulus: 1024,
                residue: 511
            }
        )
    });
    assert!(
        has_residue,
        "nested `residue(modulus = 1024, residue = 511)` form should emit \
         `ConstraintRef::Residue {{ modulus: 1024, residue: 511 }}`"
    );
}

#[derive(ConstrainedType, Default)]
#[uor(carry(site = 3))]
#[allow(dead_code)]
struct CarryShape;

#[test]
fn carry_constraint_emits_carry_variant() {
    let has_carry = CarryShape::CONSTRAINTS
        .iter()
        .any(|c| matches!(c, ConstraintRef::Carry { site: 3 }));
    assert!(
        has_carry,
        "carry(site = 3) should emit ConstraintRef::Carry"
    );
}

#[derive(ConstrainedType, Default)]
#[uor(depth(min = 0, max = 8))]
#[allow(dead_code)]
struct DepthShape;

#[test]
fn depth_constraint_emits_depth_variant() {
    let has_depth = DepthShape::CONSTRAINTS
        .iter()
        .any(|c| matches!(c, ConstraintRef::Depth { min: 0, max: 8 }));
    assert!(
        has_depth,
        "depth(min = 0, max = 8) should emit ConstraintRef::Depth"
    );
}

#[derive(ConstrainedType, Default)]
#[uor(site(position = 7))]
#[allow(dead_code)]
struct SiteShape;

#[test]
fn site_constraint_emits_site_variant() {
    let has_site = SiteShape::CONSTRAINTS
        .iter()
        .any(|c| matches!(c, ConstraintRef::Site { position: 7 }));
    assert!(
        has_site,
        "site(position = 7) should emit ConstraintRef::Site"
    );
    // SITE_COUNT heuristic: max(from_hamming, from_sites+1, from_affine)
    // — with a single site at position 7, the derived site count is 8.
    assert_eq!(SiteShape::SITE_COUNT, 8);
}

#[derive(ConstrainedType, Default)]
#[uor(affine(coefficients = [1, -1, 0, 2], bias = 5))]
#[allow(dead_code)]
struct AffineShape;

#[test]
fn affine_constraint_emits_affine_variant() {
    let has_affine = AffineShape::CONSTRAINTS.iter().any(|c| {
        if let ConstraintRef::Affine { coefficients, bias } = c {
            coefficients == &[1, -1, 0, 2] && *bias == 5
        } else {
            false
        }
    });
    assert!(
        has_affine,
        "affine(coefficients = [1, -1, 0, 2], bias = 5) should emit \
         ConstraintRef::Affine with the exact coefficient slice and bias"
    );
}

#[derive(ConstrainedType, Default)]
#[uor(residue(modulus = 256, residue = 0))]
#[uor(hamming(bound = 4))]
#[uor(carry(site = 1))]
#[uor(depth(min = 1, max = 5))]
#[uor(site(position = 3))]
#[uor(affine(coefficients = [1, 1], bias = 0))]
#[allow(dead_code)]
struct AllSixKinds;

#[test]
fn all_six_constraint_kinds_emit_together() {
    // Verify every variant from the six supported kinds appears exactly
    // once in the emitted CONSTRAINTS slice.
    let cs = AllSixKinds::CONSTRAINTS;
    assert!(cs.iter().any(|c| matches!(
        c,
        ConstraintRef::Residue {
            modulus: 256,
            residue: 0
        }
    )));
    assert!(cs
        .iter()
        .any(|c| matches!(c, ConstraintRef::Hamming { bound: 4 })));
    assert!(cs
        .iter()
        .any(|c| matches!(c, ConstraintRef::Carry { site: 1 })));
    assert!(cs
        .iter()
        .any(|c| matches!(c, ConstraintRef::Depth { min: 1, max: 5 })));
    assert!(cs
        .iter()
        .any(|c| matches!(c, ConstraintRef::Site { position: 3 })));
    assert!(cs.iter().any(|c| {
        if let ConstraintRef::Affine { coefficients, bias } = c {
            coefficients == &[1, 1] && *bias == 0
        } else {
            false
        }
    }));
    assert_eq!(
        cs.len(),
        6,
        "exactly 6 constraints should be emitted, one per kind"
    );
}
