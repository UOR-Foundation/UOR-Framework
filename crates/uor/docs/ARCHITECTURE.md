# UOR Architecture

**x86_64 Backend Implementation**

---

## Overview

This document describes the **x86_64 backend** for the UOR Framework. For the conceptual foundation, see [FRAMEWORK.md](FRAMEWORK.md). For the formal execution model, see [CELLULAR_AUTOMATA.md](CELLULAR_AUTOMATA.md).

UOR is a **pure cellular automaton** where the entire CPU register file forms one combined state, and instructions are **wavefronts** that fire across all execution ports simultaneously.

```
State     = Entire register file (624 Taxons = 4992 bits)
Wavefront = All execution ports fire in one cycle
Step      = One wavefront transforms the state
```

There is no intermediate computation - only state transformation via wavefronts.

---

## Core Principles

### 1. Wavefront Execution Model

UOR treats computation as a cellular automaton:

```
┌─────────────┐    ┌────────────────┐    ┌─────────────┐
│ UorState    │───►│   Wavefront    │───►│ UorState'   │
│ (4992 bits) │    │ (all ports)    │    │ (4992 bits) │
└─────────────┘    └────────────────┘    └─────────────┘
                         │
              ┌──────────┼──────────┐
              ▼          ▼          ▼
           Port 0    Port 1    Port 5
         (Shift/Rot)  (ALU)     (ALU)
```

All three execution ports fire **simultaneously** in a single cycle. This is the wavefront.

### 2. The Braille Bijection

Each byte value (0-255) maps bijectively to a Unicode Braille codepoint (U+2800-U+28FF):

| Value | Codepoint | Glyph | IRI |
|-------|-----------|-------|-----|
| 0 | U+2800 | ⠀ | https://uor.foundation/u/U2800 |
| 1 | U+2801 | ⠁ | https://uor.foundation/u/U2801 |
| 42 | U+282A | ⠪ | https://uor.foundation/u/U282A |
| 255 | U+28FF | ⣿ | https://uor.foundation/u/U28FF |

**The Braille codepoint IS the identity.** Everything else (domain, rank, operations) is derived.

### 3. Triadic Domain Structure

From the Triality axiom (T=3), bytes partition into three domains:

| Domain | Symbol | Residue | Cardinality | Members |
|--------|--------|---------|-------------|---------|
| Theta | θ | 0 | 86 | 0, 3, 6, 9, ..., 255 |
| Psi | ψ | 1 | 85 | 1, 4, 7, 10, ..., 253 |
| Delta | δ | 2 | 85 | 2, 5, 8, 11, ..., 254 |

Domain classification: `domain(n) = n mod 3`
Rank within domain: `rank(n) = n div 3`

---

## State Architecture

### UorState

The entire register file is ONE combined entity:

```
┌─────────────────────────────────────────────────────────────┐
│                    UorState (4992 bits)                      │
├─────────────────────────────────────────────────────────────┤
│  YMM registers (16 × 256 = 4096 bits)                       │
│  ┌──────┬──────┬──────┬──────┬──────┬──────┬──────┬──────┐  │
│  │ ymm0 │ ymm1 │ ymm2 │ ymm3 │ ymm4 │ ymm5 │ ymm6 │ ymm7 │  │
│  ├──────┼──────┼──────┼──────┼──────┼──────┼──────┼──────┤  │
│  │ ymm8 │ ymm9 │ymm10 │ymm11 │ymm12 │ymm13 │ymm14 │ymm15 │  │
│  └──────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┘  │
├─────────────────────────────────────────────────────────────┤
│  GPR registers (14 × 64 = 896 bits)                         │
│  ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐         │
│  │ rax │ rbx │ rcx │ rdx │ rsi │ rdi │ r8  │ r9  │         │
│  ├─────┼─────┼─────┼─────┼─────┼─────┼─────┴─────┤         │
│  │ r10 │ r11 │ r12 │ r13 │ r14 │ r15 │           │         │
│  └─────┴─────┴─────┴─────┴─────┴─────┘           │         │
└─────────────────────────────────────────────────────────────┘
```

| Register Set | Count | Bits | Taxons |
|--------------|-------|------|--------|
| YMM0-15 | 16 | 4096 | 512 |
| GPRs | 14 | 896 | 112 |
| **Total** | 30 | 4992 | 624 |

Key properties:
- `UorState` is `Copy` (fits entirely in registers)
- 32-byte alignment for AVX2 compatibility
- No heap allocation during execution

### Taxon

The fundamental unit - a single byte with identity:

```
┌─────────────────────────────────────────┐
│                 Taxon                    │
├─────────────────────────────────────────┤
│ value: u8         (the byte, 0-255)     │
├─────────────────────────────────────────┤
│ codepoint()  →  0x2800 + value          │
│ braille()    →  char at codepoint       │
│ domain()     →  value mod 3             │
│ rank()       →  value div 3             │
│ iri()        →  https://uor.foundation/u/U28XX │
└─────────────────────────────────────────┘
```

Key property: `#[repr(transparent)]` - Taxon is exactly one byte.

### Word\<N\>

Multi-taxon containers for structured data:

```
Word<4> layout (4 bytes, big-endian):

Index:    [0]      [1]      [2]      [3]
        ┌────────┬────────┬────────┬────────┐
        │  MSB   │        │        │  LSB   │
        └────────┴────────┴────────┴────────┘
          byte 0   byte 1   byte 2   byte 3
```

Type aliases:
- `Word2` = `Word<2>` (16-bit container)
- `Word4` = `Word<4>` (32-bit container)
- `Word8` = `Word<8>` (64-bit container)
- `Word32` = `Word<32>` (256-bit container)

**Note:** Word types are data containers only. Operations occur at the wavefront level on the entire UorState.

---

## Wavefront Architecture

### WavefrontOp

Operations that execute on specific ports:

| Category | Operations | Port(s) |
|----------|------------|---------|
| ALU | `Xor`, `And`, `Or`, `Not`, `Add`, `Sub` | 1, 5 |
| Shift/Rotate | `RotL(n)`, `RotR(n)`, `ShL(n)`, `ShR(n)` | 0 |
| SHA-NI | `Sha256Round`, `Sha256Msg1`, `Sha256Msg2` | 0, 1, 5 |
| AES-NI | `AesRound`, `AesRoundDec` | 1, 5 |
| Permutation | `Shuffle`, `Permute` | 5 |

Full enumeration:

```rust
pub enum WavefrontOp {
    // Identity
    Nop,

    // ALU Operations (Ports 1/5)
    Xor, And, Or, Not, Add, Sub,

    // Shift/Rotate Operations (Port 0)
    RotL(u8), RotR(u8), ShL(u8), ShR(u8),

    // Crypto Operations
    Sha256Round,   // SHA-NI (Port 0)
    Sha256Msg1,    // SHA-NI message schedule (Ports 1/5)
    Sha256Msg2,    // SHA-NI message schedule (Ports 1/5)
    AesRound,      // AES-NI encrypt (Ports 1/5)
    AesRoundDec,   // AES-NI decrypt (Ports 1/5)

    // Permutation Operations (Port 5)
    Shuffle,       // Byte permute within 128-bit lanes
    Permute,       // 32-bit lane permute across 256-bit
}
```

### Port Assignment

A wavefront assigns one operation to each port:

```
┌─────────────────────────────────────────────────────────────┐
│                    PortAssignment                            │
├───────────────────┬───────────────────┬─────────────────────┤
│      Port 0       │      Port 1       │       Port 5        │
│  (Shift/Rotate)   │  (ALU/AES-NI)     │  (ALU/AES-NI/Perm)  │
├───────────────────┼───────────────────┼─────────────────────┤
│   RotR(7)         │      Xor          │        Xor          │
└───────────────────┴───────────────────┴─────────────────────┘
                              │
                    All ports fire in ONE cycle
```

Example patterns:

| Pattern | Port 0 | Port 1 | Port 5 | Use Case |
|---------|--------|--------|--------|----------|
| `all_xor()` | Nop | Xor | Xor | Self-inverse test |
| `rotate_and_xor(7)` | RotR(7) | Xor | Xor | SHA-256 Σ functions |
| `sha256_round()` | Sha256Round | Sha256Msg1 | Sha256Msg2 | Hardware SHA-256 |
| `aes_round()` | Nop | AesRound | AesRound | Hardware AES |

### Wavefront

A complete wavefront specification:

```rust
pub struct Wavefront {
    pub ports: PortAssignment,  // What each port does
    pub ymm_mask: u16,          // Which YMM registers participate
    pub gpr_mask: u16,          // Which GPRs participate
}
```

By default, all 16 YMM registers and all 14 GPRs participate in every wavefront.

---

## Execution Traits

### UorStep

The core step function:

```rust
pub trait UorStep: Send + Sync {
    /// Execute one wavefront cycle.
    unsafe fn step(&self, state: &mut UorState, wavefront: &Wavefront);

    /// Execute a sequence of wavefronts.
    unsafe fn run(&self, state: &mut UorState, program: &[Wavefront]);
}
```

### UorStepLossless

Extended trait for lossless codec operations:

```rust
pub trait UorStepLossless: UorStep {
    /// Execute with complement capture (for non-invertible ops).
    unsafe fn step_tracked(
        &self,
        state: &mut UorState,
        complement: &mut UorState,
        wavefront: &Wavefront,
    );

    /// Execute inverse using captured complement.
    unsafe fn step_inverse(
        &self,
        state: &mut UorState,
        complement: &UorState,
        wavefront: &Wavefront,
    );
}
```

Non-invertible operations (ShL, ShR, AND, OR) require complement tracking for lossless encode/decode.

### UorStepFused

Register-resident execution for maximum performance:

```rust
pub trait UorStepFused: UorStep {
    /// Execute program with state in registers (1 cycle/wavefront amortized).
    unsafe fn run_fused(&self, state: &mut UorState, program: &[Wavefront]);
}
```

Fused execution:
- Load state into registers **once** at program start
- Execute all wavefronts with no memory access
- Store state **once** at program end

This achieves ~1 cycle per wavefront for large programs.

---

## Hardware Target: AMD Zen 3

### Port Capabilities

| Port | Capabilities | Example Instructions |
|------|--------------|---------------------|
| Port 0 | Shift, Rotate, SHA-NI | `vpsrld`, `vpslld`, `sha256rnds2` |
| Port 1 | ALU, AES-NI | `vpxor`, `vpand`, `aesenc` |
| Port 5 | ALU, AES-NI, Shuffle, Permute | `vpxor`, `vpand`, `aesenc`, `vperm2i128` |

### Required CPU Features

| Feature | Purpose | Fallback |
|---------|---------|----------|
| AVX2 | 256-bit SIMD operations | **None** |
| SHA-NI | SHA-256 hardware acceleration | **None** |
| AES-NI | AES hardware acceleration | **None** |

**UOR has NO software fallback.** Missing features constitute a conformance violation.

---

## Safety Contract

### Zero Spillage Guarantee

All wavefront execution uses:

```rust
asm!(
    // ... intrinsics ...
    options(nomem, nostack)
)
```

This guarantees:
- **No stack access** during wavefront execution
- **No memory access** outside the state parameter
- **Deterministic execution** (no data-dependent branches)

### Invariants

1. `UorState` MUST remain `Copy`
2. No operation on `UorState` may access heap
3. All transformations must complete in bounded time
4. Same input + wavefront = same output (determinism)

---

## Conformance Targets

| Criterion | Target | Description |
|-----------|--------|-------------|
| Single Wavefront | < 5 cycles | Individual operation latency |
| 64-Wavefront Sequence | < 200 cycles | Program throughput |
| Throughput | ≥ 512 bits/cycle | Sustained bandwidth |

State size: 4992 bits
Target wavefront latency: 5 cycles
Minimum throughput: 4992 / 5 = 998 bits/cycle (exceeds 512 target)

---

## Design Decisions

### Why Braille?

1. **Bijective**: Exactly 256 codepoints (U+2800-U+28FF) for 256 byte values
2. **Stable**: Unicode standard, will never change
3. **Visual**: Each taxon has a unique glyph representation
4. **Semantic**: Enables IRI-based addressing (https://uor.foundation/u/U28XX)

### Why Big-Endian Word Storage?

1. **Natural indexing**: `word[0]` is most significant byte
2. **Consistent with network byte order**
3. **Matches SHA-256 specification**
4. **Simplifies rotation/shift implementations**

### Why Register-File State?

1. **No memory latency**: All operations are register-to-register
2. **Maximum parallelism**: All ports fire simultaneously
3. **Predictable timing**: No cache misses possible
4. **Copy semantics**: State can be copied cheaply for lossless tracking

---

## Axiom Derivation

All constants derive from two axioms:

| Axiom | Value | Meaning |
|-------|-------|---------|
| **T** | 3 | Triality - prime order of symmetry |
| **O** | 8 | Octonion dimension |

Derived constants:

| Constant | Formula | Value | Purpose |
|----------|---------|-------|---------|
| BYTE_CARDINALITY | 2^O | 256 | Total taxon count |
| BRAILLE_BASE | - | 0x2800 | Unicode Braille start |
| DOMAIN_COUNT | T | 3 | Number of domains |
| DOMAIN_CARDINALITIES | - | [86,85,85] | Taxons per domain |

---

## Summary

This x86_64 backend implements the UOR cellular automaton:

1. **Treats the register file as state**: 624 taxons = 4992 bits
2. **Fires all ports simultaneously**: Maximum hardware utilization
3. **Provides zero-spillage execution**: No memory access during wavefronts
4. **Achieves ~1 cycle/wavefront**: With fused execution model
5. **Supports SHA-256/AES via hardware**: SHA-NI and AES-NI integration

---

## Related Documentation

| Document | Scope |
|----------|-------|
| [FRAMEWORK.md](FRAMEWORK.md) | UOR Framework conceptual foundation |
| [CELLULAR_AUTOMATA.md](CELLULAR_AUTOMATA.md) | Formal execution model and guarantees |
| [INTEGRATION.md](INTEGRATION.md) | Usage patterns for this implementation |
| [PERFORMANCE.md](PERFORMANCE.md) | Benchmarks and conformance targets |
