use std::error::Error;

use image::{imageops, DynamicImage, ImageBuffer, Luma};
use nalgebra::{DMatrix, Dyn};
use rand::{seq::SliceRandom, thread_rng};
use smartcore::{
    linalg::basic::matrix::DenseMatrix,
    linear::lasso::{Lasso, LassoParameters},
};

use crate::dct::dct_matrix;
/// Converts an image to blocks it returns
/// i
pub fn to_blocks(image: &[u8]) -> Vec<Box<[f64; 64]>> {
    if image.len() != (512 * 512) {
        panic!("invalid image size");
    }
    let mut vec = Vec::new();
    for i in (0..(512 * 512)).step_by(64) {
        let mut temp_vec = vec![0f64; 64];
        for j in 0..64 {
            temp_vec[j] = image[i + j] as f64;
        }
        let sized_block: Box<[f64; 64]> = temp_vec.try_into().unwrap();
        vec.push(sized_block);
    }
    vec
}

/// Corrupts a particular block
/// @param
///     block - 8x8 block of image to corrupt
///     samples - number of samples to keep in the blocs
/// @ret
///     corrupted_block  with samples gone
pub fn corrupt_block(block: &Box<[f64; 64]>, samples: usize) -> Box<[f64; 64]> {
    let mut indxs: Vec<usize> = (0..64).collect();
    indxs.shuffle(&mut thread_rng());
    let mut corrupted = block.clone();
    for &idx in &indxs[0..(64 - samples)] {
        corrupted[idx] = 0.0;
    }
    corrupted
}

pub fn blocks_to_image(blocks: &[Box<[f64; 64]>]) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut raw: Vec<u8> = vec![];
    for block in blocks {
        let mut temp_vec = vec![];
        for &ele in block.iter() {
            temp_vec.push(ele.clamp(0.0, 255.0) as u8);
        }
        raw.extend_from_slice(&temp_vec);
    }
    ImageBuffer::from_raw(512, 512, raw).unwrap()
}

pub fn logspace(start: f64, stop: f64, num: usize, base: f64) -> Vec<f64> {
    // Handle edge cases
    if num == 0 {
        return Vec::new();
    }

    // Calculate step size
    let step = (stop - start) / ((num - 1) as f64);

    // Generate logarithmically spaced values
    (0..num)
        .map(|i| {
            let exp = start + (i as f64) * step;
            base.powf(exp)
        })
        .collect()
}

pub fn mse(predicted: &Vec<f64>, actual: &Vec<f64>) -> f64 {
    predicted
        .iter()
        .zip(actual)
        .map(|(p, a)| (p - a).powi(2))
        .sum::<f64>()
        / predicted.len() as f64
}

/// recovering an block
/// @params
///     block: is the block to recover
///     s: number pixels known to us in the image
///     lambda: lasso model parameter
///     t: 2D 8x8 DCT Matrix
/// @returns
///     recovered_block
pub fn recover_block(
    block: &Box<[f64; 64]>,
    s: usize,
    lambda: f64,
    t: &DMatrix<f64>,
) -> Box<[f64; 64]> {
    // The code below simulates  a situation where some indices are lost and some are known the indices known are s in number
    // are selected randomaly
    let mut indices: Vec<usize> = (0..64).collect();
    let mut rng = thread_rng();
    indices.shuffle(&mut rng); // random shuffling of indices

    let (known_indices, unknown_indices) = indices.split_at(s);

    // as we are calculating Cosine = T * block
    // the data we know from the t_matrix and we predict the block coefficients
    let t_training_data: Vec<Vec<f64>> = known_indices
        .iter()
        .map(|&i| t.row(i).iter().cloned().collect())
        .collect();
    let t_trainng_data_matrix = DenseMatrix::from_2d_vec(&t_training_data).unwrap();

    let block_training_data: Vec<f64> = known_indices.iter().map(|&i| block[i]).collect();

    // LASSO model
    let params = LassoParameters::default()
        .with_alpha(lambda)
        .with_max_iter(1000)
        .with_tol(1e-4);
    let model = match Lasso::fit(&t_trainng_data_matrix, &block_training_data, params) {
        Ok(m) => m,
        Err(_) => return block.clone(),
    };

    // predict unknown pixels
    let t_data_predict: Vec<Vec<f64>> = unknown_indices
        .iter()
        .map(|&i| t.row(i).iter().cloned().collect())
        .collect();
    let t_data_predict_mat = DenseMatrix::from_2d_vec(&t_data_predict).unwrap();

    let predictions = match model.predict(&t_data_predict_mat) {
        Ok(p) => p,
        Err(_) => return block.clone(),
    }; // actual predictions
    let mut result = block.clone(); // the recovered block is the same block but with prediction values at unkwown indxes
    for (i, &indx) in unknown_indices.iter().enumerate() {
        result[indx] = predictions[i].clamp(0.0, 255.0); // clamp the pixel value
    }
    result
}


// simulats the corrupted image , recovered image given the grayscale image
pub fn simulate(image:&DynamicImage, id: String) -> Result<(),Box<dyn Error>>{
    let grayscale_image = image
        .resize(512, 512, imageops::FilterType::Gaussian)
        .grayscale();
    grayscale_image.save(format!("images/grayscale-{}.jpg",id))?;
    println!("{:?}", grayscale_image.color());
    let flat_samples = grayscale_image.as_flat_samples_u8().take().unwrap();
    let bytes = flat_samples.as_slice().to_vec();

    let blocks = to_blocks(&bytes); // 4096 blocks
    let corrupted_blocks: Vec<Box<[f64; 64]>> = blocks
        .iter()
        .map(|block| corrupt_block(block, 50))
        .collect();
    let corrupted_image = blocks_to_image(&corrupted_blocks);
    corrupted_image.save(format!("images/corrupted_image-{}.png",id))?;

    let s = 50;
    let lambda = 0.01;

    let t = dct_matrix();

    // let cv_folds = 10;
    // let s = 50;

    let recovered_blocks: Vec<Box<[f64; 64]>> = blocks
        .iter()
        .map(|block| recover_block(block, s, lambda, &t))
        .collect();
    let recovered_image = blocks_to_image(&recovered_blocks);
    recovered_image.save(format!("images/recovered_image-{}.png",id))?;
    Ok(())
}