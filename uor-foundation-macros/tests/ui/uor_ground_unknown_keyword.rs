//! Unknown declaration keyword should fail to parse.
use uor_foundation_macros::uor_ground;

fn main() {
    let _ = uor_ground! {
        unknown_shape foo {
            root_term: { 0 };
        } as u32
    };
}
