//! Benchmarks for scientific calculator operations via ElementWiseView.
//!
//! Measures O(1) lookup performance for trigonometric, exponential, logarithmic,
//! and composed scientific functions in byte domain.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use uor::lut;
use uor::view::ElementWiseView;

// ============================================================================
// Encoding helpers (used by make_norm_square and LUT-vs-f64 comparison)
// ============================================================================

fn byte_to_signed(b: u8) -> f64 {
    (b as f64 - 128.0) / 127.0
}

fn unit_to_byte(v: f64) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0) as u8
}

// ============================================================================
// View constructors
// ============================================================================

fn make_sin() -> ElementWiseView {
    ElementWiseView::from_table(lut::SIN_256)
}

fn make_cos() -> ElementWiseView {
    ElementWiseView::from_table(lut::COS_256)
}

fn make_tan() -> ElementWiseView {
    ElementWiseView::from_table(lut::TAN_256)
}

fn make_square() -> ElementWiseView {
    ElementWiseView::from_table(lut::SQUARE_256)
}

fn make_norm_square() -> ElementWiseView {
    ElementWiseView::new(|b| {
        let v = byte_to_signed(b);
        unit_to_byte(v * v)
    })
}

fn make_log2() -> ElementWiseView {
    ElementWiseView::from_table(lut::LOG2_256)
}

// ============================================================================
// Benchmark: LUT construction time
// ============================================================================

fn bench_lut_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_construction");
    group.throughput(Throughput::Elements(256));

    group.bench_function("sin", |b| {
        b.iter(|| black_box(make_sin()));
    });

    group.bench_function("cos", |b| {
        b.iter(|| black_box(make_cos()));
    });

    group.bench_function("tan", |b| {
        b.iter(|| black_box(make_tan()));
    });

    group.bench_function("square", |b| {
        b.iter(|| black_box(make_square()));
    });

    group.bench_function("log2", |b| {
        b.iter(|| black_box(make_log2()));
    });

    group.bench_function("from_table_sigmoid", |b| {
        b.iter(|| black_box(ElementWiseView::from_table(lut::SIGMOID_256)));
    });

    group.finish();
}

// ============================================================================
// Benchmark: single element apply (the O(1) claim)
// ============================================================================

fn bench_single_apply(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_single_apply");
    group.throughput(Throughput::Elements(1));

    let sin = make_sin();
    let cos = make_cos();
    let tan = make_tan();
    let sqrt = ElementWiseView::from_table(lut::SQRT_256);
    let exp = ElementWiseView::from_table(lut::EXP_256);
    let sigmoid = ElementWiseView::from_table(lut::SIGMOID_256);

    group.bench_function("sin", |b| {
        b.iter(|| black_box(sin.apply(black_box(64))));
    });

    group.bench_function("cos", |b| {
        b.iter(|| black_box(cos.apply(black_box(64))));
    });

    group.bench_function("tan", |b| {
        b.iter(|| black_box(tan.apply(black_box(64))));
    });

    group.bench_function("sqrt", |b| {
        b.iter(|| black_box(sqrt.apply(black_box(128))));
    });

    group.bench_function("exp", |b| {
        b.iter(|| black_box(exp.apply(black_box(64))));
    });

    group.bench_function("sigmoid", |b| {
        b.iter(|| black_box(sigmoid.apply(black_box(64))));
    });

    group.finish();
}

// ============================================================================
// Benchmark: single apply vs direct f64 computation
// ============================================================================

fn bench_lut_vs_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_lut_vs_f64");
    group.throughput(Throughput::Elements(1));

    let sin_lut = make_sin();
    let sqrt_lut = ElementWiseView::from_table(lut::SQRT_256);

    group.bench_function("sin_lut", |b| {
        b.iter(|| black_box(sin_lut.apply(black_box(64))));
    });

    group.bench_function("sin_f64", |b| {
        b.iter(|| {
            let angle = black_box(64u8) as f64 * std::f64::consts::TAU / 256.0;
            black_box(angle.sin())
        });
    });

    group.bench_function("sqrt_lut", |b| {
        b.iter(|| black_box(sqrt_lut.apply(black_box(128))));
    });

    group.bench_function("sqrt_f64", |b| {
        b.iter(|| black_box((black_box(128u8) as f64 / 255.0).sqrt()));
    });

    group.finish();
}

// ============================================================================
// Benchmark: batch apply (slice throughput)
// ============================================================================

fn bench_batch_apply(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_batch_apply");

    let sin = make_sin();
    let sigmoid = ElementWiseView::from_table(lut::SIGMOID_256);

    for size in [256, 1024, 4096, 16384] {
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("sin", size), &size, |b, _| {
            b.iter(|| {
                let mut copy = data.clone();
                sin.apply_slice(black_box(&mut copy));
                black_box(copy);
            });
        });

        group.bench_with_input(BenchmarkId::new("sigmoid", size), &size, |b, _| {
            b.iter(|| {
                let mut copy = data.clone();
                sigmoid.apply_slice(black_box(&mut copy));
                black_box(copy);
            });
        });
    }

    group.finish();
}

// ============================================================================
// Benchmark: composition (building composed views + applying them)
// ============================================================================

fn bench_composition(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_composition");

    let sin = make_sin();
    let norm_sq = make_norm_square();
    let sigmoid = ElementWiseView::from_table(lut::SIGMOID_256);
    let relu = ElementWiseView::from_table(lut::RELU_256);

    // Build cost: composing two views (256 lookups)
    group.throughput(Throughput::Elements(256));

    group.bench_function("build_sin_squared", |b| {
        b.iter(|| black_box(sin.then(black_box(&norm_sq))));
    });

    group.bench_function("build_sigmoid_relu", |b| {
        b.iter(|| black_box(sigmoid.then(black_box(&relu))));
    });

    group.bench_function("build_chain_3", |b| {
        b.iter(|| black_box(sin.then(&norm_sq).then(&sigmoid)));
    });

    // Apply cost: composed view is same as single view
    let sin_sq = sin.then(&norm_sq);
    let chain_3 = sin.then(&norm_sq).then(&sigmoid);

    group.throughput(Throughput::Elements(1));

    group.bench_function("apply_sin_squared", |b| {
        b.iter(|| black_box(sin_sq.apply(black_box(64))));
    });

    group.bench_function("apply_chain_3", |b| {
        b.iter(|| black_box(chain_3.apply(black_box(64))));
    });

    // Compare: composed single lookup vs sequential 3 lookups
    group.bench_function("sequential_3_lookups", |b| {
        b.iter(|| {
            let v = sin.apply(black_box(64));
            let v = norm_sq.apply(v);
            black_box(sigmoid.apply(v))
        });
    });

    group.finish();
}

// ============================================================================
// Benchmark: batch composition vs sequential (the fusion payoff)
// ============================================================================

fn bench_fused_vs_sequential(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_fused_vs_sequential");

    let sin = make_sin();
    let norm_sq = make_norm_square();
    let sigmoid = ElementWiseView::from_table(lut::SIGMOID_256);

    let fused = sin.then(&norm_sq).then(&sigmoid);

    for size in [1024, 4096, 16384] {
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        group.throughput(Throughput::Bytes(size as u64));

        // Fused: one pass with composed table
        group.bench_with_input(BenchmarkId::new("fused_1_pass", size), &size, |b, _| {
            b.iter(|| {
                let mut copy = data.clone();
                fused.apply_slice(black_box(&mut copy));
                black_box(copy);
            });
        });

        // Sequential: three separate passes
        group.bench_with_input(
            BenchmarkId::new("sequential_3_passes", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let mut copy = data.clone();
                    sin.apply_slice(&mut copy);
                    norm_sq.apply_slice(&mut copy);
                    sigmoid.apply_slice(&mut copy);
                    black_box(copy);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_lut_construction,
    bench_single_apply,
    bench_lut_vs_f64,
    bench_batch_apply,
    bench_composition,
    bench_fused_vs_sequential,
);
criterion_main!(benches);
