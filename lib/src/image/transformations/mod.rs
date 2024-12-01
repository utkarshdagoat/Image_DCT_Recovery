use serde::{Deserialize, Serialize};

use crate::{image::image::PixelFormat, normalize};
mod utils;
use super::image::{self, Image};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Transformation {
    Grayscale,
    // Resize(u32, u32),         // resize to width, height
    Crop(u32, u32, u32, u32), // crop from x0, y0 to x1, y1,
    Transpose,
}

// convert an image to grayscale preserving the alpha channel
// returns the rgb image for the same
pub fn grayscale(image_buffer: &[u8], pixel_format: PixelFormat) -> (Vec<u8>, PixelFormat) {
    let mut new_buff = Vec::new();
    let format = match pixel_format {
        PixelFormat::RGB => {
            // performance optimization it is better to make vectors with the exact size
            // than to push elements into the vector as it make reallocations
            for chunks in image_buffer.chunks(3) {
                let (r, g, b) = (
                    normalize!(chunks[0]),
                    normalize!(chunks[1]),
                    normalize!(chunks[2]),
                );
                let grayscale = 0.299 * r + 0.587 * g + 0.114 * b;
                new_buff.push((grayscale.round() * 255f32) as u8);
            }
            PixelFormat::Grayscale // return the pixel format
        }
        PixelFormat::RGBA => {
            for chunks in image_buffer.chunks(4) {
                let (r, g, b) = (
                    normalize!(chunks[0]),
                    normalize!(chunks[1]),
                    normalize!(chunks[2]),
                );
                let grayscale = 0.299 * r + 0.587 * g + 0.114 * b;
                new_buff.push((grayscale.round() * 255f32) as u8);
                new_buff.push(chunks[3]);
            }
            PixelFormat::GrayscaleA // return the pixel format
        }
        PixelFormat::Grayscale => {
            new_buff.extend_from_slice(image_buffer);
            PixelFormat::Grayscale
        }
        PixelFormat::GrayscaleA => {
            new_buff.extend_from_slice(image_buffer);
            PixelFormat::GrayscaleA
        }
        PixelFormat::YCbCr => {
            // remove only the color components
            for chunks in image_buffer.chunks(3) {
                new_buff.push(chunks[0]);
            }
            PixelFormat::Grayscale
        }
    };
    (new_buff, format)
}

pub fn crop(
    image_buffer: &[u8],
    pixel_format: PixelFormat,
    x0: u32,
    y0: u32,
    x1: u32,
    y1: u32,
) -> Vec<u8> {
    let mut new_buff = Vec::new();
    let pixel_size = match pixel_format {
        PixelFormat::Grayscale => 1,
        PixelFormat::GrayscaleA => 2,
        PixelFormat::RGB => 3,
        PixelFormat::RGBA => 4,
        PixelFormat::YCbCr => 3,
        _ => 0,
    };
    for y in y0..y1 {
        for x in x0..x1 {
            let index = (y * x1 + x) as usize * pixel_size;
            new_buff.extend_from_slice(&image_buffer[index..index + pixel_size]);
        }
    }
    new_buff
}

pub fn transpose(image_buffer: &[u8]) -> Vec<u8> {
    let mut new_buff = Vec::new();
    new_buff.extend_from_slice(image_buffer);
    new_buff
}

#[test]
pub fn test_grayscale() {
    let image = Image::read_image("assets/strips.png").unwrap();
    let (new_buff, format) = grayscale(&image.buffer, image.color_type);
    let new_image = Image::raw_image(new_buff, image.width, image.height, format);
    new_image
        .write_image("assets/output-grayscale.png")
        .unwrap();
}

#[test]
pub fn test_crop() {
    let image = Image::read_image("assets/image.png").unwrap();
    let new_buff = crop(&image.buffer, image.color_type, 20, 120, 120, 220);
    let new_image = Image::raw_image(new_buff, 100, 100, image.color_type);
    new_image.write_image("assets/output-crop.png").unwrap();
}

#[test]
pub fn test_transpose() {
    let image = Image::read_image("assets/strips.png").unwrap();
    let new_buff = transpose(&image.buffer);
    let new_image = Image::raw_image(new_buff, image.height, image.width, image.color_type);
    new_image
        .write_image("assets/output-transpose.png")
        .unwrap();
}
