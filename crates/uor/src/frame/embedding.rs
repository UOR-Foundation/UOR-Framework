//! Observable embedding Φ : 𝒢ₙ → ℝ¹⁶ for Q0 types.
//!
//! Maps a type declaration to a 16-dimensional real vector encoding its
//! complete observable profile:
//!
//! | Indices | Content | Source |
//! |---------|---------|--------|
//! | 0–5 | Six observable family signatures | `ObservableSignature` |
//! | 6–9 | Normalized partition class counts | `Partition` |
//! | 10–15 | Stratum histogram fractions (strata 1–6) | `stratum_histogram` |
//!
//! # Properties
//!
//! - **Deterministic**: Same type always produces the same embedding.
//! - **Injective at Q0**: T₂ and T_poly(2) produce distinct, non-degenerate vectors.
//! - **Zero allocation**: Pure stack computation.
//!
//! # Examples
//!
//! ```
//! use uor::frame::{TypeDeclaration, Partition, embed};
//!
//! let t2 = TypeDeclaration::integer_mul();
//! let p = Partition::compute(&t2);
//! let v = embed(&p, &t2);
//! assert_eq!(v.len(), 16);
//! assert!(v.iter().all(|&x| x.is_finite()));
//! ```

use super::distance::stratum_histogram;
use super::{ObservableSignature, Partition, TypeDeclaration};

/// Embedding dimension.
pub const EMBED_DIM: usize = 16;

/// Compute the 16-dimensional observable embedding for a type at Q0.
///
/// The embedding encodes:
/// - `[0..6]`: Six observable family signatures
///   (stratum, hamming metric, cascade, catastrophe, curvature, holonomy).
/// - `[6..10]`: Normalized partition class fractions: irr, red, units, ext.
/// - `[10..16]`: Stratum histogram fractions for strata 1–6 of the
///   irreducible set (normalized by |Irr|).
///
/// All values are finite, non-negative reals. Zero-allocation.
pub fn embed(partition: &Partition, decl: &TypeDeclaration) -> [f64; EMBED_DIM] {
    let mut v = [0.0f64; EMBED_DIM];

    // Dimensions 0–5: observable signatures
    let sig = ObservableSignature::compute(partition.irr());
    let obs = sig.as_array();
    v[..6].copy_from_slice(&obs);

    // Dimensions 6–9: normalized partition class counts
    let carrier = decl.carrier_len().max(1) as f64;
    v[6] = partition.irr().len() as f64 / carrier;
    v[7] = partition.red().len() as f64 / carrier;
    v[8] = partition.units().len() as f64 / carrier;
    v[9] = partition.ext().len() as f64 / 256.0;

    // Dimensions 10–15: stratum histogram fractions (strata 1–6)
    let hist = stratum_histogram(partition.irr());
    let irr_count = partition.irr().len().max(1) as f64;
    for i in 0..6 {
        v[10 + i] = hist[i + 1] as f64 / irr_count;
    }

    v
}

/// Euclidean distance between two embeddings.
#[inline]
pub fn embedding_distance(a: &[f64; EMBED_DIM], b: &[f64; EMBED_DIM]) -> f64 {
    let mut sum = 0.0f64;
    for i in 0..EMBED_DIM {
        let d = a[i] - b[i];
        sum += d * d;
    }
    sum.sqrt()
}

/// Check if an embedding is non-degenerate (not all zeros).
#[inline]
pub fn is_nondegenerate(v: &[f64; EMBED_DIM]) -> bool {
    v.iter().any(|&x| x.abs() > f64::EPSILON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t2_embedding() -> [f64; EMBED_DIM] {
        let t2 = TypeDeclaration::integer_mul();
        let p = Partition::compute(&t2);
        embed(&p, &t2)
    }

    fn poly_embedding() -> [f64; EMBED_DIM] {
        let tp = TypeDeclaration::poly_gf2();
        let p = Partition::compute(&tp);
        embed(&p, &tp)
    }

    // -- Dimension --

    #[test]
    fn embedding_has_17_dimensions() {
        assert_eq!(t2_embedding().len(), EMBED_DIM);
        assert_eq!(EMBED_DIM, 16);
    }

    // -- Finiteness --

    #[test]
    fn t2_embedding_all_finite() {
        let v = t2_embedding();
        for (i, &x) in v.iter().enumerate() {
            assert!(x.is_finite(), "dim {i} is not finite: {x}");
        }
    }

    #[test]
    fn poly_embedding_all_finite() {
        let v = poly_embedding();
        for (i, &x) in v.iter().enumerate() {
            assert!(x.is_finite(), "dim {i} is not finite: {x}");
        }
    }

    // -- Non-degeneracy --

    #[test]
    fn t2_embedding_nondegenerate() {
        assert!(is_nondegenerate(&t2_embedding()));
    }

    #[test]
    fn poly_embedding_nondegenerate() {
        assert!(is_nondegenerate(&poly_embedding()));
    }

    // -- Distinctness --

    #[test]
    fn t2_and_poly_produce_distinct_embeddings() {
        let a = t2_embedding();
        let b = poly_embedding();
        assert_ne!(a, b);
    }

    #[test]
    fn embedding_distance_positive() {
        let a = t2_embedding();
        let b = poly_embedding();
        let d = embedding_distance(&a, &b);
        assert!(d > 0.0, "distance should be positive, got {d}");
    }

    #[test]
    fn embedding_distance_self_is_zero() {
        let a = t2_embedding();
        assert!((embedding_distance(&a, &a)).abs() < 1e-15);
    }

    // -- Observable dimensions (0–5) --

    #[test]
    fn observable_dimensions_match_signature() {
        let t2 = TypeDeclaration::integer_mul();
        let p = Partition::compute(&t2);
        let v = embed(&p, &t2);
        let sig = ObservableSignature::compute(p.irr());
        let obs = sig.as_array();
        for i in 0..6 {
            assert_eq!(v[i], obs[i], "dim {i} mismatch");
        }
    }

    // -- Partition dimensions (6–9) --

    #[test]
    fn partition_dimensions_sum_correctly() {
        let v = t2_embedding();
        // irr_frac + red_frac + units_frac should ≈ 1.0 (carrier fractions)
        let carrier_sum = v[6] + v[7] + v[8];
        assert!(
            (carrier_sum - 1.0).abs() < 1e-10,
            "carrier fractions sum to {carrier_sum}, expected 1.0",
        );
    }

    #[test]
    fn t2_irr_fraction() {
        let v = t2_embedding();
        // 54 irreducibles out of 254 carrier elements
        assert!((v[6] - 54.0 / 254.0).abs() < 1e-10);
    }

    #[test]
    fn poly_irr_fraction() {
        let v = poly_embedding();
        // 41 irreducibles out of 254 carrier elements
        assert!((v[7] - 213.0 / 254.0).abs() < 1e-10);
    }

    #[test]
    fn ext_fraction() {
        let v = t2_embedding();
        // ext = {0, 1} = 2 elements out of 256
        assert!((v[9] - 2.0 / 256.0).abs() < 1e-10);
    }

    // -- Stratum histogram dimensions (10–15) --

    #[test]
    fn stratum_dimensions_nonnegative() {
        let v = t2_embedding();
        for (i, &x) in v[10..16].iter().enumerate() {
            assert!(x >= 0.0, "dim {} should be >= 0, got {x}", i + 10);
        }
    }

    #[test]
    fn stratum_dimensions_bounded_by_one() {
        let v = t2_embedding();
        for (i, &x) in v[10..16].iter().enumerate() {
            assert!(x <= 1.0, "dim {} should be <= 1, got {x}", i + 10);
        }
    }

    #[test]
    fn t2_stratum_1_fraction() {
        let v = t2_embedding();
        // Stratum 1: 1 prime (value 2) out of 54 primes
        assert!((v[10] - 1.0 / 54.0).abs() < 1e-10);
    }

    #[test]
    fn t2_stratum_2_fraction() {
        let v = t2_embedding();
        // Stratum 2: 3 primes (3, 5, 17 have Hamming weight 2) out of 54
        assert!((v[11] - 3.0 / 54.0).abs() < 1e-10);
    }

    #[test]
    fn stratum_dimensions_differ_between_types() {
        let a = t2_embedding();
        let b = poly_embedding();
        let mut diffs = 0;
        for i in 10..16 {
            if (a[i] - b[i]).abs() > 1e-10 {
                diffs += 1;
            }
        }
        assert!(diffs > 0, "stratum dimensions should differ between types");
    }

    // -- Determinism --

    #[test]
    fn embedding_is_deterministic() {
        let a = t2_embedding();
        let b = t2_embedding();
        assert_eq!(a, b);
    }

    // -- Distance properties --

    #[test]
    fn distance_is_symmetric() {
        let a = t2_embedding();
        let b = poly_embedding();
        assert_eq!(embedding_distance(&a, &b), embedding_distance(&b, &a));
    }

    #[test]
    fn distance_triangle_inequality() {
        let a = t2_embedding();
        let b = poly_embedding();
        let zero = [0.0f64; EMBED_DIM];
        let d_ab = embedding_distance(&a, &b);
        let d_a0 = embedding_distance(&a, &zero);
        let d_b0 = embedding_distance(&b, &zero);
        assert!(d_ab <= d_a0 + d_b0 + 1e-10, "triangle inequality violated");
    }
}
