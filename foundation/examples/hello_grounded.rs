//! v0.2.1 example: minimal `uor_ground!` producing a `Grounded<T>`.
//!
//! The smallest consumer-facing snippet that exercises the full pipeline:
//! a user struct with `#[derive(ConstrainedType)]`, an `uor_ground!`
//! invocation with a trailing `as Grounded<T>` clause, and the resulting
//! typed grounded value.
//!
//! Run with: `cargo run --example hello_grounded -p uor-foundation`

use uor_foundation::enforcement::Grounded;
use uor_foundation_macros::{uor_ground, ConstrainedType};

#[derive(ConstrainedType, Default)]
#[uor(residue = 255, hamming = 8)]
struct Pixel;

fn main() {
    let unit: Grounded<Pixel> = uor_ground! {
        compile_unit hello_pixel {
            root_term: { 0 };
            witt_level_ceiling: W8;
            thermodynamic_budget: 64.0;
            target_domains: { ComposedAlgebraic };
        } as Grounded<Pixel>
    };
    println!(
        "hello, grounded! witt_level_bits={} unit_address=0x{:032x}",
        unit.witt_level_bits(),
        unit.unit_address()
    );
}
