//! Property-based tests for microcode operations.
//!
//! Uses proptest to verify algebraic identities hold for all inputs.
//! These tests run against the scalar executor as the reference implementation.

use proptest::prelude::*;
use uor::microcode::{
    MicrocodeOps, MicrocodePrimitives, ScalarMicrocodeExecutor, ScalarPrimitives,
};

// =============================================================================
// Involution Properties
// =============================================================================

proptest! {
    /// neg is an involution: neg(neg(x)) = x
    #[test]
    fn prop_neg_involution(x: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.neg(p.neg(x)), x);
    }

    /// bnot is an involution: bnot(bnot(x)) = x
    #[test]
    fn prop_bnot_involution(x: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.bnot(p.bnot(x)), x);
    }

    /// Combined: neg and bnot are both involutions
    #[test]
    fn prop_both_involutions(x: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.neg(p.neg(x)), x, "neg is not an involution");
        prop_assert_eq!(p.bnot(p.bnot(x)), x, "bnot is not an involution");
    }
}

// =============================================================================
// Increment/Decrement Properties
// =============================================================================

proptest! {
    /// inc(dec(x)) = x (inverse relationship)
    #[test]
    fn prop_inc_dec_inverse(x: u32) {
        let exec = ScalarMicrocodeExecutor::new();
        let result = exec.inc(exec.dec(x));
        prop_assert_eq!(result, x);
    }

    /// dec(inc(x)) = x (inverse relationship, other direction)
    #[test]
    fn prop_dec_inc_inverse(x: u32) {
        let exec = ScalarMicrocodeExecutor::new();
        let result = exec.dec(exec.inc(x));
        prop_assert_eq!(result, x);
    }

    /// inc(x) = x + 1 (matches native add)
    #[test]
    fn prop_inc_is_add_one(x: u32) {
        let exec = ScalarMicrocodeExecutor::new();
        prop_assert_eq!(exec.inc(x), x.wrapping_add(1));
    }

    /// dec(x) = x - 1 (matches native sub)
    #[test]
    fn prop_dec_is_sub_one(x: u32) {
        let exec = ScalarMicrocodeExecutor::new();
        prop_assert_eq!(exec.dec(x), x.wrapping_sub(1));
    }
}

// =============================================================================
// Addition Properties
// =============================================================================

proptest! {
    /// add(a, 0) = a (identity element)
    #[test]
    fn prop_add_identity(a: u32) {
        let exec = ScalarMicrocodeExecutor::new();
        prop_assert_eq!(exec.add(a, 0), a);
    }

    /// add(0, a) = a (identity element, commutative check)
    #[test]
    fn prop_add_identity_left(a: u32) {
        let exec = ScalarMicrocodeExecutor::new();
        prop_assert_eq!(exec.add(0, a), a);
    }

    /// add(a, b) = add(b, a) (commutativity)
    #[test]
    fn prop_add_commutative(a: u32, b: u32) {
        let exec = ScalarMicrocodeExecutor::new();
        prop_assert_eq!(exec.add(a, b), exec.add(b, a));
    }

    /// add(a, b) matches native wrapping_add
    #[test]
    fn prop_add_matches_native(a: u32, b: u32) {
        let exec = ScalarMicrocodeExecutor::new();
        prop_assert_eq!(exec.add(a, b), a.wrapping_add(b));
    }

    /// add(a, neg(a)) = 0 (additive inverse)
    #[test]
    fn prop_add_inverse(a: u32) {
        let exec = ScalarMicrocodeExecutor::new();
        prop_assert_eq!(exec.add(a, exec.neg(a)), 0);
    }
}

// =============================================================================
// Subtraction Properties
// =============================================================================

proptest! {
    /// sub(a, 0) = a (identity)
    #[test]
    fn prop_sub_identity(a: u32) {
        let exec = ScalarMicrocodeExecutor::new();
        prop_assert_eq!(exec.sub(a, 0), a);
    }

    /// sub(a, a) = 0 (self-inverse)
    #[test]
    fn prop_sub_self(a: u32) {
        let exec = ScalarMicrocodeExecutor::new();
        prop_assert_eq!(exec.sub(a, a), 0);
    }

    /// sub(a, b) matches native wrapping_sub
    #[test]
    fn prop_sub_matches_native(a: u32, b: u32) {
        let exec = ScalarMicrocodeExecutor::new();
        prop_assert_eq!(exec.sub(a, b), a.wrapping_sub(b));
    }

    /// sub(a, b) = add(a, neg(b))
    #[test]
    fn prop_sub_is_add_neg(a: u32, b: u32) {
        let exec = ScalarMicrocodeExecutor::new();
        prop_assert_eq!(exec.sub(a, b), exec.add(a, exec.neg(b)));
    }
}

// =============================================================================
// Bitwise Operation Properties
// =============================================================================

proptest! {
    /// xor(a, a) = 0 (self-inverse)
    #[test]
    fn prop_xor_self_zero(a: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.xor(a, a), 0);
    }

    /// xor(a, 0) = a (identity)
    #[test]
    fn prop_xor_identity(a: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.xor(a, 0), a);
    }

    /// xor(a, b) = xor(b, a) (commutativity)
    #[test]
    fn prop_xor_commutative(a: u32, b: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.xor(a, b), p.xor(b, a));
    }

    /// and(a, a) = a (idempotent)
    #[test]
    fn prop_and_idempotent(a: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.and(a, a), a);
    }

    /// and(a, 0) = 0 (annihilator)
    #[test]
    fn prop_and_zero(a: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.and(a, 0), 0);
    }

    /// and(a, u32::MAX) = a (identity)
    #[test]
    fn prop_and_identity(a: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.and(a, u32::MAX), a);
    }

    /// and(a, b) = and(b, a) (commutativity)
    #[test]
    fn prop_and_commutative(a: u32, b: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.and(a, b), p.and(b, a));
    }

    /// or(a, a) = a (idempotent)
    #[test]
    fn prop_or_idempotent(a: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.or(a, a), a);
    }

    /// or(a, 0) = a (identity)
    #[test]
    fn prop_or_identity(a: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.or(a, 0), a);
    }

    /// or(a, u32::MAX) = u32::MAX (annihilator)
    #[test]
    fn prop_or_ones(a: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.or(a, u32::MAX), u32::MAX);
    }

    /// or(a, b) = or(b, a) (commutativity)
    #[test]
    fn prop_or_commutative(a: u32, b: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.or(a, b), p.or(b, a));
    }
}

// =============================================================================
// De Morgan's Laws
// =============================================================================

proptest! {
    /// De Morgan: bnot(and(a, b)) = or(bnot(a), bnot(b))
    #[test]
    fn prop_de_morgan_and(a: u32, b: u32) {
        let p = ScalarPrimitives;
        let lhs = p.bnot(p.and(a, b));
        let rhs = p.or(p.bnot(a), p.bnot(b));
        prop_assert_eq!(lhs, rhs);
    }

    /// De Morgan: bnot(or(a, b)) = and(bnot(a), bnot(b))
    #[test]
    fn prop_de_morgan_or(a: u32, b: u32) {
        let p = ScalarPrimitives;
        let lhs = p.bnot(p.or(a, b));
        let rhs = p.and(p.bnot(a), p.bnot(b));
        prop_assert_eq!(lhs, rhs);
    }
}

// =============================================================================
// Critical UOR Identity
// =============================================================================

proptest! {
    /// The critical UOR identity: neg(bnot(x)) = x + 1
    #[test]
    fn prop_uor_inc_identity(x: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.neg(p.bnot(x)), x.wrapping_add(1));
    }

    /// The critical UOR identity: bnot(neg(x)) = x - 1
    #[test]
    fn prop_uor_dec_identity(x: u32) {
        let p = ScalarPrimitives;
        prop_assert_eq!(p.bnot(p.neg(x)), x.wrapping_sub(1));
    }
}

// =============================================================================
// Associativity (requires 3 values)
// =============================================================================

proptest! {
    /// add(add(a, b), c) = add(a, add(b, c)) (associativity)
    #[test]
    fn prop_add_associative(a: u32, b: u32, c: u32) {
        let exec = ScalarMicrocodeExecutor::new();
        let lhs = exec.add(exec.add(a, b), c);
        let rhs = exec.add(a, exec.add(b, c));
        prop_assert_eq!(lhs, rhs);
    }

    /// xor(xor(a, b), c) = xor(a, xor(b, c)) (associativity)
    #[test]
    fn prop_xor_associative(a: u32, b: u32, c: u32) {
        let p = ScalarPrimitives;
        let lhs = p.xor(p.xor(a, b), c);
        let rhs = p.xor(a, p.xor(b, c));
        prop_assert_eq!(lhs, rhs);
    }

    /// and(and(a, b), c) = and(a, and(b, c)) (associativity)
    #[test]
    fn prop_and_associative(a: u32, b: u32, c: u32) {
        let p = ScalarPrimitives;
        let lhs = p.and(p.and(a, b), c);
        let rhs = p.and(a, p.and(b, c));
        prop_assert_eq!(lhs, rhs);
    }

    /// or(or(a, b), c) = or(a, or(b, c)) (associativity)
    #[test]
    fn prop_or_associative(a: u32, b: u32, c: u32) {
        let p = ScalarPrimitives;
        let lhs = p.or(p.or(a, b), c);
        let rhs = p.or(a, p.or(b, c));
        prop_assert_eq!(lhs, rhs);
    }
}

// =============================================================================
// Distributivity
// =============================================================================

proptest! {
    /// and(a, or(b, c)) = or(and(a, b), and(a, c)) (distributivity)
    #[test]
    fn prop_and_distributes_over_or(a: u32, b: u32, c: u32) {
        let p = ScalarPrimitives;
        let lhs = p.and(a, p.or(b, c));
        let rhs = p.or(p.and(a, b), p.and(a, c));
        prop_assert_eq!(lhs, rhs);
    }

    /// or(a, and(b, c)) = and(or(a, b), or(a, c)) (distributivity)
    #[test]
    fn prop_or_distributes_over_and(a: u32, b: u32, c: u32) {
        let p = ScalarPrimitives;
        let lhs = p.or(a, p.and(b, c));
        let rhs = p.and(p.or(a, b), p.or(a, c));
        prop_assert_eq!(lhs, rhs);
    }
}
