//! Benchmarks for the UOR Invariance Frame.

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use uor::frame::{
    closure_ratio, embed, gf2, gf3, gf5, partition_distance, partition_distance_matrix,
    stratum_distance, stratum_distance_matrix, stratum_histogram, verify_commutative,
    AlgebraicStratum, Alignment, BinaryOp, DatumSet, Frame, MultiModalBatch, MultiModalResult,
    Partition, StratumDispatch, TransformCertificate, TypeDeclaration, TypeGraph, TypeRegistry,
};

fn bench_partition_compute(c: &mut Criterion) {
    let mut group = c.benchmark_group("partition_compute");
    group.throughput(Throughput::Elements(254));

    group.bench_function("integer_mul", |b| {
        let t2 = TypeDeclaration::integer_mul();
        b.iter(|| black_box(Partition::compute(black_box(&t2))));
    });

    group.bench_function("poly_gf2", |b| {
        let tp = TypeDeclaration::poly_gf2();
        b.iter(|| black_box(Partition::compute(black_box(&tp))));
    });

    group.bench_function("poly_gf3", |b| {
        let tp = TypeDeclaration::poly_gf3();
        b.iter(|| black_box(Partition::compute(black_box(&tp))));
    });

    group.bench_function("poly_gf5", |b| {
        let tp = TypeDeclaration::poly_gf5();
        b.iter(|| black_box(Partition::compute(black_box(&tp))));
    });

    group.finish();
}

fn bench_gf_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("gf_arithmetic");
    group.throughput(Throughput::Elements(1));

    // GF(2)
    group.bench_function("gf2_mul", |b| {
        b.iter(|| black_box(gf2::mul(black_box(0xAB), black_box(0xCD))));
    });
    group.bench_function("gf2_is_irreducible", |b| {
        b.iter(|| black_box(gf2::is_irreducible(black_box(0x1D))));
    });

    // GF(3)
    group.bench_function("gf3_mul", |b| {
        b.iter(|| black_box(gf3::mul(black_box(100), black_box(200))));
    });
    group.bench_function("gf3_is_irreducible", |b| {
        b.iter(|| black_box(gf3::is_irreducible(black_box(12))));
    });

    // GF(5)
    group.bench_function("gf5_mul", |b| {
        b.iter(|| black_box(gf5::mul(black_box(30), black_box(125))));
    });
    group.bench_function("gf5_is_irreducible", |b| {
        b.iter(|| black_box(gf5::is_irreducible(black_box(31))));
    });

    group.finish();
}

fn bench_distance(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_distance");
    group.throughput(Throughput::Elements(1));

    let t2 = TypeDeclaration::integer_mul();
    let tp = TypeDeclaration::poly_gf2();
    let p2 = Partition::compute(&t2);
    let pp = Partition::compute(&tp);

    group.bench_function("partition_distance", |b| {
        b.iter(|| black_box(partition_distance(black_box(&p2), black_box(&pp), 254)));
    });

    group.bench_function("stratum_distance", |b| {
        b.iter(|| black_box(stratum_distance(black_box(&p2), black_box(&pp))));
    });

    group.bench_function("stratum_histogram", |b| {
        b.iter(|| black_box(stratum_histogram(black_box(p2.irr()))));
    });

    group.finish();
}

fn bench_datum_set(c: &mut Criterion) {
    let mut group = c.benchmark_group("datum_set");
    group.throughput(Throughput::Elements(1));

    let a = DatumSet::from_range(2, 255);
    let b = DatumSet::from_range(50, 200);

    group.bench_function("popcount", |b_iter| {
        b_iter.iter(|| black_box(black_box(a).len()));
    });

    group.bench_function("symmetric_difference", |b_iter| {
        b_iter.iter(|| black_box(black_box(a).symmetric_difference(black_box(&b))));
    });

    group.finish();
}

fn bench_certificate(c: &mut Criterion) {
    let mut group = c.benchmark_group("certificate");
    group.throughput(Throughput::Elements(1));

    group.bench_function("compute_t2", |b| {
        let t2 = TypeDeclaration::integer_mul();
        b.iter(|| black_box(TransformCertificate::compute(black_box(&t2))));
    });

    group.bench_function("compute_poly_gf2", |b| {
        let tp = TypeDeclaration::poly_gf2();
        b.iter(|| black_box(TransformCertificate::compute(black_box(&tp))));
    });

    group.bench_function("compute_poly_gf3", |b| {
        let tp = TypeDeclaration::poly_gf3();
        b.iter(|| black_box(TransformCertificate::compute(black_box(&tp))));
    });

    group.bench_function("compute_poly_gf5", |b| {
        let tp = TypeDeclaration::poly_gf5();
        b.iter(|| black_box(TransformCertificate::compute(black_box(&tp))));
    });

    group.finish();
}

fn bench_embedding(c: &mut Criterion) {
    let mut group = c.benchmark_group("embedding");
    group.throughput(Throughput::Elements(1));

    let t2 = TypeDeclaration::integer_mul();
    let p2 = Partition::compute(&t2);
    let tp = TypeDeclaration::poly_gf2();
    let pp = Partition::compute(&tp);

    group.bench_function("embed_t2", |b| {
        b.iter(|| black_box(embed(black_box(&p2), black_box(&t2))));
    });

    group.bench_function("embed_poly_gf2", |b| {
        b.iter(|| black_box(embed(black_box(&pp), black_box(&tp))));
    });

    let e1 = embed(&p2, &t2);
    let e2 = embed(&pp, &tp);

    group.bench_function("embedding_distance", |b| {
        b.iter(|| {
            black_box(uor::frame::embedding_distance(
                black_box(&e1),
                black_box(&e2),
            ))
        });
    });

    group.finish();
}

fn bench_cross_field(c: &mut Criterion) {
    let mut group = c.benchmark_group("cross_field");
    group.throughput(Throughput::Elements(16)); // 4×4 matrix

    group.bench_function("partition_distance_matrix", |b| {
        b.iter(|| black_box(partition_distance_matrix()));
    });

    group.bench_function("stratum_distance_matrix", |b| {
        b.iter(|| black_box(stratum_distance_matrix()));
    });

    group.finish();
}

fn bench_algebraic_classify(c: &mut Criterion) {
    let mut group = c.benchmark_group("algebraic_classify");
    group.throughput(Throughput::Elements(1));

    group.bench_function("classify_integer_mul", |b| {
        b.iter(|| black_box(AlgebraicStratum::classify(black_box(BinaryOp::IntegerMul))));
    });

    group.finish();
}

fn bench_property_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("property_verify");
    group.throughput(Throughput::Elements(1));

    let t2 = TypeDeclaration::integer_mul();
    group.bench_function("verify_commutative_t2", |b| {
        b.iter(|| black_box(verify_commutative(black_box(&t2))));
    });

    group.bench_function("closure_ratio_t2", |b| {
        b.iter(|| black_box(closure_ratio(black_box(&t2))));
    });

    group.finish();
}

fn bench_type_graph(c: &mut Criterion) {
    let mut group = c.benchmark_group("type_graph");
    group.throughput(Throughput::Elements(6)); // 4 choose 2 edges

    group.bench_function("compute", |b| {
        b.iter(|| black_box(TypeGraph::compute()));
    });

    group.finish();
}

// ============================================================================
// Sprint 41 benchmarks
// ============================================================================

fn bench_registry(c: &mut Criterion) {
    let mut group = c.benchmark_group("sprint41_registry");

    group.bench_function("compute", |b| {
        b.iter(|| black_box(TypeRegistry::compute()));
    });

    let reg = TypeRegistry::compute();
    group.throughput(Throughput::Elements(1));
    group.bench_function("get", |b| {
        b.iter(|| black_box(reg.get(black_box(0))));
    });

    group.bench_function("index_of", |b| {
        b.iter(|| black_box(reg.index_of(black_box(BinaryOp::PolyGf2Mul))));
    });

    group.finish();
}

fn bench_resolver(c: &mut Criterion) {
    let mut group = c.benchmark_group("sprint41_resolver");
    group.throughput(Throughput::Elements(1));

    let t2 = TypeDeclaration::integer_mul();
    let p2 = Partition::compute(&t2);
    let stratum = AlgebraicStratum::classify(t2.op());

    group.bench_function("stratum_dispatch", |b| {
        b.iter(|| {
            black_box(StratumDispatch::resolve(
                black_box(5),
                black_box(&p2),
                black_box(stratum),
            ))
        });
    });

    group.finish();
}

fn bench_frame(c: &mut Criterion) {
    let mut group = c.benchmark_group("sprint41_frame");

    group.bench_function("compute", |b| {
        b.iter(|| black_box(Frame::compute()));
    });

    let frame = Frame::compute();
    group.throughput(Throughput::Elements(4));
    group.bench_function("resolve_all", |b| {
        b.iter(|| black_box(frame.resolve_all(black_box(5))));
    });

    group.finish();
}

fn bench_alignment(c: &mut Criterion) {
    let mut group = c.benchmark_group("sprint41_alignment");
    group.throughput(Throughput::Elements(1));

    let reg = TypeRegistry::compute();

    group.bench_function("compute_t2_vs_poly2", |b| {
        b.iter(|| black_box(Alignment::compute(black_box(&reg), 0, 1)));
    });

    let a = Alignment::compute(&reg, 0, 1);
    group.bench_function("jaccard", |b| {
        b.iter(|| black_box(black_box(a).jaccard()));
    });

    group.finish();
}

fn bench_multimodal(c: &mut Criterion) {
    let mut group = c.benchmark_group("sprint41_multimodal");

    let frame = Frame::compute();
    group.throughput(Throughput::Elements(1));
    group.bench_function("classify_single", |b| {
        b.iter(|| black_box(MultiModalResult::classify(black_box(5), black_box(&frame))));
    });

    group.throughput(Throughput::Elements(256));
    group.bench_function("classify_batch", |b| {
        b.iter(|| black_box(MultiModalBatch::classify_all(black_box(&frame))));
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_partition_compute,
    bench_gf_operations,
    bench_distance,
    bench_datum_set,
    bench_certificate,
    bench_embedding,
    bench_cross_field,
    bench_algebraic_classify,
    bench_property_verify,
    bench_type_graph,
    bench_registry,
    bench_resolver,
    bench_frame,
    bench_alignment,
    bench_multimodal,
);
criterion_main!(benches);
