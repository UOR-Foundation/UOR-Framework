//! v0.2.2 W10 + W17: assertions about the `HostTypes` trait shape.
//!
//! `HostTypes` (introduced in v0.2.2 W10) is the narrow successor to the
//! v0.2.1 `Primitives` trait. It exposes exactly four associated types
//! that vary across host environments — `Decimal`, `DateTime`,
//! `HostString`, `WitnessBytes` — and the foundation-supplied
//! `DefaultHostTypes` selects `f64`/`i64`/`str`/`[u8]`. These tests
//! pin the exact shape so any drift requires a deliberate edit.

use uor_foundation::{DefaultHostTypes, HostTypes};

/// `DefaultHostTypes` must implement `HostTypes`.
fn require_host_types<H: HostTypes>() {
    let _ = core::marker::PhantomData::<H>;
}

#[test]
fn default_host_types_implements_host_types() {
    require_host_types::<DefaultHostTypes>();
}

#[test]
fn default_host_types_decimal_is_f64() {
    let _ = core::any::TypeId::of::<f64>();
    fn assert_eq_type<A: 'static, B: 'static>() -> bool {
        core::any::TypeId::of::<A>() == core::any::TypeId::of::<B>()
    }
    assert!(assert_eq_type::<
        <DefaultHostTypes as HostTypes>::Decimal,
        f64,
    >());
}

#[test]
fn default_host_types_date_time_is_i64() {
    fn assert_eq_type<A: 'static, B: 'static>() -> bool {
        core::any::TypeId::of::<A>() == core::any::TypeId::of::<B>()
    }
    assert!(assert_eq_type::<
        <DefaultHostTypes as HostTypes>::DateTime,
        i64,
    >());
}

#[test]
fn host_types_trait_is_publicly_implementable() {
    /// Downstream-style override: a marker that swaps `Decimal` to `f32`
    /// while keeping the other defaults.
    struct EmbeddedHost;
    impl HostTypes for EmbeddedHost {
        type Decimal = f32;
        type DateTime = i64;
        type HostString = str;
        type WitnessBytes = [u8];
    }
    require_host_types::<EmbeddedHost>();
}
