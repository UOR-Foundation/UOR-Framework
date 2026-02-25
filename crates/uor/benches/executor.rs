//! Executor benchmarks (TASK-143).
//!
//! Benchmarks for ScalarExecutor (all platforms) and NeonExecutor (aarch64).
//! These measure the portable reference implementation and ARM SIMD execution.
//!
//! ```bash
//! cargo bench -p uor -- executor
//! ```

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use uor::arch::portable::ScalarExecutor;
use uor::isa::{PortAssignment, UorStep, Wavefront};
use uor::state::{UorState, STATE_TAXONS};
use uor::taxon::Taxon;

#[cfg(target_arch = "aarch64")]
use uor::arch::aarch64::NeonExecutor;

/// Deterministic test state: YMM[i][j] = (i*32 + j + 1) % 256.
fn test_state() -> UorState {
    let mut s = UorState::zero();
    for i in 0..16 {
        for j in 0..32 {
            s.ymm[i][j] = Taxon::new(((i * 32 + j + 1) % 256) as u8);
        }
    }
    s
}

// ============================================================================
// ScalarExecutor Benchmarks (all platforms)
// ============================================================================

fn bench_scalar_cycle(c: &mut Criterion) {
    let exec = ScalarExecutor::new();
    let mut g = c.benchmark_group("scalar/cycle");
    g.throughput(Throughput::Elements(1));

    let ops: [(&str, Wavefront); 10] = [
        ("xor", Wavefront::all_xor()),
        ("and", Wavefront::new(PortAssignment::all_and())),
        ("or", Wavefront::new(PortAssignment::all_or())),
        ("not", Wavefront::all_not()),
        ("add", Wavefront::new(PortAssignment::all_add())),
        ("sub", Wavefront::new(PortAssignment::all_sub())),
        ("rotr_7", Wavefront::new(PortAssignment::rotr_only(7))),
        ("rotl_13", Wavefront::new(PortAssignment::rotl_only(13))),
        ("shr_3", Wavefront::new(PortAssignment::shr_only(3))),
        ("shl_10", Wavefront::new(PortAssignment::shl_only(10))),
    ];

    for (name, wf) in &ops {
        g.bench_function(*name, |b| {
            let mut s = test_state();
            b.iter(|| unsafe { exec.step(black_box(&mut s), black_box(wf)) })
        });
    }

    g.finish();
}

fn bench_scalar_crypto(c: &mut Criterion) {
    let exec = ScalarExecutor::new();
    let mut g = c.benchmark_group("scalar/crypto");
    g.throughput(Throughput::Elements(1));

    let ops: [(&str, Wavefront); 5] = [
        ("sha256_round", Wavefront::sha256_round()),
        ("sha256_msg1", Wavefront::new(PortAssignment::sha256_msg1())),
        ("sha256_msg2", Wavefront::new(PortAssignment::sha256_msg2())),
        ("aes_round", Wavefront::aes_round()),
        (
            "aes_round_dec",
            Wavefront::new(PortAssignment::aes_round_dec()),
        ),
    ];

    for (name, wf) in &ops {
        g.bench_function(*name, |b| {
            let mut s = test_state();
            b.iter(|| unsafe { exec.step(black_box(&mut s), black_box(wf)) })
        });
    }

    g.finish();
}

fn bench_scalar_permute(c: &mut Criterion) {
    let exec = ScalarExecutor::new();
    let mut g = c.benchmark_group("scalar/permute");
    g.throughput(Throughput::Elements(1));

    g.bench_function("shuffle", |b| {
        let wf = Wavefront::shuffle();
        let mut s = test_state();
        b.iter(|| unsafe { exec.step(black_box(&mut s), black_box(&wf)) })
    });

    g.bench_function("permute", |b| {
        let wf = Wavefront::permute();
        let mut s = test_state();
        b.iter(|| unsafe { exec.step(black_box(&mut s), black_box(&wf)) })
    });

    g.finish();
}

fn bench_scalar_sequence(c: &mut Criterion) {
    let exec = ScalarExecutor::new();
    let mut g = c.benchmark_group("scalar/sequence");

    for n in [16u64, 64, 256] {
        let program: Vec<Wavefront> = (0..n).map(|_| Wavefront::all_xor()).collect();
        g.throughput(Throughput::Elements(n));
        g.bench_function(format!("{n}_xor"), |b| {
            let mut s = test_state();
            b.iter(|| unsafe { exec.run(black_box(&mut s), black_box(&program)) })
        });
    }

    // Mixed operations
    let mixed: Vec<Wavefront> = (0..64)
        .map(|i| match i % 5 {
            0 => Wavefront::all_xor(),
            1 => Wavefront::new(PortAssignment::all_and()),
            2 => Wavefront::new(PortAssignment::rotr_only(7)),
            3 => Wavefront::new(PortAssignment::all_add()),
            _ => Wavefront::all_not(),
        })
        .collect();
    g.throughput(Throughput::Elements(64));
    g.bench_function("64_mixed", |b| {
        let mut s = test_state();
        b.iter(|| unsafe { exec.run(black_box(&mut s), black_box(&mixed)) })
    });

    g.finish();
}

fn bench_scalar_programs(c: &mut Criterion) {
    let exec = ScalarExecutor::new();
    let mut g = c.benchmark_group("scalar/programs");

    let sha256 = uor::wavefront::sha256_compress_program();
    g.throughput(Throughput::Elements(sha256.len() as u64));
    g.bench_function("sha256_compress", |b| {
        let mut s = test_state();
        b.iter(|| unsafe { exec.run(black_box(&mut s), black_box(&sha256)) })
    });

    let aes = uor::wavefront::aes128_encrypt_program();
    g.throughput(Throughput::Elements(aes.len() as u64));
    g.bench_function("aes128_encrypt", |b| {
        let mut s = test_state();
        b.iter(|| unsafe { exec.run(black_box(&mut s), black_box(&aes)) })
    });

    g.finish();
}

fn bench_scalar_throughput(c: &mut Criterion) {
    let exec = ScalarExecutor::new();
    let mut g = c.benchmark_group("scalar/throughput");
    g.throughput(Throughput::Bytes(STATE_TAXONS as u64));

    let wf = Wavefront::all_xor();
    g.bench_function("bits_per_cycle", |b| {
        let mut s = test_state();
        b.iter(|| unsafe { exec.step(black_box(&mut s), black_box(&wf)) })
    });

    g.finish();
}

fn bench_scalar_lossless(c: &mut Criterion) {
    use uor::isa::UorStepLossless;

    let exec = ScalarExecutor::new();
    let mut g = c.benchmark_group("scalar/lossless");
    g.throughput(Throughput::Elements(1));

    let wf = Wavefront::all_xor();

    g.bench_function("step_tracked", |b| {
        let mut s = test_state();
        let mut complement = UorState::zero();
        b.iter(|| unsafe {
            exec.step_tracked(
                black_box(&mut s),
                black_box(&mut complement),
                black_box(&wf),
            )
        })
    });

    g.bench_function("step_inverse", |b| {
        b.iter(|| unsafe {
            let mut state = test_state();
            let mut complement = UorState::zero();
            exec.step_tracked(&mut state, &mut complement, &wf);
            exec.step_inverse(
                black_box(&mut state),
                black_box(&complement),
                black_box(&wf),
            );
        })
    });

    g.finish();
}

fn bench_scalar_binary(c: &mut Criterion) {
    use uor::isa::UorStepBinary;

    let exec = ScalarExecutor::new();
    let mut g = c.benchmark_group("scalar/binary");
    g.throughput(Throughput::Elements(1));

    let wf = Wavefront::all_xor();

    g.bench_function("step_binary_xor", |b| {
        b.iter(|| unsafe {
            let mut state_a = test_state();
            let state_b = test_state();
            exec.step_binary(black_box(&mut state_a), black_box(&state_b), black_box(&wf));
        })
    });

    let wf_add = Wavefront::new(PortAssignment::all_add());
    g.bench_function("step_binary_add", |b| {
        b.iter(|| unsafe {
            let mut state_a = test_state();
            let state_b = test_state();
            exec.step_binary(
                black_box(&mut state_a),
                black_box(&state_b),
                black_box(&wf_add),
            );
        })
    });

    g.finish();
}

// ============================================================================
// NeonExecutor Benchmarks (aarch64 only)
// ============================================================================

#[cfg(target_arch = "aarch64")]
fn bench_neon_cycle(c: &mut Criterion) {
    let exec = NeonExecutor::new();
    let mut g = c.benchmark_group("neon/cycle");
    g.throughput(Throughput::Elements(1));

    let ops: [(&str, Wavefront); 10] = [
        ("xor", Wavefront::all_xor()),
        ("and", Wavefront::new(PortAssignment::all_and())),
        ("or", Wavefront::new(PortAssignment::all_or())),
        ("not", Wavefront::all_not()),
        ("add", Wavefront::new(PortAssignment::all_add())),
        ("sub", Wavefront::new(PortAssignment::all_sub())),
        ("rotr_7", Wavefront::new(PortAssignment::rotr_only(7))),
        ("rotl_13", Wavefront::new(PortAssignment::rotl_only(13))),
        ("shr_3", Wavefront::new(PortAssignment::shr_only(3))),
        ("shl_10", Wavefront::new(PortAssignment::shl_only(10))),
    ];

    for (name, wf) in &ops {
        g.bench_function(*name, |b| {
            let mut s = test_state();
            b.iter(|| unsafe { exec.step(black_box(&mut s), black_box(wf)) })
        });
    }

    g.finish();
}

#[cfg(target_arch = "aarch64")]
fn bench_neon_sequence(c: &mut Criterion) {
    let exec = NeonExecutor::new();
    let mut g = c.benchmark_group("neon/sequence");

    for n in [16u64, 64, 256] {
        let program: Vec<Wavefront> = (0..n).map(|_| Wavefront::all_xor()).collect();
        g.throughput(Throughput::Elements(n));
        g.bench_function(format!("{n}_xor"), |b| {
            let mut s = test_state();
            b.iter(|| unsafe { exec.run(black_box(&mut s), black_box(&program)) })
        });
    }

    g.finish();
}

#[cfg(target_arch = "aarch64")]
fn bench_neon_comparison(c: &mut Criterion) {
    let scalar = ScalarExecutor::new();
    let neon = NeonExecutor::new();
    let mut g = c.benchmark_group("comparison/scalar_vs_neon");

    let program: Vec<Wavefront> = (0..64).map(|_| Wavefront::all_xor()).collect();
    g.throughput(Throughput::Elements(64));

    g.bench_function("scalar_64_xor", |b| {
        let mut s = test_state();
        b.iter(|| unsafe { scalar.run(black_box(&mut s), black_box(&program)) })
    });

    g.bench_function("neon_64_xor", |b| {
        let mut s = test_state();
        b.iter(|| unsafe { neon.run(black_box(&mut s), black_box(&program)) })
    });

    g.finish();
}

#[cfg(target_arch = "aarch64")]
fn bench_neon_throughput(c: &mut Criterion) {
    let exec = NeonExecutor::new();
    let mut g = c.benchmark_group("neon/throughput");
    g.throughput(Throughput::Bytes(STATE_TAXONS as u64));

    let wf = Wavefront::all_xor();
    g.bench_function("bits_per_cycle", |b| {
        let mut s = test_state();
        b.iter(|| unsafe { exec.step(black_box(&mut s), black_box(&wf)) })
    });

    g.finish();
}

// ============================================================================
// Criterion Groups
// ============================================================================

// Groups for all platforms
criterion_group!(
    scalar_benches,
    bench_scalar_cycle,
    bench_scalar_crypto,
    bench_scalar_permute,
    bench_scalar_sequence,
    bench_scalar_programs,
    bench_scalar_throughput,
    bench_scalar_lossless,
    bench_scalar_binary,
);

// Groups for aarch64
#[cfg(target_arch = "aarch64")]
criterion_group!(
    neon_benches,
    bench_neon_cycle,
    bench_neon_sequence,
    bench_neon_comparison,
    bench_neon_throughput,
);

// Main entry point
#[cfg(target_arch = "aarch64")]
criterion_main!(scalar_benches, neon_benches);

#[cfg(not(target_arch = "aarch64"))]
criterion_main!(scalar_benches);
