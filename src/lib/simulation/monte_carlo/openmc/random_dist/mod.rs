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

pub fn watt_spectrum(a: f64, b: f64, seed: &mut u64) -> Result<f64, TehOError>{

    let w: f64 =  maxwell_spectrum(a, seed)?;
    let value = w + 0.25 * a * a * b +
        uniform_distribution(-1.0, 1.0, seed)? * (a * a * b * w).sqrt();

    return Ok(value);
}

pub fn normal_variate(mean: f64, standard_deviation: f64, seed: &mut u64)
    -> Result<f64, TehOError>{

        // in openmc, there is a do while loop
        // which means the code block executes once, before checking 
        // the condition
        // https://www.w3schools.com/cpp/cpp_do_while_loop.asp
        //
        // Rust does not have the same, but i can program the equivalent
        let mut x: f64 = uniform_distribution(-1., 1., seed)?;
        let mut y: f64= uniform_distribution(-1., 1., seed)?;
        let mut r2: f64 = x * x + y * y;

        while r2 > 1.0 || r2 == 0.0 {
            x = uniform_distribution(-1., 1., seed)?;
            y = uniform_distribution(-1., 1., seed)?;
            r2 = x * x + y * y;
        };

        let z: f64 = (-2.0 * r2.log(E) / r2).sqrt();

        return Ok(mean + standard_deviation * z * x);

        

    }
