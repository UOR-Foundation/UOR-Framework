//! Benchmarks for microcode operations.
//!
//! Measures baseline performance of:
//! - Primitive operations (bnot, neg, xor, and, or)
//! - Derived operations (inc, dec, add, sub)
//! - Kogge-Stone parallel adder vs native add
//! - Executor step execution

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use uor::microcode::{
    Derivation, MicrocodeOps, MicrocodePrimitives, MicrocodeStep, ScalarMicrocodeExecutor,
    ScalarPrimitives,
};

// =============================================================================
// Primitive Operations
// =============================================================================

fn bench_microcode_primitives(c: &mut Criterion) {
    let mut group = c.benchmark_group("microcode_primitives");
    group.throughput(Throughput::Elements(1));

    let p = ScalarPrimitives;

    // Unary operations
    group.bench_function("bnot", |b| {
        b.iter(|| black_box(p.bnot(black_box(0xDEAD_BEEFu32))));
    });

    group.bench_function("neg", |b| {
        b.iter(|| black_box(p.neg(black_box(0xDEAD_BEEFu32))));
    });

    // Binary operations
    group.bench_function("xor", |b| {
        b.iter(|| black_box(p.xor(black_box(0xAAAA_AAAAu32), black_box(0x5555_5555))));
    });

    group.bench_function("and", |b| {
        b.iter(|| black_box(p.and(black_box(0xAAAA_AAAAu32), black_box(0x5555_5555))));
    });

    group.bench_function("or", |b| {
        b.iter(|| black_box(p.or(black_box(0xAAAA_AAAAu32), black_box(0x5555_5555))));
    });

    // Derived convenience operations
    group.bench_function("nand", |b| {
        b.iter(|| black_box(p.nand(black_box(0xAAAA_AAAAu32), black_box(0x5555_5555))));
    });

    group.bench_function("nor", |b| {
        b.iter(|| black_box(p.nor(black_box(0xAAAA_AAAAu32), black_box(0x5555_5555))));
    });

    group.bench_function("xnor", |b| {
        b.iter(|| black_box(p.xnor(black_box(0xAAAA_AAAAu32), black_box(0x5555_5555))));
    });

    group.bench_function("andn", |b| {
        b.iter(|| black_box(p.andn(black_box(0xAAAA_AAAAu32), black_box(0x5555_5555))));
    });

    group.finish();
}

// =============================================================================
// INC/DEC Operations
// =============================================================================

fn bench_microcode_inc(c: &mut Criterion) {
    let mut group = c.benchmark_group("microcode_inc");
    group.throughput(Throughput::Elements(1));

    let p = ScalarPrimitives;

    // Microcode inc: neg(bnot(x))
    group.bench_function("microcode_inc", |b| {
        b.iter(|| {
            let x = black_box(41u32);
            black_box(p.neg(p.bnot(x)))
        });
    });

    // Native wrapping_add for comparison
    group.bench_function("native_add_1", |b| {
        b.iter(|| {
            let x = black_box(41u32);
            black_box(x.wrapping_add(1))
        });
    });

    // Through MicrocodeOps trait
    group.bench_function("ops_inc", |b| {
        b.iter(|| {
            let x = black_box(41u32);
            black_box(p.inc(x))
        });
    });

    group.finish();
}

fn bench_microcode_dec(c: &mut Criterion) {
    let mut group = c.benchmark_group("microcode_dec");
    group.throughput(Throughput::Elements(1));

    let p = ScalarPrimitives;

    // Microcode dec: bnot(neg(x))
    group.bench_function("microcode_dec", |b| {
        b.iter(|| {
            let x = black_box(42u32);
            black_box(p.bnot(p.neg(x)))
        });
    });

    // Native wrapping_sub for comparison
    group.bench_function("native_sub_1", |b| {
        b.iter(|| {
            let x = black_box(42u32);
            black_box(x.wrapping_sub(1))
        });
    });

    // Through MicrocodeOps trait
    group.bench_function("ops_dec", |b| {
        b.iter(|| {
            let x = black_box(42u32);
            black_box(p.dec(x))
        });
    });

    group.finish();
}

// =============================================================================
// ADD Operations (Kogge-Stone vs Native)
// =============================================================================

fn bench_microcode_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("microcode_add");
    group.throughput(Throughput::Elements(1));

    let p = ScalarPrimitives;

    // Kogge-Stone adder (through MicrocodeOps trait)
    group.bench_function("kogge_stone_32bit", |b| {
        b.iter(|| {
            let a = black_box(12345u32);
            let b_val = black_box(67890u32);
            black_box(p.add(a, b_val))
        });
    });

    // Native wrapping_add for comparison
    group.bench_function("native_add", |b| {
        b.iter(|| {
            let a = black_box(12345u32);
            let b_val = black_box(67890u32);
            black_box(a.wrapping_add(b_val))
        });
    });

    // Subtraction (add(a, neg(b)))
    group.bench_function("kogge_stone_sub", |b| {
        b.iter(|| {
            let a = black_box(67890u32);
            let b_val = black_box(12345u32);
            black_box(p.sub(a, b_val))
        });
    });

    // Native wrapping_sub for comparison
    group.bench_function("native_sub", |b| {
        b.iter(|| {
            let a = black_box(67890u32);
            let b_val = black_box(12345u32);
            black_box(a.wrapping_sub(b_val))
        });
    });

    group.finish();
}

// =============================================================================
// Executor Benchmarks
// =============================================================================

fn bench_executor_step(c: &mut Criterion) {
    let mut group = c.benchmark_group("executor_step");
    group.throughput(Throughput::Elements(1));

    let mut exec = ScalarMicrocodeExecutor::new();
    exec.set_register(0, 0xDEAD_BEEF);
    exec.set_register(1, 0xCAFE_BABE);

    // Individual step execution
    let bnot_step = MicrocodeStep::BNot { dst: 2, src: 0 };
    group.bench_function("execute_bnot", |b| {
        b.iter(|| {
            exec.execute_step(black_box(&bnot_step));
        });
    });

    let neg_step = MicrocodeStep::Neg { dst: 2, src: 0 };
    group.bench_function("execute_neg", |b| {
        b.iter(|| {
            exec.execute_step(black_box(&neg_step));
        });
    });

    let xor_step = MicrocodeStep::Xor { dst: 2, a: 0, b: 1 };
    group.bench_function("execute_xor", |b| {
        b.iter(|| {
            exec.execute_step(black_box(&xor_step));
        });
    });

    let and_step = MicrocodeStep::And { dst: 2, a: 0, b: 1 };
    group.bench_function("execute_and", |b| {
        b.iter(|| {
            exec.execute_step(black_box(&and_step));
        });
    });

    let or_step = MicrocodeStep::Or { dst: 2, a: 0, b: 1 };
    group.bench_function("execute_or", |b| {
        b.iter(|| {
            exec.execute_step(black_box(&or_step));
        });
    });

    group.finish();
}

fn bench_executor_derivation(c: &mut Criterion) {
    let mut group = c.benchmark_group("executor_derivation");

    let mut exec = ScalarMicrocodeExecutor::new();

    // Standard derivations
    let inc = uor::microcode::derivation::standard::inc();
    let dec = uor::microcode::derivation::standard::dec();
    let nand = uor::microcode::derivation::standard::nand();
    let nor = uor::microcode::derivation::standard::nor();
    let xnor = uor::microcode::derivation::standard::xnor();

    group.throughput(Throughput::Elements(1));

    group.bench_function("inc_derivation", |b| {
        b.iter(|| {
            exec.set_register(0, black_box(41));
            exec.execute_derivation(black_box(&inc));
            black_box(exec.get_register(0))
        });
    });

    group.bench_function("dec_derivation", |b| {
        b.iter(|| {
            exec.set_register(0, black_box(42));
            exec.execute_derivation(black_box(&dec));
            black_box(exec.get_register(0))
        });
    });

    group.bench_function("nand_derivation", |b| {
        b.iter(|| {
            exec.set_register(0, black_box(0xAAAA_AAAA));
            exec.set_register(1, black_box(0x5555_5555));
            exec.execute_derivation(black_box(&nand));
            black_box(exec.get_register(0))
        });
    });

    group.bench_function("nor_derivation", |b| {
        b.iter(|| {
            exec.set_register(0, black_box(0xAAAA_AAAA));
            exec.set_register(1, black_box(0x5555_5555));
            exec.execute_derivation(black_box(&nor));
            black_box(exec.get_register(0))
        });
    });

    group.bench_function("xnor_derivation", |b| {
        b.iter(|| {
            exec.set_register(0, black_box(0xAAAA_AAAA));
            exec.set_register(1, black_box(0x5555_5555));
            exec.execute_derivation(black_box(&xnor));
            black_box(exec.get_register(0))
        });
    });

    group.finish();
}

// =============================================================================
// Batch Operations
// =============================================================================

fn bench_batch_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_operations");

    let p = ScalarPrimitives;

    // Batch sizes
    for count in [100, 1000, 10000] {
        group.throughput(Throughput::Elements(count as u64));

        // Batch inc via microcode
        group.bench_with_input(
            BenchmarkId::new("batch_inc_microcode", count),
            &count,
            |b, &n| {
                let mut values: Vec<u32> = (0..n).map(|i| i as u32).collect();
                b.iter(|| {
                    for v in values.iter_mut() {
                        *v = p.inc(black_box(*v));
                    }
                    black_box(&values);
                });
            },
        );

        // Batch inc via native
        group.bench_with_input(
            BenchmarkId::new("batch_inc_native", count),
            &count,
            |b, &n| {
                let mut values: Vec<u32> = (0..n).map(|i| i as u32).collect();
                b.iter(|| {
                    for v in values.iter_mut() {
                        *v = black_box(*v).wrapping_add(1);
                    }
                    black_box(&values);
                });
            },
        );

        // Batch add via Kogge-Stone
        group.bench_with_input(
            BenchmarkId::new("batch_add_kogge", count),
            &count,
            |b, &n| {
                let a: Vec<u32> = (0..n).map(|i| i as u32).collect();
                let c: Vec<u32> = (0..n).map(|i| (i * 2) as u32).collect();
                let mut results = vec![0u32; n as usize];
                b.iter(|| {
                    for i in 0..n as usize {
                        results[i] = p.add(black_box(a[i]), black_box(c[i]));
                    }
                    black_box(&results);
                });
            },
        );

        // Batch add via native
        group.bench_with_input(
            BenchmarkId::new("batch_add_native", count),
            &count,
            |b, &n| {
                let a: Vec<u32> = (0..n).map(|i| i as u32).collect();
                let c: Vec<u32> = (0..n).map(|i| (i * 2) as u32).collect();
                let mut results = vec![0u32; n as usize];
                b.iter(|| {
                    for i in 0..n as usize {
                        results[i] = black_box(a[i]).wrapping_add(black_box(c[i]));
                    }
                    black_box(&results);
                });
            },
        );
    }

    group.finish();
}

// =============================================================================
// Custom Derivation Execution
// =============================================================================

fn bench_custom_derivation(c: &mut Criterion) {
    let mut group = c.benchmark_group("custom_derivation");
    group.throughput(Throughput::Elements(1));

    let mut exec = ScalarMicrocodeExecutor::new();

    // Create a custom derivation: compute (a XOR b) AND (NOT a)
    // r0 = a, r1 = b
    // r2 = a XOR b
    // r3 = NOT a
    // r0 = r2 AND r3
    let custom_steps = vec![
        MicrocodeStep::Xor { dst: 2, a: 0, b: 1 },
        MicrocodeStep::BNot { dst: 3, src: 0 },
        MicrocodeStep::And { dst: 0, a: 2, b: 3 },
    ];
    let custom = Derivation::new("custom_xor_andn", custom_steps);

    group.bench_function("3_step_custom", |b| {
        b.iter(|| {
            exec.set_register(0, black_box(0xAAAA_AAAA));
            exec.set_register(1, black_box(0x5555_5555));
            exec.execute_derivation(black_box(&custom));
            black_box(exec.get_register(0))
        });
    });

    // 5-step derivation
    let longer_steps = vec![
        MicrocodeStep::Xor { dst: 2, a: 0, b: 1 },
        MicrocodeStep::BNot { dst: 3, src: 0 },
        MicrocodeStep::And { dst: 4, a: 2, b: 3 },
        MicrocodeStep::Or { dst: 5, a: 4, b: 1 },
        MicrocodeStep::BNot { dst: 0, src: 5 },
    ];
    let longer = Derivation::new("5_step", longer_steps);

    group.bench_function("5_step_custom", |b| {
        b.iter(|| {
            exec.set_register(0, black_box(0xAAAA_AAAA));
            exec.set_register(1, black_box(0x5555_5555));
            exec.execute_derivation(black_box(&longer));
            black_box(exec.get_register(0))
        });
    });

    // 10-step derivation
    let ten_steps = vec![
        MicrocodeStep::Xor { dst: 2, a: 0, b: 1 },
        MicrocodeStep::BNot { dst: 3, src: 0 },
        MicrocodeStep::And { dst: 4, a: 2, b: 3 },
        MicrocodeStep::Or { dst: 5, a: 4, b: 1 },
        MicrocodeStep::BNot { dst: 6, src: 5 },
        MicrocodeStep::Xor { dst: 7, a: 6, b: 2 },
        MicrocodeStep::And { dst: 8, a: 7, b: 3 },
        MicrocodeStep::Or { dst: 9, a: 8, b: 4 },
        MicrocodeStep::BNot { dst: 10, src: 9 },
        MicrocodeStep::Xor {
            dst: 0,
            a: 10,
            b: 5,
        },
    ];
    let ten = Derivation::new("10_step", ten_steps);

    group.bench_function("10_step_custom", |b| {
        b.iter(|| {
            exec.set_register(0, black_box(0xAAAA_AAAA));
            exec.set_register(1, black_box(0x5555_5555));
            exec.execute_derivation(black_box(&ten));
            black_box(exec.get_register(0))
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_microcode_primitives,
    bench_microcode_inc,
    bench_microcode_dec,
    bench_microcode_add,
    bench_executor_step,
    bench_executor_derivation,
    bench_batch_operations,
    bench_custom_derivation,
);
criterion_main!(benches);
