//! Invalid Witt level name in #[uor_grounded] attribute should fail
//! type-check because `WittLevel::W99` is not a generated constant.
use uor_foundation_macros::uor_grounded;

#[uor_grounded(level = "W99")]
fn invalid() -> u8 {
    42
}

fn main() {
    let _ = invalid();
}
