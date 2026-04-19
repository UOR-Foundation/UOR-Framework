//! Phase X.4 — constraint-nerve Betti tuple.
//!
//! Verifies that [`primitive_simplicial_nerve_betti`] computes the correct
//! Betti numbers for the 2-complex nerve built from `T::CONSTRAINTS`. The
//! nerve has:
//! - vertices = constraints
//! - 1-simplices = pairs of constraints with intersecting site-support
//! - 2-simplices = triples of constraints with a common site
//!
//! The primitive reduces to the rank of the boundary operators ∂_1, ∂_2
//! computed modulo a prime by [`integer_matrix_rank`]; rank-over-ℚ equals
//! rank-over-ℤ for totally unimodular boundary matrices.

use uor_foundation::enforcement::{primitive_simplicial_nerve_betti, MAX_BETTI_DIMENSION};
use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef};

/// Two Site constraints at distinct sites → 2 disconnected vertices, no edges.
struct DisconnectedPair;
impl ConstrainedTypeShape for DisconnectedPair {
    const IRI: &'static str = "https://example.org/phase_x4/DisconnectedPair";
    const SITE_COUNT: usize = 4;
    const CONSTRAINTS: &'static [ConstraintRef] = &[
        ConstraintRef::Site { position: 0 },
        ConstraintRef::Site { position: 1 },
    ];
}

#[test]
fn disconnected_pair_has_b0_equals_two() {
    let b = primitive_simplicial_nerve_betti::<DisconnectedPair>();
    assert_eq!(b[0], 2, "two disconnected components");
    assert_eq!(b[1], 0, "no 1-cycles");
    assert_eq!(b[2], 0, "no 2-cycles");
    for (k, &bk) in b.iter().enumerate().take(MAX_BETTI_DIMENSION).skip(3) {
        assert_eq!(bk, 0, "b_{k} vanishes above dimension 2");
    }
}

/// Three Residue constraints: every support is the full site set, so every
/// pair intersects and every triple shares a site. The nerve is the full
/// 2-simplex on 3 vertices — a filled triangle. Expected: `[1, 0, 0, ...]`.
struct FilledTriangle;
impl ConstrainedTypeShape for FilledTriangle {
    const IRI: &'static str = "https://example.org/phase_x4/FilledTriangle";
    const SITE_COUNT: usize = 4;
    const CONSTRAINTS: &'static [ConstraintRef] = &[
        ConstraintRef::Residue {
            modulus: 7,
            residue: 1,
        },
        ConstraintRef::Residue {
            modulus: 11,
            residue: 2,
        },
        ConstraintRef::Residue {
            modulus: 13,
            residue: 3,
        },
    ];
}

#[test]
fn filled_triangle_is_contractible() {
    let b = primitive_simplicial_nerve_betti::<FilledTriangle>();
    assert_eq!(b[0], 1, "one connected component");
    assert_eq!(b[1], 0, "the 2-simplex fills the 1-cycle");
    assert_eq!(b[2], 0, "2-simplex is a boundary, not a cycle");
}

/// Three Affine constraints with pairwise-overlapping but not triple-overlapping
/// supports: `{0,1}, {1,2}, {0,2}`. Three edges, zero 2-simplices — a 1-cycle.
/// Expected Betti: `[1, 1, 0, ...]`.
struct CircleNerve;
impl ConstrainedTypeShape for CircleNerve {
    const IRI: &'static str = "https://example.org/phase_x4/CircleNerve";
    const SITE_COUNT: usize = 4;
    const CONSTRAINTS: &'static [ConstraintRef] = &[
        ConstraintRef::Affine {
            coefficients: &[1, 1, 0, 0],
            bias: 0,
        },
        ConstraintRef::Affine {
            coefficients: &[0, 1, 1, 0],
            bias: 0,
        },
        ConstraintRef::Affine {
            coefficients: &[1, 0, 1, 0],
            bias: 0,
        },
    ];
}

#[test]
fn circle_nerve_has_one_nontrivial_loop() {
    let b = primitive_simplicial_nerve_betti::<CircleNerve>();
    assert_eq!(b[0], 1, "connected");
    assert_eq!(b[1], 1, "one independent 1-cycle (the triangle cycle)");
    assert_eq!(b[2], 0, "no 2-simplex to fill the loop");
    for &bk in b.iter().take(MAX_BETTI_DIMENSION).skip(3) {
        assert_eq!(bk, 0);
    }
}

/// Four Affine constraints laid out so every pair and every triple share at
/// least one site, but the quadruple does not collapse the complex: this is
/// the 2-skeleton of a tetrahedron — a triangulated 2-sphere.
/// Expected: `[1, 0, 1, 0, ...]`.
struct TetrahedronBoundary;
impl ConstrainedTypeShape for TetrahedronBoundary {
    const IRI: &'static str = "https://example.org/phase_x4/TetrahedronBoundary";
    const SITE_COUNT: usize = 7;
    const CONSTRAINTS: &'static [ConstraintRef] = &[
        ConstraintRef::Affine {
            coefficients: &[1, 1, 1, 1, 0, 0, 0],
            bias: 0,
        },
        ConstraintRef::Affine {
            coefficients: &[1, 1, 1, 0, 1, 0, 0],
            bias: 0,
        },
        ConstraintRef::Affine {
            coefficients: &[1, 1, 0, 1, 1, 0, 0],
            bias: 0,
        },
        ConstraintRef::Affine {
            coefficients: &[1, 0, 1, 1, 1, 0, 0],
            bias: 0,
        },
    ];
}

#[test]
fn tetrahedron_boundary_is_a_two_sphere() {
    let b = primitive_simplicial_nerve_betti::<TetrahedronBoundary>();
    assert_eq!(b[0], 1, "2-sphere is connected");
    assert_eq!(b[1], 0, "2-sphere has no 1-cycles");
    assert_eq!(b[2], 1, "2-sphere has one independent 2-cycle");
}

/// Degenerate: an empty constraint system is a 0-complex. Per the primitive's
/// convention `b_0 = 1` (the vacuous connected component).
struct Empty;
impl ConstrainedTypeShape for Empty {
    const IRI: &'static str = "https://example.org/phase_x4/Empty";
    const SITE_COUNT: usize = 4;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
}

#[test]
fn empty_nerve_has_unit_component() {
    let b = primitive_simplicial_nerve_betti::<Empty>();
    assert_eq!(b[0], 1);
    for &bk in b.iter().take(MAX_BETTI_DIMENSION).skip(1) {
        assert_eq!(bk, 0);
    }
}

/// Input determinism: calling the primitive twice on the same shape yields
/// the same Betti tuple.
#[test]
fn betti_is_content_deterministic() {
    let a = primitive_simplicial_nerve_betti::<CircleNerve>();
    let b = primitive_simplicial_nerve_betti::<CircleNerve>();
    assert_eq!(a, b);
}

/// Input-variation: different shapes with different nerve topologies produce
/// different Betti tuples.
#[test]
fn distinct_shapes_yield_distinct_betti_tuples() {
    let triangle = primitive_simplicial_nerve_betti::<FilledTriangle>();
    let circle = primitive_simplicial_nerve_betti::<CircleNerve>();
    let sphere = primitive_simplicial_nerve_betti::<TetrahedronBoundary>();
    let pair = primitive_simplicial_nerve_betti::<DisconnectedPair>();
    assert_ne!(triangle, circle);
    assert_ne!(triangle, sphere);
    assert_ne!(triangle, pair);
    assert_ne!(circle, sphere);
    assert_ne!(circle, pair);
    assert_ne!(sphere, pair);
}
