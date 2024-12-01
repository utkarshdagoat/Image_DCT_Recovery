// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use sp1_sdk::{SP1Proof, SP1ProofWithPublicValues, SP1VerifyingKey};
use veriphoto_lib::image::image::Image;
use veriphoto_lib::image::transformations::Transformation;

pub fn main() {
    let img = sp1_zkvm::io::read::<Image>();
    let transformations = sp1_zkvm::io::read::<Vec<Transformation>>();
    if let Some(prev_proof) = &img.prev_proof {
        let (proof, vk) = prev_proof.split_once(';').unwrap();
        let proof: SP1ProofWithPublicValues = serde_json::from_str(proof).unwrap();
        let vk: SP1VerifyingKey = serde_json::from_str(vk).unwrap();
        // sp1_zkvm::lib::syscall_verify_sp1_proof(vk_digest, pv_digest);
        //Todo: verify proof
    }

    let mut transformed_image: Image = img.clone();
    for transformation in transformations {
        transformed_image = img.apply_transformation(transformation);
    }
    let height_buf = transformed_image.height.to_le_bytes();
    let width_buf = transformed_image.width.to_le_bytes();
    let pixel_format_buf = (transformed_image.color_type as u32).to_le_bytes();

    sp1_zkvm::io::write(
        transformed_image.buffer.len() as u32,
        &transformed_image.buffer[..],
    );
    sp1_zkvm::io::write(4, &height_buf);
    sp1_zkvm::io::write(4, &width_buf);
    sp1_zkvm::io::write(4, &pixel_format_buf);
}
