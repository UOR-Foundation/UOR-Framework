//! v0.2.1 example: query the Witt level a contraction shape can represent.
//!
//! Reproduces the consumer-facing one-liner from the v0.2.1 ergonomics spec:
//!
//! > "At what Witt level does this K-fold MAC admit representation?" has a
//! > one-line answer:
//! >
//! > ```rust,ignore
//! > let cert: Validated<LiftChainCertificate> =
//! >     TowerCompletenessResolver::new().certify(&contraction_shape)?;
//! > let accum_level: WittLevel = cert.target_level();
//! > ```
//!
//! Run with: `cargo run --example witt_level_query -p uor-foundation`

use uor_foundation::enforcement::{
    Certify, GenericImpossibilityWitness, LiftChainCertificate, TowerCompletenessResolver,
    Validated,
};
use uor_foundation::WittLevel;
use uor_foundation_macros::ConstrainedType;

/// K-fold MAC contraction shape: 255-residue + 8-bit Hamming (one site per
/// accumulator column). The real consumer crate would write a richer
/// attribute set; v0.2.1 accepts this minimal form.
#[derive(ConstrainedType, Default)]
#[uor(residue = 255, hamming = 8)]
struct ContractionShape;

fn main() {
    let contraction_shape = ContractionShape;
    let result: Result<Validated<LiftChainCertificate>, GenericImpossibilityWitness> =
        TowerCompletenessResolver::new().certify(&contraction_shape);

    match result {
        Ok(cert) => {
            let accum_level: WittLevel = cert.target_level();
            println!(
                "K-fold MAC admits representation at W{}",
                accum_level.witt_length()
            );
        }
        Err(_witness) => {
            println!(
                "TowerCompletenessResolver returned an impossibility witness for \
                 the contraction shape."
            );
        }
    }
}
