use std::{f32::consts::E, fs::File};

use anyhow::{Ok, Result};
use png::ColorType;
use serde::{Deserialize, Serialize};

use super::transformations::{crop, grayscale, transpose, Transformation};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PixelFormat {
    RGB = 2,        // normal rgb image
    YCbCr = 5,      // for jpeg  images
    Grayscale = 0,  // grayscale images
    RGBA = 6,       // rgb images with alpha channel
    GrayscaleA = 4, // grayscale images with alpha channel
}
// to store the image it can be stored in row major or column major format
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetaData {}

/// A struct to hold the image it contains information about rows and columns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub buffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub color_type: PixelFormat,
    pub metadata: MetaData, // Hashmap for meta data which are key,
    pub prev_proof: Option<String>,
}

impl Image {
    pub fn raw_image(buffer: Vec<u8>, width: u32, height: u32, color_type: PixelFormat) -> Self {
        Self {
            buffer,
            width,
            height,
            color_type,
            metadata: MetaData {},
            prev_proof: None,
        }
    }

    pub fn apply_transformation(&self, transformation: Transformation) -> Image {
        match transformation {
            Transformation::Grayscale => {
                let (new_buff, format) = grayscale(&self.buffer, self.color_type);
                Image::raw_image(new_buff, self.width, self.height, format)
            }
            Transformation::Crop(x0, y0, x1, y1) => {
                let new_buff = crop(&self.buffer, self.color_type, x0, y0, x1, y1);
                Image::raw_image(new_buff, x1 - x0, y1 - y0, self.color_type)
            }
            Transformation::Transpose => {
                let new_buff = transpose(&self.buffer);
                Image::raw_image(new_buff, self.height, self.width, self.color_type)
            }
        }
    }

    pub fn read_image(file_path: &str) -> Result<Image> {
        // decode a png image using png.rs crate https://docs.rs/png/latest/png/
        let decoder = png::Decoder::new(File::open(file_path).unwrap());
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;
        let (buffer, format) = match info.color_type {
            ColorType::Rgb => (buf, PixelFormat::RGB),
            ColorType::Rgba => (buf, PixelFormat::RGBA),
            ColorType::Grayscale => (buf, PixelFormat::Grayscale),
            ColorType::GrayscaleAlpha => (buf, PixelFormat::GrayscaleA),
            _ => unreachable!("uncovered color type"),
        };
        let proof_header = reader
            .info()
            .utf8_text
            .iter()
            .find(|x| x.keyword == "transformation_proof");
        let prev_proof = proof_header.map(|x| x.get_text().unwrap());
        Ok(Image {
            buffer,
            width: info.width,
            height: info.height,
            color_type: format,
            metadata: MetaData {},
            prev_proof,
        })
    }

    pub fn write_image(&self, file_path: &str) -> Result<()> {
        let file = File::create(file_path).unwrap();
        let w = std::io::BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(match self.color_type {
            PixelFormat::RGB => ColorType::Rgb,
            PixelFormat::RGBA => ColorType::Rgba,
            PixelFormat::Grayscale => ColorType::Grayscale,
            PixelFormat::GrayscaleA => ColorType::GrayscaleAlpha,
            _ => unreachable!("uncovered color type"),
        });
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&self.buffer).unwrap();
        Ok(())
    }

    pub fn write_image_with_proof(&self, file_path: &str, proof_string: String) -> Result<()> {
        let file = File::create(file_path).unwrap();
        let w = std::io::BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(match self.color_type {
            PixelFormat::RGB => ColorType::Rgb,
            PixelFormat::RGBA => ColorType::Rgba,
            PixelFormat::Grayscale => ColorType::Grayscale,
            PixelFormat::GrayscaleA => ColorType::GrayscaleAlpha,
            _ => unreachable!("uncovered color type"),
        });
        encoder.set_depth(png::BitDepth::Eight);
        encoder.add_itxt_chunk(String::from("transformation_proof"), proof_string)?;
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.buffer)?;
        Ok(())
    }
}
