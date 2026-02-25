//! UOR Wavefront Benchmarks
//!
//! Measures cellular automaton performance across executors and operation types.
//!
//! | Metric | Target |
//! |--------|--------|
//! | Single wavefront | < 5 cycles |
//! | 64-wavefront sequence | < 200 cycles |
//! | Throughput | >= 512 bits/cycle |
//!
//! ```bash
//! RUSTFLAGS="-C target-feature=+avx2,+sha,+aes" cargo bench -p uor
//! ```

use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(target_arch = "x86_64")]
use criterion::{black_box, Throughput};
#[cfg(target_arch = "x86_64")]
use uor::arch::x86_64::Zen3AsmExecutor;
#[cfg(target_arch = "x86_64")]
use uor::arch::Zen3Executor;
#[cfg(target_arch = "x86_64")]
use uor::isa::{PortAssignment, UorStep, Wavefront, WavefrontOp};
#[cfg(target_arch = "x86_64")]
use uor::state::{UorState, STATE_TAXONS};
#[cfg(target_arch = "x86_64")]
use uor::taxon::Taxon;
#[cfg(target_arch = "x86_64")]
use uor::wavefront::{aes, rotate, sha256};

/// Deterministic test state: YMM[i][j] = (i*32 + j) % 256.
#[cfg(target_arch = "x86_64")]
fn test_state() -> UorState {
    let mut s = UorState::zero();
    for i in 0..16 {
        for j in 0..32 {
            s.ymm[i][j] = Taxon::new(((i * 32 + j) % 256) as u8);
        }
    }
    s
}

// ============================================================================
// Intrinsics Executor
// ============================================================================

/// Single wavefront cycle per operation type (intrinsics).
#[cfg(target_arch = "x86_64")]
fn bench_wavefront_cycle(c: &mut Criterion) {
    let exec = Zen3Executor::new();
    let mut g = c.benchmark_group("wavefront/cycle");
    g.throughput(Throughput::Elements(1));

    let ops: [(&str, Wavefront); 5] = [
        ("xor", Wavefront::all_xor()),
        ("rotate_xor", Wavefront::rotate_xor(7)),
        ("and", Wavefront::new(PortAssignment::all_and())),
        ("or", Wavefront::new(PortAssignment::all_or())),
        ("add", Wavefront::new(PortAssignment::all_add())),
    ];
    for (name, wf) in &ops {
        g.bench_function(*name, |b| {
            let mut s = test_state();
            b.iter(|| unsafe { exec.step(black_box(&mut s), black_box(wf)) })
        });
    }

    g.finish();
}

/// Wavefront program sequences (intrinsics).
#[cfg(target_arch = "x86_64")]
fn bench_wavefront_sequence(c: &mut Criterion) {
    let exec = Zen3Executor::new();
    let mut g = c.benchmark_group("wavefront/sequence");

    let programs: [(&str, Vec<Wavefront>); 4] = [
        ("64_xor", (0..64).map(|_| Wavefront::all_xor()).collect()),
        ("sha256_compress", uor::wavefront::sha256_compress_program()),
        ("aes128_encrypt", uor::wavefront::aes128_encrypt_program()),
        (
            "64_mixed",
            (0..64)
                .map(|i| match i % 3 {
                    0 => Wavefront::rotate_xor((i % 32 + 1) as u8),
                    1 => Wavefront::all_xor(),
                    _ => Wavefront::new(PortAssignment::all_and()),
                })
                .collect(),
        ),
    ];

    for (name, program) in &programs {
        g.throughput(Throughput::Elements(program.len() as u64));
        g.bench_function(*name, |b| {
            let mut s = test_state();
            b.iter(|| unsafe { exec.run(black_box(&mut s), black_box(program)) })
        });
    }

    g.finish();
}

/// Bits-per-cycle throughput (intrinsics).
#[cfg(target_arch = "x86_64")]
fn bench_throughput_bits(c: &mut Criterion) {
    let exec = Zen3Executor::new();
    let mut g = c.benchmark_group("wavefront/throughput");
    g.throughput(Throughput::Bytes(STATE_TAXONS as u64));

    let wf = Wavefront::all_xor();
    g.bench_function("bits_per_cycle", |b| {
        let mut s = test_state();
        b.iter(|| unsafe { exec.step(black_box(&mut s), black_box(&wf)) })
    });

    g.finish();
}

/// State operations (copy, init, self-inverse).
#[cfg(target_arch = "x86_64")]
fn bench_state_operations(c: &mut Criterion) {
    let exec = Zen3Executor::new();
    let mut g = c.benchmark_group("state/operations");

    g.bench_function("xor_self_inverse", |b| {
        b.iter(|| {
            let mut s = test_state();
            let wf = Wavefront::all_xor();
            unsafe { exec.step(&mut s, &wf) };
            black_box(s)
        })
    });

    g.bench_function("state_copy", |b| {
        let s = UorState::zero();
        b.iter(|| black_box(s))
    });

    g.bench_function("state_init", |b| b.iter(|| black_box(UorState::zero())));

    g.finish();
}

/// SHA-256, AES, and rotation pattern benchmarks.
#[cfg(target_arch = "x86_64")]
fn bench_patterns(c: &mut Criterion) {
    let exec = Zen3Executor::new();
    let mut g = c.benchmark_group("patterns");

    let patterns: [(&str, Vec<Wavefront>); 10] = [
        ("sha256_big_sigma0", sha256::big_sigma0().to_vec()),
        ("sha256_big_sigma1", sha256::big_sigma1().to_vec()),
        ("sha256_small_sigma0", sha256::small_sigma0().to_vec()),
        ("sha256_small_sigma1", sha256::small_sigma1().to_vec()),
        ("sha256_ch", sha256::ch().to_vec()),
        ("sha256_maj", sha256::maj().to_vec()),
        ("aes_enc_round", vec![aes::enc_round()]),
        ("rotate_right_7", vec![rotate::right(7)]),
        ("rotate_right_13", vec![rotate::right(13)]),
        ("rotate_right_22", vec![rotate::right(22)]),
    ];

    for (name, pattern) in &patterns {
        g.bench_function(*name, |b| {
            let mut s = test_state();
            b.iter(|| unsafe {
                for wf in pattern {
                    exec.step(black_box(&mut s), black_box(wf));
                }
            })
        });
    }

    g.finish();
}

/// All operation types for systematic conformance comparison.
#[cfg(target_arch = "x86_64")]
fn bench_conformance_validation(c: &mut Criterion) {
    let exec = Zen3Executor::new();
    let mut g = c.benchmark_group("conformance");
    g.throughput(Throughput::Elements(1));

    let ops: Vec<(&str, Wavefront)> = vec![
        ("xor", Wavefront::all_xor()),
        ("and", Wavefront::new(PortAssignment::all_and())),
        ("or", Wavefront::new(PortAssignment::all_or())),
        ("not", Wavefront::all_not()),
        ("add", Wavefront::new(PortAssignment::all_add())),
        ("sub", Wavefront::new(PortAssignment::all_sub())),
        ("rotr_7", Wavefront::new(PortAssignment::rotr_only(7))),
        ("rotr_13", Wavefront::new(PortAssignment::rotr_only(13))),
        ("rotr_22", Wavefront::new(PortAssignment::rotr_only(22))),
        ("rotl_7", Wavefront::new(PortAssignment::rotl_only(7))),
        ("shr_3", Wavefront::new(PortAssignment::shr_only(3))),
        ("shl_10", Wavefront::new(PortAssignment::shl_only(10))),
        ("shuffle", Wavefront::shuffle()),
        ("permute", Wavefront::permute()),
        ("sha256_round", Wavefront::sha256_round()),
        ("aes_round", Wavefront::aes_round()),
    ];

    for (name, wf) in &ops {
        g.bench_function(*name, |b| {
            let mut s = test_state();
            b.iter(|| unsafe { exec.step(black_box(&mut s), black_box(wf)) })
        });
    }

    g.finish();
}

/// Port utilization: single, dual, triple, max complexity.
#[cfg(target_arch = "x86_64")]
fn bench_port_efficiency(c: &mut Criterion) {
    let exec = Zen3Executor::new();
    let mut g = c.benchmark_group("port_efficiency");
    g.throughput(Throughput::Elements(1));

    let configs: [(&str, PortAssignment); 4] = [
        (
            "single_port",
            PortAssignment {
                port0: WavefrontOp::Nop,
                port1: WavefrontOp::Xor,
                port5: WavefrontOp::Nop,
            },
        ),
        (
            "two_ports",
            PortAssignment {
                port0: WavefrontOp::Nop,
                port1: WavefrontOp::Xor,
                port5: WavefrontOp::And,
            },
        ),
        (
            "all_ports",
            PortAssignment {
                port0: WavefrontOp::RotR(7),
                port1: WavefrontOp::Xor,
                port5: WavefrontOp::And,
            },
        ),
        (
            "max_complexity",
            PortAssignment {
                port0: WavefrontOp::Sha256Round,
                port1: WavefrontOp::AesRound,
                port5: WavefrontOp::Shuffle,
            },
        ),
    ];

    for (name, pa) in &configs {
        let wf = Wavefront::new(*pa);
        g.bench_function(*name, |b| {
            let mut s = test_state();
            b.iter(|| unsafe { exec.step(black_box(&mut s), black_box(&wf)) })
        });
    }

    g.finish();
}

// ============================================================================
// Inline Assembly Executor
// ============================================================================

/// Single wavefront cycle (inline asm).
#[cfg(target_arch = "x86_64")]
fn bench_asm_cycle(c: &mut Criterion) {
    let exec = Zen3AsmExecutor::new();
    let mut g = c.benchmark_group("wavefront_asm/cycle");
    g.throughput(Throughput::Elements(1));

    let wf = Wavefront::all_xor();
    g.bench_function("xor", |b| {
        let mut s = test_state();
        b.iter(|| unsafe { exec.step(black_box(&mut s), black_box(&wf)) })
    });

    g.finish();
}

/// Wavefront sequences (inline asm): run and step_n.
#[cfg(target_arch = "x86_64")]
fn bench_asm_sequence(c: &mut Criterion) {
    let exec = Zen3AsmExecutor::new();
    let mut g = c.benchmark_group("wavefront_asm/sequence");
    let wf = Wavefront::all_xor();

    for n in [64u64, 256] {
        let program: Vec<Wavefront> = (0..n).map(|_| Wavefront::all_xor()).collect();
        g.throughput(Throughput::Elements(n));
        g.bench_function(format!("{n}_xor"), |b| {
            let mut s = test_state();
            b.iter(|| unsafe { exec.run(black_box(&mut s), black_box(&program)) })
        });
    }

    g.throughput(Throughput::Elements(64));
    g.bench_function("step_n_64", |b| {
        let mut s = test_state();
        b.iter(|| unsafe { exec.step_n(black_box(&mut s), black_box(&wf), 64) })
    });

    g.finish();
}

// ============================================================================
// Intrinsics vs ASM Comparison
// ============================================================================

/// Side-by-side: intrinsics vs inline asm for 64 XOR wavefronts.
#[cfg(target_arch = "x86_64")]
fn bench_comparison(c: &mut Criterion) {
    let program: Vec<Wavefront> = (0..64).map(|_| Wavefront::all_xor()).collect();
    let mut g = c.benchmark_group("comparison/xor_64");
    g.throughput(Throughput::Elements(64));

    let intrinsics = Zen3Executor::new();
    g.bench_function("intrinsics", |b| {
        let mut s = test_state();
        b.iter(|| unsafe { intrinsics.run(black_box(&mut s), black_box(&program)) })
    });

    let asm = Zen3AsmExecutor::new();
    g.bench_function("asm", |b| {
        let mut s = test_state();
        b.iter(|| unsafe { asm.run(black_box(&mut s), black_box(&program)) })
    });

    g.finish();
}

// ============================================================================
// Criterion Groups
// ============================================================================

#[cfg(target_arch = "x86_64")]
criterion_group!(
    benches,
    bench_wavefront_cycle,
    bench_wavefront_sequence,
    bench_throughput_bits,
    bench_state_operations,
    bench_patterns,
    bench_conformance_validation,
    bench_port_efficiency,
    bench_asm_cycle,
    bench_asm_sequence,
    bench_comparison,
);

#[cfg(not(target_arch = "x86_64"))]
fn bench_noop(_c: &mut Criterion) {}

#[cfg(not(target_arch = "x86_64"))]
criterion_group!(benches, bench_noop);

criterion_main!(benches);
