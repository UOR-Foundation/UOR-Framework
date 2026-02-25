//! Benchmarks for view system operations.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use uor::view::ElementWiseView;

fn bench_view_apply_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("view_apply_single");
    group.throughput(Throughput::Elements(1));

    let inc = ElementWiseView::new(|x| x.wrapping_add(1));
    let xor_ff = ElementWiseView::new(|x| x ^ 0xFF);

    group.bench_function("increment", |b| {
        b.iter(|| black_box(inc.apply(black_box(42))));
    });

    group.bench_function("xor_ff", |b| {
        b.iter(|| black_box(xor_ff.apply(black_box(42))));
    });

    group.finish();
}

fn bench_view_composition(c: &mut Criterion) {
    let mut group = c.benchmark_group("view_composition");
    group.throughput(Throughput::Elements(256));

    let inc = ElementWiseView::new(|x| x.wrapping_add(1));
    let xor_ff = ElementWiseView::new(|x| x ^ 0xFF);

    group.bench_function("compose_two", |b| {
        b.iter(|| black_box(inc.then(&xor_ff)));
    });

    let composed = inc.then(&xor_ff);
    group.bench_function("compose_three", |b| {
        b.iter(|| black_box(composed.then(&inc)));
    });

    group.finish();
}

fn bench_view_apply_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("view_apply_slice");

    let inc = ElementWiseView::new(|x| x.wrapping_add(1));

    for size in [16, 64, 256, 1024, 4096, 16384].iter() {
        group.throughput(Throughput::Bytes(*size as u64));

        let mut data = vec![0u8; *size];
        for (i, byte) in data.iter_mut().enumerate() {
            *byte = (i % 256) as u8;
        }

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let mut data_copy = data.clone();
                inc.apply_slice(black_box(&mut data_copy));
                black_box(data_copy);
            });
        });
    }

    group.finish();
}

fn bench_view_apply_to(c: &mut Criterion) {
    let mut group = c.benchmark_group("view_apply_to");

    let inc = ElementWiseView::new(|x| x.wrapping_add(1));

    for size in [16, 64, 256, 1024, 4096, 16384].iter() {
        group.throughput(Throughput::Bytes(*size as u64));

        let input = vec![0u8; *size];
        let mut output = vec![0u8; *size];

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                inc.apply_to(black_box(&input), black_box(&mut output));
                black_box(&output);
            });
        });
    }

    group.finish();
}

fn bench_view_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("view_inverse");
    group.throughput(Throughput::Elements(256));

    let inc = ElementWiseView::new(|x| x.wrapping_add(1));
    let xor_ff = ElementWiseView::new(|x| x ^ 0xFF);

    group.bench_function("increment", |b| {
        b.iter(|| black_box(inc.inverse()));
    });

    group.bench_function("xor_ff", |b| {
        b.iter(|| black_box(xor_ff.inverse()));
    });

    group.finish();
}

fn bench_view_is_bijective(c: &mut Criterion) {
    let mut group = c.benchmark_group("view_is_bijective");
    group.throughput(Throughput::Elements(256));

    let inc = ElementWiseView::new(|x| x.wrapping_add(1));
    let constant = ElementWiseView::constant(42);

    group.bench_function("bijective", |b| {
        b.iter(|| black_box(inc.is_bijective()));
    });

    group.bench_function("not_bijective", |b| {
        b.iter(|| black_box(constant.is_bijective()));
    });

    group.finish();
}

fn bench_fused_activations(c: &mut Criterion) {
    let mut group = c.benchmark_group("fused_activations");

    // Simulate common activation function chains
    // These byte-level operations model the pattern of activation fusion
    let activation1 = ElementWiseView::new(|x| {
        // Simplified "sigmoid-like" operation
        x.wrapping_mul(2).wrapping_add(1)
    });

    let activation2 = ElementWiseView::new(|x| {
        // Simplified "tanh-like" operation
        x ^ 0xAA
    });

    let activation3 = ElementWiseView::new(|x| {
        // Simplified "relu-like" operation
        if x > 127 {
            x
        } else {
            0
        }
    });

    // Compose all three activations into one lookup table at compile time
    let fused = activation1.then(&activation2).then(&activation3);

    // Test multiple sizes to show scaling behavior
    for size in [1024, 4096, 16384, 65536] {
        group.throughput(Throughput::Bytes(size as u64));

        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();

        // Unfused: 3 separate passes over the data
        group.bench_with_input(BenchmarkId::new("unfused_3_passes", size), &size, |b, _| {
            b.iter(|| {
                let mut data_copy = data.clone();
                activation1.apply_slice(&mut data_copy);
                activation2.apply_slice(&mut data_copy);
                activation3.apply_slice(&mut data_copy);
                black_box(data_copy);
            });
        });

        // Fused: 1 pass with composed lookup table (uses SIMD)
        group.bench_with_input(BenchmarkId::new("fused_1_pass", size), &size, |b, _| {
            b.iter(|| {
                let mut data_copy = data.clone();
                fused.apply_slice(&mut data_copy);
                black_box(data_copy);
            });
        });

        // Scalar baseline: manual loop without SIMD
        group.bench_with_input(
            BenchmarkId::new("fused_scalar_baseline", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let mut data_copy = data.clone();
                    let table = fused.table();
                    for byte in &mut data_copy {
                        *byte = table[*byte as usize];
                    }
                    black_box(data_copy);
                });
            },
        );
    }

    group.finish();
}

fn bench_simd_vs_scalar(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_vs_scalar");

    // Create a non-trivial lookup table (not identity or simple transform)
    let complex_view = ElementWiseView::new(|x| {
        // Mix of operations to avoid trivial patterns
        x.wrapping_mul(17).wrapping_add(31) ^ (x >> 3)
    });

    for size in [256, 1024, 4096, 16384, 65536].iter() {
        group.throughput(Throughput::Bytes(*size as u64));

        let mut data: Vec<u8> = (0..*size).map(|i| (i % 256) as u8).collect();

        group.bench_with_input(BenchmarkId::new("apply_slice", size), size, |b, _| {
            b.iter(|| {
                complex_view.apply_slice(black_box(&mut data));
                black_box(&data);
            });
        });
    }

    group.finish();
}

fn bench_apply_to_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("apply_to_throughput");

    let view = ElementWiseView::new(|x| x.wrapping_mul(7).wrapping_add(13));

    // Test larger sizes to see sustained throughput
    for size in [4096, 16384, 65536, 262144].iter() {
        group.throughput(Throughput::Bytes(*size as u64));

        let input: Vec<u8> = (0..*size).map(|i| (i % 256) as u8).collect();
        let mut output = vec![0u8; *size];

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                view.apply_to(black_box(&input), black_box(&mut output));
                black_box(&output);
            });
        });
    }

    group.finish();
}

fn bench_memory_bandwidth(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_bandwidth");

    // Test memory-bound operations to measure achieved bandwidth
    // Theoretical DDR4-3200: ~25.6 GB/s per channel (typical desktop: 2 channels = ~51 GB/s)
    // Theoretical DDR5-4800: ~38.4 GB/s per channel (typical: 2 channels = ~77 GB/s)
    // L3 cache bandwidth: ~200-400 GB/s (varies by CPU)
    // L2 cache bandwidth: ~400-800 GB/s
    // L1 cache bandwidth: ~1000+ GB/s

    let identity = ElementWiseView::identity();
    let transform = ElementWiseView::new(|x| x.wrapping_add(1));

    // Test sizes that span different cache levels
    // L1: ~32-48 KB, L2: ~256 KB - 1 MB, L3: ~8-32 MB
    let test_sizes = [
        (8 * 1024, "L1_8KB"),
        (32 * 1024, "L1_32KB"),
        (128 * 1024, "L2_128KB"),
        (512 * 1024, "L2_512KB"),
        (2 * 1024 * 1024, "L3_2MB"),
        (8 * 1024 * 1024, "L3_8MB"),
        (32 * 1024 * 1024, "RAM_32MB"),
    ];

    for (size, label) in test_sizes {
        group.throughput(Throughput::Bytes(size as u64));

        let input: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        let mut output = vec![0u8; size];

        // Identity transform (pure memory copy through lookup)
        group.bench_with_input(BenchmarkId::new("identity_copy", label), &size, |b, _| {
            b.iter(|| {
                identity.apply_to(black_box(&input), black_box(&mut output));
                black_box(&output);
            });
        });

        // Transform (memory read + lookup + write)
        group.bench_with_input(BenchmarkId::new("transform", label), &size, |b, _| {
            b.iter(|| {
                transform.apply_to(black_box(&input), black_box(&mut output));
                black_box(&output);
            });
        });

        // In-place transform (read-modify-write)
        let mut data = input.clone();
        group.bench_with_input(BenchmarkId::new("inplace", label), &size, |b, _| {
            b.iter(|| {
                transform.apply_slice(black_box(&mut data));
                black_box(&data);
            });
        });
    }

    group.finish();
}

fn bench_streaming_vs_cached(c: &mut Criterion) {
    let mut group = c.benchmark_group("streaming_vs_cached");

    // Compare performance when data fits in cache vs streaming from RAM
    let view = ElementWiseView::new(|x| x.wrapping_mul(7).wrapping_add(13));

    // Cached: 16KB (fits in L1)
    let cached_size = 16 * 1024;
    group.throughput(Throughput::Bytes(cached_size as u64));

    let cached_data: Vec<u8> = (0..cached_size).map(|i| (i % 256) as u8).collect();
    let mut cached_output = vec![0u8; cached_size];

    group.bench_function("cached_16KB", |b| {
        b.iter(|| {
            view.apply_to(black_box(&cached_data), black_box(&mut cached_output));
            black_box(&cached_output);
        });
    });

    // Streaming: 64MB (much larger than L3)
    let streaming_size = 64 * 1024 * 1024;
    group.throughput(Throughput::Bytes(streaming_size as u64));

    let streaming_data: Vec<u8> = (0..streaming_size).map(|i| (i % 256) as u8).collect();
    let mut streaming_output = vec![0u8; streaming_size];

    group.bench_function("streaming_64MB", |b| {
        b.iter(|| {
            view.apply_to(black_box(&streaming_data), black_box(&mut streaming_output));
            black_box(&streaming_output);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_view_apply_single,
    bench_view_composition,
    bench_view_apply_slice,
    bench_view_apply_to,
    bench_view_inverse,
    bench_view_is_bijective,
    bench_fused_activations,
    bench_simd_vs_scalar,
    bench_apply_to_throughput,
    bench_memory_bandwidth,
    bench_streaming_vs_cached,
);
criterion_main!(benches);
