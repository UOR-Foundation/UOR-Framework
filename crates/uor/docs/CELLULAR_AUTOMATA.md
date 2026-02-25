# UOR Cellular Automata

**Formalization of the UOR Execution Model**

---

## Overview

UOR executes as a **cellular automaton** where:
- **State** is the device's register file (or equivalent)
- **Transition** is a wavefront (all ports fire simultaneously)
- **Time** is measured in processor clock cycles

This document formalizes the cellular automaton model and its execution guarantees.

---

## The Coherence-Norm

### Fundamental Invariant

**Every type and every operation decomposes to the coherence-norm of the processor's clock.**

The coherence-norm is the processor's clock cycle—the indivisible quantum of computational time. UOR guarantees that:

1. Every wavefront executes in **exactly one clock cycle**
2. No operation takes **more** than one cycle
3. No operation **waits** (no memory latency, cache miss, or branch delay)
4. All ports execute in **lockstep** with the clock

This invariant is the foundation of UOR's timing guarantees.

### Why Clock Coherence Matters

Traditional computation has variable timing:
- Memory access: 4-200+ cycles depending on cache level
- Branch misprediction: 10-20 cycles penalty
- Data-dependent operations: timing varies with input values

UOR eliminates this variance by:
- Keeping all state in registers (no memory access)
- Using only port operations (no branches)
- Computing operation sequences at compile time (no runtime decisions)

The result: **deterministic, predictable, constant-time execution**.

---

## State Formalization

### Definition

```
State S = (R, W)
where:
  R = contents of the register file
  W = total width in bits (determined by backend)
```

The state encompasses the **entire register file** of the device. Nothing exists outside the state during execution.

### Backend-Specific State Width

| Backend | State Width | Composition |
|---------|-------------|-------------|
| x86_64 (AVX2) | 4992 bits | 16×256-bit YMM + 14×64-bit GPR |
| aarch64 (NEON) | 4096+ bits | 32×128-bit V registers + GPRs |
| aarch64 (SVE) | Variable | Scalable vector length |
| GPU | Per-thread | Thread-local register allocation |
| TPU | Matrix-sized | Systolic array registers |

### State Properties

1. **Bounded**: State width is fixed for a given backend
2. **Complete**: All computation occurs within the state
3. **Isolated**: No external memory access during execution
4. **Copyable**: State can be duplicated for lossless tracking

---

## Wavefront Formalization

### Definition

```
Wavefront W = (P₀, P₁, ..., Pₙ₋₁)
where:
  n = number of execution ports
  Pᵢ = operation assigned to port i
  All Pᵢ execute simultaneously
  Duration = 1 clock cycle
```

A wavefront is a **parallel operation tuple**. All ports fire in the same clock cycle, executing their assigned operations on the state.

### Step Function

```
step: State × Wavefront → State
step(S, W) = S'

Properties:
  - step completes in exactly 1 clock cycle
  - step is deterministic: same S and W always produce same S'
  - step has no side effects outside S'
```

### Program Execution

A program is a sequence of wavefronts:

```
Program P = [W₀, W₁, ..., Wₘ₋₁]

run(S, P) = step(step(...step(S, W₀), W₁)..., Wₘ₋₁)

Execution time = m clock cycles (exactly)
```

---

## Type Decomposition Theorem

### Statement

For any type T and operation Op:

```
Op(T) decomposes to [W₀, W₁, ..., Wₙ₋₁]
where:
  n = ⌈bits(T) / port_width⌉
  each Wᵢ executes in exactly 1 clock cycle
  n is computed at compile time
```

### Implications

1. **Compile-time cost**: The wavefront count n is known before execution
2. **Linear scaling**: Larger types require proportionally more wavefronts
3. **Fixed per-type**: All operations on type T have deterministic cost
4. **No runtime variance**: Execution time depends only on type, not values

### Examples

| Type | Port Width | Wavefronts for XOR |
|------|------------|-------------------|
| u8 | 256-bit | 1 |
| u32 | 256-bit | 1 |
| u256 | 256-bit | 1 |
| u512 | 256-bit | 2 |
| u1024 | 256-bit | 4 |

---

## Zero Spillage Invariant

### Statement

**No memory access occurs during wavefront execution.**

All state resides in registers/ports. Memory is accessed only at program boundaries:
- **Load**: Before first wavefront (move data into registers)
- **Store**: After last wavefront (move data out of registers)

### Guarantees

| Property | Guarantee |
|----------|-----------|
| Cache variance | Eliminated (no cache access) |
| Memory latency | Eliminated (no memory access) |
| Side-channel timing | Eliminated (constant-time) |
| Throughput ceiling | Maximized (register bandwidth only) |

### Enforcement

Backends must ensure zero spillage through:
- Inline assembly with `nomem` constraints
- Register allocation that fits state width
- Compile-time verification of register usage

---

## Fused Execution Model

### Non-Fused Execution

```
for each wavefront W in program:
    load state from memory
    execute W (1 cycle)
    store state to memory

Cost = n × (load + 1 + store)
     ≈ n × 25 cycles (typical)
```

### Fused Execution

```
load state from memory (once)
for each wavefront W in program:
    execute W (1 cycle)
store state to memory (once)

Cost = load + n + store
     ≈ 16 + n + 8 cycles (typical)
```

### Amortization

| Program Size | Non-Fused | Fused | Cycles/Wavefront |
|--------------|-----------|-------|------------------|
| 1 wavefront | 25 | 25 | 25.0 |
| 10 wavefronts | 250 | 34 | 3.4 |
| 64 wavefronts | 1600 | 88 | 1.4 |
| 1000 wavefronts | 25000 | 1024 | 1.0 |

For large programs, fused execution approaches **1 cycle per wavefront**.

---

## Backend Contract

A conforming UOR backend must implement:

### 1. Port Declaration

```
struct Backend {
    port_count: usize,
    port_width: usize,  // bits
    port_capabilities: Vec<OperationSet>,
}
```

### 2. Operation Mapping

```
fn map_operation(op: UorOp, port: usize) -> NativeInstruction;
```

Each UOR operation maps to a native instruction (or sequence) for each capable port.

### 3. State Mapping

```
fn load_state(memory: &[u8]) -> BackendState;
fn store_state(state: &BackendState, memory: &mut [u8]);
```

State transfers between memory and backend-native representation.

### 4. Step Implementation

```
fn step(state: &mut BackendState, wavefront: &Wavefront);
```

Execute one wavefront in exactly one clock cycle.

---

## Conformance Criteria

A backend conforms to UOR if it satisfies:

| Criterion | Requirement |
|-----------|-------------|
| **Wavefront duration** | Exactly 1 clock cycle |
| **Memory access** | Zero during step() |
| **Timing determinism** | No data-dependent variance |
| **Port utilization** | All declared ports fire per wavefront |
| **State isolation** | No side effects outside state |

### Verification

Conformance can be verified by:
1. **Cycle counting**: Measure rdtsc/cntvct around step()
2. **Memory tracing**: Verify no loads/stores during execution
3. **Statistical analysis**: Confirm timing variance is zero
4. **Formal verification**: Prove invariants on backend implementation

---

## Formal Properties

### Determinism

```
∀ S, W: step(S, W) is unique
```

The same state and wavefront always produce the same result.

### Composability

```
step(step(S, W₁), W₂) = run(S, [W₁, W₂])
```

Sequential wavefronts compose into programs.

### Timing Linearity

```
time(run(S, P)) = |P| clock cycles
```

Program execution time is exactly the wavefront count.

### Invertibility (for invertible operations)

```
∀ invertible W: ∃ W⁻¹: step(step(S, W), W⁻¹) = S
```

Invertible wavefronts can be reversed.

### Lossless Tracking (for non-invertible operations)

```
∀ W: step_tracked(S, W) = (S', C)
     step_inverse(S', C, W) = S
```

Non-invertible operations track complements for lossless reversal.

---

## Summary

UOR Cellular Automata formalizes execution as:

1. **State**: The complete register file, fixed width per backend
2. **Wavefront**: Parallel operation tuple, exactly 1 clock cycle
3. **Step**: Deterministic state transformation
4. **Program**: Sequence of wavefronts, linear time cost

The **coherence-norm** guarantees that all computation reduces to clock cycles. The **zero spillage invariant** ensures deterministic timing. **Fused execution** amortizes overhead for multi-wavefront programs.

These properties make UOR suitable for:
- Cryptographic operations (constant-time requirement)
- High-throughput processing (maximum port utilization)
- Timing-critical systems (predictable execution)
- Formal verification (deterministic semantics)
