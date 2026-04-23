//! Smoke tests for the three SDK shape-constructor macros.
//!
//! Each test constructs a combined shape from two simple leaf shapes and
//! verifies the resulting `ConstrainedTypeShape` impl matches the amendment's
//! site-count / site-budget arithmetic.

use uor_foundation::pipeline::{CartesianProductShape, ConstrainedTypeShape, ConstraintRef};
use uor_foundation_sdk::{cartesian_product_shape, coproduct_shape, product_shape};

// Leaf shapes — SDK operands must contain only the SDK-supported
// ConstraintRef variants (Site, Carry, Residue, Hamming, Depth, SatClauses,
// Bound), never Affine or Conjunction.

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
        ConstraintRef::Affine { coefficients, bias } => {
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
        ConstraintRef::Affine { coefficients, bias } => {
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
