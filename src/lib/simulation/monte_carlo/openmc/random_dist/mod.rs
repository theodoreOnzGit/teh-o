use std::f64::consts::E;
use std::f64::consts::PI;

use crate::teh_o_error::TehOError;

use super::random_lcg::prn;

pub fn uniform_distribution(a: f64, b: f64, seed: &mut u64) -> Result<f64,TehOError>{

    let value = a + (b - a) * prn(seed)?;

    return Ok(value);

}


#[allow(non_snake_case)]
pub fn maxwell_spectrum(T: f64, seed: &mut u64) -> Result<f64, TehOError> {

    // Set the random numbers 
    let r1: f64 = prn(seed)?;
    let r2: f64 = prn(seed)?;
    let r3: f64 = prn(seed)?;

    // determine cosine of pi/2 * r 
    let c: f64 = (PI/2.0 * r3).cos();

    // determine outgoing energy 
    // in openmc, std::log is used
    // Which in this documentation
    // https://cplusplus.com/reference/cmath/log/
    // is the natural logarithm

    let value = -T * (r1.log(E) + r2.log(E) * c * c);

    return Ok(value);

}
