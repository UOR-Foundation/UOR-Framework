// @codegen-exempt — hand-written blanket impls for Path-3 classes.
// See docs/orphan-closure/completion-plan.md §Phase 11 for the full
// allow-list, supertrait-closure rule, and emission contract. The
// emit::write_file banner check (Phase 11c) preserves this file
// across `uor-crate` regeneration runs.
//
// Each Path-3-allow-listed trait gets a blanket impl on
// `Validated<T, Phase>` that delegates to a foundation primitive in
// `enforcement.rs` or `pipeline.rs`. The supertrait `Observable<H>`
// and any intermediate marker traits (e.g. `ThermoObservable<H>`)
// receive matching blanket impls so the trait stack closes for
// every consumer that holds a `Validated<T, Phase>` and reaches for
// the ontology-level observable surface.
//
// Coherence with Phase 7 + Phase 8:
//   - `impl {Foo}<H> for Null{Foo}<H>`           — Phase 7 (resolver-absent)
//   - `impl {Foo}<H> for Resolved{Foo}<'r, R, H>` — Phase 8 (content-addressed)
//   - `impl {Foo}<H> for Validated<T, Phase>`     — Phase 11 (primitive-backed)
// `Null{Foo}<H>`, `Resolved{Foo}<'r, R, H>`, and `Validated<T, Phase>`
// are mutually disjoint concrete types, so each impl closes the orphan
// without overlapping.

#![allow(clippy::module_name_repetitions)]

use crate::bridge::derivation::DerivationDepthObservable;
use crate::bridge::observable::{JacobianObservable, LandauerBudget, Observable, ThermoObservable};
use crate::bridge::partition::FreeRankObservable;
use crate::enforcement::{Validated, ValidationPhase};
use crate::enums::MeasurementUnit;
use crate::kernel::carry::CarryDepthObservable;
use crate::pipeline::ConstrainedTypeShape;
use crate::{DecimalTranscendental, HostTypes};

// ── Observable<H> blanket — supertrait of every Path-3 leaf trait ──
//
// `value()` returns `H::EMPTY_DECIMAL` as a generic placeholder. The
// kind-specific value is provided by each leaf trait's primitive-backed
// method (e.g. `LandauerBudget::landauer_nats`); Observable's `value`
// is satisfied by the marker default because `Validated<T, Phase>` is
// not pinned to a single observable kind. Source/target are the
// host-defined empty references; `has_unit` returns the enum default.
impl<T, Phase, H> Observable<H> for Validated<T, Phase>
where
    T: ConstrainedTypeShape,
    Phase: ValidationPhase,
    H: HostTypes,
{
    #[inline]
    fn value(&self) -> H::Decimal {
        H::EMPTY_DECIMAL
    }

    #[inline]
    fn source(&self) -> &H::HostString {
        H::EMPTY_HOST_STRING
    }

    #[inline]
    fn target(&self) -> &H::HostString {
        H::EMPTY_HOST_STRING
    }

    #[inline]
    fn has_unit(&self) -> MeasurementUnit {
        MeasurementUnit::default()
    }
}

// ── ThermoObservable<H> — supertrait of LandauerBudget ──
//
// `hardness_estimate` is the only method beyond Observable's surface;
// returns `EMPTY_DECIMAL` as the placeholder. A future Phase 12-style
// primitive (e.g. complexity-class -> nats) can replace this.
impl<T, Phase, H> ThermoObservable<H> for Validated<T, Phase>
where
    T: ConstrainedTypeShape,
    Phase: ValidationPhase,
    H: HostTypes,
{
    #[inline]
    fn hardness_estimate(&self) -> H::Decimal {
        H::EMPTY_DECIMAL
    }
}

// ── LandauerBudget<H> — primitive-backed via descent metrics ──
//
// The Landauer cost in nats equals `entropy_nats` returned by
// `primitive_descent_metrics`, which itself derives from the constraint
// nerve's Betti tuple. Phase 9 returns the value as IEEE-754 bits in
// `u64`; Phase 11 lifts back into `H::Decimal` via
// `DecimalTranscendental::from_bits`. Failure paths fall through to
// `EMPTY_DECIMAL` rather than panicking — the blanket impl is meant
// to be a quality-enhanced default, not a verification gate.
impl<T, Phase, H> LandauerBudget<H> for Validated<T, Phase>
where
    T: ConstrainedTypeShape,
    Phase: ValidationPhase,
    H: HostTypes,
{
    #[inline]
    fn landauer_nats(&self) -> H::Decimal {
        let nerve = match crate::enforcement::primitive_simplicial_nerve_betti::<T>() {
            Ok(b) => b,
            Err(_) => return H::EMPTY_DECIMAL,
        };
        let (_residual, entropy_bits) = crate::enforcement::primitive_descent_metrics::<T>(&nerve);
        <H::Decimal as DecimalTranscendental>::from_bits(entropy_bits)
    }
}

// ── JacobianObservable<H> — Observable marker ──
//
// The per-site Jacobian row is computed by
// `primitive_curvature_jacobian::<T>()`; the leaf trait carries no
// extra method beyond Observable's `value`, so the impl is the
// marker form. The Jacobian primitive is reached through inherent
// methods on shapes that need it.
impl<T, Phase, H> JacobianObservable<H> for Validated<T, Phase>
where
    T: ConstrainedTypeShape,
    Phase: ValidationPhase,
    H: HostTypes,
{
}

// ── CarryDepthObservable<H> — Observable marker ──
impl<T, Phase, H> CarryDepthObservable<H> for Validated<T, Phase>
where
    T: ConstrainedTypeShape,
    Phase: ValidationPhase,
    H: HostTypes,
{
}

// ── DerivationDepthObservable<H> — Observable marker ──
impl<T, Phase, H> DerivationDepthObservable<H> for Validated<T, Phase>
where
    T: ConstrainedTypeShape,
    Phase: ValidationPhase,
    H: HostTypes,
{
}

// ── FreeRankObservable<H> — Observable marker ──
impl<T, Phase, H> FreeRankObservable<H> for Validated<T, Phase>
where
    T: ConstrainedTypeShape,
    Phase: ValidationPhase,
    H: HostTypes,
{
}
