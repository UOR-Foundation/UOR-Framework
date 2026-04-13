//! v0.2.1 example: query the carrier non-emptiness of a constrained type.
//!
//! Reproduces the inhabitance verdict one-liner from the v0.2.1 docx:
//!
//! ```rust,ignore
//! let cert: Validated<InhabitanceCertificate> =
//!     InhabitanceResolver::new().certify(&shape)?;
//! let witness: Option<&[u8]> = cert.witness();
//! ```
//!
//! Run with: `cargo run --example inhabitance_query -p uor-foundation`

use uor_foundation::enforcement::{
    Certify, InhabitanceCertificate, InhabitanceImpossibilityWitness, InhabitanceResolver,
    Validated,
};
use uor_foundation_macros::ConstrainedType;

/// Query shape: residue + hamming constraints classify as residual-vacuous,
/// so the pipeline certifies the carrier is non-empty.
#[derive(ConstrainedType, Default)]
#[uor(residue = 255, hamming = 8)]
struct QueryShape;

fn main() {
    let shape = QueryShape;
    let result: Result<Validated<InhabitanceCertificate>, InhabitanceImpossibilityWitness> =
        InhabitanceResolver::new().certify(&shape);

    match result {
        Ok(cert) => {
            // cert auto-derefs to InhabitanceCertificate; witness() returns
            // the underlying ValueTuple bytes when verified is true.
            match cert.witness() {
                Some(bytes) => println!("inhabited: witness = {bytes:?}"),
                None => println!("inhabited (verified true, witness elided)"),
            }
        }
        Err(_) => {
            println!(
                "InhabitanceResolver returned an impossibility witness — \
                 this shape has an empty carrier."
            );
        }
    }
}
