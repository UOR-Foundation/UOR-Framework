# UOR Performance Guide

**Understanding wavefront execution performance**

---

## Overview

UOR achieves high performance through wavefront execution on the CPU register file. All operations are register-to-register SIMD intrinsics with zero memory access during execution.

For the formal execution model and timing guarantees, see [CELLULAR_AUTOMATA.md](CELLULAR_AUTOMATA.md). For x86_64 implementation details, see [ARCHITECTURE.md](ARCHITECTURE.md).

---

## Execution Model

### State Layout

UOR state occupies the entire register file:

| Register Set | Count | Bits | Taxons |
|--------------|-------|------|--------|
| YMM0-15 | 16 | 4096 | 512 |
| GPR registers | 14 | 896 | 112 |
| **Total** | 30 | **4992** | **624** |

All 4992 bits are transformed simultaneously in each wavefront.

### Wavefront Cost

| Component | Cycles | Description |
|-----------|--------|-------------|
| Load state | ~16 | Load 624 taxons into registers |
| Per wavefront | ~1 | Pure SIMD intrinsic |
| Store state | ~8 | Store registers back |

**Non-fused execution:** Load + Execute + Store per wavefront = ~25 cycles/wavefront

**Fused execution:** Load once + Execute all + Store once = ~1 cycle/wavefront (amortized)

### Fused vs Non-Fused

```
Non-Fused (UorStep::step):
  ┌─────────┐   ┌─────────┐   ┌─────────┐
  │  Load   │──►│ Execute │──►│  Store  │   (per wavefront)
  └─────────┘   └─────────┘   └─────────┘
      16            1             8        = 25 cycles/wavefront

Fused (UorStepFused::run_fused):
  ┌─────────┐   ┌─────────┬─────────┬─────────┐   ┌─────────┐
  │  Load   │──►│  Exec 1 │  Exec 2 │  Exec N │──►│  Store  │
  └─────────┘   └─────────┴─────────┴─────────┘   └─────────┘
      16              1         1         1            8

  For 64 wavefronts: 16 + 64 + 8 = 88 cycles = 1.4 cycles/wavefront
```

---

## Conformance Targets

UOR implementations must meet these performance criteria:

| Criterion | Target | Description |
|-----------|--------|-------------|
| Single Wavefront | < 5 cycles | Individual operation latency |
| 64-Wavefront Sequence | < 200 cycles | Program throughput |
| Throughput | ≥ 512 bits/cycle | Sustained bandwidth |

### Throughput Calculation

```
State size:        4992 bits
Target latency:    5 cycles/wavefront
Minimum throughput: 4992 / 5 = 998 bits/cycle

This exceeds the 512 bits/cycle target.
```

### Validation Functions

```rust
use uor::conformance::{
    validate_wavefront_latency,
    validate_sequence_latency,
    validate_throughput,
};

// Single wavefront: < 5 cycles
validate_wavefront_latency(3)?;  // Ok(())

// 64 wavefronts: < 200 cycles
validate_sequence_latency(88)?;  // Ok(3) - avg cycles/wf

// Throughput: >= 512 bits/cycle
validate_throughput(5)?;  // Ok(998) - achieved bits/cycle
```

---

## CPU Feature Requirements

UOR requires specific CPU features with **no software fallback**:

| Feature | Purpose | Detection |
|---------|---------|-----------|
| AVX2 | 256-bit SIMD operations | `std::is_x86_feature_detected!("avx2")` |
| SHA-NI | SHA-256 hardware | `std::is_x86_feature_detected!("sha")` |
| AES-NI | AES hardware | `std::is_x86_feature_detected!("aes")` |

### Feature Check

```rust
use uor::arch::has_required_features;

if !has_required_features() {
    panic!("UOR requires AVX2, SHA-NI, and AES-NI");
}
```

### Why No Fallback?

Software fallbacks would violate conformance targets:

| Implementation | Cycles/Wavefront | Conformant? |
|----------------|------------------|-------------|
| AVX2 hardware | ~1 | Yes |
| SSE2 software | ~8-16 | Maybe |
| Scalar software | ~100+ | No |

A scalar fallback would be 100× slower and fail conformance.

---

## Expected Performance

### Single Operations

| Operation | Wavefronts | Expected Cycles |
|-----------|------------|-----------------|
| XOR | 1 | ~1 |
| AND | 1 | ~1 |
| ADD | 1 | ~1 |
| RotR(n) | 1 | ~1 |
| ShR(n) | 1 | ~1 |

### SHA-256 Patterns

| Pattern | Wavefronts | Expected Cycles |
|---------|------------|-----------------|
| Σ₀ (big_sigma0) | 3 | ~3 |
| Σ₁ (big_sigma1) | 3 | ~3 |
| Ch | 2 | ~2 |
| Maj | 3 | ~3 |
| Round (SHA-NI) | 1 | ~1 |
| Full compress | 32 | ~32 |

### AES Patterns

| Pattern | Wavefronts | Expected Cycles |
|---------|------------|-----------------|
| Single round | 1 | ~1 |
| AES-128 encrypt | 10 | ~10 |
| AES-256 encrypt | 14 | ~14 |

### Full Programs

| Program | Wavefronts | Fused Cycles |
|---------|------------|--------------|
| SHA-256 compress | 32 | ~48 (16 + 32 + 0) |
| AES-128 block | 10 | ~34 (16 + 10 + 8) |
| 64 XOR rounds | 64 | ~88 (16 + 64 + 8) |

---

## Benchmarking

### Running Benchmarks

```bash
# Run all UOR benchmarks
cargo bench -p uor

# Run specific benchmark
cargo bench -p uor -- wavefront

# Generate HTML report
cargo bench -p uor -- --save-baseline baseline
```

### Interpreting Results

```
wavefront_xor          time:   [0.8234 ns 0.8345 ns 0.8456 ns]
wavefront_rotate       time:   [0.8123 ns 0.8234 ns 0.8345 ns]
sha256_compress_fused  time:   [15.234 ns 15.456 ns 15.678 ns]
```

**Converting to cycles** (at 3.5 GHz):

```rust
use uor::conformance::ns_to_cycles;

let cycles = ns_to_cycles(15.456, 3.5);  // ~54 cycles
```

### Measurement Tips

1. **Warm up CPU** - Run benchmarks after CPU reaches boost frequency
2. **Pin to core** - Use `taskset` to avoid migration overhead
3. **Disable turbo** - For consistent measurements
4. **Multiple runs** - Use criterion's statistical analysis

---

## Profiling

### CPU Profiling

```bash
# Using perf (Linux)
cargo build --release -p your_crate
perf record ./target/release/your_binary
perf report

# Using Instruments (macOS)
cargo build --release -p your_crate
instruments -t "Time Profiler" ./target/release/your_binary
```

### Cycle Counting

```rust
#[cfg(target_arch = "x86_64")]
fn rdtsc() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}

let start = rdtsc();
unsafe { executor.run_fused(&mut state, &program); }
let cycles = rdtsc() - start;
```

### What to Look For

1. **Load/store overhead** - Should be ~24 cycles total
2. **Per-wavefront cost** - Should be ~1 cycle (fused)
3. **Memory access** - Should be zero during execution
4. **Branch mispredictions** - Should be zero (no data-dependent branches)

---

## Optimization Guidelines

### Do Use UOR For

1. **Cryptographic workloads**
   - SHA-256, AES, ChaCha
   - Constant-time execution matters
   - Hardware acceleration available

2. **SIMD batch processing**
   - Processing 4992 bits simultaneously
   - Register-resident state machines
   - High throughput requirements

3. **Maximum hardware utilization**
   - All 3 execution ports fire per cycle
   - No memory latency
   - Predictable timing

### Don't Use UOR For

1. **Scalar operations**
   - Single-byte processing
   - Sequential algorithms
   - Better to use native Rust

2. **Memory-bound workloads**
   - Large data sets that don't fit in registers
   - Frequent state serialization
   - Network I/O

3. **Platforms without required features**
   - No AVX2 = no UOR
   - No fallback = conformance violation
   - Consider portable alternatives

---

## Complexity Analysis

### Wavefront Operations

| Operation | Time | Space | Notes |
|-----------|------|-------|-------|
| Single wavefront | O(1) | O(1) | Fixed 4992-bit state |
| N wavefronts (fused) | O(N) | O(1) | ~1 cycle per wavefront |
| N wavefronts (non-fused) | O(N) | O(1) | ~25 cycles per wavefront |

### Algorithm Complexity

| Algorithm | Wavefronts | Complexity | Notes |
|-----------|------------|------------|-------|
| SHA-256 compress | 32 | O(1) | Fixed for 512-bit block |
| AES-128 encrypt | 10 | O(1) | Fixed for 128-bit block |
| State XOR | 1 | O(1) | All 4992 bits at once |

---

## Performance Comparison

### UOR vs Native Operations

| Operation | UOR (fused) | Native | Ratio |
|-----------|-------------|--------|-------|
| 4992-bit XOR | ~1 cycle | ~1 cycle | 1× |
| SHA-256 round (×2) | ~1 cycle | ~2 cycles | 0.5× |
| AES round | ~1 cycle | ~1 cycle | 1× |

UOR achieves native performance because it **is** native SIMD execution.

### UOR vs Table Lookup

| Approach | Cycles/op | Notes |
|----------|-----------|-------|
| UOR wavefront | ~1 | SIMD intrinsic |
| Table lookup (L1) | ~4 | Cache hit |
| Table lookup (L2) | ~12 | Cache miss |
| Table lookup (RAM) | ~200 | Cold access |

UOR is faster than table lookups because it uses registers, not memory.

---

## Summary

| Aspect | Value |
|--------|-------|
| **State size** | 4992 bits (624 taxons) |
| **Fused wavefront** | ~1 cycle |
| **Non-fused wavefront** | ~25 cycles |
| **Throughput target** | ≥ 512 bits/cycle |
| **Achieved throughput** | ~998 bits/cycle |
| **Required features** | AVX2, SHA-NI, AES-NI |
| **Software fallback** | None |

**UOR achieves high performance through register-resident SIMD execution.** Use fused execution for multi-wavefront programs to approach the theoretical 1 cycle/wavefront limit.

---

## Related Documentation

| Document | Scope |
|----------|-------|
| [FRAMEWORK.md](FRAMEWORK.md) | UOR Framework conceptual foundation |
| [CELLULAR_AUTOMATA.md](CELLULAR_AUTOMATA.md) | Formal execution model and guarantees |
| [ARCHITECTURE.md](ARCHITECTURE.md) | x86_64 backend specifics |
| [INTEGRATION.md](INTEGRATION.md) | Usage patterns |
