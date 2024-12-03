/// contains code for comuting the dct matri
use core::f64;

use nalgebra::DMatrix;

// alpha of u in dct
pub fn alpha(u: usize) -> f64 {
    if u == 0 {
        let num = 1f64 / 8f64; //sqrt(1/n)
        num.sqrt()
    } else {
        let num = 1f64 / 4f64; // sqrt(2/N)
        num.sqrt()
    }
}

pub fn dct_coff(x: usize, u: usize) -> f64 {
    let cos_inside = (2f64 * (x as f64) + (1f64)) * (u as f64) * (f64::consts::FRAC_PI_8 / 2f64);

    alpha(u) * (cos_inside.cos())
}

pub fn dct_matrix() -> DMatrix<f64> {
    let mut tranformation = DMatrix::zeros(8*8, 8*8);
    for y in 0..8 {
        for x in 0..8 {
            for u in 0..8 {
                for v in 0..8 {
                    tranformation[(y*8 + x, u*8 + v)] = dct_coff(x, u) * dct_coff(y, v);
                }
            }
        }
    }
    for indx in 0..tranformation.nrows(){
        tranformation[(indx,0)] = 1.0;
    }
    tranformation 
}
