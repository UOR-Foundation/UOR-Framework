//! UOR Invariance Frame — Type declarations and irreducibility partitions.
//!
//! Implements the type layer from the SIH formal specification:
//! a type declaration `T = (A, ⊗, ε)` selects a carrier set, binary
//! operation, and identity element from the ambient ring ℤ/(2⁸)ℤ.
//! The partition map classifies every datum as irreducible, reducible,
//! unit, or external.
//!
//! # Memory Budget
//!
//! | Component | Bytes |
//! |-----------|-------|
//! | DatumSet  | 32    |
//! | TypeDeclaration | 40 |
//! | Partition  | 128  |
//! | PRIMES_Q0  | 32   |
//! | GF2_IRREDUCIBLES_Q0 | 32 |
//! | GF3_IRREDUCIBLES_Q0 | 32 |
//! | GF5_IRREDUCIBLES_Q0 | 32 |
//! | **Total**  | **432** |
//!
//! All fits within L1 alongside the existing ~32KB LUT tables.
//!
//! # Example
//!
//! ```
//! use uor::frame::{TypeDeclaration, Partition};
//!
//! let t2 = TypeDeclaration::integer_mul();
//! let partition = Partition::compute(&t2);
//! assert_eq!(partition.irr().len(), 54); // 54 primes < 256
//! ```

pub mod gf2;
pub mod gf3;
pub mod gf5;

mod partition;
pub use partition::Partition;

mod distance;
pub use distance::{partition_distance, stratum_distance, stratum_histogram};

mod emanation;
pub use emanation::emanation;

mod observable;
pub use observable::{
    cascade_signature, catastrophe_signature, curvature_signature, hamming_metric_signature,
    holonomy_signature, stratum_signature, ObservableFamily, ObservableSignature,
};

mod certificate;
pub use certificate::{CertificateAddress, DatumClass, TransformCertificate};

mod embedding;
pub use embedding::{embed, embedding_distance, is_nondegenerate, EMBED_DIM};

mod cross_field;
pub use cross_field::{
    partition_distance_matrix, stratum_distance_matrix, TYPE_COUNT, TYPE_LABELS,
};

mod stratum_classify;
pub use stratum_classify::{
    closure_ratio, verify_commutative, zero_divisor_count, AlgebraicStratum,
};

mod type_graph;
pub use type_graph::{TypeEdge, TypeGraph};

mod registry;
pub use registry::{RegistryIter, TypeRegistry};

mod resolver;
pub use resolver::{EuclideanResolver, ExhaustiveResolver, Resolver, StratumDispatch};

mod assembly;
pub use assembly::Frame;

mod alignment;
pub use alignment::{all_alignments, Alignment};

mod multimodal;
pub use multimodal::{MultiModalBatch, MultiModalResult};

// ============================================================================
// DatumSet — 256-bit bitset for subsets of Q0
// ============================================================================

/// A 256-bit bitset representing a subset of Q0 datum values (0–255).
///
/// Each bit position corresponds to a datum value. Operations are O(1)
/// via bitwise operations on four `u64` words.
///
/// Size: 32 bytes, aligned to 32 bytes for potential SIMD.
///
/// # Examples
///
/// ```
/// use uor::frame::DatumSet;
///
/// let set = DatumSet::EMPTY.insert(2).insert(3).insert(5);
/// assert!(set.contains(3));
/// assert!(!set.contains(4));
/// assert_eq!(set.len(), 3);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(align(32))]
pub struct DatumSet([u64; 4]);

impl DatumSet {
    /// Empty set.
    pub const EMPTY: Self = Self([0; 4]);

    /// Full set (all 256 values).
    pub const FULL: Self = Self([u64::MAX; 4]);

    /// Set containing a single value.
    #[inline]
    pub const fn singleton(value: u8) -> Self {
        Self::EMPTY.insert(value)
    }

    /// Insert a value into the set.
    #[inline]
    pub const fn insert(mut self, value: u8) -> Self {
        let word = (value >> 6) as usize;
        let bit = value & 63;
        self.0[word] |= 1u64 << bit;
        self
    }

    /// Remove a value from the set.
    #[inline]
    pub const fn remove(mut self, value: u8) -> Self {
        let word = (value >> 6) as usize;
        let bit = value & 63;
        self.0[word] &= !(1u64 << bit);
        self
    }

    /// Test membership.
    #[inline]
    pub const fn contains(&self, value: u8) -> bool {
        let word = (value >> 6) as usize;
        let bit = value & 63;
        (self.0[word] >> bit) & 1 == 1
    }

    /// Population count.
    #[inline]
    pub const fn len(&self) -> u32 {
        self.0[0].count_ones()
            + self.0[1].count_ones()
            + self.0[2].count_ones()
            + self.0[3].count_ones()
    }

    /// Check if empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0[0] == 0 && self.0[1] == 0 && self.0[2] == 0 && self.0[3] == 0
    }

    /// Symmetric difference (XOR).
    #[inline]
    pub const fn symmetric_difference(&self, other: &Self) -> Self {
        Self([
            self.0[0] ^ other.0[0],
            self.0[1] ^ other.0[1],
            self.0[2] ^ other.0[2],
            self.0[3] ^ other.0[3],
        ])
    }

    /// Intersection (AND).
    #[inline]
    pub const fn intersection(&self, other: &Self) -> Self {
        Self([
            self.0[0] & other.0[0],
            self.0[1] & other.0[1],
            self.0[2] & other.0[2],
            self.0[3] & other.0[3],
        ])
    }

    /// Union (OR).
    #[inline]
    pub const fn union(&self, other: &Self) -> Self {
        Self([
            self.0[0] | other.0[0],
            self.0[1] | other.0[1],
            self.0[2] | other.0[2],
            self.0[3] | other.0[3],
        ])
    }

    /// Complement (NOT).
    #[inline]
    pub const fn complement(&self) -> Self {
        Self([!self.0[0], !self.0[1], !self.0[2], !self.0[3]])
    }

    /// Difference (self AND NOT other).
    #[inline]
    pub const fn difference(&self, other: &Self) -> Self {
        Self([
            self.0[0] & !other.0[0],
            self.0[1] & !other.0[1],
            self.0[2] & !other.0[2],
            self.0[3] & !other.0[3],
        ])
    }

    /// Check if two sets are disjoint.
    #[inline]
    pub const fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }

    /// Check if `self` is a subset of `other`.
    #[inline]
    pub const fn is_subset(&self, other: &Self) -> bool {
        self.difference(other).is_empty()
    }

    /// Inclusive range `[lo, hi]`.
    pub const fn from_range(lo: u8, hi: u8) -> Self {
        let mut set = Self::EMPTY;
        let mut v = lo as u16;
        while v <= hi as u16 {
            set = set.insert(v as u8);
            v += 1;
        }
        set
    }

    /// Iterate over set members in ascending order.
    pub fn iter(&self) -> DatumSetIter {
        DatumSetIter {
            words: self.0,
            word_idx: 0,
        }
    }

    /// Raw words (for testing/debugging).
    pub const fn words(&self) -> &[u64; 4] {
        &self.0
    }
}

impl core::fmt::Debug for DatumSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DatumSet({})", self.len())
    }
}

/// Iterator over `DatumSet` members in ascending order.
pub struct DatumSetIter {
    words: [u64; 4],
    word_idx: usize,
}

impl Iterator for DatumSetIter {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        while self.word_idx < 4 {
            let w = self.words[self.word_idx];
            if w != 0 {
                let bit = w.trailing_zeros();
                self.words[self.word_idx] &= !(1u64 << bit);
                return Some((self.word_idx as u32 * 64 + bit) as u8);
            }
            self.word_idx += 1;
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining: u32 = self.words[self.word_idx..]
            .iter()
            .map(|w| w.count_ones())
            .sum();
        (remaining as usize, Some(remaining as usize))
    }
}

impl ExactSizeIterator for DatumSetIter {}

// ============================================================================
// BinaryOp — enum dispatch for type operations (0 bytes static data)
// ============================================================================

/// Binary operation for a type declaration.
///
/// Operations are computed inline — no lookup tables, no cache pressure.
/// Each variant maps to a concrete O(1) function.
///
/// # Memory
///
/// Zero bytes of static data. Pure computation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    /// Integer multiplication: `a * b`.
    ///
    /// Used for T₂ (integer primes). Product may exceed u8.
    IntegerMul,

    /// Carryless polynomial multiplication over GF(2).
    ///
    /// Used for T_poly(2). XOR-based, no carries. Product may exceed u8.
    PolyGf2Mul,

    /// Polynomial multiplication over GF(3).
    ///
    /// Used for T_poly(3). Base-3 encoded coefficients. Product may exceed u8.
    PolyGf3Mul,

    /// Polynomial multiplication over GF(5).
    ///
    /// Used for T_poly(5). Base-5 encoded coefficients. Product may exceed u8.
    PolyGf5Mul,
}

impl BinaryOp {
    /// Apply the operation. Returns `u16` because products can exceed u8.
    #[inline]
    pub const fn apply(self, a: u8, b: u8) -> u16 {
        match self {
            Self::IntegerMul => a as u16 * b as u16,
            Self::PolyGf2Mul => gf2::mul(a, b),
            Self::PolyGf3Mul => gf3::mul(a, b),
            Self::PolyGf5Mul => gf5::mul(a, b),
        }
    }

    /// Look up the precomputed irreducible bitset for this operation.
    ///
    /// Returns the compile-time precomputed `DatumSet` of all irreducible
    /// elements in Q0 under this operation. This avoids runtime trial division.
    #[inline]
    pub fn precomputed_irreducibles(&self) -> &'static DatumSet {
        match self {
            Self::IntegerMul => &PRIMES_Q0,
            Self::PolyGf2Mul => &GF2_IRREDUCIBLES_Q0,
            Self::PolyGf3Mul => &GF3_IRREDUCIBLES_Q0,
            Self::PolyGf5Mul => &GF5_IRREDUCIBLES_Q0,
        }
    }

    /// Check if `divisor` divides `value` under this operation.
    ///
    /// Returns `Some(quotient)` if divisible, `None` otherwise.
    #[inline]
    pub const fn trial_divide(self, value: u8, divisor: u8) -> Option<u8> {
        match self {
            Self::IntegerMul => {
                if divisor == 0 {
                    return None;
                }
                if value.is_multiple_of(divisor) {
                    Some(value / divisor)
                } else {
                    None
                }
            }
            Self::PolyGf2Mul => gf2::trial_divide(value, divisor),
            Self::PolyGf3Mul => gf3::trial_divide(value, divisor),
            Self::PolyGf5Mul => gf5::trial_divide(value, divisor),
        }
    }
}

// ============================================================================
// TypeDeclaration — T = (A, ⊗, ε), 40 bytes
// ============================================================================

/// A type declaration `T = (A, ⊗, ε)` over the ambient ring ℤ/(2⁸)ℤ.
///
/// Selects a carrier set `A`, a binary operation `⊗`, and an identity
/// element `ε`. The type does not create structure — it selects a view
/// of the frame's pre-existing structure.
///
/// # Memory
///
/// 40 bytes: 32 (carrier) + 1 (op) + 1 (identity) + 6 (padding).
///
/// # Examples
///
/// ```
/// use uor::frame::TypeDeclaration;
///
/// let t2 = TypeDeclaration::integer_mul();
/// assert_eq!(t2.carrier().len(), 254); // {2..=255}
/// assert_eq!(t2.identity(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeDeclaration {
    carrier: DatumSet,
    op: BinaryOp,
    identity: u8,
}

impl TypeDeclaration {
    /// Construct a type declaration from components.
    pub const fn new(carrier: DatumSet, op: BinaryOp, identity: u8) -> Self {
        Self {
            carrier,
            op,
            identity,
        }
    }

    /// Integer multiplication type T₂.
    ///
    /// Carrier = {2..=255}, operation = integer multiplication, identity = 1.
    /// Irreducibles under this type are exactly the primes < 256.
    pub const fn integer_mul() -> Self {
        Self {
            carrier: DatumSet::from_range(2, 255),
            op: BinaryOp::IntegerMul,
            identity: 1,
        }
    }

    /// GF(2) polynomial multiplication type T_poly(2).
    ///
    /// Carrier = monic polynomials of degree ≥ 1 = {2..=255},
    /// operation = carryless polynomial multiplication,
    /// identity = 1.
    pub const fn poly_gf2() -> Self {
        Self {
            carrier: DatumSet::from_range(2, 255),
            op: BinaryOp::PolyGf2Mul,
            identity: 1,
        }
    }

    /// GF(3) polynomial multiplication type T_poly(3).
    ///
    /// Carrier = polynomials of degree ≥ 1 in base-3 encoding = {3..=255},
    /// operation = polynomial multiplication over GF(3),
    /// identity = 1.
    ///
    /// Values 0, 1, 2 are constants (degree 0) and excluded from the carrier.
    pub const fn poly_gf3() -> Self {
        Self {
            carrier: DatumSet::from_range(3, 255),
            op: BinaryOp::PolyGf3Mul,
            identity: 1,
        }
    }

    /// GF(5) polynomial multiplication type T_poly(5).
    ///
    /// Carrier = polynomials of degree ≥ 1 in base-5 encoding = {5..=255},
    /// operation = polynomial multiplication over GF(5),
    /// identity = 1.
    ///
    /// Values 0–4 are constants (degree 0) and excluded from the carrier.
    pub const fn poly_gf5() -> Self {
        Self {
            carrier: DatumSet::from_range(5, 255),
            op: BinaryOp::PolyGf5Mul,
            identity: 1,
        }
    }

    /// Carrier set.
    #[inline]
    pub const fn carrier(&self) -> &DatumSet {
        &self.carrier
    }

    /// Carrier size.
    #[inline]
    pub const fn carrier_len(&self) -> u32 {
        self.carrier.len()
    }

    /// Test carrier membership.
    #[inline]
    pub const fn contains(&self, value: u8) -> bool {
        self.carrier.contains(value)
    }

    /// Identity element.
    #[inline]
    pub const fn identity(&self) -> u8 {
        self.identity
    }

    /// Binary operation.
    #[inline]
    pub const fn op(&self) -> BinaryOp {
        self.op
    }

    /// Check if `value` is a unit (has a multiplicative inverse in the carrier).
    pub fn is_unit(&self, value: u8) -> bool {
        if !self.carrier.contains(value) {
            return false;
        }
        for candidate in self.carrier.iter() {
            let product = self.op.apply(value, candidate);
            if product == self.identity as u16 {
                return true;
            }
        }
        false
    }
}

// ============================================================================
// Pre-computed const statics
// ============================================================================

/// Pre-computed prime bitset for Q0 (54 primes < 256).
///
/// Size: 32 bytes. Computed at compile time via sieve of Eratosthenes.
///
/// # Examples
///
/// ```
/// use uor::frame::PRIMES_Q0;
///
/// assert!(PRIMES_Q0.contains(2));
/// assert!(PRIMES_Q0.contains(251));
/// assert!(!PRIMES_Q0.contains(4));
/// assert_eq!(PRIMES_Q0.len(), 54);
/// ```
pub static PRIMES_Q0: DatumSet = {
    let mut sieve = [true; 256];
    sieve[0] = false;
    sieve[1] = false;
    let mut i = 2usize;
    while i < 256 {
        if sieve[i] {
            let mut j = i * 2;
            while j < 256 {
                sieve[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let mut set = DatumSet::EMPTY;
    let mut v = 0u16;
    while v < 256 {
        if sieve[v as usize] {
            set = set.insert(v as u8);
        }
        v += 1;
    }
    set
};

/// Pre-computed GF(2) irreducible polynomial bitset for Q0 (41 irreducibles).
///
/// Size: 32 bytes. Computed at compile time.
///
/// # Examples
///
/// ```
/// use uor::frame::GF2_IRREDUCIBLES_Q0;
///
/// assert!(GF2_IRREDUCIBLES_Q0.contains(7));   // x²+x+1
/// assert!(GF2_IRREDUCIBLES_Q0.contains(25));  // x⁴+x³+1
/// assert!(!GF2_IRREDUCIBLES_Q0.contains(5));  // x²+1 = (x+1)²
/// assert_eq!(GF2_IRREDUCIBLES_Q0.len(), 41);
/// ```
pub static GF2_IRREDUCIBLES_Q0: DatumSet = {
    let mut set = DatumSet::EMPTY;
    let mut v = 2u16;
    while v < 256 {
        if gf2::is_irreducible(v as u8) {
            set = set.insert(v as u8);
        }
        v += 1;
    }
    set
};

/// Pre-computed GF(3) irreducible polynomial bitset for Q0.
///
/// Size: 32 bytes. Computed at compile time via `gf3::is_irreducible()`.
///
/// Degrees 1–4: 6 + 6 + 16 + 36 = 64, plus partial degree-5 (values 243–255).
///
/// # Examples
///
/// ```
/// use uor::frame::GF3_IRREDUCIBLES_Q0;
///
/// assert!(GF3_IRREDUCIBLES_Q0.contains(4));   // x+1 (degree 1)
/// assert!(!GF3_IRREDUCIBLES_Q0.contains(9));  // x² = x·x (reducible)
/// assert!(GF3_IRREDUCIBLES_Q0.len() >= 64);
/// ```
pub static GF3_IRREDUCIBLES_Q0: DatumSet = {
    let mut set = DatumSet::EMPTY;
    let mut v = 2u16;
    while v < 256 {
        if gf3::is_irreducible(v as u8) {
            set = set.insert(v as u8);
        }
        v += 1;
    }
    set
};

/// Pre-computed GF(5) irreducible polynomial bitset for Q0.
///
/// Size: 32 bytes. Computed at compile time via `gf5::is_irreducible()`.
///
/// Degrees 1–2: 20 + 40 = 60, plus partial degree-3 (values 125–255).
///
/// # Examples
///
/// ```
/// use uor::frame::GF5_IRREDUCIBLES_Q0;
///
/// assert!(GF5_IRREDUCIBLES_Q0.contains(6));   // x+1 (degree 1)
/// assert!(!GF5_IRREDUCIBLES_Q0.contains(25)); // x² = x·x (reducible)
/// assert!(GF5_IRREDUCIBLES_Q0.len() >= 60);
/// ```
pub static GF5_IRREDUCIBLES_Q0: DatumSet = {
    let mut set = DatumSet::EMPTY;
    let mut v = 2u16;
    while v < 256 {
        if gf5::is_irreducible(v as u8) {
            set = set.insert(v as u8);
        }
        v += 1;
    }
    set
};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -- DatumSet tests --

    #[test]
    fn datum_set_empty() {
        assert!(DatumSet::EMPTY.is_empty());
        assert_eq!(DatumSet::EMPTY.len(), 0);
    }

    #[test]
    fn datum_set_insert_contains() {
        let set = DatumSet::EMPTY.insert(0).insert(42).insert(255);
        assert!(set.contains(0));
        assert!(set.contains(42));
        assert!(set.contains(255));
        assert!(!set.contains(1));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn datum_set_remove() {
        let set = DatumSet::EMPTY.insert(10).insert(20).remove(10);
        assert!(!set.contains(10));
        assert!(set.contains(20));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn datum_set_from_range() {
        let set = DatumSet::from_range(2, 255);
        assert_eq!(set.len(), 254);
        assert!(!set.contains(0));
        assert!(!set.contains(1));
        assert!(set.contains(2));
        assert!(set.contains(255));
    }

    #[test]
    fn datum_set_symmetric_difference() {
        let a = DatumSet::EMPTY.insert(1).insert(2).insert(3);
        let b = DatumSet::EMPTY.insert(2).insert(3).insert(4);
        let sd = a.symmetric_difference(&b);
        assert!(sd.contains(1));
        assert!(!sd.contains(2));
        assert!(!sd.contains(3));
        assert!(sd.contains(4));
        assert_eq!(sd.len(), 2);
    }

    #[test]
    fn datum_set_intersection() {
        let a = DatumSet::EMPTY.insert(1).insert(2);
        let b = DatumSet::EMPTY.insert(2).insert(3);
        let i = a.intersection(&b);
        assert_eq!(i.len(), 1);
        assert!(i.contains(2));
    }

    #[test]
    fn datum_set_iterator() {
        let set = DatumSet::EMPTY.insert(5).insert(200).insert(0).insert(64);
        let values: alloc::vec::Vec<u8> = set.iter().collect();
        assert_eq!(values, alloc::vec![0, 5, 64, 200]);
    }

    #[test]
    fn datum_set_full() {
        assert_eq!(DatumSet::FULL.len(), 256);
        for v in 0..=255u8 {
            assert!(DatumSet::FULL.contains(v));
        }
    }

    #[test]
    fn datum_set_disjoint_cover() {
        let a = DatumSet::from_range(0, 127);
        let b = DatumSet::from_range(128, 255);
        assert!(a.is_disjoint(&b));
        assert_eq!(a.union(&b), DatumSet::FULL);
    }

    // -- BinaryOp tests --

    #[test]
    fn integer_mul_apply() {
        assert_eq!(BinaryOp::IntegerMul.apply(6, 7), 42);
        assert_eq!(BinaryOp::IntegerMul.apply(16, 16), 256);
    }

    #[test]
    fn integer_trial_divide() {
        assert_eq!(BinaryOp::IntegerMul.trial_divide(42, 7), Some(6));
        assert_eq!(BinaryOp::IntegerMul.trial_divide(42, 5), None);
        assert_eq!(BinaryOp::IntegerMul.trial_divide(0, 5), Some(0));
    }

    #[test]
    fn poly_gf2_apply() {
        // (x+1)*(x+1) = x²+1 in GF(2)
        assert_eq!(BinaryOp::PolyGf2Mul.apply(3, 3), 5);
    }

    // -- TypeDeclaration tests --

    #[test]
    fn type_integer_mul() {
        let t = TypeDeclaration::integer_mul();
        assert_eq!(t.carrier_len(), 254);
        assert_eq!(t.identity(), 1);
        assert_eq!(t.op(), BinaryOp::IntegerMul);
        assert!(t.contains(2));
        assert!(!t.contains(1));
    }

    #[test]
    fn type_poly_gf2() {
        let t = TypeDeclaration::poly_gf2();
        assert_eq!(t.carrier_len(), 254);
        assert_eq!(t.identity(), 1);
        assert_eq!(t.op(), BinaryOp::PolyGf2Mul);
    }

    // -- Pre-computed statics tests --

    #[test]
    fn primes_q0_count() {
        assert_eq!(PRIMES_Q0.len(), 54);
    }

    #[test]
    fn primes_q0_spot_check() {
        for &p in &[
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251u8,
        ] {
            assert!(PRIMES_Q0.contains(p), "prime {} not in PRIMES_Q0", p);
        }
        assert!(!PRIMES_Q0.contains(4));
        assert!(!PRIMES_Q0.contains(9));
        assert!(!PRIMES_Q0.contains(100));
    }

    #[test]
    fn gf2_irreducibles_q0_count() {
        assert_eq!(GF2_IRREDUCIBLES_Q0.len(), 41);
    }

    #[test]
    fn gf2_irreducibles_q0_first_10() {
        let first_10: alloc::vec::Vec<u8> = GF2_IRREDUCIBLES_Q0.iter().take(10).collect();
        assert_eq!(first_10, alloc::vec![2, 3, 7, 11, 13, 19, 25, 31, 37, 41]);
    }

    // -- GF(3) precomputed table --

    #[test]
    fn gf3_irreducibles_q0_at_least_64() {
        assert!(
            GF3_IRREDUCIBLES_Q0.len() >= 64,
            "expected >= 64 GF(3) irreducibles, got {}",
            GF3_IRREDUCIBLES_Q0.len()
        );
    }

    #[test]
    fn gf3_irreducibles_q0_samples() {
        assert!(GF3_IRREDUCIBLES_Q0.contains(4)); // x+1
        assert!(!GF3_IRREDUCIBLES_Q0.contains(9)); // x² = x·x
        assert!(!GF3_IRREDUCIBLES_Q0.contains(0)); // zero
        assert!(!GF3_IRREDUCIBLES_Q0.contains(1)); // constant
    }

    #[test]
    fn gf3_irreducibles_q0_matches_partition() {
        let t3 = TypeDeclaration::poly_gf3();
        let p = Partition::compute(&t3);
        // The precomputed table should contain exactly the irreducibles
        // that Partition::compute finds (within the carrier set)
        for v in t3.carrier().iter() {
            assert_eq!(
                p.irr().contains(v),
                GF3_IRREDUCIBLES_Q0.contains(v),
                "mismatch at {v}"
            );
        }
    }

    // -- GF(5) precomputed table --

    #[test]
    fn gf5_irreducibles_q0_at_least_60() {
        assert!(
            GF5_IRREDUCIBLES_Q0.len() >= 60,
            "expected >= 60 GF(5) irreducibles, got {}",
            GF5_IRREDUCIBLES_Q0.len()
        );
    }

    #[test]
    fn gf5_irreducibles_q0_samples() {
        assert!(GF5_IRREDUCIBLES_Q0.contains(6)); // x+1
        assert!(!GF5_IRREDUCIBLES_Q0.contains(25)); // x² = x·x
        assert!(!GF5_IRREDUCIBLES_Q0.contains(0)); // zero
        assert!(!GF5_IRREDUCIBLES_Q0.contains(1)); // constant
    }

    #[test]
    fn gf5_irreducibles_q0_matches_partition() {
        let t5 = TypeDeclaration::poly_gf5();
        let p = Partition::compute(&t5);
        for v in t5.carrier().iter() {
            assert_eq!(
                p.irr().contains(v),
                GF5_IRREDUCIBLES_Q0.contains(v),
                "mismatch at {v}"
            );
        }
    }
}
