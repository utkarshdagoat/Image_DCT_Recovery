use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    // Retrieve command-line arguments
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        eprintln!("Usage: {} <input_file>", arguments[0]);
        std::process::exit(1);
    }

    let input_file = &arguments[1];

    // Decode the raw file 
    let raw_image = match rawloader::decode_file(input_file) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("Failed to decode raw file: {}", err);
            std::process::exit(1);
        }
    };

    // Print image metadata for inspection
    println!("Camera make: {}", raw_image.make);
    println!("Camera model: {}", raw_image.model);
    println!("Normalized make: {}", raw_image.clean_make);
    println!("Normalized model: {}", raw_image.clean_model);
    println!("Image dimensions: {}x{}", raw_image.width, raw_image.height);
    println!("Color planes: {}", raw_image.cpp);
    println!("White balance coefficients: {:?}", raw_image.wb_coeffs);
    println!("White levels: {:?}", raw_image.whitelevels);
    println!("Black levels: {:?}", raw_image.blacklevels);
    println!(
        "Conversion matrix (XYZ to Camera): {:?}",
        raw_image.xyz_to_cam
    );
    println!("Crop regions: {:?}", raw_image.crops);
    println!("Black areas: {:?}", raw_image.blackareas);
    println!("Image orientation: {:?}", raw_image.orientation);

    // Create an output file for grayscale image
    let ppm_filename = format!("{}.ppm", input_file);
    let output_file = match File::create(&ppm_filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create output file: {}", err);
            std::process::exit(1);
        }
    };
    let mut writer = BufWriter::new(output_file);

    // Write PPM file header
    let header = format!("P6\n{} {}\n65535\n", raw_image.width, raw_image.height);
    writer
        .write_all(header.as_bytes())
        .expect("Failed to write PPM header");

    // Process raw image data (only integer data supported)
    if let rawloader::RawImageData::Integer(pixel_data) = raw_image.data {
        for pixel_value in pixel_data {
            // Simplistic grayscale mapping: map all color channels to the same intensity
            let high_byte = (pixel_value >> 8) as u8;
            let low_byte = (pixel_value & 0xFF) as u8;
            let grayscale_pixel = [
                high_byte, low_byte, high_byte, low_byte, high_byte, low_byte,
            ];
            writer
                .write_all(&grayscale_pixel)
                .expect("Failed to write pixel data");
        }
    } else {
        eprintln!("Unsupported raw image data format");
    }
}
