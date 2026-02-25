//! Cross-field distance matrix — pairwise distances between type instances.
//!
//! Computes partition distance and stratum distance between all pairs of
//! type declarations, producing a 4×4 distance matrix for the four Q0
//! types: T₂, T_poly(2), T_poly(3), T_poly(5).
//!
//! # Properties
//!
//! Both distance functions satisfy:
//! - **Identity**: d(T, T) = 0
//! - **Symmetry**: d(T, T') = d(T', T)
//! - **Triangle inequality**: d(T, T'') ≤ d(T, T') + d(T', T'')
//!
//! # Examples
//!
//! ```
//! use uor::frame::{TypeDeclaration, Partition, partition_distance};
//!
//! let t2 = TypeDeclaration::integer_mul();
//! let t3 = TypeDeclaration::poly_gf3();
//! let p2 = Partition::compute(&t2);
//! let p3 = Partition::compute(&t3);
//!
//! let d = partition_distance(&p2, &p3, 254);
//! assert!(d > 0.0);
//! assert!(d <= 1.0);
//! ```

use super::distance::{partition_distance, stratum_distance};
use super::{Partition, TypeDeclaration};

/// Number of types in the cross-field comparison.
pub const TYPE_COUNT: usize = 4;

/// Compute partition distance matrix for the four Q0 types.
///
/// Returns a 4×4 matrix where entry `[i][j]` is `partition_distance(T_i, T_j)`.
/// Types are ordered: T₂, T_poly(2), T_poly(3), T_poly(5).
pub fn partition_distance_matrix() -> [[f64; TYPE_COUNT]; TYPE_COUNT] {
    let decls = all_decls();
    let parts: [Partition; TYPE_COUNT] = core::array::from_fn(|i| Partition::compute(&decls[i]));
    let mut matrix = [[0.0f64; TYPE_COUNT]; TYPE_COUNT];
    for i in 0..TYPE_COUNT {
        for j in 0..TYPE_COUNT {
            matrix[i][j] = partition_distance(
                &parts[i],
                &parts[j],
                decls[i].carrier_len().max(decls[j].carrier_len()),
            );
        }
    }
    matrix
}

/// Compute stratum distance matrix for the four Q0 types.
///
/// Returns a 5×5 matrix where entry `[i][j]` is `stratum_distance(T_i, T_j)`.
pub fn stratum_distance_matrix() -> [[f64; TYPE_COUNT]; TYPE_COUNT] {
    let decls = all_decls();
    let parts: [Partition; TYPE_COUNT] = core::array::from_fn(|i| Partition::compute(&decls[i]));
    let mut matrix = [[0.0f64; TYPE_COUNT]; TYPE_COUNT];
    for i in 0..TYPE_COUNT {
        for j in 0..TYPE_COUNT {
            matrix[i][j] = stratum_distance(&parts[i], &parts[j]);
        }
    }
    matrix
}

/// The four Q0 type declarations in canonical order.
fn all_decls() -> [TypeDeclaration; TYPE_COUNT] {
    [
        TypeDeclaration::integer_mul(),
        TypeDeclaration::poly_gf2(),
        TypeDeclaration::poly_gf3(),
        TypeDeclaration::poly_gf5(),
    ]
}

/// Type labels for display and debugging.
pub const TYPE_LABELS: [&str; TYPE_COUNT] = ["T₂", "T_poly(2)", "T_poly(3)", "T_poly(5)"];

#[cfg(test)]
mod tests {
    use super::*;

    /// Check metric axioms on a distance matrix.
    fn assert_metric_axioms(m: &[[f64; TYPE_COUNT]; TYPE_COUNT], label: &str) {
        // Diagonal zero
        for (i, row) in m.iter().enumerate() {
            assert_eq!(row[i], 0.0, "{label}: diagonal [{i}][{i}] should be 0");
        }
        // Symmetry
        for (i, row_i) in m.iter().enumerate() {
            for (j, &d_ij) in row_i.iter().enumerate() {
                assert_eq!(d_ij, m[j][i], "{label}: asymmetric at [{i}][{j}]");
            }
        }
        // Positive off-diagonal
        for (i, row_i) in m.iter().enumerate() {
            for (j, &d_ij) in row_i.iter().enumerate() {
                if i != j {
                    assert!(d_ij > 0.0, "{label}: [{i}][{j}] should be positive: {d_ij}");
                }
            }
        }
        // Triangle inequality
        for (i, row_i) in m.iter().enumerate() {
            for (j, _) in m.iter().enumerate() {
                for (k, &d_ik) in row_i.iter().enumerate() {
                    assert!(
                        d_ik <= m[i][j] + m[j][k] + 1e-10,
                        "{label}: triangle [{i}][{k}]={d_ik} > [{i}][{j}]={} + [{j}][{k}]={}",
                        m[i][j],
                        m[j][k]
                    );
                }
            }
        }
        // Bounded by 1
        for (i, row) in m.iter().enumerate() {
            for (j, &d) in row.iter().enumerate() {
                assert!(d <= 1.0, "{label}: [{i}][{j}] = {d} exceeds 1.0");
            }
        }
    }

    // -- Partition distance matrix --

    #[test]
    fn partition_distance_metric_axioms() {
        let m = partition_distance_matrix();
        assert_metric_axioms(&m, "partition");
    }

    #[test]
    fn partition_distance_t2_vs_poly2_exact() {
        let m = partition_distance_matrix();
        let expected = 41.0 / 254.0;
        assert!(
            (m[0][1] - expected).abs() < 1e-6,
            "dΠ(T₂, T_poly(2)) = {}, expected {expected}",
            m[0][1],
        );
    }

    // -- Stratum distance matrix --

    #[test]
    fn stratum_distance_metric_axioms() {
        let m = stratum_distance_matrix();
        assert_metric_axioms(&m, "stratum");
    }

    // -- Cross-field distinctness --

    #[test]
    fn all_four_types_mutually_distinct() {
        let m = partition_distance_matrix();
        let mut distances = alloc::vec::Vec::new();
        for (i, row) in m.iter().enumerate() {
            for &d in &row[i + 1..] {
                distances.push(d);
            }
        }
        // 4 choose 2 = 6 unique pairs
        assert_eq!(distances.len(), 6);
        for &d in &distances {
            assert!(d > 0.0);
        }
    }

    #[test]
    fn euclidean_cluster_is_non_degenerate() {
        let m = partition_distance_matrix();
        for (i, row) in m.iter().enumerate().take(TYPE_COUNT) {
            for (j, distance) in row.iter().enumerate().take(TYPE_COUNT) {
                if i != j {
                    assert!(*distance > 0.0, "expected positive distance for ({i}, {j})");
                }
            }
        }
    }
}
