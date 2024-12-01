//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use std::fs::File;
use std::io::{BufWriter, Write};

use alloy_sol_types::SolType;
use clap::Parser;
use serde::Serialize;
use sp1_sdk::{include_elf, ProverClient, SP1Prover, SP1Stdin};
use veriphoto_lib::image::image::Image;
use veriphoto_lib::image::transformations::Transformation;
/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const VERIPHOTO_ELF: &[u8] = include_elf!("veriphoto-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The path to the image file.
    #[clap(long, default_value = "assets/strips.png")]
    file_path: String,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    // Setup the prover client.
    let client = ProverClient::mock();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();

    let file_path = args.file_path;
    let image = Image::read_image(&file_path).unwrap();
    stdin.write(&image);
    stdin.write(&vec![Transformation::Crop(0, 0, 10, 10)]);
    let execution_stdin = stdin.clone();
    let (val, report) = client
        .execute(VERIPHOTO_ELF, execution_stdin)
        .run()
        .unwrap();
    println!(
        "Execution completed successfully with instruction count: {:?}",
        report.total_instruction_count()
    );

    let (pk, vk) = client.setup(VERIPHOTO_ELF);
    let proof = client.prove(&pk, stdin).run().expect("Proving failed");
    println!("Proof generated successfully");

    let proof_json = serde_json::to_string(&proof).unwrap();
    let vk_json = serde_json::to_string(&vk).unwrap();
    let proof_string = format!("{}:{}", proof_json, vk_json);
    write_file("proof_crop.json", proof_json.as_bytes());

    let ouput_image = image.apply_transformation(Transformation::Crop(0, 0, 10, 10));
    ouput_image
        .write_image_with_proof("assets/crop_with_proof.png", proof_string)
        .unwrap();
}

fn write_file(file_path: &str, buffer: &[u8]) {
    let mut file = File::create(file_path).unwrap();
    file.write_all(buffer).unwrap();
}
