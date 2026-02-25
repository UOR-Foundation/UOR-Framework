//! Type-relativity cross-check: verifies the SIH's central prediction that
//! irreducibility is type-relative by comparing T₂ and T_poly(2) partitions.

use uor::frame::{
    emanation, partition_distance, stratum_histogram, Partition, TypeDeclaration,
    GF2_IRREDUCIBLES_Q0, PRIMES_Q0,
};

/// Four-way cross-classification: BOTH, INT_ONLY, POLY_ONLY, NEITHER.
#[test]
fn type_relativity_counts() {
    let t2 = TypeDeclaration::integer_mul();
    let tp = TypeDeclaration::poly_gf2();
    let p2 = Partition::compute(&t2);
    let pp = Partition::compute(&tp);

    let both = p2.irr().intersection(pp.irr());
    let int_only = p2.irr().difference(pp.irr());
    let poly_only = pp.irr().difference(p2.irr());

    assert_eq!(both.len(), 27, "BOTH: expected 27, got {}", both.len());
    assert_eq!(
        int_only.len(),
        27,
        "INT_ONLY: expected 27, got {}",
        int_only.len()
    );
    assert_eq!(
        poly_only.len(),
        14,
        "POLY_ONLY: expected 14, got {}",
        poly_only.len()
    );
}

/// Datum 5 is INT_ONLY: prime (5) but reducible polynomial (x²+1 = (x+1)²).
#[test]
fn datum_5_is_int_only() {
    let t2 = TypeDeclaration::integer_mul();
    let tp = TypeDeclaration::poly_gf2();
    let p2 = Partition::compute(&t2);
    let pp = Partition::compute(&tp);

    assert!(p2.irr().contains(5), "5 should be prime");
    assert!(pp.red().contains(5), "5 should be reducible polynomial");
}

/// Datum 25 is POLY_ONLY: composite (5×5) but irreducible polynomial (x⁴+x³+1).
#[test]
fn datum_25_is_poly_only() {
    let t2 = TypeDeclaration::integer_mul();
    let tp = TypeDeclaration::poly_gf2();
    let p2 = Partition::compute(&t2);
    let pp = Partition::compute(&tp);

    assert!(p2.red().contains(25), "25 should be composite");
    assert!(pp.irr().contains(25), "25 should be irreducible polynomial");
}

/// Partition distance dΠ(T₂, T_poly(2)) ≈ 41/254 ≈ 0.161.
#[test]
fn partition_distance_value() {
    let t2 = TypeDeclaration::integer_mul();
    let tp = TypeDeclaration::poly_gf2();
    let p2 = Partition::compute(&t2);
    let pp = Partition::compute(&tp);

    let d = partition_distance(&p2, &pp, 254);
    let expected = 41.0 / 254.0;
    assert!(
        (d - expected).abs() < 1e-6,
        "dΠ = {d}, expected ≈ {expected}"
    );
}

/// Stratum 4 has 13 primes and 0 irreducible polynomials.
#[test]
fn stratum_4_divergence() {
    let t2 = TypeDeclaration::integer_mul();
    let tp = TypeDeclaration::poly_gf2();
    let p2 = Partition::compute(&t2);
    let pp = Partition::compute(&tp);

    let h_primes = stratum_histogram(p2.irr());
    let h_polys = stratum_histogram(pp.irr());

    assert_eq!(h_primes[4], 13);
    assert_eq!(h_polys[4], 0);
}

/// Emanation sequences diverge at k=3.
#[test]
fn emanation_divergence_at_k3() {
    let t2 = TypeDeclaration::integer_mul();
    let tp = TypeDeclaration::poly_gf2();
    let p2 = Partition::compute(&t2);
    let pp = Partition::compute(&tp);

    // k=1,2 agree
    assert_eq!(emanation(&p2, 1), emanation(&pp, 1));
    assert_eq!(emanation(&p2, 2), emanation(&pp, 2));

    // k=3 diverges
    assert_eq!(emanation(&p2, 3), Some(5));
    assert_eq!(emanation(&pp, 3), Some(7));
}

/// Partition invariant: four sets are disjoint and cover all 256 values.
#[test]
fn partition_invariant_both_types() {
    let t2 = TypeDeclaration::integer_mul();
    let tp = TypeDeclaration::poly_gf2();
    assert!(Partition::compute(&t2).verify());
    assert!(Partition::compute(&tp).verify());
}

/// Pre-computed statics agree with runtime partition.
#[test]
fn statics_agree_with_partition() {
    let t2 = TypeDeclaration::integer_mul();
    let tp = TypeDeclaration::poly_gf2();
    let p2 = Partition::compute(&t2);
    let pp = Partition::compute(&tp);

    assert_eq!(*p2.irr(), PRIMES_Q0);
    assert_eq!(*pp.irr(), GF2_IRREDUCIBLES_Q0);
}
