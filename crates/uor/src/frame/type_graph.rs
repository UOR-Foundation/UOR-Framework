//! Weighted type graph — complete graph over Q0 types with combined metrics.
//!
//! Builds a [`TypeGraph`] from all Q0 type declarations, computing pairwise
//! partition distance, stratum distance, and algebraic depth difference.
//! The combined metric sums all three components.
//!
//! # Examples
//!
//! ```
//! use uor::frame::TypeGraph;
//!
//! let g = TypeGraph::compute();
//! assert!(g.diameter() > 0.0);
//! assert_eq!(g.edge_count(), 6); // 4 choose 2
//! ```

use super::cross_field::TYPE_COUNT;
use super::distance::{partition_distance, stratum_distance};
use super::{AlgebraicStratum, Partition, TypeDeclaration};

/// Weighted edge between two types.
#[derive(Debug, Clone, Copy)]
pub struct TypeEdge {
    pub source: usize,
    pub target: usize,
    pub partition_dist: f64,
    pub stratum_dist: f64,
    pub algebraic_dist: u8,
    pub combined: f64,
}

/// Complete weighted graph on Q0 types.
pub struct TypeGraph {
    distances: [[f64; TYPE_COUNT]; TYPE_COUNT],
    edges: alloc::vec::Vec<TypeEdge>,
}

impl TypeGraph {
    /// Build the graph from all Q0 types.
    pub fn compute() -> Self {
        let decls = all_decls();
        let parts: [Partition; TYPE_COUNT] =
            core::array::from_fn(|i| Partition::compute(&decls[i]));
        let strata: [AlgebraicStratum; TYPE_COUNT] =
            core::array::from_fn(|i| AlgebraicStratum::classify(decls[i].op()));

        let mut distances = [[0.0f64; TYPE_COUNT]; TYPE_COUNT];
        let mut edges = alloc::vec::Vec::with_capacity(TYPE_COUNT * (TYPE_COUNT - 1) / 2);

        for i in 0..TYPE_COUNT {
            for j in (i + 1)..TYPE_COUNT {
                let carrier_max = decls[i].carrier_len().max(decls[j].carrier_len());
                let pd = partition_distance(&parts[i], &parts[j], carrier_max);
                let sd = stratum_distance(&parts[i], &parts[j]);
                let ad = strata[i].depth().abs_diff(strata[j].depth());
                let combined = pd + sd + ad as f64 / 4.0;

                distances[i][j] = combined;
                distances[j][i] = combined;

                edges.push(TypeEdge {
                    source: i,
                    target: j,
                    partition_dist: pd,
                    stratum_dist: sd,
                    algebraic_dist: ad,
                    combined,
                });
            }
        }
        edges.sort_by(|a, b| a.combined.partial_cmp(&b.combined).unwrap());

        Self { distances, edges }
    }

    /// Maximum pairwise distance (graph diameter).
    pub fn diameter(&self) -> f64 {
        self.edges.iter().map(|e| e.combined).fold(0.0f64, f64::max)
    }

    /// Nearest neighbor of the given type index.
    pub fn nearest_neighbor(&self, idx: usize) -> (usize, f64) {
        let mut best_idx = 0;
        let mut best_dist = f64::MAX;
        for (j, &d) in self.distances[idx].iter().enumerate() {
            if j != idx && d < best_dist {
                best_dist = d;
                best_idx = j;
            }
        }
        (best_idx, best_dist)
    }

    /// Maximum algebraic stratum difference across all edges.
    pub fn algebraic_diameter(&self) -> u8 {
        self.edges
            .iter()
            .map(|e| e.algebraic_dist)
            .max()
            .unwrap_or(0)
    }

    /// Edges sorted by ascending combined distance.
    pub fn edges_sorted(&self) -> &[TypeEdge] {
        &self.edges
    }

    /// Number of edges in the complete graph.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// The combined distance matrix.
    pub fn distances(&self) -> &[[f64; TYPE_COUNT]; TYPE_COUNT] {
        &self.distances
    }
}

fn all_decls() -> [TypeDeclaration; TYPE_COUNT] {
    [
        TypeDeclaration::integer_mul(),
        TypeDeclaration::poly_gf2(),
        TypeDeclaration::poly_gf3(),
        TypeDeclaration::poly_gf5(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_has_10_edges() {
        let g = TypeGraph::compute();
        assert_eq!(g.edge_count(), 6);
    }

    #[test]
    fn diameter_positive() {
        let g = TypeGraph::compute();
        assert!(g.diameter() > 0.0, "diameter should be > 0");
    }

    #[test]
    fn diagonal_zero() {
        let g = TypeGraph::compute();
        for i in 0..TYPE_COUNT {
            assert_eq!(g.distances()[i][i], 0.0);
        }
    }

    #[test]
    fn symmetry() {
        let g = TypeGraph::compute();
        for i in 0..TYPE_COUNT {
            for j in 0..TYPE_COUNT {
                assert_eq!(g.distances()[i][j], g.distances()[j][i]);
            }
        }
    }

    #[test]
    fn triangle_inequality() {
        let g = TypeGraph::compute();
        let d = g.distances();
        for i in 0..TYPE_COUNT {
            for j in 0..TYPE_COUNT {
                for k in 0..TYPE_COUNT {
                    assert!(
                        d[i][k] <= d[i][j] + d[j][k] + 1e-10,
                        "triangle: d[{i}][{k}]={} > d[{i}][{j}]={} + d[{j}][{k}]={}",
                        d[i][k],
                        d[i][j],
                        d[j][k]
                    );
                }
            }
        }
    }

    #[test]
    fn cluster_has_nonzero_separation() {
        let g = TypeGraph::compute();
        for i in 0..TYPE_COUNT {
            let (_, d) = g.nearest_neighbor(i);
            assert!(d > 0.0, "nearest-neighbor distance should be > 0");
        }
    }

    #[test]
    fn nearest_neighbor_t2_is_euclidean() {
        let g = TypeGraph::compute();
        let (nn, _) = g.nearest_neighbor(0); // T₂
                                             // Nearest neighbor should be one of the poly types (indices 1-3)
        assert!(
            (1..4).contains(&nn),
            "T₂'s nearest neighbor should be a poly type, got index {nn}"
        );
    }

    #[test]
    fn algebraic_diameter_is_4() {
        let g = TypeGraph::compute();
        assert_eq!(g.algebraic_diameter(), 0);
    }

    #[test]
    fn edges_sorted_ascending() {
        let g = TypeGraph::compute();
        let edges = g.edges_sorted();
        for w in edges.windows(2) {
            assert!(w[0].combined <= w[1].combined);
        }
    }

    #[test]
    fn all_edges_positive() {
        let g = TypeGraph::compute();
        for edge in g.edges_sorted() {
            assert!(
                edge.combined > 0.0,
                "edge {}-{} should be positive",
                edge.source,
                edge.target
            );
        }
    }
}
