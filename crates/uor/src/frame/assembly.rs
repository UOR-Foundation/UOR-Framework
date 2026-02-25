//! Frame assembly — top-level struct wiring all nine components.
//!
//! The [`Frame`] pre-computes the [`TypeRegistry`] and [`TypeGraph`];
//! embeddings and certificates are computed on-demand.
//!
//! # Nine Components
//!
//! | # | Component | Source |
//! |---|-----------|--------|
//! | 1 | TypeDeclaration | registry |
//! | 2 | Partition | registry |
//! | 3 | AlgebraicStratum | registry |
//! | 4 | TypeRegistry | pre-computed |
//! | 5 | ResolverDispatch | on-demand |
//! | 6 | ObservableSignature | registry |
//! | 7 | Embedding | on-demand |
//! | 8 | TransformCertificate | on-demand |
//! | 9 | TypeGraph | pre-computed |
//!
//! # Examples
//!
//! ```
//! use uor::frame::{Frame, DatumClass};
//!
//! let frame = Frame::compute();
//! let classes = frame.resolve_all(5);
//! assert_eq!(classes[0], DatumClass::Irreducible); // T₂: 5 is prime
//! ```

use super::cross_field::TYPE_COUNT;
use super::embedding::{embed, embedding_distance, EMBED_DIM};
use super::{DatumClass, StratumDispatch, TransformCertificate, TypeGraph, TypeRegistry};

/// Top-level invariance frame wiring all nine components.
pub struct Frame {
    registry: TypeRegistry,
    graph: TypeGraph,
}

impl Frame {
    /// Build the complete frame: registry + type graph.
    pub fn compute() -> Self {
        Self {
            registry: TypeRegistry::compute(),
            graph: TypeGraph::compute(),
        }
    }

    /// The type registry (components 1–4, 6).
    #[inline]
    pub fn registry(&self) -> &TypeRegistry {
        &self.registry
    }

    /// The type graph (component 9).
    #[inline]
    pub fn graph(&self) -> &TypeGraph {
        &self.graph
    }

    /// Classify a datum under a single type via stratum dispatch (component 5).
    #[inline]
    pub fn resolve(&self, value: u8, type_idx: usize) -> DatumClass {
        let (_, part, stratum, _) = self.registry.get_full(type_idx);
        StratumDispatch::resolve(value, part, stratum)
    }

    /// Classify a datum under all four types.
    #[inline]
    pub fn resolve_all(&self, value: u8) -> [DatumClass; TYPE_COUNT] {
        core::array::from_fn(|i| self.resolve(value, i))
    }

    /// Compute the observable embedding for a type (component 7, on-demand).
    pub fn embed(&self, type_idx: usize) -> [f64; EMBED_DIM] {
        let (decl, part) = self.registry.get(type_idx);
        embed(part, decl)
    }

    /// Compute a transform certificate for a type (component 8, on-demand).
    pub fn certificate(&self, type_idx: usize) -> TransformCertificate {
        let (decl, _) = self.registry.get(type_idx);
        TransformCertificate::compute(decl)
    }

    /// Combined distance between two types (delegates to graph).
    #[inline]
    pub fn type_distance(&self, a: usize, b: usize) -> f64 {
        self.graph.distances()[a][b]
    }

    /// Embedding distance between two types (on-demand computation).
    pub fn embedding_distance(&self, a: usize, b: usize) -> f64 {
        let ea = self.embed(a);
        let eb = self.embed(b);
        embedding_distance(&ea, &eb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construction_succeeds() {
        let frame = Frame::compute();
        assert_eq!(frame.registry().len(), 4);
    }

    #[test]
    fn registry_has_5_types() {
        let frame = Frame::compute();
        assert_eq!(frame.registry().iter().count(), 4);
    }

    #[test]
    fn graph_has_10_edges() {
        let frame = Frame::compute();
        assert_eq!(frame.graph().edge_count(), 6);
    }

    #[test]
    fn resolve_prime_under_t2() {
        let frame = Frame::compute();
        assert_eq!(frame.resolve(5, 0), DatumClass::Irreducible);
        assert_eq!(frame.resolve(4, 0), DatumClass::Reducible);
    }

    #[test]
    fn resolve_all_datum_5() {
        let frame = Frame::compute();
        let classes = frame.resolve_all(5);
        assert_eq!(classes[0], DatumClass::Irreducible); // T₂: prime
        assert_eq!(classes[1], DatumClass::Reducible); // T_poly(2): (x+1)²
    }

    #[test]
    fn resolve_all_datum_0_external() {
        let frame = Frame::compute();
        let classes = frame.resolve_all(0);
        for (i, c) in classes.iter().enumerate() {
            assert_eq!(
                *c,
                DatumClass::External,
                "type {i} should classify 0 as External"
            );
        }
    }

    #[test]
    fn embed_matches_manual() {
        let frame = Frame::compute();
        let v = frame.embed(0);
        let (decl, part) = frame.registry().get(0);
        let expected = embed(part, decl);
        assert_eq!(v, expected);
    }

    #[test]
    fn certificate_matches_manual() {
        let frame = Frame::compute();
        let cert = frame.certificate(0);
        let (decl, _) = frame.registry().get(0);
        let expected = TransformCertificate::compute(decl);
        assert_eq!(cert.irr_count(), expected.irr_count());
        assert!(cert.verified());
    }

    #[test]
    fn type_distance_delegation() {
        let frame = Frame::compute();
        let d = frame.type_distance(0, 1);
        assert_eq!(d, frame.graph().distances()[0][1]);
    }

    #[test]
    fn embedding_distance_positive() {
        let frame = Frame::compute();
        let d = frame.embedding_distance(0, 1);
        assert!(d > 0.0);
    }

    #[test]
    fn embedding_distance_self_zero() {
        let frame = Frame::compute();
        let d = frame.embedding_distance(0, 0);
        assert!(d.abs() < 1e-15);
    }

    #[test]
    fn resolve_all_matches_individual() {
        let frame = Frame::compute();
        for v in 0..=255u8 {
            let all = frame.resolve_all(v);
            for (i, &c) in all.iter().enumerate() {
                assert_eq!(c, frame.resolve(v, i), "mismatch at {v} type {i}");
            }
        }
    }
}
