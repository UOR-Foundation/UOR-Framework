//! Smoke tests for the three SDK shape-constructor macros.
//!
//! Each test constructs a combined shape from two simple leaf shapes and
//! verifies the resulting `ConstrainedTypeShape` impl matches the amendment's
//! site-count / site-budget arithmetic.

use uor_foundation::pipeline::{
    CartesianProductShape, ConstrainedTypeShape, ConstraintRef, AFFINE_MAX_COEFFS,
};
use uor_foundation_sdk::{cartesian_product_shape, coproduct_shape, product_shape};

// Leaf shapes — Phase 17 expanded the SDK operand-support catalogue
// to every ConstraintRef variant. Affine and Conjunction now compose
// correctly through the macros because the variants store fixed-size
// arrays the const-eval can build inline.

pub struct LeafA;
impl ConstrainedTypeShape for LeafA {
    const IRI: &'static str = "https://example.org/sdk-smoke/LeafA";
    const SITE_COUNT: usize = 2;
    // SITE_BUDGET defaults to SITE_COUNT.
    const CONSTRAINTS: &'static [ConstraintRef] = &[
        ConstraintRef::Site { position: 0 },
        ConstraintRef::Site { position: 1 },
    ];
}

pub struct LeafB;
impl ConstrainedTypeShape for LeafB {
    const IRI: &'static str = "https://example.org/sdk-smoke/LeafB";
    const SITE_COUNT: usize = 3;
    const CONSTRAINTS: &'static [ConstraintRef] = &[
        ConstraintRef::Site { position: 0 },
        ConstraintRef::Carry { site: 1 },
        ConstraintRef::Site { position: 2 },
    ];
}

// --- product_shape! -------------------------------------------------------

product_shape!(LeafATimesLeafB, LeafA, LeafB);

#[test]
fn product_shape_site_budgets_add() {
    // PT_1: siteBudget(A × B) = siteBudget(A) + siteBudget(B).
    assert_eq!(<LeafATimesLeafB as ConstrainedTypeShape>::SITE_BUDGET, 5);
    // Layout invariant ProductLayoutWidth: SITE_COUNTs add.
    assert_eq!(<LeafATimesLeafB as ConstrainedTypeShape>::SITE_COUNT, 5);
}

#[test]
fn product_shape_constraints_splice_with_shift() {
    let constraints = <LeafATimesLeafB as ConstrainedTypeShape>::CONSTRAINTS;
    assert_eq!(constraints.len(), 5);
    // A's constraints copied verbatim.
    assert!(matches!(
        constraints[0],
        ConstraintRef::Site { position: 0 }
    ));
    assert!(matches!(
        constraints[1],
        ConstraintRef::Site { position: 1 }
    ));
    // B's constraints shifted by A::SITE_COUNT = 2.
    assert!(matches!(
        constraints[2],
        ConstraintRef::Site { position: 2 }
    ));
    assert!(matches!(constraints[3], ConstraintRef::Carry { site: 3 }));
    assert!(matches!(
        constraints[4],
        ConstraintRef::Site { position: 4 }
    ));
}

#[test]
fn product_shape_canonicalized_iri() {
    // Operand canonicalization sorts by token string: LeafA < LeafB.
    assert_eq!(
        <LeafATimesLeafB as ConstrainedTypeShape>::IRI,
        "urn:uor:product:LeafA:LeafB"
    );
}

// --- coproduct_shape! -----------------------------------------------------

coproduct_shape!(LeafAPlusLeafB, LeafA, LeafB);

#[test]
fn coproduct_shape_site_budget_maxes() {
    // ST_1: siteBudget(A + B) = max(siteBudget(A), siteBudget(B)).
    assert_eq!(<LeafAPlusLeafB as ConstrainedTypeShape>::SITE_BUDGET, 3);
    // CoproductLayoutWidth: SITE_COUNT = max(SITE_COUNT(A), SITE_COUNT(B)) + 1.
    assert_eq!(<LeafAPlusLeafB as ConstrainedTypeShape>::SITE_COUNT, 4);
}

#[test]
fn coproduct_shape_emits_two_tag_pinners() {
    let constraints = <LeafAPlusLeafB as ConstrainedTypeShape>::CONSTRAINTS;
    // A's constraints (2) + A's tag-pinner (1) + B's constraints (3) + B's tag-pinner (1) = 7.
    assert_eq!(constraints.len(), 7);

    // Tag site is at max(SITE_COUNT(A), SITE_COUNT(B)) = 3.
    // A's tag-pinner comes after A's constraints at index 2.
    match constraints[2] {
        ConstraintRef::Affine {
            coefficients,
            coefficient_count: _,
            bias,
        } => {
            assert_eq!(bias, 0, "left variant tag-pinner carries bias 0");
            assert_eq!(coefficients[3], 1, "coefficient at tag_site = 1");
        }
        _ => panic!(
            "expected Affine tag-pinner at index 2, got {:?}",
            constraints[2]
        ),
    }

    // B's tag-pinner comes after B's constraints at index 6.
    match constraints[6] {
        ConstraintRef::Affine {
            coefficients,
            coefficient_count: _,
            bias,
        } => {
            assert_eq!(bias, -1, "right variant tag-pinner carries bias -1");
            assert_eq!(coefficients[3], 1, "coefficient at tag_site = 1");
        }
        _ => panic!(
            "expected Affine tag-pinner at index 6, got {:?}",
            constraints[6]
        ),
    }
}

#[test]
fn coproduct_shape_canonicalized_iri() {
    assert_eq!(
        <LeafAPlusLeafB as ConstrainedTypeShape>::IRI,
        "urn:uor:coproduct:LeafA:LeafB"
    );
}

// --- cartesian_product_shape! ---------------------------------------------

cartesian_product_shape!(LeafATensorLeafB, LeafA, LeafB);

#[test]
fn cartesian_product_shape_site_budgets_add() {
    // CPT_1: siteBudget(A ⊠ B) = siteBudget(A) + siteBudget(B).
    assert_eq!(<LeafATensorLeafB as ConstrainedTypeShape>::SITE_BUDGET, 5);
    // CartesianLayoutWidth: SITE_COUNTs add.
    assert_eq!(<LeafATensorLeafB as ConstrainedTypeShape>::SITE_COUNT, 5);
}

#[test]
fn cartesian_product_shape_implements_marker() {
    // The macro emits the CartesianProductShape marker impl so the
    // Künneth-Betti primitive is selected.
    fn require_marker<S: CartesianProductShape>() {}
    require_marker::<LeafATensorLeafB>();
}

#[test]
fn cartesian_product_shape_canonicalized_iri() {
    assert_eq!(
        <LeafATensorLeafB as ConstrainedTypeShape>::IRI,
        "urn:uor:cartesian:LeafA:LeafB"
    );
}

// --- Phase 17: Affine + Conjunction operand support ----------------------

const AFFINE_TWO_PLUS_THREE: ([i64; AFFINE_MAX_COEFFS], u32) = {
    let mut a = [0i64; AFFINE_MAX_COEFFS];
    a[0] = 2;
    a[1] = 3;
    (a, 2)
};

/// Leaf shape carrying an `Affine` constraint — pre-Phase-17 this would
/// have been unsupported by the SDK macros.
pub struct LeafAffine;
impl ConstrainedTypeShape for LeafAffine {
    const IRI: &'static str = "https://example.org/sdk-smoke/LeafAffine";
    const SITE_COUNT: usize = 2;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Affine {
        coefficients: AFFINE_TWO_PLUS_THREE.0,
        coefficient_count: AFFINE_TWO_PLUS_THREE.1,
        bias: 0,
    }];
}

product_shape!(LeafAffineTimesLeafB, LeafAffine, LeafB);

#[test]
fn product_shape_supports_affine_operand() {
    // Pre-Phase-17 this expansion produced a `Site { position: u32::MAX }`
    // sentinel for the Affine constraint and the combined shape's
    // `validate_const()` rejected it. Post-Phase-17 the const-eval builds
    // a real shifted Affine — assert the constraint count covers L's
    // Affine + R's three constraints.
    let constraints = <LeafAffineTimesLeafB as ConstrainedTypeShape>::CONSTRAINTS;
    assert_eq!(constraints.len(), 4, "1 (L Affine) + 3 (R) = 4");
    // L's Affine pass-through (no shift since it's the first operand).
    match constraints[0] {
        ConstraintRef::Affine {
            coefficient_count, ..
        } => {
            assert_eq!(coefficient_count, 2, "L's affine prefix length preserved");
        }
        _ => panic!("expected Affine at index 0"),
    }
}

coproduct_shape!(LeafAffinePlusLeafB, LeafAffine, LeafB);

#[test]
fn coproduct_shape_supports_affine_operand() {
    let constraints = <LeafAffinePlusLeafB as ConstrainedTypeShape>::CONSTRAINTS;
    // L's Affine + L's tag-pinner + R's 3 + R's tag-pinner = 6.
    assert_eq!(constraints.len(), 6);
    match constraints[0] {
        ConstraintRef::Affine {
            coefficient_count, ..
        } => {
            assert_eq!(coefficient_count, 2, "L's Affine prefix length preserved");
        }
        _ => panic!("expected Affine at index 0"),
    }
    // L's tag-pinner at index 1.
    match constraints[1] {
        ConstraintRef::Affine {
            coefficient_count,
            bias,
            ..
        } => {
            assert!(coefficient_count > 0, "tag-pinner has non-zero prefix");
            assert_eq!(bias, 0, "L tag-pinner bias 0");
        }
        _ => panic!("expected Affine tag-pinner at index 1"),
    }
}
