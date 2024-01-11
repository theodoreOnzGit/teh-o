use std::ops::Deref;

use crate::teh_o_error::TehOError;

/// pseudo random number generator
///
///
/// https://github.com/openmc-dev/openmc/blob/develop/docs/source/methods/random_numbers.rst
///
///
/// OpenMC uses a future seeding algorithm using the following literature
/// to skip N random numbers ahead:
/// @article{brown1994random,
///  title={Random number generation with arbitrary strides},
///  author={Brown, Forrest B},
///  journal={Transactions of the American Nuclear Society},
///  volume={71},
///  number={CONF-941102-},
///  year={1994}
///}

static PRN_MULT: u64 = 6364136223846793005;
static PRN_ADD: u64 = 1442695040888963407;
static PRN_STRIDE: u64 = 152917;

pub fn prn(seed: &mut u64) -> Result<f64,TehOError> {

    // advance LCG
    *seed = PRN_MULT * seed.deref() + PRN_ADD;

    // permutate the output,
    // OpenMC usses some bit shifting magic here, so i don't know what's 
    // really going on, I'll just use it.
    //
    // Apparently the hat operator ^ is a bitwise exclusive or 
    // https://learn.microsoft.com/en-us/cpp/cpp/bitwise-exclusive-or-operator-hat?view=msvc-170
    //
    // It also means a bitwise exclusive or in Rust, so thankfully, it's 
    // mostly a copy/paste
    // https://doc.rust-lang.org/book/appendix-02-operators.html

    let word: u64 = 
        ((*seed  >> ((*seed >> 59 as u64) + 5 as u64)) ^ *seed) * 12605985483714917081 as u64;

    let result: u64 = (word >> 43 as u64) ^ word ;

    // the ldexp (load exponent, is called)
    // it multiplies a floating point value arg by the number 2 raised to the exp power.
    // https://en.cppreference.com/w/c/numeric/math/ldexp

    return ldexp(result as f64, -64);
}

/// it multiplies a floating point value arg by the number 2 raised to the exp power.
pub fn ldexp(arg: f64, int: i64) -> Result<f64, TehOError>{
    let value: f64 = arg * (2.0 as f64).powf(int as f64);

    return Ok(value);
}

/// The algorithm here to determine the parameters used to skip ahead is
/// described in F. Brown, "Random Number Generation with Arbitrary Stride,"
/// Trans. Am. Nucl. Soc. (Nov. 1994). This algorithm is able to skip ahead in
/// O(log2(N)) operations instead of O(N). Basically, it computes parameters G
/// and C which can then be used to find x_N = G*x_0 + C mod 2^M.
///
/// Still need to test
pub fn future_seed(n: u64, seed: u64) -> Result<u64, TehOError>{

    // initialise constants
    let mut g = PRN_MULT;
    let mut c = PRN_ADD;
    let mut g_new: u64 = 1;
    let mut c_new: u64 = 0;

    // assign a local mutable n 
    let mut local_n = n;
    
    while local_n > 0 {
        //Check if least significant bit is 1 

        if (local_n & 1) > 0 {
            g_new *= g;
            c_new = c_new * g + c;
        }

        c *= g + 1;
        g *= g;

        // Move bits right, dropping least significant bit 

        local_n >>= 1;

    };

    return Ok(g_new * seed + c_new);

}
