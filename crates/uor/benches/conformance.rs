//! UOR Conformance Tier Benchmarks
//!
//! Measures performance against three conformance tiers:
//! - **MINIMUM**: 5 cycles/wavefront, 512 bits/cycle
//! - **OPTIMAL**: 3 cycles/wavefront, 1600 bits/cycle
//! - **THEORETICAL**: 1 cycle/wavefront, 4992 bits/cycle
//!
//! ```bash
//! RUSTFLAGS="-C target-feature=+avx2,+sha,+aes" cargo bench -p uor conformance
//! ```

use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(target_arch = "x86_64")]
use criterion::{black_box, Throughput};
#[cfg(target_arch = "x86_64")]
use uor::arch::x86_64::Zen3AsmExecutor;
#[cfg(target_arch = "x86_64")]
use uor::conformance::{
    MIN_BITS_PER_CYCLE, MIN_SEQUENCE_64_CYCLES, MIN_SINGLE_WAVEFRONT_CYCLES, OPT_BITS_PER_CYCLE,
    OPT_SEQUENCE_64_CYCLES, OPT_SINGLE_WAVEFRONT_CYCLES, THEORETICAL_SEQUENCE_64_CYCLES,
    THEORETICAL_SINGLE_WAVEFRONT_CYCLES,
};
#[cfg(target_arch = "x86_64")]
use uor::isa::{PortAssignment, UorStep, Wavefront};
#[cfg(target_arch = "x86_64")]
use uor::state::{UorState, STATE_BITS};
#[cfg(target_arch = "x86_64")]
use uor::taxon::Taxon;

/// Deterministic test state: YMM[i][j] = (i*32 + j) * 7.
#[cfg(target_arch = "x86_64")]
fn test_state() -> UorState {
    let mut s = UorState::zero();
    for i in 0..16 {
        for j in 0..32 {
            s.ymm[i][j] = Taxon::new(((i * 32 + j) * 7) as u8);
        }
    }
    s
}

/// Single wavefront latency per operation type.
#[cfg(target_arch = "x86_64")]
fn bench_single_wavefront(c: &mut Criterion) {
    let exec = Zen3AsmExecutor::new();
    let mut g = c.benchmark_group("conformance/single");
    g.throughput(Throughput::Elements(1));

    let ops: [(&str, Wavefront); 5] = [
        ("xor", Wavefront::all_xor()),
        ("and", Wavefront::new(PortAssignment::all_and())),
        ("or", Wavefront::new(PortAssignment::all_or())),
        ("add", Wavefront::new(PortAssignment::all_add())),
        ("rotr_7", Wavefront::new(PortAssignment::rotr_only(7))),
    ];
    for (name, wf) in &ops {
        g.bench_function(*name, |b| {
            let mut s = test_state();
            b.iter(|| unsafe { exec.step(black_box(&mut s), black_box(wf)) })
        });
    }

    g.finish();
    println!("\nSINGLE WAVEFRONT: MIN <{MIN_SINGLE_WAVEFRONT_CYCLES}  OPT <{OPT_SINGLE_WAVEFRONT_CYCLES}  THEORETICAL {THEORETICAL_SINGLE_WAVEFRONT_CYCLES} cycles");
}

/// 64-wavefront fused sequence latency.
#[cfg(target_arch = "x86_64")]
fn bench_fused_64(c: &mut Criterion) {
    let exec = Zen3AsmExecutor::new();
    let mut g = c.benchmark_group("conformance/fused_64");
    g.throughput(Throughput::Elements(64));

    let program: Vec<Wavefront> = (0..64).map(|_| Wavefront::all_xor()).collect();
    g.bench_function("xor_run", |b| {
        let mut s = test_state();
        b.iter(|| unsafe { exec.run(black_box(&mut s), black_box(&program)) })
    });

    let step_ops: [(&str, Wavefront); 2] = [
        ("xor_step_n", Wavefront::all_xor()),
        ("and_step_n", Wavefront::new(PortAssignment::all_and())),
    ];
    for (name, wf) in &step_ops {
        g.bench_function(*name, |b| {
            let mut s = test_state();
            b.iter(|| unsafe { exec.step_n(black_box(&mut s), black_box(wf), 64) })
        });
    }

    g.finish();
    println!("\nFUSED 64: MIN <{MIN_SEQUENCE_64_CYCLES}  OPT <{OPT_SEQUENCE_64_CYCLES}  THEORETICAL {THEORETICAL_SEQUENCE_64_CYCLES} cycles");
}

/// Throughput in bits per cycle.
#[cfg(target_arch = "x86_64")]
fn bench_throughput(c: &mut Criterion) {
    let exec = Zen3AsmExecutor::new();
    let mut g = c.benchmark_group("conformance/throughput");
    g.throughput(Throughput::Bytes(STATE_BITS as u64 / 8));

    let wf = Wavefront::all_xor();
    g.bench_function("bits_per_cycle", |b| {
        let mut s = test_state();
        b.iter(|| unsafe { exec.step(black_box(&mut s), black_box(&wf)) })
    });

    g.finish();
    println!("\nTHROUGHPUT: MIN >={MIN_BITS_PER_CYCLE}  OPT >={OPT_BITS_PER_CYCLE}  THEORETICAL {STATE_BITS} bits/cycle");
}

/// Unrolled step_n scaling: 1, 4, 8, 16, 64, 128 iterations.
#[cfg(target_arch = "x86_64")]
fn bench_unrolled(c: &mut Criterion) {
    let exec = Zen3AsmExecutor::new();
    let mut g = c.benchmark_group("conformance/unrolled");
    let wf = Wavefront::all_xor();

    for n in [1u64, 4, 8, 16, 64, 128] {
        g.throughput(Throughput::Elements(n));
        g.bench_function(format!("xor_{n}"), |b| {
            let mut s = test_state();
            b.iter(|| unsafe { exec.step_n(black_box(&mut s), black_box(&wf), n as usize) })
        });
    }

    g.finish();
}

#[cfg(target_arch = "x86_64")]
criterion_group!(
    conformance_benches,
    bench_single_wavefront,
    bench_fused_64,
    bench_throughput,
    bench_unrolled,
);

#[cfg(not(target_arch = "x86_64"))]
fn bench_noop(_c: &mut Criterion) {}

#[cfg(not(target_arch = "x86_64"))]
criterion_group!(conformance_benches, bench_noop);

criterion_main!(conformance_benches);
