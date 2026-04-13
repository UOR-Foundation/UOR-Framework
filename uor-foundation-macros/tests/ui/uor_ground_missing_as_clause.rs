//! Missing `as Grounded<T>` clause should fail with a helpful error.
use uor_foundation_macros::{uor_ground, ConstrainedType};

#[derive(ConstrainedType, Default)]
#[uor(residue = 255, hamming = 8)]
struct Shape;

fn main() {
    let _ = uor_ground! {
        compile_unit foo {
            root_term: { 0 };
            witt_level_ceiling: W8;
            thermodynamic_budget: 64.0;
            target_domains: { ComposedAlgebraic };
        }
    };
}
